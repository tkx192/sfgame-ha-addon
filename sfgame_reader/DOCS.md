# Shakes & Fidget Reader

This app uses the unofficial [`sf-api`](https://github.com/the-marenga/sf-api) Rust library in read-only mode to retrieve a character GameState and publish selected values to Home Assistant over MQTT.

## Configuration

- `sfgame_username`: Your Shakes & Fidget account login.
- `sfgame_password`: Your Shakes & Fidget account password.
- `character_name`: Optional. If the account has multiple characters, use this to select one by character name. If empty, the first returned character is monitored.
- `poll_interval_seconds`: Minimum 60 seconds. Defaults to 300 seconds.
- `publish_full_gamestate`: When enabled, the complete parsed GameState is published as MQTT JSON. Keep this disabled until needed; the full structure is much larger.

The app consumes the Home Assistant `mqtt` service, so MQTT host and credentials do not need to be entered manually.

## MQTT

State topic:

`sfgame/<character_slug>/state`

Home Assistant MQTT Discovery is published automatically under:

`homeassistant/sensor/sfgame_<character_slug>/...`

## Safety

The initial version only sends `Update` commands and does not perform game actions such as quests, purchases, attacks, or inventory changes.
