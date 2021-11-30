use futures::channel::mpsc;
use futures::stream;
use futures::FutureExt;
use futures::{Sink, SinkExt, Stream, StreamExt};
use std::future::Future;
use std::io;
use std::pin::Pin;

pub struct X {
    chain: Vec<Box<dyn FnOnce(&mut X) -> Pin<Box<dyn Future<Output = ()> + '_>>>>,
}

pub trait XFnOnce<'a> {
    type Output: Future<Output = ()> + 'a;
    fn call_once(self, x: &'a mut X) -> Self::Output;
}

impl<'a, F, R> XFnOnce<'a> for F
where
    F: FnOnce(&'a mut X) -> R,
    R: Future<Output = ()> + 'a,
{
    type Output = R;
    fn call_once(self, x: &'a mut X) -> R {
        self(x)
    }
}

impl X {
    pub fn new() -> X {
        X { chain: Vec::new() }
    }

    pub fn add<F>(&mut self, f: F)
    where
        F: 'static + for<'a> XFnOnce<'a>,
    {
        self.chain.push(Box::new(|x| Box::pin(f.call_once(x))));
    }

    pub async fn run(&mut self) {
        while let Some(x) = self.chain.pop() {
            x(self).await;
        }
    }
}

pub async fn test(_: &mut X) {
    println!("b");
}

pub trait AnAsyncCallback<'a> {
    type Output: Future<Output = ()> + 'a;
    fn call(&self, arg: &'a mut usize) -> Self::Output;
}

impl<'a, Fut: 'a, F> AnAsyncCallback<'a> for F
where
    F: Fn(&'a mut usize) -> Fut,
    Fut: Future<Output = ()> + 'a,
{
    type Output = Fut;

    fn call(&self, arg: &'a mut usize) -> Self::Output {
        self(arg)
    }
}

// The trait can then be used to constrain a function's argument:
pub async fn take_async_callback<F>(async_fn: F)
where
    F: for<'a> AnAsyncCallback<'a>,
{
    let mut number = 0;
    async_fn.call(&mut number).await;
    async_fn.call(&mut number).await;
}

pub async fn add(arg: &mut usize) {
    *arg += 1;
}
pub fn add_explicit<'a>(number: &'a mut usize) -> (impl Future<Output = ()> + 'a) {
    async move {
        *number += 1;
    }
}

pub async fn doit<F>(f: F)
where
    for<'x> F: Fn(&'x mut i32) -> Pin<Box<dyn Future<Output = ()> + 'x>>,
    F: 'static,
{
    let mut v: i32 = 6;
    f(&mut v).await
}

pub struct Struct;

pub struct Read {
    pub toolset: Toolset,
}

pub struct Toolset {
    pub drill: Drill,
}

pub struct Drill;

// impl Struct {
//     async fn read(self: &'_ Self) -> &'_ Read {
//         &Read {
//             toolset: Toolset { drill: Drill },
//         }
//     }
//
//     pub async fn async_borrow_drill<C, Output>(self: &'_ Self, f: C) -> Output
//     where
//         for<'any> C: FnOnce<(&'any Drill,), Output = Pin<Box<dyn 'any + Future<Output = Output>>>>,
//         for<'any> Pin<Box<dyn 'any + Future<Output = Output>>>: Future<Output = Output>,
//         // for<'any>
//         //     <C as FnOnce<(&'any Drill,)>>::Output : Future<Output = Output>
//         // ,
//     {
//         let read = self.read().await; // RwLock Read handle
//         f(&read.toolset.drill).await
//     }
// }

pub trait Helper<'a, T> {
    type Output: Send + 'a;
    fn call(self, arg: &'a T) -> Self::Output;
}

