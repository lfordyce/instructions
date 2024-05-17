use futures::future::BoxFuture;

use futures::stream::Stream;
use std::mem;
use std::pin::Pin;
use std::task::{Context, Poll};

use async_trait::async_trait;

pub struct DiyStream {
    counter: usize,
    provider_or_fut: ProviderOrFut,
}

enum ProviderOrFut {
    Provider(Box<dyn Provider + Send + Sync>),
    Future(BoxFuture<'static, (Box<dyn Provider + Send + Sync>, Vec<usize>)>),
    DummyValue,
}

#[async_trait]
pub trait Provider {
    async fn get(&self, n: usize) -> Vec<usize>;
}

impl Stream for DiyStream {
    type Item = Vec<usize>;

    fn poll_next(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Option<Self::Item>> {
        let me = self.get_mut();
        let inner = mem::replace(&mut me.provider_or_fut, ProviderOrFut::DummyValue);
        match inner {
            ProviderOrFut::Provider(provider) => {
                let counter = me.counter;
                me.counter += 1;
                // use an async block to take ownership of the provider
                let fut = async move {
                    let result = provider.get(counter).await;
                    (provider, result)
                };

                me.provider_or_fut = ProviderOrFut::Future(Box::pin(fut));
                Pin::new(me).poll_next(cx)
            }
            ProviderOrFut::Future(mut fut) => match fut.as_mut().poll(cx) {
                Poll::Pending => {
                    me.provider_or_fut = ProviderOrFut::Future(fut);
                    Poll::Pending
                }
                Poll::Ready((provider, value)) => {
                    me.provider_or_fut = ProviderOrFut::Provider(provider);
                    Poll::Ready(Some(value))
                }
            },
            ProviderOrFut::DummyValue => panic!("Call to poll_next after panic."),
        }
    }
}
