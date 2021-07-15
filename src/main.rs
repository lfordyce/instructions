use async_trait::async_trait;
use std::fmt;
use std::fmt::Debug;
use std::sync::Arc;
use tokio::runtime;
use tokio::spawn;
use tokio::sync::broadcast;
use tokio::sync::Mutex;
use tokio::task::JoinHandle;

#[async_trait]
pub trait Machine<T>: Send + Sync + 'static {
    async fn disconnected(&self);
    async fn recv(&self, cmd: T);
}

pub trait InstructionSet: Clone {
    type InstructionSet;
}

fn send_cmd<T: Send + Sync + 'static>(sender: broadcast::Sender<T>, cmd: T) {
    if sender.send(cmd).is_err() {}
}

#[derive(Debug, Clone)]
pub enum TestMessage {
    /// TestData has a single parameter, as a tuple
    TestData(usize),
    /// AddSender can be implemented to push a sender onto a list of senders
    AddSender(TestMessageSender),
    /// Notify, is setup for notification via TestData, where usize is a message count.
    Notify(TestMessageSender, usize),
}

/// TestMessageSender is shorthand for a sender of a TestMessage instruction.
pub type TestMessageSender = broadcast::Sender<TestMessage>;

impl InstructionSet for TestMessage {
    type InstructionSet = TestMessage;
}

#[derive(Default)]
struct Forwarder {
    /// a id, mostly used for logging
    id: usize,
    /// The mutable bits...
    mutable: Mutex<ForwarderMutable>,
}

/// This is a mutable part of the Forwarder
pub struct ForwarderMutable {
    /// collection of senders, each will be sent any received message.
    senders: Vec<TestMessageSender>,
    /// received_count is the count of messages received by this forwarder.
    received_count: usize,
    /// send_count is the count of messages sent by this forward.
    send_count: usize,
    /// notify_count is compared against receiver_count for means of notification.
    notify_count: usize,
    /// notify_sender is sent a TestData message with the data being the number of messages received.
    notify_sender: Option<TestMessageSender>,
    /// forwarding multiplier,
    forwarding_multiplier: usize,
    /// for TestData, this is the next in sequence
    next_seq: usize,
}

impl Default for ForwarderMutable {
    fn default() -> Self {
        Self::new()
    }
}

impl ForwarderMutable {
    fn new() -> Self {
        ForwarderMutable {
            senders: Vec::<TestMessageSender>::new(),
            received_count: 0,
            send_count: 0,
            notify_count: 0,
            notify_sender: None,
            forwarding_multiplier: 1,
            next_seq: 0,
        }
    }

    fn drop_all_senders(&mut self) {
        self.senders.clear();
        self.notify_sender = None;
    }

    /// if msg is TestData, validate the sequence or reset if 0
    fn validate_sequence(&mut self, msg: TestMessage) -> Result<TestMessage, TestMessage> {
        match msg {
            TestMessage::TestData(seq) if seq == self.next_seq => self.next_seq += 1,
            TestMessage::TestData(seq) if seq == 0 => self.next_seq = 1,
            TestMessage::TestData(_) => return Err(msg),
            _ => (),
        }
        // bump receiver count
        self.received_count += 1;
        Ok(msg)
    }

    /// If msg is a configuration msg, handle it otherwise return it as an error
    fn handle_config(&mut self, msg: TestMessage, id: usize) -> Result<(), TestMessage> {
        match msg {
            TestMessage::Notify(sender, on_receive_count) => {
                println!("forwarder {}: added notifier", id);
                self.notify_sender = Some(sender);
                self.notify_count = on_receive_count;
            }
            TestMessage::AddSender(sender) => {
                println!("forwarder {}: added sender", id);
                self.senders.push(sender);
            }
            msg => return Err(msg),
        }
        Ok(())
    }

    /// handle the action messages
    fn handle_action(&mut self, message: TestMessage, id: usize) {
        match message {
            TestMessage::TestData(_) => {
                println!("forwarder {}: received TestData", id);
                for sender in &self.senders {
                    for _ in 0..self.forwarding_multiplier {
                        send_cmd(sender.clone(), TestMessage::TestData(self.send_count));
                        self.send_count += 1;
                    }
                }
            }
            _ => self.senders.iter().for_each(|sender| {
                for _ in 0..self.forwarding_multiplier {
                    send_cmd(sender.clone(), message.clone());
                }
            }),
        }
    }

