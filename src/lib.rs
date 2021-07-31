use async_trait::async_trait;
use std::pin::Pin;
use std::sync::Arc;
use std::time::SystemTime;
use tokio::macros::support::Future;

pub mod abstractions;
pub mod diy_stream;
pub mod factory;
pub mod hrtb_generic;
pub mod line_writer;
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

impl<'a, P: 'a, Fut: Future + 'a, F: FnOnce(&'a mut P) -> Fut> AsyncFnMutArg<'a, P, Fut::Output>
    for F
{
    type Fut = Fut;
    fn call(self, arg: &'a mut P) -> Self::Fut {
        self(arg)
    }
}

pub async fn wrapper<F>(f: F)
where
    F: for<'a> AsyncFnMutArg<'a, i32, ()>,
{
    let mut i = 41;
    f.call(&mut i).await;
}

pub async fn add_one(i: &mut i32) {
    *i = *i + 1;
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
