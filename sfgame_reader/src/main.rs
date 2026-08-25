use std::{env, time::Duration};

use log::{error, info, warn};
use rumqttc::{AsyncClient, Event, EventLoop, LastWill, MqttOptions, QoS};
use serde_json::{json, Value};
use sf_api::{command::Command, session::SimpleSession};
use tokio::time::sleep;

const DISCOVERY_PREFIX: &str = "homeassistant";
const ROOT_TOPIC: &str = "sfgame";

#[derive(Debug, Clone)]
struct Settings {
    username: String,
    password: String,
    character_name: Option<String>,
    poll_interval: Duration,
    publish_full_gamestate: bool,
    mqtt_host: String,
    mqtt_port: u16,
    mqtt_username: Option<String>,
    mqtt_password: Option<String>,
}

fn env_required(name: &str) -> String {
    match env::var(name) {
        Ok(value) if !value.trim().is_empty() => value,
        _ => panic!("required environment variable {name} is missing"),
    }
}

fn env_optional(name: &str) -> Option<String> {
    env::var(name)
        .ok()
        .filter(|value| !value.trim().is_empty())
}

fn read_settings() -> Settings {
    let username = env_required("SFGAME_USERNAME");
    let password = env_required("SFGAME_PASSWORD");
    let poll_interval = env::var("POLL_INTERVAL_SECONDS")
        .ok()
        .and_then(|v| v.parse::<u64>().ok())
        .filter(|v| *v >= 60)
        .unwrap_or(300);
    let mqtt_port = env::var("MQTT_PORT")
        .ok()
        .and_then(|v| v.parse::<u16>().ok())
        .unwrap_or(1883);

    Settings {
        username,
        password,
        character_name: env_optional("CHARACTER_NAME"),
        poll_interval: Duration::from_secs(poll_interval),
        publish_full_gamestate: env::var("PUBLISH_FULL_GAMESTATE")
            .map(|v| v.eq_ignore_ascii_case("true"))
            .unwrap_or(false),
        mqtt_host: env::var("MQTT_HOST").unwrap_or_else(|_| "core-mosquitto".to_string()),
        mqtt_port,
        mqtt_username: env_optional("MQTT_USERNAME"),
        mqtt_password: env_optional("MQTT_PASSWORD"),
    }
}

fn slugify(input: &str) -> String {
    let mut out = String::with_capacity(input.len());
    for c in input.chars() {
        if c.is_ascii_alphanumeric() {
            out.push(c.to_ascii_lowercase());
        } else if !out.ends_with('_') {
            out.push('_');
        }
    }
    out.trim_matches('_').to_string()
}

async fn mqtt_client(settings: &Settings) -> (AsyncClient, EventLoop) {
    let mut options = MqttOptions::new(
        "sfgame_reader",
        settings.mqtt_host.clone(),
        settings.mqtt_port,
    );
    options.set_keep_alive(Duration::from_secs(30));
    options.set_last_will(LastWill::new(
        format!("{ROOT_TOPIC}/status"),
        "offline",
        QoS::AtLeastOnce,
        true,
    ));

    if let (Some(username), Some(password)) = (&settings.mqtt_username, &settings.mqtt_password) {
        options.set_credentials(username, password);
    }

    AsyncClient::new(options, 20)
}

async fn keep_event_loop_running(mut event_loop: EventLoop) {
    loop {
        match event_loop.poll().await {
            Ok(Event::Incoming(_)) | Ok(Event::Outgoing(_)) => {}
            Err(err) => {
                error!("MQTT event loop stopped: {err}");
                sleep(Duration::from_secs(5)).await;
                break;
            }
        }
    }
}

async fn publish_discovery(client: &AsyncClient, character_slug: &str) -> Result<(), Box<dyn std::error::Error>> {
    let device = json!({
        "identifiers": [format!("sfgame_{character_slug}")],
        "name": "Shakes & Fidget",
        "manufacturer": "Playa Games",
        "model": "Shakes & Fidget Reader"
    });

    let entities = [
        ("character", "Charakter", "{{ value_json.character.name }}", Some("mdi:account")),
        ("level", "Level", "{{ value_json.character.level }}", None),
        ("gold", "Gold", "{{ value_json.character.gold }}", None),
        ("mushrooms", "Pilze", "{{ value_json.character.mushrooms }}", Some("mdi:mushroom")),
        ("honor", "Ehre", "{{ value_json.character.honor }}", Some("mdi:sword-cross")),
        ("rank", "Rang", "{{ value_json.character.rank }}", None),
        ("experience", "Erfahrung", "{{ value_json.character.experience }}", Some("mdi:star-four-points")),
        ("xp_percent", "XP Prozent", "{{ value_json.character.xp_percent }}", Some("mdi:progress-star")),
        ("class", "Klasse", "{{ value_json.character.class }}", Some("mdi:shield-account")),
        ("guild", "Gilde", "{{ value_json.guild.name if value_json.guild is defined and value_json.guild else 'Keine Gilde' }}", Some("mdi:account-group")),
    ];

    for (id, name, value_template, icon) in entities {
        let mut config = json!({
            "name": name,
            "unique_id": format!("sfgame_{character_slug}_{id}"),
            "state_topic": format!("{ROOT_TOPIC}/{character_slug}/state"),
            "value_template": value_template,
            "json_attributes_topic": format!("{ROOT_TOPIC}/{character_slug}/state"),
            "device": device,
        });
        if let Some(icon) = icon {
            config["icon"] = Value::String(icon.to_string());
        }
        let topic = format!("{DISCOVERY_PREFIX}/sensor/sfgame_{character_slug}/{id}/config");
        client.publish(topic, QoS::AtLeastOnce, true, serde_json::to_vec(&config)?).await?;
    }

    client
        .publish(
            format!("{ROOT_TOPIC}/status"),
            QoS::AtLeastOnce,
            true,
            "online",
        )
        .await?;

    Ok(())
}

