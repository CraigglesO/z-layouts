# z-layouts

A [Zellij](https://zellij.dev) plugin for quickly searching
and switching between layouts.

![usage](https://github.com/CraigglesO/z-layouts/raw/main/img/usage.gif)

## Usage

- `Tab` to cycle through layout list
- `Up` and `Down` to cycle through layout list
- `Enter` to switch to the selected layout
- Start typing to filter the layout list
- `Esc` or `Ctrl + c` to exit

## Why?

I rename my tabs so once I have a lot of them I have to start
counting and then press `Ctrl + t` then `<tab num>`. So I wanted something
that let’s me type to filter the tab list and then press enter to jump to the selected tab.

## Installation

Download `z-layouts.wasm` from the [latest release](https://github.com/CraigglesO/z-layouts/releases/latest)

- `mkdir -p ~/.config/zellij/plugins/`
- `mv z-layouts.wasm ~/.config/zellij/plugins/`

> You don't need to keep `z-layouts.wasm` at this specified location. It's just where I like to
> keep my zellij plugins.

### Quick Install

```
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
