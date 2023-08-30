# apcupsd-mqtt-exporter

This is a docker application to publish apcupsd metrics to MQTT.

To run, make sure to set the environment variables, or use the default values provided below.

```sh
APCUPSD_HOST="127.0.0.1"  # host running apcupsd
APCUPSD_PORT=3551         # port apcupsd is listening on
APCUPSD_STRIP_UNITS=true  # strip units from apcupsd values
APCUPSD_POLL_INTERVAL=10  # seconds between polling apcupsd
APCUPSD_POLL_TIMEOUT=5    # seconds between polling apcupsd
MQTT_HOST="127.0.0.1"     # host running MQTT broker
MQTT_PORT=1883            # port MQTT broker is listening on
MQTT_USERNAME=""          # MQTT username (optional)
MQTT_PASSWORD=""          # MQTT password (optional)
MQTT_CLIENT_ID="apcupsd"  # MQTT client ID
MQTT_TOPIC="apcupsd/ups"  # MQTT topic to publish to
MQTT_SUFFIX=""            # MQTT topic suffix (optional)
HOME_ASSISTANT_MODE=false # publish MQTT messages in Home Assistant-compatible JSON
```

To use locally with the `.env` file, make sure to enable the "dotenv" feature.

To use with Home Assistant, the following `.env` will enable automatic discovery:

```sh
APCUPSD_HOST="localhost"                     # host running apcupsd
APCUPSD_PORT=3551                            # port apcupsd is listening on
APCUPSD_STRIP_UNITS=true                     # strip units from apcupsd values
APCUPSD_POLL_INTERVAL=10                     # seconds between polling apcupsd
APCUPSD_POLL_TIMEOUT=5                       # seconds between polling apcupsd
MQTT_HOST="localhost"                        # host running MQTT broker
MQTT_PORT=1883                               # port MQTT broker is listening on
MQTT_USERNAME=""                             # MQTT username (optional)
MQTT_PASSWORD=""                             # MQTT password (optional)
MQTT_CLIENT_ID="upcupsd"                     # MQTT client ID
MQTT_TOPIC="homeassistant/sensor/ups"        # MQTT topic to publish to
MQTT_SUFFIX="status"                         # MQTT topic suffix (optional)
HOME_ASSISTANT_MODE=true                     # publish MQTT messages in Home Assistant-compatible JSON
HOME_ASSISTANT_UUID_PREFIX="apcupsd_"        # prefix for Home Assistant UUIDs
```
