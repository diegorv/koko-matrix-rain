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

## Quick start

```sh
koko-rain                        # default green, 0/1 falling
koko-rain -s                     # enable tail fade
koko-rain -S 20,80               # fast rain
koko-rain -S 120,300             # slow, dramatic rain
```

Quit: `q`, `ESC` or `Ctrl+C`.

## Characters

### Groups (`-g`)

Use `-g` to pick a predefined character set:

```sh
koko-rain -g jap -s              # half-width katakana
koko-rain -g emojis -c yellow    # random emojis
koko-rain -g cards -c cyan       # playing cards
koko-rain -g classic -B black -s # cmatrix style
```

<details>
<summary>All available groups</summary>

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

### Custom (`--chars`)

Or pass your own character pool (conflicts with `-g`):

```sh
koko-rain --chars "ABCDEF0123456789"               # hex
koko-rain --chars "!@#$%&*+-=~^"                   # symbols
koko-rain --chars "∑∏∫∂√∞≈≠≤≥" -c cyan -s          # math
koko-rain --chars "🔥💀👾🤖💎⚡" -c yellow             # emoji rain
```

## Colors

```sh
koko-rain -c cyan                # body color by name
koko-rain -c "255,100,0"        # body color by RGB
koko-rain -H white               # head (leading char) color
koko-rain -B black               # background color
koko-rain -s                     # enable tail fade
koko-rain -G "40,0,0"           # fade target (used with -s)
```

Named colors: `black` `white` `red` `green` `blue` `yellow` `cyan` `magenta` `orange` `purple`

Any color flag also accepts an RGB tuple like `"R,G,B"`.

## Themes

Combine colors, characters, and speed for different looks:

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

# moon phases on dark sky
koko-rain -g moon -c "200,200,255" -B "0,5,15" -s

# emoji chaos
koko-rain -g emojis -c yellow -s
```

## All flags

Run `koko-rain --help` for full details.

| Flag | Description | Default |
|---|---|---|
| `-c, --color` | Body color (name or `R,G,B`) | `green` |
| `-H, --head` | Head character color | `white` |
| `-B, --bg` | Background color | terminal default |
| `-s, --shade` | Enable tail fade | off |
| `-G, --fade-to` | Fade target color | `black` |
| `-S, --speed` | Tick range in ms (`min,max`) | `40,180` |
| `-g, --group` | Predefined character group | — |
| `--chars` | Custom character pool | `01` |

## Development

### Testing

```sh
cargo test                    # run all tests, including snapshots
cargo insta review            # review snapshot changes
```

### Project structure

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
