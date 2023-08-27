# apcupsd-mqtt-exporter

This is a docker application to publish apcupsd metrics to MQTT.

To run, make sure to set the environment variables, or use the default values provided below.

```sh
APCUPSD_HOST="127.0.0.1" # host running apcupsd
APCUPSD_PORT=3551        # port apcupsd is listening on
APCUPSD_STRIP_UNITS=true # strip units from apcupsd values
APCUPSD_POLL_INTERVAL=10 # seconds between polling apcupsd
MQTT_HOST="127.0.0.1"    # host running MQTT broker
MQTT_PORT=1883           # port MQTT broker is listening on
MQTT_USERNAME=""         # MQTT username (optional)
MQTT_PASSWORD=""         # MQTT password (optional)
MQTT_CLIENT_ID="apcupsd" # MQTT client ID
MQTT_TOPIC="apcupsd/ups" # MQTT topic to publish to
```

To use locally with the `.env` file, make sure to enable the "dotenv" feature.
