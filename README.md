# rain

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

The binary will be at `target/release/rain`.

## Install

### Via cargo install (recommended)

```sh
cargo install --path .
```

This compiles in release mode and copies the binary to `~/.cargo/bin/rain`.

### Add to zsh PATH

If `~/.cargo/bin` is not already in your PATH, add it to `~/.zshrc`:

```sh
echo 'export PATH="$HOME/.cargo/bin:$PATH"' >> ~/.zshrc
source ~/.zshrc
```

Verify:

```sh
which rain    # should print ~/.cargo/bin/rain
rain --version
```

### Manual install

If you prefer placing the binary elsewhere:

```sh
cargo build --release
cp target/release/rain /usr/local/bin/
```

## Usage

```sh
rain                                          # default green, 0/1 falling
rain -c "0,255,70" -H white -s               # shade enabled
rain -c cyan -H white -B black -s            # black background, cyan with fade
rain --chars "ABCDEF0123456789"              # hex character pool
rain -S 20,80                                # fast rain
rain -c red --chars "01" -G "40,0,0"         # red fading to dark brown
```

Quit: `q`, `ESC` or `Ctrl+C`.

## Flags

| Flag | Default | Description |
|---|---|---|
| `-c, --color` | `green` | Body color (name or `R,G,B`) |
| `-H, --head` | `white` | Head character color |
| `-B, --bg` | -- | Background color |
| `-s, --shade` | off | Enable tail fade |
| `-G, --fade-to` | `black` | Fade target color |
| `-S, --speed` | `40,180` | ms between ticks per column (`min,max`) |
| `--chars` | `01` | Character pool |

Named colors: `black` `white` `red` `green` `blue` `yellow` `cyan` `magenta` `orange` `purple`.

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

To distribute, just send the `target/release/rain` file. The recipient can place it anywhere on their PATH:

```sh
chmod +x rain
mv rain /usr/local/bin/
```

### Cross-compilation (Apple Silicon + Intel)

To build a universal binary that runs on both architectures:

```sh
rustup target add x86_64-apple-darwin
cargo build --release --target aarch64-apple-darwin
cargo build --release --target x86_64-apple-darwin
lipo -create \
  target/aarch64-apple-darwin/release/rain \
  target/x86_64-apple-darwin/release/rain \
  -output rain
```

## Project structure

| File | Responsibility |
|---|---|
| `src/cli.rs` | Args, color and speed parsing |
| `src/rain.rs` | Pure simulation (no I/O, fully testable with seed) |
| `src/main.rs` | Terminal setup + render loop |

## License

MIT
