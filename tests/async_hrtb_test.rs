use futures::channel::mpsc;
use futures::{sink::drain, stream, Sink, SinkExt, Stream, StreamExt};
use instructions::hrtb_generic::{test, X};
use instructions::line_writer::LineWriter;
use instructions::plugin::{ClientConn, ClientConnectContext, Plugin};
use instructions::{a, add_one, b, wrapper, System};
use std::convert::TryFrom;
use std::fmt::Display;

// fully qualified syntax for traits:
// `<T as TryFrom<&'a [u8]>>::Error` to say which instance we want to
// pick the error type from, proved that we know what `'a` is supposed to be.
//
// Higher-ranked trait bounds:
//
// You need to say <T as Trait>::AssocType to make it clear which trait the type is from
// even if it should be obvious from the context.
pub struct Msg<T>
where
    T: for<'a> TryFrom<&'a [u8]>,
    for<'a> <T as TryFrom<&'a [u8]>>::Error: Display,
{
    pub payload: Box<T>,
}

struct Adder<T>(std::marker::PhantomData<T>);

impl<T> Adder<T> {
    fn add<A, B>(a: A, b: B) -> T
    where
        T: std::ops::Add<Output = T>,
        A: Into<T>,
        B: Into<T>,
    {
        return a.into() + b.into();
    }
}

pub struct Container<'slice> {
    pub data: &'slice [u8],
}

impl<'slice> Container<'slice> {
    pub fn new(data: &[u8]) -> Container {
        Container { data }
    }
}

#[test]
fn test_container_new() {
    Container::new("".as_bytes());
}

#[test]
fn test_adder() {
    println!("add int int:   {}", Adder::<i32>::add(1, 2));
    println!("add int float: {}", Adder::<f64>::add(1, 2.3));
    println!("add float int: {}", Adder::<f64>::add(2.3, 1));
}

#[tokio::test]
async fn hrtb() {
    wrapper(add_one).await
}

#[tokio::test]
async fn async_ref_functions() {
    let mut v: Vec<Box<dyn System>> = vec![];
    let mut n: u8 = 2;

    v.push(Box::new(a));
    v.push(Box::new(b));

    println!("pushed");

    for i in &v {
        println!("{:?}", i.call(&mut n).await);
    }

    for i in &v {
        println!("{:?}", i.call(&mut n).await);
    }
}

#[tokio::test]
async fn async_hrtb_generic() {
    let mut x = X::new();

    x.add(test);
    println!("a");
    x.run().await;
    // tokio::task::spawn(x.run());
    // tokio::task::spawn_blocking(x.run());
    println!("c")
}

#[tokio::test]
async fn line_writer_test() {
    let mut lw = LineWriter {
        state: "begin".to_string(),
    };

    let result = lw
        .start_writing_lines(|l| Box::pin(l.write_newline()))
        .await;

    println!("{:?}", result)
}

#[tokio::test]
async fn test_client_plugin() {
    let plugin1 = 0usize;
    let plugin2 = Plugin {
        this: plugin1,
        next: 1usize,
    };
    let plugin3 = Plugin {
        this: plugin2,
        next: 2usize,
    };
    let mut ctx = ClientConnectContext::new(plugin3);
    ctx.next(&mut ClientConn).await;
}

#[tokio::test]
async fn test_stream_sink() {
    // create sink
    let sink = drain().with(|value: i32| {
        Box::pin(async move {
            // do sometihng with `value`
            println!("got value: {:?}", value);
            Ok::<_, Box<dyn std::error::Error>>(())
        })
    });

    stream_it(sink).await.unwrap();
}

async fn stream_it(
    sink: impl Sink<i32, Error = Box<dyn std::error::Error>> + Unpin,
) -> Result<(), Box<dyn std::error::Error>> {
    stream::iter(vec![1, 1, 2, 3, 5, 8, 13])
        .map(Ok)
        .forward(sink)
        .await
}

// let ok_stream: stream::Map<S, fn(_) -> Result<_, mpsc::SendError>> = stream.map(Ok);
//     let forwarder = StreamForwarder {
//         inner: ok_stream.forward(tx),
//     };
pub struct StreamForwarder<S: Stream> {
    inner: stream::Forward<
        stream::Map<S, fn(S::Item) -> Result<S::Item, mpsc::SendError>>,
        mpsc::Sender<S::Item>,
    >,
}
