use crate::config::AppConfig;
use derive_getters::Getters;
use ethers_providers::{Http, Provider};
use std::{convert::TryFrom, sync::Arc, time::Duration};

#[derive(Clone, Getters)]
pub struct AppState {
    provider: Provider<Http>,
}

impl AppState {
    pub async fn init(config: AppConfig) -> anyhow::Result<Arc<Self>> {
        let provider = Provider::<Http>::try_from(config.rpc_url)
            .unwrap()
            .interval(Duration::from_millis(10u64));

        Ok(Self { provider }.into())
    }
}
