# z-layouts

A [Zellij](https://zellij.dev) plugin for quickly searching
and switching between layouts.

Shoutout to [Lucas Rosa](https://github.com/rvcas) for the inspiration.

![usage](https://github.com/CraigglesO/z-layouts/raw/main/img/usage.png)

## Usage

- `Tab` to cycle through layout list
- `Up` and `Down` to cycle through layout list
- `Enter` to switch to the selected layout
- Start typing to filter the layout list
- `Esc` or `Ctrl + c` to exit

## NOTICE

Requires Zellij `v0.41.0+`

## Why?

I have a lot of unique projects I work on, and to quickly switch between them and have the correct software already ready to go, I use layouts. Instead of memorizing all of them and typing out commands, I figured a plugin using `ctrl + l` would be a good way to do it.

## Installation

Download `z-layouts.wasm` from the [latest release](https://github.com/CraigglesO/z-layouts/releases/latest)

- `mkdir -p ~/.config/zellij/plugins/`
- `mv z-layouts.wasm ~/.config/zellij/plugins/`

> You don't need to keep `z-layouts.wasm` at this specified location. It's just where I like to
> keep my zellij plugins.

### Quick Install

```sh
curl -L "https://github.com/CraigglesO/z-layouts/releases/latest/download/z-layouts.wasm" -o ~/.config/zellij/plugins/z-layouts.wasm
```

## Keybinding

Add the following to your [zellij config](https://zellij.dev/documentation/configuration.html)
somewhere inside the [keybinds](https://zellij.dev/documentation/keybindings.html) section:

```kdl
shared_except "locked" {
    bind "Ctrl l" {
        LaunchOrFocusPlugin "file:~/.config/zellij/plugins/z-layouts.wasm" {
            floating true
            ignore_case true
        }
    }
}
```

> You likely already have a `shared_except "locked"` section in your configs. Feel free to add `bind` there.

The `ignore_case` defaults to `false` if absent. If set to `true`, filtering the tab names ignores
the case of the filter string and the tab name.

## Contributing

If you find any issues or want to suggest ideas please [open an issue](https://github.com/CraigglesO/z-layouts/issues/new).

### Development

- Install [rust](https://rustup.rs/)
- Install **wasm32-wasi**: `rustup target add wasm32-wasi`

#### Build

```sh
cargo build --target wasm32-wasi
```

#### Test

```sh
zellij action new-tab --layout ./dev.kdl
```

### Release Guide

See [Release](./.github/workflows/release.yml)

Example release command:

```sh
git tag -a v1.0.0 -m "Version 1.0.0"
git push origin v1.0.0
```
