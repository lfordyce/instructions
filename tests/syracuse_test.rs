use derive_more::From;
use futures::future::BoxFuture;
use num_traits::{identities::Zero, PrimInt};
use serde::{Deserialize, Serialize};
use std::fmt::Display;
use std::io;
use std::ops::{Add, Div, Mul, Rem};
use thiserror::Error;

#[derive(Copy, Clone)]
struct Big {
    things: [u64; 100],
}

impl Add<Big> for Big {
    type Output = Big;

    #[inline(never)]
    fn add(self, rhs: Big) -> Big {
        let mut c = Big { things: [0; 100] };
        for i in 0..100 {
            c.things[i] = self.things[i] + rhs.things[i];
        }
        c
    }
}

fn syracuse<T>(n: T) -> T
where
    T: Copy
        + Eq
        + Add<Output = T>
        + Mul<Output = T>
        + Div<Output = T>
        + Rem<Output = T>
        + From<u32>,
{
    let zero: T = 0.into();
    match n % 2.into() == zero {
        true => n / (2.into()),
        false => n * (3.into()) + 1.into(),
    }
}

pub trait Fizzy {
    fn fizzy(&self) -> String;
}

impl<T> Fizzy for T
where
    T: PrimInt + std::fmt::Display,
{
    fn fizzy(&self) -> String {
        let zero = T::zero();
        let three = T::from(3).unwrap();
        let five = T::from(5).unwrap();
        match (*self % three, *self % five) {
            (x, y) if x == zero && y == zero => String::from("FizzBuzz"),
            (x, _) if x == zero => String::from("Fizz"),
            (_, y) if y == zero => String::from("Buzz"),
            _ => format!("{}", self),
        }
    }
}

trait Shared {
    fn f(&self) -> usize;
}

struct A;
impl Shared for A {
    fn f(&self) -> usize {
        1
    }
}

struct B;
impl Shared for B {
    fn f(&self) -> usize {
        2
    }
}

enum E {
    VarA(A),
    VarB(B),
}

impl Shared for E {
    fn f(&self) -> usize {
        match self {
            E::VarA(x) => x.f(),
            E::VarB(x) => x.f(),
        }
    }
}

#[derive(Debug)]
pub struct UpdateWithCx<R, Upd> {
    pub requester: R,
    pub update: Upd,
}

pub trait UpdateWithCxRequesterType {
    type Requester;
}

impl<R, Upd> UpdateWithCxRequesterType for UpdateWithCx<R, Upd> {
    type Requester = R;
}

impl<R> UpdateWithCx<R, Message>
where
    R: Requester,
{
    pub async fn answer<T>(&self, text: T)
    where
        T: Into<String> + Display,
    {
        println!("ANSWER FOUND: {}", text);
    }
}

