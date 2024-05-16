use serde::{Deserialize, Serialize};
use std::future::Future;
use std::marker::PhantomData;
use std::pin::Pin;
use std::sync::{Arc, Mutex, MutexGuard};
use std::time::Instant;
use tokio::sync::mpsc::{channel, Receiver};

struct DynFutureWithLt<'max, 'min, O> {
    future: Pin<Box<dyn 'min + Future<Output = O>>>,
    phantom: PhantomData<&'min &'max ()>,
}

impl<'max, 'min, O> DynFutureWithLt<'max, 'min, O> {
    fn new(future: Pin<Box<dyn 'min + Future<Output = O>>>) -> Self {
        Self {
            future,
            phantom: PhantomData,
        }
    }
}

async fn call_closure<'max, C>(mut closure: C)
where
    C: for<'a> FnMut(&'a str) -> DynFutureWithLt<'max, 'a, ()>,
{
    let s = String::from("Hello World!");
    closure(&s).future.await;
}

trait Something {
    fn something(&self) {}
}

struct Executor<I> {
    inner: I,
}

impl<I> Executor<I>
where
    I: Something,
{
    pub async fn run<'a>(&'a self) {
        call_closure::<'a, _>(|s| {
            DynFutureWithLt::new(Box::pin(async move {
                self.inner.something();
                println!("{s}");
            }))
        })
        .await;
    }
}

pub trait WithPhantom<P: ?Sized> {}

impl<T: ?Sized, P: ?Sized> WithPhantom<P> for T {}

pub trait FutureWithPhantom<O, P: ?Sized>: Future<Output = O> + WithPhantom<P> {}

impl<O, P: ?Sized, T> FutureWithPhantom<O, P> for T where T: Future<Output = O> {}

async fn call_other_closure<'max, C>(mut closure: C)
where
    C: for<'a> FnMut(
        &'a str,
    ) -> Pin<Box<dyn 'a + FutureWithPhantom<(), PhantomData<&'a &'max ()>>>>,
{
    let s = String::from("Hello World!");
    closure(&s).await;
}

trait OtherSomething {
    fn something(&self) {}
}

struct OtherExecutor<I> {
    inner: I,
}

impl<I> OtherExecutor<I>
where
    I: OtherSomething,
{
    pub async fn run<'a>(&'a self) {
        call_other_closure::<'a, _>(|s| {
            Box::pin(async move {
                self.inner.something();
                println!("1st: {s}");
            })
        })
        .await;
        call_other_closure::<'a, _>(|s| {
            Box::pin(async move {
                self.inner.something();
                println!("2nd: {s}");
            })
        })
        .await;
    }
}

trait Getter<'a> {
    fn get<'b>(&'b self) -> &'a u8
    where
        'a: 'b;
}

impl<'a> Getter<'a> for &'a u8 {
    fn get<'b>(&'b self) -> &'a u8
    where
        'a: 'b,
    {
        *self
    }
}

trait Data {
    fn dump(&mut self); // this needs to be mutable by intention, as the code is highly reduced
}

struct MyData<'a> {
    val1: &'a mut u8,
    val2: u8,
}

impl<'a> Data for MyData<'a> {
    fn dump(&mut self) {
        println!("val = {}", self.val2);
    }
}

fn create_data<'a>(num: &'a mut u8) -> Box<dyn Data + 'a> {
    Box::new(MyData { val1: num, val2: 8 })
}

struct TimestampValue<V> {
    timestamp: Instant, // dummy type
    v: V,
}

impl<V> TimestampValue<V> {
    fn update(&mut self) {
        self.timestamp = Instant::now(); // dummy function
    }
    fn last_updated(&self) -> Instant {
        self.timestamp
    }
    fn get(&self) -> &V {
        &self.v
    }
    fn set(&mut self, v: V) -> V {
        self.update();
        std::mem::replace(&mut self.v, v)
    }
}

struct MyStruct {
    my_field: TimestampValue<i32>,
    my_other_field: TimestampValue<i32>,
}

impl MyStruct {
    fn my_field(&self) -> i32 {
        *self.my_field.get()
    }
    fn set_my_field(&mut self, my_field: i32) -> i32 {
        self.my_field.set(my_field)
    }
}

