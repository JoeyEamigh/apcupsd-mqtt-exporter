#!/bin/sh

if [ -z "$1" ]; then
  echo "No version supplied"
  exit 1
fi

docker buildx build . -t ghcr.io/joeyeamigh/apcupsd-mqtt-exporter/apcupsd-mqtt-exporter:$1
docker tag ghcr.io/joeyeamigh/apcupsd-mqtt-exporter/apcupsd-mqtt-exporter:$1 ghcr.io/joeyeamigh/apcupsd-mqtt-exporter/apcupsd-mqtt-exporter:latest

docker push ghcr.io/joeyeamigh/apcupsd-mqtt-exporter/apcupsd-mqtt-exporter:$1
docker push ghcr.io/joeyeamigh/apcupsd-mqtt-exporter/apcupsd-mqtt-exporter:latest