#[derive(Debug, Error)]
pub enum RequestError {
    #[error("An I/O error: {0}")]
    Io(#[source] io::Error),
}

/// A type returned from a FSM (sub)transition function.
pub type TransitionOut<D, E = crate::RequestError> = Result<DialogueStage<D>, E>;

pub trait SubtransitionOutputType {
    type Output;
    type Error;
}

impl<D, E> SubtransitionOutputType for TransitionOut<D, E> {
    type Output = D;
    type Error = E;
}

/// An input passed into a FSM (sub)transition function.
pub type TransitionIn<R> = UpdateWithCx<R, Message>;

#[derive(Debug, Copy, Clone, Eq, Hash, PartialEq)]
pub enum DialogueStage<D> {
    Next(D),
    Exit,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct Message {
    /// Unique message identifier inside this chat.
    #[serde(rename = "message_id")]
    pub id: i32,

    /// Date the message was sent in Unix time.
    pub date: i32,
}

impl Message {
    pub fn text(&self) -> Option<&str> {
        Some("23")
        // None
    }
}

pub fn next<Dialogue, State, E>(new_state: State) -> TransitionOut<Dialogue, E>
where
    Dialogue: From<State>,
{
    Ok(DialogueStage::Next(Dialogue::from(new_state)))
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct StartState;

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct HaveNumberState {
    pub number: i32,
}

impl From<HaveNumberState> for Dialogue {
    fn from(original: HaveNumberState) -> Self {
        Self::HaveNumber(original)
    }
}

impl From<StartState> for Dialogue {
    fn from(original: StartState) -> Self {
        Self::Start(original)
    }
}

// #[derive(Clone, From, Debug)]
#[derive(Clone, Debug)]
pub enum Dialogue {
    Start(StartState),
    HaveNumber(HaveNumberState),
}

impl Default for Dialogue {
    fn default() -> Self {
        Self::Start(StartState)
    }
}

// impl From<Dialogue> for Dialogue {
//     fn from(d: Dialogue) -> Self {
//         match d {
//             Dialogue::Start(s) => Self::Start(s),
//             Dialogue::ReceiveFullName(r) => Self::ReceiveFullName(r),
//         }
//     }
// }

/// Represents a transition function of a dialogue FSM.
pub trait Transition: Sized {
    type Aux;
    type Error;
    type Requester;

    /// Turns itself into another state, depending on the input message.
    ///
    /// `aux` will be passed to each subtransition function.
    fn react(
        self,
        cx: TransitionIn<Self::Requester>,
        aux: Self::Aux,
    ) -> BoxFuture<'static, TransitionOut<Self, Self::Error>>;
}

pub trait Subtransition
where
    Self::Dialogue: Transition<Aux = Self::Aux>,
{
    type Aux;
    type Dialogue;
    type Error;
    type Requester;

    /// Turns itself into another state, depending on the input message.
    ///
    /// `aux` is something that is provided by the call side, for example,
    /// message's text.
    fn react(
        self,
        cx: TransitionIn<Self::Requester>,
        aux: Self::Aux,
    ) -> BoxFuture<'static, TransitionOut<Self::Dialogue, Self::Error>>;
}

// impl teloxide::dispatching::dialogue::Transition for Dialogue {
//     type Aux = <StartState as teloxide::dispatching::dialogue::Subtransition>::Aux;
//     type Error = <StartState as teloxide::dispatching::dialogue::Subtransition>::Error;
//     type Requester = <StartState as teloxide::dispatching::dialogue::Subtransition>::Requester;
//     fn react(self, cx: teloxide::dispatching::dialogue::TransitionIn<Self::Requester>, aux: Self::Aux) -> futures::future::BoxFuture<'static, teloxide::dispatching::dialogue::TransitionOut<Self, Self::Error>> {
//         futures::future::FutureExt::boxed(async move {
//             match self {
//                 Dialogue::Start(state) => teloxide::dispatching::dialogue::Subtransition::react(state, cx, aux).await,
//                 Dialogue::ReceiveFullName(state) => teloxide::dispatching::dialogue::Subtransition::react(state, cx, aux).await,
//                 Dialogue::ReceiveAge(state) => teloxide::dispatching::dialogue::Subtransition::react(state, cx, aux).await,
//                 Dialogue::ReceiveLocation(state) => teloxide::dispatching::dialogue::Subtransition::react(state, cx, aux).await,
//             }
//         })
//     }
// }

impl Transition for Dialogue {
    type Aux = <StartState as Subtransition>::Aux;
    type Error = <StartState as Subtransition>::Error;
    type Requester = <StartState as Subtransition>::Requester;

    fn react(
        self,
        cx: TransitionIn<Self::Requester>,
        aux: Self::Aux,
    ) -> BoxFuture<'static, TransitionOut<Self, Self::Error>> {
        Box::pin(async move {
            match self {
                Dialogue::Start(state) => Subtransition::react(state, cx, aux).await,
                Dialogue::HaveNumber(state) => Subtransition::react(state, cx, aux).await,
            }
        })
    }
}

pub trait Requester {}

#[derive(Default)]
pub struct Bot {}

impl Requester for Bot {}

impl Subtransition for StartState {
    type Aux = String;
    type Dialogue = <TransitionOut<Dialogue> as SubtransitionOutputType>::Output;
    type Error = <TransitionOut<Dialogue> as SubtransitionOutputType>::Error;
    type Requester = <TransitionIn<Bot> as UpdateWithCxRequesterType>::Requester;

