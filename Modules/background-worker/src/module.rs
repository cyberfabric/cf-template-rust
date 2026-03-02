use crate::infra::Client;
use modkit::{Module, ModuleCtx, RunnableCapability, async_trait};
use std::sync::Arc;
use tokio::sync::Mutex;
use tokio::task::JoinHandle;

#[modkit::module(name = "background-worker", capabilities = [stateful])]
#[derive(Default)]
pub struct BackgroundWorkerModule {
    task_handle: Arc<Mutex<Option<JoinHandle<()>>>>,
}

#[async_trait]
impl Module for BackgroundWorkerModule {
    async fn init(&self, _ctx: &ModuleCtx) -> modkit::Result<()> {
        tracing::info!("Initializing {{module_name}} module");
        Ok(())
    }
}

#[async_trait]
impl RunnableCapability for BackgroundWorkerModule {
    async fn start(&self, cancel: tokio_util::sync::CancellationToken) -> modkit::Result<()> {
        let repository = Arc::new(Client::new()?);

        tracing::info!("Starting background-worker background fetcher");

        let handle = tokio::spawn(async move {
            let mut interval = tokio::time::interval(tokio::time::Duration::from_secs(5));
            interval.set_missed_tick_behavior(tokio::time::MissedTickBehavior::Skip);

            loop {
                tokio::select! {
                    _ = cancel.cancelled() => {
                        tracing::info!("background-worker fetcher cancelled");
                        break;
                    }
                    _ = interval.tick() => {
                        match repository.fetch_data().await {
                            Ok(data) => {
                                // Use debug level to avoid logging PII in production
                                tracing::debug!("Fetched data: {data:?}");
                            }
                            Err(e) => {
                                tracing::error!("Failed to fetch data: {e}");
                            }
                        }
                    }
                }
            }
        });

        // Store the handle for graceful shutdown
        *self.task_handle.lock().await = Some(handle);

        Ok(())
    }

    async fn stop(&self, _cancel: tokio_util::sync::CancellationToken) -> modkit::Result<()> {
        tracing::info!("Stopping background-worker module");

        // Wait for the background task to complete
        if let Some(handle) = self.task_handle.lock().await.take() {
            if let Err(e) = handle.await {
                tracing::error!("background-worker task panicked: {e}");
            } else {
                tracing::info!("background-worker task completed gracefully");
            }
        } else {
            tracing::warn!("background-worker task was not running");
        }

        Ok(())
    }
}