fn game_state_json(gs: &sf_api::gamestate::GameState, full: bool) -> Value {
    if full {
        return serde_json::to_value(gs).unwrap_or_else(|err| {
            warn!("Could not serialize complete GameState: {err}");
            Value::Null
        });
    }

    let xp_percent = if gs.character.next_level_xp > 0 {
        (gs.character.experience as f64 / gs.character.next_level_xp as f64) * 100.0
    } else {
        100.0
    };

    json!({
        "character": {
            "player_id": gs.character.player_id,
            "name": gs.character.name,
            "level": gs.character.level,
            "silver": gs.character.silver,
            "gold": gs.character.silver / 100,
            "mushrooms": gs.character.mushrooms,
            "class": format!("{:?}", gs.character.class),
            "race": format!("{:?}", gs.character.race),
            "experience": gs.character.experience,
            "next_level_xp": gs.character.next_level_xp,
            "xp_percent": (xp_percent * 100.0).round() / 100.0,
            "honor": gs.character.honor,
            "rank": gs.character.rank,
            "armor": gs.character.armor,
            "min_damage": gs.character.min_damage,
            "max_damage": gs.character.max_damage,
        },
        "guild": gs.guild.as_ref().map(|guild| json!({"name": guild.name}))
    })
}

async fn publish_state(
    client: &AsyncClient,
    character_slug: &str,
    gs: &sf_api::gamestate::GameState,
    full: bool,
) -> Result<(), Box<dyn std::error::Error>> {
    let payload = serde_json::to_vec(&game_state_json(gs, full))?;
    client
        .publish(
            format!("{ROOT_TOPIC}/{character_slug}/state"),
            QoS::AtLeastOnce,
            true,
            payload,
        )
        .await?;
    Ok(())
}

async fn run_session(
    mut session: SimpleSession,
    settings: &Settings,
    client: &AsyncClient,
) -> Result<(), Box<dyn std::error::Error>> {
    let mut state = session.send_command(Command::Update).await?;
    let character_slug = slugify(&state.character.name);
    publish_discovery(client, &character_slug).await?;
    publish_state(client, &character_slug, &state, settings.publish_full_gamestate).await?;

    info!("Monitoring character '{}'", state.character.name);

    loop {
        sleep(settings.poll_interval).await;
        state = session.send_command(Command::Update).await?;
        publish_state(client, &character_slug, &state, settings.publish_full_gamestate).await?;
        info!(
            "Updated '{}' (level {}, gold {}, mushrooms {})",
            state.character.name,
            state.character.level,
            state.character.silver / 100,
            state.character.mushrooms
        );
    }
}

#[tokio::main]
async fn main() {
    env_logger::init();
    let settings = read_settings();

    let (client, event_loop) = mqtt_client(&settings).await;
    tokio::spawn(keep_event_loop_running(event_loop));

    loop {
        info!("Logging in to Shakes & Fidget account");
        match SimpleSession::login_sf_account(&settings.username, &settings.password).await {
            Ok(mut sessions) => {
                info!("S&F account returned {} character session(s)", sessions.len());

                let selected = if let Some(name) = &settings.character_name {
                    let mut selected = None;
                    for (idx, session) in sessions.iter_mut().enumerate() {
                        match session.send_command(Command::Update).await {
                            Ok(state) if state.character.name.eq_ignore_ascii_case(name) => {
                                selected = Some(idx);
                                break;
                            }
                            Ok(_) => {}
                            Err(err) => warn!("Could not inspect character session {idx}: {err}"),
                        }
                    }
                    selected
                } else {
                    Some(0)
                };

                if let Some(index) = selected {
                    let session = sessions.swap_remove(index);
                    if let Err(err) = run_session(session, &settings, &client).await {
                        error!("S&F session ended: {err}");
                    }
                } else {
                    error!(
                        "Configured character '{}' was not found",
                        settings.character_name.as_deref().unwrap_or("<unknown>")
                    );
                }
            }
            Err(err) => error!("S&F login failed: {err}"),
        }

        warn!("Retrying S&F login in 60 seconds");
        sleep(Duration::from_secs(60)).await;
    }
}