    fn react(
        self,
        cx: TransitionIn<Self::Requester>,
        aux: Self::Aux,
    ) -> BoxFuture<'static, TransitionOut<Self::Dialogue, Self::Error>> {
        async fn start(
            state: StartState,
            cx: TransitionIn<Bot>,
            ans: String,
        ) -> TransitionOut<Dialogue> {
            if let Ok(number) = ans.parse() {
                cx.answer(format!(
                    "Remembered number {}. Now use /get or /reset",
                    number
                ))
                .await;
                next(HaveNumberState { number })
            } else {
                cx.answer("Please, send me a number").await;
                next(state)
            }
        }
        futures::future::FutureExt::boxed(start(self, cx, aux))
    }
}

impl Subtransition for HaveNumberState {
    type Aux = String;
    type Dialogue = <TransitionOut<Dialogue> as SubtransitionOutputType>::Output;
    type Error = <TransitionOut<Dialogue> as SubtransitionOutputType>::Error;
    type Requester = <TransitionIn<Bot> as UpdateWithCxRequesterType>::Requester;

    fn react(
        self,
        cx: TransitionIn<Self::Requester>,
        aux: Self::Aux,
    ) -> BoxFuture<'static, TransitionOut<Self::Dialogue, Self::Error>> {
        async fn have_number(
            state: HaveNumberState,
            cx: TransitionIn<Bot>,
            ans: String,
        ) -> TransitionOut<Dialogue> {
            let num = state.number;

            if ans.starts_with("/get") {
                cx.answer(format!("Here is your number: {}", num)).await;
                next(state)
            } else if ans.starts_with("/reset") {
                cx.answer("Resetted number").await;
                next(StartState)
            } else {
                cx.answer("Please, send /get or /reset").await;
                next(state)
            }
        }
        futures::future::FutureExt::boxed(have_number(self, cx, aux))
    }
}

async fn handle_message(
    cx: UpdateWithCx<Bot, Message>,
    dialogue: Dialogue,
) -> TransitionOut<Dialogue> {
    match cx.update.text().map(ToOwned::to_owned) {
        None => {
            cx.answer("Send me a text message.").await;
            next(dialogue)
        }
        Some(ans) => dialogue.react(cx, ans).await,
    }
}

#[derive(Debug)]
pub struct DialogueWithCx<R, Upd, D, E> {
    pub cx: UpdateWithCx<R, Upd>,
    pub dialogue: Result<D, E>,
}

impl<Upd, R, D, E> DialogueWithCx<R, Upd, D, E> {
    /// Creates a new instance with the provided fields.
    pub fn new(cx: UpdateWithCx<R, Upd>, dialogue: D) -> Self {
        Self {
            cx,
            dialogue: Ok(dialogue),
        }
    }
}

type In = DialogueWithCx<Bot, Message, Dialogue, RequestError>;

#[test]
fn syracuse_test() {
    let input: Vec<(u128, u128)> = vec![(4, 2), (71, 214)];
    input
        .into_iter()
        .for_each(|(input, output)| assert_eq!(syracuse(input), output));
}

#[test]
fn fizzy_test() {
    for x in 1..=100 {
        println!("{}", x.fizzy())
    }
}

#[test]
fn test_trait_enum() {
    let val = E::VarA(A);

    println!("{}", val.f());
}

#[test]
fn test_transition() {
    // let dialoge = Dialogue;

    let a = HaveNumberState { number: 35 };
    let dialogue = Dialogue::from(a);
    println!("{:?}", dialogue);

    // let result: TransitionOut<Dialogue> = next(HaveNumberState { number: 35 });
    let result: TransitionOut<Dialogue> = next(Dialogue::default());
    println!("{:?}", result);
}

#[test]
fn test_add_thing() {
    let a = Big { things: [1; 100] };
    let b = Big { things: [2; 100] };
    let c = a + b;

    println!("{} + {} = {}", a.things[0], b.things[0], c.things[0])
}

#[tokio::test]
async fn test_transitions() {
    let d: In = DialogueWithCx::new(
        UpdateWithCx {
            requester: Bot::default(),
            update: Message::default(),
        },
        Dialogue::default(),
    );

    match handle_message(d.cx, d.dialogue.unwrap()).await.unwrap() {
        DialogueStage::Next(new_dialogue) => {
            println!("{:?}", new_dialogue);
        }
        DialogueStage::Exit => {
            println!("Dialogue finished, exiting...")
        }
    }
}
