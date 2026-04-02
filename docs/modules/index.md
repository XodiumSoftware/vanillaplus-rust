# Modules

PumpkinPlus is built around independent modules. Each module is controlled by an `enabled` flag in `config.json`.

## All modules

| Module                | Summary                               | Commands   | Status |
|-----------------------|---------------------------------------|------------|--------|
| [Player](player.md)   | Custom join, leave, and kick messages | —          | Active |
| [MOTD](motd.md)       | Custom server list MOTD               | —          | Stub   |
| [Tablist](tablist.md) | Custom tab-list header/footer         | —          | Stub   |
| [Locator](locator.md) | Personalise locator bar colour        | `/locator` | Stub   |

## Configuration

All module settings live in `config.json` in the plugin's data folder. Each module's section contains at minimum an `enabled` boolean. See [Configuration](../configuration.md) for details.
