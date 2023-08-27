// APCUPSD_HOST: "127.0.0.1"           # host running apcupsd
// APCUPSD_PORT: 3551                # port apcupsd is listening on
// APCUPSD_STRIP_UNITS: true         # strip units from apcupsd values
// APCUPSD_POLL_INTERVAL: 10         # seconds between polling apcupsd
// MQTT_HOST: "127.0.0.1"              # host running MQTT broker
// MQTT_PORT: 1883                   # port MQTT broker is listening on
// MQTT_USERNAME: ""                 # MQTT username (optional)
// MQTT_PASSWORD: ""                 # MQTT password (optional)
// MQTT_CLIENT_ID: "apcupsd"         # MQTT client ID
// MQTT_TOPIC: "apcupsd/ups"         # MQTT topic to publish to

use tracing::trace;

#[derive(Debug)]
pub struct Config {
  pub apcupsd_host: String,
  pub apcupsd_port: u16,
  pub apcupsd_strip_units: bool,
  pub apcupsd_poll_interval: u64,
  pub mqtt_host: String,
  pub mqtt_port: u16,
  pub mqtt_has_credentials: bool,
  pub mqtt_username: String,
  pub mqtt_password: String,
  pub mqtt_client_id: String,
  pub mqtt_topic: String,
}

impl Default for Config {
  fn default() -> Self {
    Self {
      apcupsd_host: "127.0.0.1".to_string(),
      apcupsd_port: 3551,
      apcupsd_strip_units: true,
      apcupsd_poll_interval: 10,
      mqtt_host: "127.0.0.1".to_string(),
      mqtt_port: 1883,
      mqtt_has_credentials: false,
      mqtt_username: "".to_string(),
      mqtt_password: "".to_string(),
      mqtt_client_id: "apcupsd".to_string(),
      mqtt_topic: "apcupsd/ups".to_string(),
    }
  }
}

impl Config {
  pub fn new() -> Self {
    #[cfg(feature = "dotenv")]
    dotenv::dotenv().ok();

    let mut config = Config::default();

    if let Ok(apcupsd_host) = std::env::var("APCUPSD_HOST") {
      config.apcupsd_host = apcupsd_host;
    }

    if let Ok(apcupsd_port) = std::env::var("APCUPSD_PORT") {
      config.apcupsd_port = apcupsd_port.parse::<u16>().unwrap();
    }

    if let Ok(apcupsd_strip_units) = std::env::var("APCUPSD_STRIP_UNITS") {
      config.apcupsd_strip_units = apcupsd_strip_units.parse::<bool>().unwrap();
    }

    if let Ok(apcupsd_poll_interval) = std::env::var("APCUPSD_POLL_INTERVAL") {
      config.apcupsd_poll_interval = apcupsd_poll_interval.parse::<u64>().unwrap();
    }

    if let Ok(mqtt_host) = std::env::var("MQTT_HOST") {
      config.mqtt_host = mqtt_host;
    }

    if let Ok(mqtt_port) = std::env::var("MQTT_PORT") {
      config.mqtt_port = mqtt_port.parse::<u16>().unwrap();
    }

    if let Ok(mqtt_username) = std::env::var("MQTT_USERNAME")
        && let Ok(mqtt_password) = std::env::var("MQTT_PASSWORD") {
      config.mqtt_username = mqtt_username;
      config.mqtt_password = mqtt_password;
      config.mqtt_has_credentials = true;
    }

    if let Ok(mqtt_client_id) = std::env::var("MQTT_CLIENT_ID") {
      config.mqtt_client_id = mqtt_client_id;
    }

    if let Ok(mqtt_topic) = std::env::var("MQTT_TOPIC") {
      config.mqtt_topic = mqtt_topic;
    }

    trace!("initialized config: {:?}", config);

    config
  }
}