impl<'a, D: 'a, F, T: 'a> Helper<'a, T> for F
where
    F: FnOnce(&'a T) -> D,
    D: Send,
{
    type Output = D;

    fn call(self, arg: &'a T) -> Self::Output {
        self(arg)
    }
}

// pub trait HelperSystem: Send + 'static {
//     fn call<'a>(&'a self, arg: &'a Vec<String>) -> Vec<&'a str>;
// }
//
// impl<S> HelperSystem for S
// where
//     S: for<'a> Helper<'a, Vec<String>>,
// {
//     fn call<'a>(&'a self, arg: &'a Vec<String>) -> Vec<&'a str> {
//         self.call(arg)
//     }
// }

// pub fn par_iter_with_setup<T, S, M, F>(t: T, setup: S, closure: F)
// where
//     T: Send + 'static,
//     M: Send,
//     // for<'a> S: Helper<'a, T, Output = M> + Send + 'static,
//     S: for<'a> Helper<'a, T, Output = M> + Send + 'static,
//     F: FnOnce(M) + Send + 'static,
// {
//     std::thread::spawn(move || {
//         let middle = setup.call(&t);
//         closure(middle);
//     });
// }

pub fn setup<'a>(v: &'a Vec<String>) -> Vec<&'a str> {
    v.iter().map(|s| s.as_str()).collect::<Vec<&str>>()
}

//idea here is AsyncClosure argument and return future have the same lifetime
trait AsyncClosure<'a, Argument, Output> {
    type Fut: Future<Output = Output> + 'a;
    fn call(self, arg: &'a Argument) -> Self::Fut;
}

//blanket impl
impl<'a, Fu: 'a, F, Argument: 'static, Output> AsyncClosure<'a, Argument, Output> for F
where
    F: FnOnce(&'a Argument) -> Fu,
    Fu: Future<Output = Output> + 'a,
{
    type Fut = Fu;
    fn call(self, rt: &'a Argument) -> Fu {
        self(rt)
    }
}

async fn with_async_closure<C, R>(c: C) -> R
where
    for<'a> C: AsyncClosure<'a, u8, R>,
{
    let a = 3; //closure borrows this
    c.call(&a) //returned future borrows it as well
        .await
    //no longer borrowed here
}

async fn function_target(arg: &u8) {
    println!("{:?}", arg);
}

// async fn connection_fn(conn: &mut Endpoint) -> Result<Channel, tonic::transport::Error> {
//     conn.connect().await
// }

/// Hide the returned future's type so that we can use HRTBs to specify lifetimes.
///
// let mut endpoint = Channel::from_static(&CONFIG.grpc_server.endpoint)
//     .keep_alive_timeout(Duration::from_millis(
//         CONFIG.grpc_server.keep_alive_timeout.milliseconds,
//     ))
//     .connect_timeout(Duration::from_millis(
//         CONFIG.grpc_server.connect_timeout.milliseconds,
//     ));
// let channel = retry(&mut endpoint, connection_fn).await;

// trait AsyncFnMut<A> {
//     type Output;
//     type Future: Future<Output = Self::Output>;
//
//     fn call(&mut self, args: A) -> Self::Future;
// }
//
// impl<Arg, F, Fut, R> AsyncFnMut<(Arg,)> for F
//     where
//         F: FnMut(Arg) -> Fut,
//         Fut: Future<Output = R>,
// {
//     type Output = R;
//     type Future = Fut;
//
//     fn call(&mut self, (arg1,): (Arg,)) -> Self::Future {
//         self(arg1)
//     }
// }
//
// async fn retry<F>(conn: &mut Endpoint, mut cb: F) -> Channel
//     where
//         F: for<'a> AsyncFnMut<(&'a mut Endpoint,), Output = Result<Channel, tonic::transport::Error>>,
// {
//     loop {
//         match cb.call((conn,)).await {
//             Err(e) => {
//                 tracing::error!("endpoint connection failure: {}", e);
//                 tokio::time::sleep(Duration::from_secs(1)).await;
//             }
//             Ok(channel) => return channel,
//         }
//     }
// }

trait WhatIWant<'a> {
    type Output;
    type Error: std::error::Error;
    type Future: Future<Output = Result<Self::Output, Self::Error>> + 'a;

    fn read(&'a mut self) -> Self::Future;
}

struct Connection;

impl Connection {
    async fn do_read(&mut self) -> Result<(), io::Error> {
        // reading from a underlaying stream (tcp stream)
        Ok(())
    }
}

impl<'a> WhatIWant<'a> for Connection {
    type Output = ();
    type Error = io::Error;
    type Future = Pin<Box<dyn Future<Output = Result<Self::Output, Self::Error>> + 'a>>;

    fn read(&'a mut self) -> Self::Future {
        Box::pin(self.do_read())
    }
}

// async fn use_trait<'a, C: 'a>(mut conn: C) -> Result<(), io::Error>
// where
//     C: WhatIWant<'a, Output = (), Error = io::Error>,
// {
//     if let Ok(_) = conn.read().await {
//         // do something
//     }
//
//     Ok(())
// }

trait DataSource<'a> {
    fn get_value(&'a self, index: usize) -> &'a str;
}

struct OddOrEven {
    odd: String,
    even: String,
}

impl<'a> DataSource<'a> for OddOrEven {
    fn get_value(&'a self, index: usize) -> &'a str {
        if index % 2 == 0 {
            self.even.as_str()
        } else {
            self.odd.as_str()
        }
    }
}

impl<'a, F: Fn(usize) -> &'a str> DataSource<'a> for F {
    fn get_value(&'a self, n: usize) -> &'a str {
        (self)(n)
    }
}

pub enum Arg {
    Predicate(Box<dyn Fn() -> bool>),
}

// usage: Arg::from(|| true);
impl<T: 'static + Fn() -> bool> From<T> for Arg {
    fn from(predicate: T) -> Self {
        Arg::Predicate(Box::new(predicate))
    }
}

trait Object {}

struct HolderBox<'lifetime> {
    objects: Vec<Box<dyn Object + 'lifetime>>,
}

