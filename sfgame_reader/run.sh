#!/bin/bash
set -e

echo "[INFO] Starting Shakes & Fidget Reader"

for var in MQTT_HOST MQTT_PORT MQTT_USERNAME MQTT_PASSWORD; do
    if [[ -n "${!var:-}" ]]; then
        echo "[INFO] $var is present"
    else
        echo "[INFO] $var is missing"
    fi
done

exec /usr/bin/sfgame-reader
