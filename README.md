# rain

Minimal Matrix-style rain effect for the terminal. Built for macOS (iTerm2 / Ghostty), one binary, zero config.

## Install

```sh
cargo install --path .
```

Or build a release binary:

```sh
cargo build --release
# binário em target/release/rain
```

## Uso

```sh
rain                                          # verde padrão, 0/1 caindo
rain -c "0,255,70" -H white -s                # shade ativado
rain -c cyan -H white -B black -s             # fundo preto, ciano com fade
rain --chars "ABCDEF0123456789"               # pool hexadecimal
rain -S 20,80                                 # chuva rápida
rain -c red --chars "01" -G "40,0,0"          # vermelho desbotando pra marrom
```

Sair: `q`, `ESC` ou `Ctrl+C`.

## Flags

| Flag | Padrão | Descrição |
|---|---|---|
| `-c, --color` | `green` | Cor do corpo (nome ou `R,G,B`) |
| `-H, --head` | `white` | Cor do primeiro char |
| `-B, --bg` | — | Cor de fundo |
| `-s, --shade` | off | Ativa o fade da cauda |
| `-G, --fade-to` | `black` | Cor-alvo do fade |
| `-S, --speed` | `40,180` | ms entre ticks por coluna (`min,max`) |
| `--chars` | `01` | Pool de caracteres |

Cores nomeadas: `black white red green blue yellow cyan magenta orange purple`.

## Desenvolvimento

```sh
cargo test                    # roda tudo, incluindo snapshots
cargo insta review            # revisa mudanças de snapshot
cargo run -- -c cyan -s       # testa visualmente
```

Estrutura:

- `src/cli.rs` — args, parsing de cor e velocidade
- `src/rain.rs` — simulação pura (sem I/O, totalmente testável com seed)
- `src/main.rs` — setup do terminal + loop de desenho

## Licença

MIT
