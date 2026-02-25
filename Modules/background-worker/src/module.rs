use crate::domain::DataRepository;
use crate::infra::HttpClient;
use modkit::{async_trait, Module, ModuleCtx, RunnableCapability};
use std::sync::Arc;
use tokio::sync::Mutex;
use tokio::task::JoinHandle;

/// {{description}}
#[modkit::module(name = "{{module_name}}", capabilities = [stateful])]
pub struct {{struct_module_name}}Module {
    task_handle: Arc<Mutex<Option<JoinHandle<()>>>>,
}

impl Default for {{struct_module_name}}Module {
    fn default() -> Self {
        Self {
            task_handle: Arc::new(Mutex::new(None)),
        }
    }
}

#[async_trait]
impl Module for {{struct_module_name}}Module {
    async fn init(&self, _ctx: &ModuleCtx) -> modkit::Result<()> {
        tracing::info!("Initializing {{module_name}} module");
        Ok(())
    }
}

#[async_trait]
impl RunnableCapability for {{struct_module_name}}Module {
    async fn start(&self, cancel: tokio_util::sync::CancellationToken) -> modkit::Result<()> {
        let repository: Arc<dyn DataRepository> = Arc::new(HttpClient::new("{{http_url}}".to_string()));

        tracing::info!("Starting {{module_name}} background fetcher");

        let handle = tokio::spawn(async move {
            let mut interval = tokio::time::interval(tokio::time::Duration::from_secs({{fetch_interval_secs}}));
            interval.set_missed_tick_behavior(tokio::time::MissedTickBehavior::Skip);

            loop {
                tokio::select! {
                    _ = cancel.cancelled() => {
                        tracing::info!("{{module_name}} fetcher cancelled");
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
        tracing::info!("Stopping {{module_name}} module");

        // Wait for the background task to complete
        if let Some(handle) = self.task_handle.lock().await.take() {
            if let Err(e) = handle.await {
                tracing::error!("{{module_name}} task panicked: {e}");
            } else {
                tracing::info!("{{module_name}} task completed gracefully");
            }
        } else {
            tracing::warn!("{{module_name}} task was not running");
        }

        Ok(())
    }
}
