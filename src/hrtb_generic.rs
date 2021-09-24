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

async fn connection_fn(conn: &mut Endpoint) -> Result<Channel, tonic::transport::Error> {
    conn.connect().await
}

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

#[cfg(test)]
mod tests {
    use super::*;

    // #[test]
    // fn basic_test() {
    //     let v: Vec<String> = vec!["gus".to_string(), "gus2".to_string()];
    //
    //     par_iter_with_setup(v, setup, |iter| ());
    // }

    #[tokio::test]
    async fn test_async_closure_thing() {
        with_async_closure(function_target).await;
    }
}
