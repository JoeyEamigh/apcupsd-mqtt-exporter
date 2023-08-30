use super::config::Config;
use std::collections::HashMap;
use tokio::sync::mpsc;
use tracing::{debug, warn};

pub struct MQTTSender {
  client: rumqttc::AsyncClient,
  eventloop: rumqttc::EventLoop,
  topic: String,
  suffix: String,
  home_assistant_mode: bool,
  home_assistant_uuid_prefix: String,
}

type MQTTNewSenderReturn = (
  MQTTSender,
  mpsc::Receiver<HashMap<String, String>>,
  mpsc::Sender<HashMap<String, String>>,
);

impl MQTTSender {
  pub fn new(config: &Config) -> MQTTNewSenderReturn {
    let mut options = rumqttc::MqttOptions::new(
      config.mqtt_client_id.clone(),
      config.mqtt_host.clone(),
      config.mqtt_port,
    );

    if config.mqtt_has_credentials {
      options.set_credentials(config.mqtt_username.clone(), config.mqtt_password.clone());
    }

    let (client, eventloop) = rumqttc::AsyncClient::new(options, 1);

    let (tx, rx) = mpsc::channel(1);

    (
      Self {
        client,
        eventloop,
        topic: config.mqtt_topic.clone(),
        suffix: config.mqtt_suffix.clone(),
        home_assistant_mode: config.home_assistant_mode,
        home_assistant_uuid_prefix: config.home_assistant_uuid_prefix.clone(),
      },
      rx,
      tx,
    )
  }

  pub async fn listen(&mut self, mut rx: mpsc::Receiver<HashMap<String, String>>) {
    let qos = rumqttc::QoS::AtLeastOnce;
    let retain = false;
    let topic = self.topic.clone();
    let suffix = self.suffix.clone();
    let client = self.client.clone();
    let home_assistant_mode = self.home_assistant_mode;
    let home_assistant_uuid_prefix = self.home_assistant_uuid_prefix.clone();

    tokio::spawn(async move {
      let mut seen_before: HashMap<String, bool> = HashMap::new();

      loop {
        let data = rx.recv().await;
        let data = if let Some(data) = data {
          data
        } else {
          continue;
        };

        debug!("Received data from apcaccess: {:?}", data);

        if home_assistant_mode {
          for (param, value) in data.clone() {
            let uuid = format!("{}{}", home_assistant_uuid_prefix, param);

            let config_topic = format!("{}/{}/config", topic, uuid);
            let state_topic = format!("{}/{}/{}", topic, uuid, suffix);

            if !seen_before.contains_key(&param) {
              let payload = serde_json::json!({
                "name": format!("{}", param),
                "unique_id": uuid,
                "device": {
                  "identifiers": format!("{}", topic),
                  "name": "APC UPS",
                },
                "state_topic": state_topic,
                "value_template": "{{ value_json.state }}",
                "expire_after": 60,
              })
              .to_string();

              debug!(
                "Publishing Initial Home Assistant Config MQTT message: {} -> {:?}",
                config_topic, payload
              );

              if let Err(err) = client.publish(config_topic, qos, retain, payload).await {
                warn!(
                  "Failed to publish Initial Home Assistant Config MQTT message: {:?}",
                  err
                );
              }

              seen_before.insert(param.clone(), true);
            }

            let payload = serde_json::json!({ "state": value }).to_string();

            debug!(
              "Publishing Home Assistant State MQTT message: {} -> {:?}",
              state_topic, payload
            );

            if let Err(err) = client.publish(state_topic, qos, retain, payload).await {
              warn!("Failed to publish Home Assistant State MQTT message: {:?}", err);
            }
          }

          continue;
        }

        for (param, value) in data {
          let mut topic = format!("{}/{}", topic, param.to_lowercase().replace(' ', "-"));
          if !suffix.is_empty() {
            topic = format!("{}/{}", topic, suffix);
          }

          debug!("Publishing MQTT message: {} -> {:?}", topic, value.clone());

          if let Err(err) = client.publish(topic, qos, retain, value).await {
            warn!("Failed to publish MQTT message: {:?}", err);
          }
        }
      }
    });

    loop {
      let _ = self.eventloop.poll().await;
      // let mqtt_msg = self.eventloop.poll().await;
      // trace!("Received MQTT message: {:?}", mqtt_msg);
    }
  }
}