impl<'lifetime> HolderBox<'lifetime> {
    fn add_object<T: Object + 'lifetime>(self: &'_ mut Self, object: T) {
        self.objects.push(Box::new(object));
    }
}

struct HolderNoBox<'lifetime> {
    objects: Vec<&'lifetime (dyn Object + 'lifetime)>,
}

impl<'lifetime> HolderNoBox<'lifetime> {
    fn add_object<T: Object + 'lifetime>(self: &'_ mut Self, object: &'lifetime T) {
        self.objects.push(object);
    }
}

struct DBConnection;
#[derive(Debug)]
struct DBError;

impl DBConnection {
    async fn query(&'_ mut self, key: &'_ str) -> Result<usize, DBError> {
        Ok(key.len())
    }
}

async fn transaction<T: 'static, F, C>(
    connection: &'_ mut DBConnection,
    ctx: C,
    fun: F,
) -> Result<T, DBError>
where
    F: for<'c> Fn(
        &'c mut DBConnection,
        &'c C,
    ) -> Pin<Box<dyn Future<Output = Result<Option<T>, DBError>> + 'c>>,
{
    // Retry the transaction function as long as it returns `None`
    loop {
        let response: Option<T> = fun(connection, &ctx).await?;
        match response {
            None => continue, // Retry the transaction
            Some(value) => {
                return Ok(value);
            }
        }
    }
}

async fn test_transaction() {
    let key = "database_key".to_string();
    // The lambda returns a Future which is tied to the lifetime of "key" and
    // "connection", but the output type of that Future is tied to neither
    let r = transaction(&mut DBConnection {}, &key, |connection, key| {
        // We want to move a reference to "key" into the Future, not "key" itself
        async move {
            let value = connection.query(key).await?;
            Ok(Some(value))
        }
        .boxed()
    })
    .await;

    let key_size = r.unwrap();
    assert_eq!(12, key_size)
}

async fn foo(
    v: Vec<(
        Pin<Box<dyn Sink<u8, Error = mpsc::SendError>>>,
        Pin<Box<dyn Stream<Item = u8>>>,
    )>,
) {
    for (mut tx, mut rx) in v {
        let _ = tx.send(0);
        let _ = rx.next().await;
    }
}

struct Services {
    s1: Box<dyn for<'a> FnOnce(&'a mut Vec<usize>) -> Pin<Box<dyn Future<Output = ()> + 'a>>>,
}

impl Services {
    fn new(
        f: Box<dyn for<'a> FnOnce(&'a mut Vec<usize>) -> Pin<Box<dyn Future<Output = ()> + 'a>>>,
    ) -> Self {
        Services { s1: f }
    }
}

