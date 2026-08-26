#!/usr/bin/env bash

set -euo pipefail

echo "[INFO] Starting Shakes & Fidget Reader"

OPTIONS="/data/options.json"

export SFGAME_USERNAME="$(jq -r '.sfgame_username // empty' "$OPTIONS")"
export SFGAME_PASSWORD="$(jq -r '.sfgame_password // empty' "$OPTIONS")"
export CHARACTER_NAME="$(jq -r '.character_name // empty' "$OPTIONS")"
export POLL_INTERVAL_SECONDS="$(jq -r '.poll_interval_seconds // 300' "$OPTIONS")"
export PUBLISH_FULL_GAMESTATE="$(jq -r '.publish_full_gamestate // false' "$OPTIONS")"

if [[ -z "${SFGAME_USERNAME}" || -z "${SFGAME_PASSWORD}" ]]; then
    echo "[ERROR] Shakes & Fidget credentials are not configured"
    exit 1
fi

if [[ -z "${SUPERVISOR_TOKEN:-}" ]]; then
    echo "[ERROR] Supervisor API token is not available"
    exit 1
fi

MQTT_JSON="$(
    curl --fail --silent --show-error \
        -H "Authorization: Bearer ${SUPERVISOR_TOKEN}" \
        "http://supervisor/services/mqtt"
)"

export MQTT_HOST="$(jq -r '.host // empty' <<< "$MQTT_JSON")"
export MQTT_PORT="$(jq -r '.port // 1883' <<< "$MQTT_JSON")"
export MQTT_USERNAME="$(jq -r '.username // empty' <<< "$MQTT_JSON")"
export MQTT_PASSWORD="$(jq -r '.password // empty' <<< "$MQTT_JSON")"

if [[ -z "${MQTT_HOST}" || -z "${MQTT_USERNAME}" || -z "${MQTT_PASSWORD}" ]]; then
    echo "[ERROR] MQTT service configuration is incomplete"
    exit 1
fi

echo "[INFO] Configuration loaded"
echo "[INFO] MQTT service configuration loaded"

exec /usr/bin/sfgame-reader
