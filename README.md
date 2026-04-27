# zellij-spawn-silent

A Zellij plugin that spawns a command pane without stealing focus.

Send a JSON payload via `zellij pipe` and the plugin opens the command in a
background pane, then reveals it without taking focus away from your current
pane.

## Usage

```bash
zellij pipe --plugin zellij-spawn-silent -- '{"command":"htop"}'
```

Full example with all fields:

```bash
zellij pipe --plugin zellij-spawn-silent -- '{
  "command": "htop",
  "args": ["-d", "10"],
  "cwd": "/tmp",
  "float": true,
  "direction": "right"
}'
```

## JSON fields

| Field       | Type             | Required | Default | Description                          |
| ----------- | ---------------- | -------- | ------- | ------------------------------------ |
| `command`   | string           | yes      |         | Command to run                       |
| `args`      | array of strings | no       | `[]`    | Arguments passed to the command      |
| `cwd`       | string           | no       | none    | Working directory for the command    |
| `float`     | bool             | no       | `false` | Open as a floating pane              |
| `direction` | string           | no       | none    | Split direction (e.g. `"right"`)     |

## Building

Requires the `wasm32-wasip1` target:

```bash
rustup target add wasm32-wasip1
cargo build --target wasm32-wasip1
```

The compiled plugin is at `target/wasm32-wasip1/debug/zellij_spawn_silent.wasm`.

## License

MIT
