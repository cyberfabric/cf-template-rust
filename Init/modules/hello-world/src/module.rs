use modkit::{Module, ModuleCtx, RunnableCapability, async_trait};

#[derive(Default)]
#[modkit::module(name = "hello-world", capabilities = [stateful])]
pub struct HelloWorldModule;

#[async_trait]
impl Module for HelloWorldModule {
    async fn init(&self, _ctx: &ModuleCtx) -> modkit::Result<()> {
        tracing::info!("Init hello world module");
        Ok(())
    }
}

#[async_trait]
impl RunnableCapability for HelloWorldModule {
    async fn start(&self, cancel: tokio_util::sync::CancellationToken) -> modkit::Result<()> {
        tokio::spawn(async move {
            loop {
                tokio::select! {
                    () = cancel.cancelled() => {
                        tracing::info!("Cancelled World");
                        break
                    },
                    () = tokio::time::sleep(tokio::time::Duration::from_secs(1)) => {
                        tracing::info!("Hello World");
                    }
                }
            }
        });
        Ok(())
    }

    async fn stop(&self, _cancel: tokio_util::sync::CancellationToken) -> modkit::Result<()> {
        tracing::info!("Goodbye World");
        Ok(())
    }
}
