#!/usr/bin/with-contenv bash
set -euo pipefail

bashio::log.info "Starting Shakes & Fidget Reader"

export SFGAME_USERNAME="$(bashio::config 'sfgame_username')"
export SFGAME_PASSWORD="$(bashio::config 'sfgame_password')"
export CHARACTER_NAME="$(bashio::config 'character_name')"
export POLL_INTERVAL_SECONDS="$(bashio::config 'poll_interval_seconds')"
export PUBLISH_FULL_GAMESTATE="$(bashio::config 'publish_full_gamestate')"

export MQTT_HOST="$(bashio::services mqtt 'host')"
export MQTT_PORT="$(bashio::services mqtt 'port')"
export MQTT_USERNAME="$(bashio::services mqtt 'username')"
export MQTT_PASSWORD="$(bashio::services mqtt 'password')"

exec /usr/bin/sfgame-reader