    /// handle sending out a notification and resetting counters when notificaiton is sent
    fn handle_notification(&mut self, id: usize) {
        if self.received_count == self.notify_count {
            let count = self.clear_received_count();
            if let Some(notifier) = self.notify_sender.as_ref() {
                println!("forwarder {}: sending notification", id);
                send_cmd(notifier.clone(), TestMessage::TestData(count));
            }
        }
    }

    /// get the current received count and clear counters
    fn clear_received_count(&mut self) -> usize {
        let received_count = self.received_count;
        self.received_count = 0;
        self.send_count = 0;
        received_count
    }
}

impl Forwarder {
    pub fn new(id: usize) -> Self {
        Self {
            id,
            ..Default::default()
        }
    }
    pub const fn get_id(&self) -> usize {
        self.id
    }
}

#[async_trait]
impl Machine<TestMessage> for Forwarder {
    async fn disconnected(&self) {
        println!("forwarder {}: disconnected", self.get_id());
        // drop senders
        let mut mutable = self.mutable.lock().await;
        mutable.drop_all_senders();
    }

    async fn recv(&self, message: TestMessage) {
        let mut mutable = self.mutable.lock().await;
        match mutable.handle_config(message, self.get_id()) {
            Ok(_) => (),
            Err(msg) => match mutable.validate_sequence(msg) {
                Ok(msg) => {
                    mutable.handle_action(msg, self.get_id());
                    mutable.handle_notification(self.get_id());
                }
                Err(msg) => panic!("sequence error fwd {}, msg {:#?}", self.get_id(), msg),
            },
        }
    }
}

struct Adapter<T: InstructionSet<InstructionSet = T>> {
    sender: broadcast::Sender<T>,
    pub receiver: broadcast::Receiver<T>,
    pub machine: Arc<dyn Machine<<T as InstructionSet>::InstructionSet>>,
}

impl<T: InstructionSet<InstructionSet = T>> Clone for Adapter<T> {
    fn clone(&self) -> Self {
        Self {
            sender: self.sender.clone(),
            receiver: self.sender.subscribe(),
            machine: self.machine.clone(),
        }
    }
}

fn build_bounded<T, U>(raw: U, capacity: usize) -> (Arc<U>, broadcast::Sender<T>, Adapter<T>)
where
    U: Machine<T> + Send + Sync + 'static,
    T: InstructionSet<InstructionSet = T>,
{
    let instance = Arc::new(raw);
    let (sender, receiver) = broadcast::channel::<T>(capacity);
    let cloned_sender = sender.clone();
    let machine = instance.clone() as Arc<dyn Machine<T>>;
    let adapter = Adapter::<T> {
        sender,
        receiver,
        machine,
    };
    (instance, cloned_sender, adapter)
}

#[derive(Debug, Clone, PartialEq)]
pub struct Error {
    message: String,
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> Result<(), fmt::Error> {
        write!(f, "{}", self.message)
    }
}

fn spawn_task<T>(mut a: Adapter<T>) -> JoinHandle<()>
where
    T: InstructionSet<InstructionSet = T> + Send + 'static,
{
    spawn(async move {
        loop {
            let cmd = a.receiver.recv().await;
            if cmd.is_ok() {
                a.machine.recv(cmd.unwrap()).await;
            } else {
                break;
            }
        }
        a.machine.disconnected().await;
    })
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let rt = runtime::Runtime::new()?;

    let mut tasks = Vec::new();
    let mut first = None;
    let mut prev = None;
    rt.block_on(async {
        // setup a daisy-chain of 5 machines
        for id in 1..=5 {
            // create sender and adapter wrapping forwarder
            let (_instance, s, a) = build_bounded(Forwarder::new(id), 100);
            // create a task to run the recv loop -- consider using stream
            // why can't this be refactored into a fn that returns task?
            let task = spawn_task(a);

            // save the task
            tasks.push(task);

            if prev.is_none() {
                // first time save the sender
                first = Some(s.clone());
            } else {
                // tell previous sender to send to this sender
                send_cmd(prev.unwrap(), TestMessage::AddSender(s.clone()));
            }
            prev = Some(s);
        }
        // create notifier and tell the last to send to it
        let (s, mut r) = broadcast::channel::<TestMessage>(10);
        send_cmd(prev.unwrap(), TestMessage::Notify(s, 1));
        // send to the first
        send_cmd(first.unwrap(), TestMessage::TestData(0));
        // wait for the notification
        if let Ok(msg) = r.recv().await {
            println!("got notification with msg: {:?}", msg);
        }
        println!("done");
    });
    Ok(())
}
