#![feature(let_chains)]

use tracing::info;

mod apcupsd;
mod config;
mod mqtt;

#[tokio::main]
async fn main() {
  init_logger();

  let config = config::Config::new();

  let (mut mqtt_sender, tx) = mqtt::MQTTSender::new(&config);

  info!("Starting apcupsd_mqtt_exporter");

  tokio::spawn(async move {
    apcupsd::APCUPSdPolling::new(&config, tx).poll().await;
  });

  mqtt_sender.listen().await;
}

fn init_logger() {
  use tracing::metadata::LevelFilter;
  use tracing_subscriber::{
    fmt, prelude::__tracing_subscriber_SubscriberExt, util::SubscriberInitExt, EnvFilter, Layer,
  };

  #[cfg(debug_assertions)]
  let filter_directives = if let Ok(filter) = std::env::var("RUST_LOG") {
    filter
  } else {
    "apcupsd_mqtt_exporter=trace".to_string()
  };

  #[cfg(debug_assertions)]
  let filter = EnvFilter::builder()
    .with_default_directive(LevelFilter::TRACE.into())
    .parse_lossy(filter_directives);

  #[cfg(not(debug_assertions))]
  let filter_directives = if let Ok(filter) = std::env::var("RUST_LOG") {
    filter
  } else {
    "apcupsd_mqtt_exporter=info".to_string()
  };

  #[cfg(not(debug_assertions))]
  let filter = EnvFilter::builder()
    .with_default_directive(LevelFilter::TRACE.into())
    .parse_lossy(filter_directives);

  tracing_subscriber::registry()
    .with(fmt::layer().with_filter(filter))
    .init();
}
