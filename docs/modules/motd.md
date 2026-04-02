# MOTD

Sets a custom server list MOTD (the lines shown below the server name in the multiplayer screen).

> **Status: Stub** — The required Pumpkin API (`server.set_motd`) is not yet available. This module has no effect until the API is implemented.

## Configuration

| Field     | Default | Description                                |
|-----------|---------|--------------------------------------------|
| `enabled` | `false` | Whether this module is active              |
| `motd`    | `[]`    | Lines of text displayed in the server list |

## Behaviour

- Once the API is available, the configured `motd` lines will be joined with newlines and set as the server's MOTD.
