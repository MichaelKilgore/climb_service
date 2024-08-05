use std::env;
use async_trait::async_trait;

#[async_trait]
pub trait GeneralUtils: Send + Sync {
    async fn is_request_a_test_request(&self) -> bool {
        true
    }

}

#[derive(Clone)]
pub struct GeneralConfig {
    cloud_run_name: String,
}

impl GeneralConfig {
    pub(crate) fn new() -> Self {
        GeneralConfig {
            cloud_run_name: env::var("CLOUD_RUN_NAME").unwrap(),
        }
    }
}

pub struct GeneralUtilsImpl {
    pub(crate) general_config: GeneralConfig
}

#[async_trait]
impl GeneralUtils for GeneralUtilsImpl {
    async fn is_request_a_test_request(&self) -> bool {
        self.general_config.cloud_run_name == "integ-cloud-run"
    } 
}
