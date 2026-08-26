#!/usr/bin/env bash

set -euo pipefail

echo "[INFO] Starting Shakes & Fidget Reader"

export SFGAME_USERNAME="$(jq -r '.sfgame_username // empty' /data/options.json)"
export SFGAME_PASSWORD="$(jq -r '.sfgame_password // empty' /data/options.json)"
export CHARACTER_NAME="$(jq -r '.character_name // empty' /data/options.json)"
export POLL_INTERVAL_SECONDS="$(jq -r '.poll_interval_seconds // 300' /data/options.json)"
export PUBLISH_FULL_GAMESTATE="$(jq -r '.publish_full_gamestate // false' /data/options.json)"

echo "[INFO] Configuration loaded"

exec /usr/bin/sfgame-reader
