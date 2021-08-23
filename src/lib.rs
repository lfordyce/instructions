use async_trait::async_trait;
use std::collections::HashMap;
use std::pin::Pin;
use std::sync::Arc;
use std::time::SystemTime;
use tokio::macros::support::Future;

pub mod abstractions;
pub mod diy_stream;
pub mod factory;
pub mod hrtb_generic;
pub mod line_writer;
pub mod patterns;
pub mod plugin;
pub mod ray_tracing;

#[async_trait]
pub trait AsyncEvaluator {
    async fn evaluate(self, reducer: Arc<String>);
}

#[async_trait]
pub trait Watcher<T>: Send + Sync + 'static {
    type Err;
    async fn notify(&self, msg: T, token: u64, time: SystemTime) -> Result<(), Self::Err>
    where
        T: Send + 'static;
}

#[async_trait]
pub trait Binder<E> {
    type Steam: futures::Stream<Item = Result<Self::RW, E>>;
    type RW: tokio::io::AsyncRead + tokio::io::AsyncWrite;

    async fn bind(&self) -> Self::Steam;
}

pub struct SomeType;

#[async_trait]
impl AsyncEvaluator for &mut SomeType {
    async fn evaluate(self, reducer: Arc<String>) {
        todo!()
    }
}

pub fn spawn_evaluation<'b, T>(t: T, reducer: Arc<String>) -> tokio::task::JoinHandle<()>
where
    T: std::marker::Send + 'static,
    for<'a> &'a mut T: AsyncEvaluator + std::marker::Send,
{
    tokio::task::spawn(async move {
        let mut evaluator = t;
        let reference = &mut evaluator;
        reference.evaluate(reducer).await;
    })
}

pub trait AsyncFnMutArg<'a, P: 'a, T> {
    type Fut: Future<Output = T> + 'a;
    fn call(self, arg: &'a mut P) -> Self::Fut;
}

impl<'a, P: 'a, Fut, F> AsyncFnMutArg<'a, P, Fut::Output> for F
where
    F: FnOnce(&'a mut P) -> Fut,
    Fut: Future + 'a,
{
    type Fut = Fut;

    fn call(self, arg: &'a mut P) -> Self::Fut {
        self(arg)
    }
}

// impl<'a, P: 'a, Fut: Future + 'a, F: FnOnce(&'a mut P) -> Fut> AsyncFnMutArg<'a, P, Fut::Output>
//     for F
// {
//     type Fut = Fut;
//     fn call(self, arg: &'a mut P) -> Self::Fut {
//         self(arg)
//     }
// }

pub async fn wrapper<F>(f: F) -> i32
where
    F: for<'a> AsyncFnMutArg<'a, i32, i32>,
{
    let mut i = 41;
    f.call(&mut i).await
}

pub async fn add_one(i: &mut i32) -> i32 {
    // *i = *i + 1;
    *i + 1
}

pub async fn add_ten(i: &mut i32) -> i32 {
    *i + 10
}

pub trait AsyncFn<'a>: Send + Sync + 'static {
    fn f(&'a self, n: &'a mut u8) -> Pin<Box<dyn Future<Output = u8> + Send + 'a>>;
}

impl<'a, T, F> AsyncFn<'a> for T
where
    T: Fn(&'a mut u8) -> F + Send + Sync + 'static,
    F: Future<Output = u8> + Send + 'a,
{
    fn f(&'a self, n: &'a mut u8) -> Pin<Box<dyn Future<Output = u8> + Send + 'a>> {
        Box::pin((self)(n))
    }
}

pub trait System: Send + Sync + 'static {
    fn call<'a>(&'a self, n: &'a mut u8) -> Pin<Box<dyn Future<Output = u8> + Send + 'a>>;
}

impl<T> System for T
where
    T: for<'r> AsyncFn<'r>,
{
    fn call<'a>(&'a self, n: &'a mut u8) -> Pin<Box<dyn Future<Output = u8> + Send + 'a>> {
        self.f(n)
    }
}

pub async fn a(n: &mut u8) -> u8 {
    println!("executing a");
    *n + 1
}

pub async fn b(n: &mut u8) -> u8 {
    println!("executing b");
    *n = *n + 1;
    *n + 2
}

pub trait BoolTrait<'f> {
    fn check(&'f self, value: &'f i32) -> Pin<Box<dyn Future<Output = bool> + 'f>>;
}

impl<'f, F, Fut> BoolTrait<'f> for F
where
    F: Fn(&'f i32) -> Fut,
    Fut: Future<Output = bool> + 'f,
{
    fn check(&'f self, value: &'f i32) -> Pin<Box<dyn Future<Output = bool> + 'f>> {
        Box::pin(self(value))
    }
}

