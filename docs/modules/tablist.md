# Tablist

Provides custom tab-list header and footer displayed to players.

> **Status: Active** — Sets header/footer on player join. Waiting for upstream API to fully implement.

## Configuration

| Field     | Default | Description                                                             |
|-----------|---------|-------------------------------------------------------------------------|
| `enabled` | `false` | Whether this module is active                                           |
| `header`  | `""`    | Text displayed at the top of the tab list. Supports formatting codes    |
| `footer`  | `""`    | Text displayed at the bottom of the tab list. Supports formatting codes |

## Example

```json
{
  "tablist": {
    "enabled": true,
    "header": "\u00266Welcome to My Server!\u0026r\n\u00267Have fun!",
    "footer": "\u00267play.example.com"
  }
}
```

**Note:** Header and footer support Minecraft formatting codes (e.g., `\u00266` for gold, `\u00267` for gray, `\u0026r` to reset). Use `\n` for newlines.
