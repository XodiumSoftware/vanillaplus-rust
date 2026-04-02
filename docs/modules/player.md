# Player

Handles custom messages shown to the server when a player joins, leaves, or is kicked during login.

## Configuration

| Field       | Default | Description                                                   |
|-------------|---------|---------------------------------------------------------------|
| `enabled`   | `false` | Whether this module is active                                 |
| `join_msg`  | `""`    | Message broadcast to the server when a player joins           |
| `leave_msg` | `""`    | Message broadcast to the server when a player leaves          |
| `kick_msg`  | `""`    | Message shown to the player when they are kicked during login |

If a message field is left empty, the corresponding default server behaviour is preserved.

## Placeholders

| Placeholder | Available in                        |
|-------------|-------------------------------------|
| `{player}`  | `join_msg`, `leave_msg`, `kick_msg` |

## Behaviour

- Listens on `PlayerJoinEvent`, `PlayerLeaveEvent`, and `PlayerLoginEvent` at `Highest` priority.
- Replaces the respective event message with the configured string (after placeholder substitution).
- If the configured string is empty, the event message is left unchanged.
