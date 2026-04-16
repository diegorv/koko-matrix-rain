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

### Character groups (`-g, --group`)

Use `-g` to pick a predefined character set instead of `--chars`:

```sh
koko-rain -g jap -s          # half-width katakana
koko-rain -g emojis          # random emojis
koko-rain -g cards -c cyan   # playing cards
```

| Group | Description |
|---|---|
| `all` | Most groups combined |
| `alphalow` | Lowercase alphabet (a-z) |
| `alphaup` | Uppercase alphabet (A-Z) |
| `arrow` | Arrow symbols |
| `bin` | Binary digits (0, 1) |
| `braille` | Braille dot patterns |
| `cards` | Playing card suits |
| `classic` | Katakana + digits + symbols (cmatrix style) |
| `clock` | Clock face emojis |
| `crab` | 🦀 |
| `dominosh` | Horizontal domino tiles |
| `dominosv` | Vertical domino tiles |
| `earth` | 🌍🌎🌏 |
| `emojis` | Broad emoji set |
| `jap` / `katakana` | Half-width Japanese katakana |
| `large-letters` | Full-width Latin (Ａ-Ｚ) |
| `moon` | Moon phase emojis |
| `num` / `digits` | Digits (0-9) |
| `numbered-balls` | Circled numbers (①-⑳) |
| `numbered-cubes` / `lettered-cubes` | Squared letters (🅰-🆈) |
| `plants` | Plant and fruit emojis |
| `shapes` | Colored squares and circles |
| `smile` | Smiley face emojis |

### Custom characters (`--chars`)

```sh
koko-rain --chars "ABCDEF0123456789"               # hex
koko-rain --chars "!@#$%&*+-=~^"                   # symbols
koko-rain --chars "∑∏∫∂√∞≈≠≤≥" -c cyan -s          # math
koko-rain --chars "🔥💀👾🤖💎⚡" -c yellow             # emoji rain
```

Quit: `q`, `ESC` or `Ctrl+C`.

## Customization

<details>
<summary>Full CLI Options</summary>

```
Efeito de chuva Matrix minimalista para o terminal

Usage: koko-rain [OPTIONS]

Options:
  -c, --color <COLOR>
          Set the body color of the rain trails.
          Named colors: black, white, red, green, blue, yellow, cyan, magenta, orange, purple
          Or an RGB tuple: "R,G,B" (e.g. "0,255,70")

          [default: green]

  -H, --head <HEAD>
          Set the color of the leading (head) character in each column.
          Accepts the same color formats as --color.

          [default: white]

  -B, --bg <BG>
          Set the background color.
          When omitted the terminal's default background is used.
          Accepts the same color formats as --color.

  -s, --shade
          Enable tail fade.
          Each cell in the trail gradually blends from the body color toward the fade target (--fade-to).

  -G, --fade-to <FADE_TO>
          Set the target color for the tail fade. Only visible when --shade is enabled.
          Accepts the same color formats as --color.

          [default: black]

  -S, --speed <SPEED>
          Set the tick interval range in milliseconds (format: "min,max").
          Each column picks a random speed within this range.
          Lower values = faster rain, higher values = slower rain.
          Examples: "20,80" (fast), "40,180" (default), "120,300" (slow)

          [default: 40,180]

      --chars <CHARS>
          Set a custom character pool for the rain.
          Each tick picks a random character from this string.
          Supports ASCII, Unicode, and emoji.
          Conflicts with --group.

          [default: 01]

  -g, --group <GROUP>
          Use a predefined character group instead of --chars.
          Available groups:

            all             Most groups combined
            alphalow        Lowercase alphabet (a-z)
            alphaup         Uppercase alphabet (A-Z)
            arrow           Arrow symbols
            bin             Binary digits (0, 1)
            braille         Braille dot patterns
            cards           Playing card suits
            classic         Katakana + digits + symbols (cmatrix style)
            clock           Clock face emojis
            crab            🦀
            dominosh        Horizontal domino tiles
            dominosv        Vertical domino tiles
            earth           🌍🌎🌏
            emojis          Broad emoji set
            jap / katakana  Half-width Japanese katakana
            large-letters   Full-width Latin letters
            moon            Moon phase emojis
            num / digits    Digits (0-9)
            numbered-balls  Circled numbers
            numbered-cubes  Squared letters
            plants          Plant and fruit emojis
            shapes          Colored squares and circles
            smile           Smiley face emojis

  -h, --help
          Print help (see a summary with '-h')

  -V, --version
          Print version
```

</details>

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
