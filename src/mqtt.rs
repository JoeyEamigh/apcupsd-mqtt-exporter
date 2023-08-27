use super::config::Config;
use std::collections::HashMap;
use tokio::sync::mpsc;
use tracing::{debug, trace, warn};

pub struct MQTTSender {
  client: rumqttc::AsyncClient,
  eventloop: rumqttc::EventLoop,
  topic: String,

  rx: mpsc::Receiver<HashMap<String, String>>,
}

impl MQTTSender {
  pub fn new(config: &Config) -> (Self, mpsc::Sender<HashMap<String, String>>) {
    let mut options = rumqttc::MqttOptions::new(
      config.mqtt_client_id.clone(),
      config.mqtt_host.clone(),
      config.mqtt_port,
    );

    if config.mqtt_has_credentials {
      options.set_credentials(config.mqtt_username.clone(), config.mqtt_password.clone());
    }

    let (client, eventloop) = rumqttc::AsyncClient::new(options, 100);

    let (tx, rx) = mpsc::channel(10);

    (
      Self {
        client,
        eventloop,
        topic: config.mqtt_topic.clone(),
        rx,
      },
      tx,
    )
  }

  pub async fn listen(&mut self) {
    let qos = rumqttc::QoS::AtLeastOnce;
    let retain = false;

    loop {
      tokio::select! {
        mqtt_msg = self.eventloop.poll() => {
          trace!("Received MQTT message: {:?}", mqtt_msg);
        }
        data = self.rx.recv() => {
          debug!("Received data from apcaccess: {:?}", data);

          let data = if let Some(data) = data {
            data
          } else {
            continue;
          };

          let status = data.get("STATUS");
          let has_ac_power = match status {
            Some(status) => {
              if status == "ONLINE" {
                "true"
              } else {
                "false"
              }
            }
            None => "false",
          };

          for (param, value) in data {
            let topic = format!("{}/{}", self.topic, param.to_lowercase().replace(' ', "-"));

            debug!("Publishing MQTT message: {} -> {:?}", topic, value.clone());

            if let Err(err) = self.client.publish(topic, qos, retain, value).await {
              warn!("Failed to publish MQTT message: {:?}", err);
            }
          }

          let topic = format!("{}/{}", self.topic, "acpower");

          debug!("Publishing MQTT message: {} -> {:?}", topic, has_ac_power);

          if let Err(err) = self.client.publish(topic, qos, retain, has_ac_power).await {
            warn!("Failed to publish MQTT message: {:?}", err);
          }
        }
      }
    }
  }
}
