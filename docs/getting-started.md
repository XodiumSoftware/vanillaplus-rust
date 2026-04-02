# Getting Started

## Installation

1. Download the latest `pumpkinplus.wasm` from [GitHub Releases](https://github.com/XodiumSoftware/PumpkinPlus/releases).
2. Drop it into your Pumpkin server's `plugins/` folder.
3. Start (or restart) the server.

On first start, a `config.json` file is created in the plugin's data folder with all defaults. Edit it and restart to apply changes.

## Building from source

```bash
cargo build --release --target wasm32-wasip2
```

The output is at `target/wasm32-wasip2/release/pumpkinplus.wasm`.

## Customizing behaviour

All settings live in `config.json` in the plugin's data folder. See [Configuration](configuration.md) for the full reference and [Module reference](modules/index.md) for per-module fields.
