use std::future::Future;
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
//     {
//         let read = self.read().await; // RwLock Read handle
//         f(&read.toolset.drill).await
//     }
// }
