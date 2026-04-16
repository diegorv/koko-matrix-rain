# koko-rain

Matrix-style rain effect for the terminal. Single binary, zero runtime dependencies, built with Rust.

Works on any terminal with true-color support (iTerm2, Ghostty, Kitty, Alacritty, WezTerm...).

## Requirements

- [Rust](https://www.rust-lang.org/tools/install) >= 1.70

```sh
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```

## Build

Development build (fast compile, no optimizations):

```sh
cargo build
```

Release build (optimized binary with LTO and stripped symbols):

```sh
cargo build --release
```

The binary will be at `target/release/koko-rain`.

## Install

### Via cargo install (recommended)

```sh
cargo install --path .
```

This compiles in release mode and copies the binary to `~/.cargo/bin/koko-rain`.

### Add to zsh PATH

If `~/.cargo/bin` is not already in your PATH, add it to `~/.zshrc`:

```sh
echo 'export PATH="$HOME/.cargo/bin:$PATH"' >> ~/.zshrc
source ~/.zshrc
```

Verify:

```sh
which koko-rain    # should print ~/.cargo/bin/koko-rain
koko-rain --version
```

### Manual install

If you prefer placing the binary elsewhere:

```sh
cargo build --release
cp target/release/koko-rain /usr/local/bin/
```

## Usage

```sh
# basic
koko-rain                        # default green, 0/1 falling
koko-rain -s                     # enable tail fade
koko-rain -S 20,80               # fast rain
koko-rain -S 120,300             # slow, dramatic rain
```

### Themes

```sh
# classic matrix — green katakana with fade on black
koko-rain -c green -H white -B black -s --chars "アイウエオカキクケコサシスセソタチツテト"

# ocean — cyan fading to deep blue
koko-rain -c cyan -H white -B "0,0,30" -s -G "0,0,80" --chars "~-=≈"

# fire — red/orange fading to dark red
koko-rain -c "255,100,0" -H "255,255,100" -B black -s -G "80,0,0"

# purple haze — magenta fading to dark purple
koko-rain -c magenta -H white -B "10,0,20" -s -G "30,0,50"

# gold — amber on black
koko-rain -c "255,180,0" -H "255,255,150" -B black -s -G "60,30,0"

# arctic — white fading to ice blue
koko-rain -c white -H white -B "0,5,15" -s -G "0,40,80" --chars ".:*+="

# blood — dark red binary
koko-rain -c "180,0,0" -H "255,50,50" -B black -s -G "40,0,0"
```

### Character sets

```sh
koko-rain --chars "ABCDEF0123456789"               # hex
koko-rain --chars "!@#$%&*+-=~^"                   # symbols
koko-rain --chars "01"                              # binary (default)
koko-rain --chars "アイウエオカキクケコサシスセソ" -s  # katakana
koko-rain --chars "🔥💀👾🤖💎⚡" -c yellow             # emoji rain
koko-rain --chars "∑∏∫∂√∞≈≠≤≥" -c cyan -s          # math
```

Quit: `q`, `ESC` or `Ctrl+C`.

## Flags

**`-c, --color <COLOR>`**
Body color of the rain trails. Accepts a named color or `R,G,B` tuple.
`[default: green]`

**`-H, --head <COLOR>`**
Color of the leading (first) character in each column.
`[default: white]`

**`-B, --bg <COLOR>`**
Background color. When omitted the terminal's default background is used.

**`-s, --shade`**
Enable tail fade. Each cell in the trail gradually blends from the body color toward the fade target.

**`-G, --fade-to <COLOR>`**
Target color for the tail fade. Only visible when `--shade` is enabled.
`[default: black]`

**`-S, --speed <MIN,MAX>`**
Tick interval range in milliseconds. Each column picks a random speed within this range, so lower values produce faster rain.
`[default: 40,180]`

**`--chars <STRING>`**
Character pool used to generate the rain. Each tick picks a random character from this string.
`[default: 01]`

### Named colors

`black` `white` `red` `green` `blue` `yellow` `cyan` `magenta` `orange` `purple`

Any flag that accepts `<COLOR>` also takes an RGB tuple like `"0,255,70"`.

## Testing

```sh
cargo test                    # run all tests, including snapshots
cargo insta review            # review snapshot changes
```

## Distributing

To produce a standalone binary that runs on any Mac without Rust installed:

```sh
cargo build --release
```

The release profile is already configured with LTO, strip, and codegen-units=1 -- the binary comes out small with no debug symbols.

To distribute, just send the `target/release/koko-rain` file. The recipient can place it anywhere on their PATH:

```sh
chmod +x koko-rain
mv koko-rain /usr/local/bin/
```

### Cross-compilation (Apple Silicon + Intel)

To build a universal binary that runs on both architectures:

```sh
rustup target add x86_64-apple-darwin
cargo build --release --target aarch64-apple-darwin
cargo build --release --target x86_64-apple-darwin
lipo -create \
  target/aarch64-apple-darwin/release/koko-rain \
  target/x86_64-apple-darwin/release/koko-rain \
  -output koko-rain
```

## Project structure

| File | Responsibility |
|---|---|
| `src/cli.rs` | Args, color and speed parsing |
| `src/rain.rs` | Pure simulation (no I/O, fully testable with seed) |
| `src/main.rs` | Terminal setup + render loop |

## Inspiration

This project was inspired by [rusty-rain](https://github.com/cowboy8625/rusty-rain). No code was copied — everything was written from scratch.

## License

MIT