struct Svc<F> {
    s1: F,
}

impl<F, G> Svc<F>
where
    F: FnOnce(Vec<usize>) -> G,
    G: Future<Output = Vec<usize>>,
{
    fn new(f: F) -> Self {
        Svc { s1: f }
    }
}
trait Workaround {
    type Fn: FnOnce(Vec<usize>) -> Self::Fut;
    type Fut: Future<Output = Vec<usize>>;
    fn fun(self) -> Self::Fn;
}

impl<F, G> Workaround for Svc<F>
where
    F: FnOnce(Vec<usize>) -> G,
    G: Future<Output = Vec<usize>>,
{
    type Fn = F;
    type Fut = G;

    fn fun(self) -> Self::Fn {
        self.s1
    }
}

#[derive(Clone, Copy)]
enum NumberOperation {
    AddOne,
    MinusOne,
}

fn service(op: NumberOperation) -> impl Workaround {
    Svc::new(move |mut numbers| async move {
        for n in &mut numbers {
            match op {
                NumberOperation::AddOne => *n = *n + 1,
                NumberOperation::MinusOne => *n = *n - 1,
            };
        }
        numbers
    })
}

struct Pager<T, F>
where
    F: Future<Output = T> + 'static,
{
    fetch: fn(i32) -> F,
}

impl<T, F> Pager<T, F>
where
    // F: Future<Output = T> + 'static, // with static the ('a) lifetime can be removed
    F: Future<Output = T>,
{
    fn stream<'a>(&'a self) -> Pin<Box<dyn Stream<Item = T> + 'a>> {
        let fetch = self.fetch;
        let stream = stream::unfold(0, move |state| async move {
            let yielded = fetch(state).await;
            let next_state = state + 1;
            Some((yielded, next_state))
        });
        Box::pin(stream)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    // #[test]
    // fn basic_test() {
    //     let v: Vec<String> = vec!["gus".to_string(), "gus2".to_string()];
    //
    //     par_iter_with_setup(v, setup, |iter| ());
    // }

    #[test]
    fn test_odd_or_even() {
        let oe = OddOrEven {
            odd: "a".into(),
            even: "b".into(),
        };

        dbg!(oe.get_value(0));
        dbg!(oe.get_value(1));
    }

    #[tokio::test]
    async fn test_async_closure_thing() {
        with_async_closure(function_target).await;
    }

    // #[tokio::test]
    // async fn test_async_trait_lifetime() {
    //     let con = Connection;
    //     use_trait(con)
    // }

    #[tokio::test]
    async fn test_closure_async_retry() {
        test_transaction().await;
    }

    // #[ignore]
    // #[tokio::test]
    // async fn test_associated_type_async_func_param() {
    //     let (tx, rx) = mpsc::channel(32);
    //     foo(vec![(Box::pin(tx), Box::pin(rx))]).await;
    // }

    // #[tokio::test]
    // async fn test_num_op_services() {
    //     let mut input = vec![1, 2, 3];
    //     let op = NumberOperation::AddOne;
    //
    //     let s = Services::new(Box::new(|numbers: &mut Vec<usize>| {
    //         Box::pin(async move {
    //             for n in numbers {
    //                 match op {
    //                     NumberOperation::AddOne => *n = *n + 1,
    //                     NumberOperation::MinusOne => *n = *n - 1,
    //                 };
    //             }
    //         })
    //     }));
    //
    //     (s.s1)(&mut input).await;
    //     assert_eq!(input, vec![2, 3, 4]);
    // }

    #[tokio::test]
    async fn test_num_op_services_alt() {
        let input = vec![1, 2, 3];

        let f = service(NumberOperation::AddOne).fun();

        let output = f(input).await;
        assert_eq!(output, vec![2, 3, 4]);
    }

    #[tokio::test]
    async fn stream_pager() {
        async fn fetch(page: i32) -> String {
            format!("page: {}", page)
        }

        let pages = Pager { fetch }
            .stream()
            .take(10)
            .collect::<Vec<String>>()
            .await;
        println!("Result: {:?}", pages);
    }
}
