use futures::FutureExt;

#[derive(Default)]
struct Settings {
    field_a: String,
}

#[derive(Default)]
struct ConsumeParams {
    id: String,
}
#[derive(Default)]
struct Payload {
    id: String,
}

#[derive(Clone)]
struct Queuer {}

impl Queuer {
    fn new() -> Self {
        Self {}
    }

    async fn consume<F>(
        &self,
        fn_consume: F,
        _: Settings,
    ) -> Result<(), Box<dyn std::error::Error + Send + Sync + 'static>>
    where
        F: Fn(
            ConsumeParams,
        ) -> futures::future::BoxFuture<
            'static,
            Result<(), Box<dyn std::error::Error + Send + Sync + 'static>>,
        >,
    {
        let mut counter = 0;
        loop {
            if counter > 0 {
                return Ok(());
            }
            counter = counter + 1;
            match fn_consume(ConsumeParams::default()).await {
                Ok(_) => println!("success"),
                Err(err) => return Err(err),
            }
        }
    }
}

#[derive(Clone)]
struct Syncer {
    queuer: Queuer,
}

impl Syncer {
    fn new() -> Self {
        Self {
            queuer: Queuer::new(),
        }
    }
    pub async fn perform_actions<F, RespMsg>(
        &self,
        fn_processor: F,
        consume_settings: Settings,
    ) -> Result<(), Box<dyn std::error::Error + Send + Sync + 'static>>
    where
        F: Fn(
                ConsumeParams,
            ) -> futures::future::BoxFuture<
                'static,
                Result<RespMsg, Box<dyn std::error::Error + Send + Sync + 'static>>,
            > + Send
            + Sync
            + 'static,
        RespMsg: Into<Payload> + Send,
    {
        let shared_processor = std::sync::Arc::new(fn_processor);
        let clone_queuer = self.queuer.clone();
        clone_queuer
            .consume(
                |params: ConsumeParams| {
                    let shared_processor_clone = shared_processor.clone();
                    let syncer_clone = self.clone();
                    Box::pin(async move {
                        syncer_clone.util_function().await.unwrap();
                        let fn_processor_future = shared_processor_clone(params);
                        let fn_process_result = fn_processor_future.await;
                        match fn_process_result {
                            Ok(_) => println!("Success"),
                            Err(err) => return Err(err),
                        };
                        syncer_clone.util_function().await.unwrap();
                        Ok(())
                    })
                },
                consume_settings,
            )
            .await?;
        Ok(())
    }

    async fn util_function(
        &self,
    ) -> Result<(), Box<dyn std::error::Error + Send + Sync + 'static>> {
        tokio::time::sleep(std::time::Duration::from_secs(1)).await;
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_queuer() {
        let queuer = Queuer::new();
        queuer
            .consume(
                |_: ConsumeParams| {
                    Box::pin(async move {
                        println!("testing consume");
                        Ok(())
                    })
                },
                Settings::default(),
            )
            .await
            .unwrap();
        let syncer = Syncer::new();
        syncer
            .perform_actions(
                |_: ConsumeParams| {
                    async move {
                        tokio::time::sleep(std::time::Duration::from_secs(1)).await;
                        Ok(Payload::default())
                    }
                    .boxed()
                },
                Settings::default(),
            )
            .await
            .unwrap();
        println!("Hello, world!");
    }
}
