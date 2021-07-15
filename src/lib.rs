use async_trait::async_trait;
use std::pin::Pin;
use std::sync::Arc;
use tokio::macros::support::Future;

pub mod diy_stream;
pub mod hrtb_generic;
pub mod line_writer;
pub mod plugin;
pub mod ray_tracing;

#[async_trait]
pub trait AsyncEvaluator {
    async fn evaluate(self, reducer: Arc<String>);
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
    *n = *n + 1;
    *n + 2
}