struct WorkflowProcess {
    rx: Receiver<String>,
}

#[derive(Deserialize, Serialize)]
struct Msg {
    version: i32,
}

impl WorkflowProcess {
    async fn process<T>(&mut self, callback: impl Fn(T))
    where
        for<'a> T: Deserialize<'a>,
    {
        let r = self.rx.recv().await;
        if let Some(v) = r {
            let deserialized: T = {
                let s: &str = v.as_str();
                serde_json::from_str(s).unwrap()
            };
            callback(deserialized);
        }
    }
}

// Some data that is not copyable (just dummy integer here)
// struct NonCopyableData {
//     data: u32,
// }
// impl NonCopyableData {
//     fn new(value: u32) -> Self {
//         Self { data: value }
//     }
// }
//
// pub fn lifetime_stuffs<'b: 'a, 'a>(values: Vec<Arc<Mutex<NonCopyableData>>>) {
//     // 'a is the (required) lifetime for items in `vec`
//     let mut vec: Vec<&'a u32> = Vec::new();
//
//     for value in &values {
//         // 'b is the lifetime of `locked`
//         let locked: MutexGuard<'b, _> = value.lock().unwrap();
//         // the lifetime of &locked.data is less than or equal to 'b, because we are borrowing from `locked`
//         vec.push(&locked.data);
//         // 'b ends here
//     }
//
//     // "Do the thing" with `vec`
//
//     // 'a ends here
// }

#[cfg(test)]
mod tests {
    use super::*;
    use std::sync::{Arc, Mutex, MutexGuard};

    #[test]
    fn test_lifetimes() {
        // Define somewhere in our app
        // let values = vec![
        //     Arc::new(Mutex::new(NonCopyableData::new(5))),
        //     Arc::new(Mutex::new(NonCopyableData::new(4))),
        //     Arc::new(Mutex::new(NonCopyableData::new(2))),
        //     Arc::new(Mutex::new(NonCopyableData::new(1))),
        // ];

        // ...
        // {
        //     // Function scope
        //     // Loop over it & gather values in a function
        //     // 'a is the (required) lifetime for items in `vec`
        //     let mut vec: Vec<&'a u32> = Vec::new();
        //
        //     // let mut vec = Vec::new();
        //     // for value in &values {
        //     //     let locked = value.lock().unwrap();
        //     //     vec.push(&locked.data);
        //     // }
        //     // Do something with vec before exiting function
        //     for value in &values {
        //         // 'b is the lifetime of `locked`
        //         let locked: MutexGuard<'b, _> = value.lock().unwrap();
        //         // the lifetime of &locked.data is less than or equal to 'b, because we are borrowing from `locked`
        //         vec.push(&locked.data);
        //         // 'b ends here
        //     }
        //
        //     // "Do the thing" with `vec`
        // }
        // lifetime_stuffs(values);
    }

    #[test]
    fn test_lifetime_trait_fun() {
        let x = 10u8;
        let x_ref = &x;

        {
            let r = x_ref.get();
            println!("{:?}", r);
        }
        {
            let r = x_ref.get();
            println!("{:?}", r);
        }
    }

    #[test]
    fn test_lifetime() {
        let mut a: u8 = 4;
        let mut x = create_data(&mut a);
        x.dump();
    }

    #[tokio::test]
    async fn test_executor_thing() {
        struct S;
        impl Something for S {}
        Executor { inner: S }.run().await;
    }

    #[tokio::test]
    async fn test_executor_thing_other() {
        struct S;
        impl OtherSomething for S {}
        OtherExecutor { inner: S }.run().await;
    }

    #[tokio::test]
    async fn test_workflow() {
        let (tx, rx) = channel::<String>(100);
        let mut workflow = WorkflowProcess { rx };
        let worker = tokio::spawn(async move {
            let x = |msg: Msg| {
                assert_eq!(msg.version, 32);
            };
            workflow.process(x).await;
        });
        let serialized = serde_json::to_string(&Msg { version: 32 }).unwrap();
        tx.send(serialized).await;
        worker.await;
    }
}
