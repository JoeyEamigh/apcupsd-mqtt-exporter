use super::config;

use apcaccess::{APCAccess, APCAccessConfig};
use std::collections::HashMap;
use tokio::sync::mpsc;
use tracing::{error, trace};

pub struct APCUPSdPolling {
  apc: APCAccess,
  poll_interval: u64,
  timeout: u64,

  tx: mpsc::Sender<HashMap<String, String>>,
}

impl APCUPSdPolling {
  pub fn new(app_config: &config::Config, tx: mpsc::Sender<HashMap<String, String>>) -> Self {
    let config = APCAccessConfig {
      host: app_config.apcupsd_host.clone(),
      port: app_config.apcupsd_port,
      strip_units: app_config.apcupsd_strip_units,
      timeout: app_config.apcupsd_poll_timeout,
    };

    Self {
      apc: APCAccess::new(Some(config)),
      poll_interval: app_config.apcupsd_poll_interval,
      timeout: app_config.apcupsd_poll_timeout,

      tx,
    }
  }

  pub async fn poll(&self) {
    loop {
      trace!("polling apcupsd");
      let apc = self.apc.clone();

      let data = match tokio::time::timeout(
        std::time::Duration::from_secs(self.timeout),
        tokio::spawn(async move { apc.fetch() }),
      )
      .await
      {
        Ok(data) => match data {
          Ok(data) => match data {
            Ok(data) => data,
            Err(err) => {
              error!("failed to fetch data from apcupsd: {:?}", err);
              continue;
            }
          },
          Err(err) => {
            error!("failed to join tokio thread {:?}", err);
            continue;
          }
        },
        Err(_) => {
          error!(
            "timeout while fetching data from apcupsd: timeout of {} seconds elapsed",
            self.timeout
          );
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
