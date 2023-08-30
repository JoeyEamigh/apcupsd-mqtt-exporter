// APCUPSD_HOST: "127.0.0.1"         # host running apcupsd
// APCUPSD_PORT: 3551                # port apcupsd is listening on
// APCUPSD_STRIP_UNITS: true         # strip units from apcupsd values
// APCUPSD_POLL_INTERVAL: 10         # seconds between polling apcupsd
// APCUPSD_POLL_TIMEOUT: 5           # seconds to wait for apcupsd to respond
// MQTT_HOST: "127.0.0.1"            # host running MQTT broker
// MQTT_PORT: 1883                   # port MQTT broker is listening on
// MQTT_USERNAME: ""                 # MQTT username (optional)
// MQTT_PASSWORD: ""                 # MQTT password (optional)
// MQTT_CLIENT_ID: "apcupsd"         # MQTT client ID
// MQTT_TOPIC: "apcupsd/ups"         # MQTT topic to publish to
// MQTT_SUFFIX: ""                   # suffix to append to MQTT topic
// HOME_ASSISTANT_MODE: false        # publish MQTT messages in Home Assistant-compatible JSON
// HOME_ASSISTANT_UUID_PREFIX: ""    # prefix for Home Assistant UUIDs

use tracing::trace;

#[derive(Debug)]
pub struct Config {
  pub apcupsd_host: String,
  pub apcupsd_port: u16,
  pub apcupsd_strip_units: bool,
  pub apcupsd_poll_interval: u64,
  pub apcupsd_poll_timeout: u64,
  pub mqtt_host: String,
  pub mqtt_port: u16,
  pub mqtt_has_credentials: bool,
  pub mqtt_username: String,
  pub mqtt_password: String,
  pub mqtt_client_id: String,
  pub mqtt_topic: String,
  pub mqtt_suffix: String,
  pub home_assistant_mode: bool,
  pub home_assistant_uuid_prefix: String,
}

impl Default for Config {
  fn default() -> Self {
    Self {
      apcupsd_host: "127.0.0.1".to_string(),
      apcupsd_port: 3551,
      apcupsd_strip_units: true,
      apcupsd_poll_interval: 10,
      apcupsd_poll_timeout: 5,
      mqtt_host: "127.0.0.1".to_string(),
      mqtt_port: 1883,
      mqtt_has_credentials: false,
      mqtt_username: "".to_string(),
      mqtt_password: "".to_string(),
      mqtt_client_id: "apcupsd".to_string(),
      mqtt_topic: "apcupsd/ups".to_string(),
      mqtt_suffix: "".to_string(),
      home_assistant_mode: false,
      home_assistant_uuid_prefix: "".to_string(),
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

    if let Ok(apcupsd_poll_timeout) = std::env::var("APCUPSD_POLL_TIMEOUT") {
      config.apcupsd_poll_timeout = apcupsd_poll_timeout.parse::<u64>().unwrap();
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

    if let Ok(mqtt_suffix) = std::env::var("MQTT_SUFFIX") {
      config.mqtt_suffix = mqtt_suffix;
    }

    if let Ok(home_assistant_mode) = std::env::var("HOME_ASSISTANT_MODE") {
      config.home_assistant_mode = home_assistant_mode.parse::<bool>().unwrap();
    }

    if let Ok(home_assistant_uuid_prefix) = std::env::var("HOME_ASSISTANT_UUID_PREFIX") {
      config.home_assistant_uuid_prefix = home_assistant_uuid_prefix;
    }

    if config.home_assistant_mode {
      if config.mqtt_suffix.is_empty() {
        config.mqtt_suffix = "status".to_string();
      }

      if config.home_assistant_uuid_prefix.is_empty() {
        config.home_assistant_uuid_prefix = "apcupsd_".to_string();
      }
    }

    trace!("initialized config: {:?}", config);

    config
  }
}
