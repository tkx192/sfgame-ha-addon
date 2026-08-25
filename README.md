### Bitte noch nicht nutzen! ###
### App noch ohne Funktion! ###

# Shakes & Fidget Reader – Home Assistant App

Eine experimentelle Home Assistant App (früher Add-on) für read-only Monitoring eines **Shakes & Fidget**-Charakters.

Die S&F-Kommunikation wird nicht selbst neu implementiert. Das App verwendet das aktuelle inoffizielle Rust-Projekt [`the-marenga/sf-api`](https://github.com/the-marenga/sf-api) für Login, Session-Handling, Verschlüsselung und das Parsen des GameState.

## Installation

1. Dieses Repository auf GitHub (oder eine andere Git-Quelle) veröffentlichen.
2. In Home Assistant **Settings → Apps → App store → Repositories** die Repository-URL eintragen.
3. **Shakes & Fidget Reader** installieren.
4. Benutzername und Passwort des S&F-Accounts konfigurieren.
5. Optional einen konkreten Charakternamen eintragen.
6. App starten und Logs prüfen.

Das App nutzt den Home-Assistant-MQTT-Service. MQTT-Host und Zugangsdaten müssen deshalb nicht separat konfiguriert werden.

## Erste MQTT-Entities

Nach dem ersten erfolgreichen Login werden automatisch MQTT-Discovery-Entities für Charaktername, Level, Gold, Pilze, Ehre, Rang und Erfahrung angelegt.

Der Rohzustand liegt unter:

`sfgame/<character_slug>/state`
