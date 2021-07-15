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
