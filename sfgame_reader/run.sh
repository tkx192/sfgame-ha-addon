#!/usr/bin/env bashio

set -e

bashio::log.info "Starting Shakes & Fidget Reader"

USERNAME="$(bashio::config 'sfgame_username')"
PASSWORD="$(bashio::config 'sfgame_password')"
CHARACTER="$(bashio::config 'character_name')"
POLL_INTERVAL="$(bashio::config 'poll_interval_seconds')"
PUBLISH_FULL="$(bashio::config 'publish_full_gamestate')"

MQTT_HOST="$(bashio::services mqtt 'host')"
MQTT_PORT="$(bashio::services mqtt 'port')"
MQTT_USERNAME="$(bashio::services mqtt 'username')"
MQTT_PASSWORD="$(bashio::services mqtt 'password')"

export SFGAME_USERNAME="$USERNAME"
export SFGAME_PASSWORD="$PASSWORD"
export SFGAME_CHARACTER_NAME="$CHARACTER"
export SFGAME_POLL_INTERVAL_SECONDS="$POLL_INTERVAL"
export SFGAME_PUBLISH_FULL_GAMESTATE="$PUBLISH_FULL"

export MQTT_HOST
export MQTT_PORT
export MQTT_USERNAME
export MQTT_PASSWORD

exec /usr/bin/sfgame-reader
