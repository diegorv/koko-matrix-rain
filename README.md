# koko-rain

Matrix-style rain effect for the terminal. Single binary, zero runtime dependencies, built with Rust.

Works on any terminal with true-color support (iTerm2, Ghostty, Kitty, Alacritty, WezTerm...).

## Install

```sh
cargo install --path .
```

Or build manually:

```sh
cargo build --release
# binary at target/release/koko-rain
```

## Usage

```sh
koko-rain                        # default green, 0/1 falling
koko-rain -s                     # enable tail fade
koko-rain -S 20,80               # fast rain
koko-rain -S 120,300             # slow, dramatic rain
```

Quit: `q`, `ESC` or `Ctrl+C`.

### Themes

```sh
# classic matrix — green katakana with fade on black
koko-rain -g jap -c green -H white -B black -s

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

# cmatrix style
koko-rain -g classic -c green -H white -B black -s

# moon phases on dark sky
koko-rain -g moon -c "200,200,255" -B "0,5,15" -s

# playing cards
koko-rain -g cards -c cyan -s

# emoji chaos
koko-rain -g emojis -c yellow -s
```

### Character groups (`-g`)

<details>
<summary>Available groups</summary>

| Group | Description |
|---|---|
| `all` | Most groups combined |
| `alphalow` / `alphaup` | Lowercase / uppercase alphabet |
| `arrow` | Arrow symbols |
| `bin` | Binary digits (0, 1) |
| `braille` | Braille dot patterns |
| `cards` | Playing card suits |
| `classic` | Katakana + digits + symbols (cmatrix style) |
| `clock` | Clock face emojis |
| `crab` | 🦀 |
| `dominosh` / `dominosv` | Horizontal / vertical domino tiles |
| `earth` | 🌍🌎🌏 |
| `emojis` | Broad emoji set |
| `jap` / `katakana` | Half-width Japanese katakana |
| `large-letters` | Full-width Latin (Ａ-Ｚ) |
| `moon` | Moon phase emojis |
| `num` / `digits` | Digits (0-9) |
| `numbered-balls` | Circled numbers (①-⑳) |
| `numbered-cubes` | Squared letters (🅰-🆈) |
| `plants` | Plant and fruit emojis |
| `shapes` | Colored squares and circles |
| `smile` | Smiley face emojis |

</details>

### Custom characters (`--chars`)

```sh
koko-rain --chars "ABCDEF0123456789"               # hex
koko-rain --chars "!@#$%&*+-=~^"                   # symbols
koko-rain --chars "∑∏∫∂√∞≈≠≤≥" -c cyan -s          # math
koko-rain --chars "🔥💀👾🤖💎⚡" -c yellow             # emoji rain
```

### All options

Run `koko-rain --help` for the full list of flags (`--color`, `--head`, `--bg`, `--shade`, `--fade-to`, `--speed`).

## Cross-compilation (Apple Silicon + Intel)

```sh
rustup target add x86_64-apple-darwin
cargo build --release --target aarch64-apple-darwin
cargo build --release --target x86_64-apple-darwin
lipo -create \
  target/aarch64-apple-darwin/release/koko-rain \
  target/x86_64-apple-darwin/release/koko-rain \
  -output koko-rain
```

## Testing

```sh
cargo test                    # run all tests, including snapshots
cargo insta review            # review snapshot changes
```

## Project structure

| File | Responsibility |
|---|---|
| `src/cli.rs` | Args, color and speed parsing |
| `src/rain.rs` | Pure simulation (no I/O, fully testable with seed) |
| `src/main.rs` | Terminal setup + render loop |

## Inspiration & Attribution

This project was inspired by [rusty-rain](https://github.com/cowboy8625/rusty-rain). No code was copied — everything was written from scratch.

The character groups (`-g`) use Unicode ranges sourced from:

| What | Source | License |
|---|---|---|
| Unicode ranges for all groups (katakana, emoji, cards, etc.) | [ezemoji](https://github.com/cowboy8625/ezemoji) crate by cowboy8625 | MIT |
| "classic" group composition (katakana + digits + symbols) | Inspired by [rusty-rain](https://github.com/cowboy8625/rusty-rain)'s cmatrix-style group | Apache-2.0 |

## License

MIT
