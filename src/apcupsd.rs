use super::config;

use apcaccess::{APCAccess, APCAccessConfig};
use std::collections::HashMap;
use tokio::sync::mpsc;
use tracing::{error, trace};

pub struct APCUPSdPolling {
  apc: APCAccess,
  poll_interval: u64,

  tx: mpsc::Sender<HashMap<String, String>>,
}

impl APCUPSdPolling {
  pub fn new(app_config: &config::Config, tx: mpsc::Sender<HashMap<String, String>>) -> Self {
    let config = APCAccessConfig {
      host: app_config.apcupsd_host.clone(),
      port: app_config.apcupsd_port,
      strip_units: app_config.apcupsd_strip_units,
    };

    Self {
      apc: APCAccess::new(Some(config)),
      poll_interval: app_config.apcupsd_poll_interval,

      tx,
    }
  }

  pub async fn poll(&self) {
    loop {
      let data = match self.apc.fetch() {
        Ok(data) => data,
        Err(err) => {
          error!("Failed to fetch data from apcupsd: {:?}", err);
          continue;
        }
      };

      trace!("successfully received data from apcupsd: {:?}", data);

      if let Err(err) = self.tx.send(data).await {
        error!("Failed to send data to MQTT: {:?}", err);
      }

      tokio::time::sleep(tokio::time::Duration::from_secs(self.poll_interval)).await;
    }
  }
}
