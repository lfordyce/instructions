use async_trait::async_trait;
use std::fmt::Debug;

type Result<T> = std::result::Result<T, ()>;

pub struct ClientConn;

impl ClientConn {
    pub async fn client_connect<T: ClientConnectPlugin + Sync + Send>(
        &self,
        _ctx: &mut ClientConnectContext<T>,
    ) -> Result<()> {
        Ok(())
    }
}

pub struct ClientConnectContext<T: ClientConnectPlugin + Sync + Send> {
    next_plugin: Plugin<T>,
}

impl<T: ClientConnectPlugin + Sync + Send> ClientConnectContext<T> {
    pub fn new(plugin: T) -> Self {
        Self {
            next_plugin: Plugin {
                this: plugin,
                next: (),
            },
        }
    }

    /// each plugin will call the next plugin in the chain
    pub async fn next(&self, client: &mut ClientConn) -> Result<()> {
        self.next_plugin.client_connected(client).await
    }
}

#[derive(Debug)]
pub struct Plugin<T: ClientConnectPlugin + Sync + Send, S: ClientConnectPlugin + Send + Sync = ()> {
    pub this: T,
    pub next: S,
}

#[async_trait]
pub trait ClientConnectPlugin: Debug {
    async fn client_connected(&self, client: &mut ClientConn) -> Result<()>;
}

#[async_trait]
impl ClientConnectPlugin for usize {
    async fn client_connected(&self, _client: &mut ClientConn) -> Result<()> {
        println!("called on {}", self);
        Ok(())
    }
}

#[async_trait]
impl ClientConnectPlugin for () {
    async fn client_connected(&self, _client: &mut ClientConn) -> Result<()> {
        Ok(())
    }
}

#[async_trait]
impl<T: ClientConnectPlugin + Sync + Send, S: ClientConnectPlugin + Sync + Send> ClientConnectPlugin
    for Plugin<S, T>
{
    async fn client_connected(&self, client: &mut ClientConn) -> Result<()> {
        self.this.client_connected(client).await;
        self.next.client_connected(client).await;
        Ok(())
    }
}