pub trait CheckerSystem {
    fn call<'a>(&'a self, n: &'a i32) -> Pin<Box<dyn Future<Output = bool> + 'a>>;
}

impl<T> CheckerSystem for T
where
    T: for<'r> BoolTrait<'r>,
{
    fn call<'a>(&'a self, n: &'a i32) -> Pin<Box<dyn Future<Output = bool> + 'a>> {
        self.check(n)
    }
}

async fn test<BT>(bt: BT)
where
    BT: for<'a> BoolTrait<'a>,
{
    let v = 42;
    bt.check(&v).await;
}

pub async fn check_me(v: &i32) -> bool {
    println!("checking...");
    *v == 42
}

pub struct Service;

// This inherent `async fn` implements `Handler<S, P, R>`.
impl Service {
    async fn my_handler(&self) -> Result<(), ()> {
        Ok(())
    }
}

// But this async trait method does not (replace the above code with the below).
//
// #[async_trait]
// trait MyHandlers {
//     async fn my_handler(&self) -> Result<(), ()>;
// }
//
// #[async_trait]
// impl MyHandlers for Service {
//     async fn my_handler(&self) -> Result<(), ()> {
//         Ok(())
//     }
// }

// -= Definitions =-

pub struct Request;
pub struct Response;

type BoxedHandler<S> =
    Box<dyn Fn(Arc<S>, Request) -> Pin<Box<dyn Future<Output = Response> + Send>>>;

pub struct Router<S> {
    state: Arc<S>,
    handlers: HashMap<String, BoxedHandler<S>>,
}

impl<S> Router<S>
where
    S: Send + Sync + 'static,
{
    pub fn new(state: S) -> Self {
        Router {
            state: Arc::new(state),
            handlers: HashMap::new(),
        }
    }

    pub fn define_method<H, P, R>(&mut self, method: impl Into<String>, handler: H)
    where
        P: FromRequest,
        H: for<'a> Handler<'a, S, P, R> + Send + Sync + 'static,
    {
        let handler = into_boxed_handler(handler);
        self.handlers.insert(method.into(), handler);
    }
}

fn into_boxed_handler<S, P, R, H>(handler: H) -> BoxedHandler<S>
where
    S: Send + Sync + 'static,
    P: FromRequest,
    H: for<'a> Handler<'a, S, P, R> + Send + Sync + 'static,
{
    let handler = Arc::new(handler);

    let inner = move |state: Arc<S>, request: Request| -> Pin<Box<dyn Future<Output = _> + Send>> {
        let handler = handler.clone();
        Box::pin(async move {
            let params = P::from_request(request); // Omitted: Convert Request into tuple of P.
            let _response = handler.call(&*state, params).await; // Omitted: Convert into Response
            Response
        })
    };

    Box::new(inner)
}

pub trait Handler<'a, S, P, R> {
    type ResponseFuture: Future<Output = R> + Send;

    fn call(&self, state: &'a S, params: P) -> Self::ResponseFuture;
}

impl<'a, S, R, I, F> Handler<'a, S, (), R> for F
where
    S: Send + Sync + 'static,
    I: Future<Output = R> + Send,
    F: Fn(&'a S) -> I + Sync,
{
    type ResponseFuture = I;

    fn call(&self, state: &'a S, _: ()) -> Self::ResponseFuture {
        (self)(state)
    }
}

impl<'a, S, P, R, I, F> Handler<'a, S, (P,), R> for F
where
    S: Send + Sync + 'static,
    P: FromRequest,
    I: Future<Output = R> + Send,
    P: Send + 'static,
    F: Fn(&'a S, P) -> I + Sync,
{
    type ResponseFuture = I;

    fn call(&self, state: &'a S, params: (P,)) -> Self::ResponseFuture {
        (self)(state, params.0)
    }
}

pub trait FromRequest: Send {
    fn from_request(request: Request) -> Self;
}

impl FromRequest for () {
    fn from_request(_: Request) -> Self {
        ()
    }
}

impl<P: Send> FromRequest for (P,) {
    fn from_request(_: Request) -> Self {
        unimplemented!()
    }
}

//
// pub trait Handler<'a, T, R> {
//     type ResponseFuture: Future<Output = R> + 'a;
//     fn call(&self, server: &'a T) -> Self::ResponseFuture;
// }
// impl<'a, T, R, F, Fut> Handler<'a, T, R> for F
// where
//     T: 'a,
//     R: 'static,
//     F: Fn(&'a T) -> Fut,
//     Fut: Future<Output = R> + 'a,
// {
//     type ResponseFuture = Fut;
//
//     fn call(&self, server: &'a T) -> Self::ResponseFuture {
//         (self)(server)
//     }
// }
// pub fn accept_async<'a, T: 'a, R, F>(callback: F)
// where
//     F: Handler<'a, T, R>,
// {
//     callback.call()
// }
