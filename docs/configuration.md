# Configuration

PumpkinPlus uses a single **`config.json`** file stored in the plugin's data folder. It is created automatically on first load with all defaults. Edit it and restart the server to apply changes.

## Structure

```json
{
  "player_module": {
    "enabled": false,
    "join_msg": "",
    "leave_msg": "",
    "kick_msg": ""
  }
}
```

Each top-level key corresponds to one module. Setting `"enabled": false` disables that module entirely; its events and commands will not be registered.

## Placeholders

String fields that are displayed as in-game messages support the `{player}` placeholder:

| Placeholder | Replaced with             |
|-------------|---------------------------|
| `{player}`  | The player's in-game name |

## Adding new modules

When a new module is added to the plugin, its `Config` struct is added as a field on `ConfigManager`. Existing `config.json` files that are missing the new field will fail to deserialize — delete the file to regenerate it with all defaults, or add the missing field manually.
