mod cli;
mod rain;

use clap::Parser;
use cli::{Cli, Rgb};
use crossterm::{
    cursor, event, execute, queue,
    style::{Color, Print, SetBackgroundColor, SetForegroundColor},
    terminal::{self, Clear, ClearType, EnterAlternateScreen, LeaveAlternateScreen},
};
use rain::{Rain, RainConfig};
use std::{
    io::{stdout, BufWriter, Stdout, Write},
    time::{Duration, Instant},
};
use unicode_width::UnicodeWidthChar;

/// Loop poll interval. ~30 fps is plenty for a rain effect and keeps CPU quiet.
const POLL_INTERVAL: Duration = Duration::from_millis(33);

fn to_color(c: Rgb) -> Color {
    Color::Rgb {
        r: c.r,
        g: c.g,
        b: c.b,
    }
}

/// RAII wrapper: enters raw mode + alternate screen on `new`, restores on drop.
/// This is what makes the terminal survive panics and Ctrl+C cleanly.
struct Term {
    out: BufWriter<Stdout>,
}

impl Term {
    fn new() -> std::io::Result<Self> {
        let mut out = BufWriter::with_capacity(64 * 1024, stdout());
        terminal::enable_raw_mode()?;
        execute!(out, EnterAlternateScreen, cursor::Hide, Clear(ClearType::All))?;
        Ok(Self { out })
    }
}

impl Drop for Term {
    fn drop(&mut self) {
        let _ = execute!(self.out, cursor::Show, LeaveAlternateScreen);
        let _ = terminal::disable_raw_mode();
    }
}

fn draw(rain: &Rain, term: &mut Term, bg: Option<Rgb>, char_width: usize) -> std::io::Result<()> {
    let out = &mut term.out;
    let grid = rain.render();
    let blank = " ".repeat(char_width);
    for (y, row) in grid.iter().enumerate() {
        queue!(out, cursor::MoveTo(0, y as u16))?;
        if let Some(bg) = bg {
            queue!(out, SetBackgroundColor(to_color(bg)))?;
        }
        for cell in row.iter() {
            match cell {
                Some(c) => {
                    queue!(out, SetForegroundColor(to_color(c.color)), Print(c.ch))?;
                }
                None => {
                    queue!(out, Print(&blank))?;
                }
            }
        }
    }
    out.flush()
}

fn is_exit(k: &event::KeyEvent) -> bool {
    use event::{KeyCode, KeyModifiers};
    matches!(
        k,
        event::KeyEvent { code: KeyCode::Esc, .. }
            | event::KeyEvent { code: KeyCode::Char('q' | 'Q'), .. }
            | event::KeyEvent {
                code: KeyCode::Char('c'),
                modifiers: KeyModifiers::CONTROL,
                ..
            }
    )
}

fn run(cli: Cli) -> std::io::Result<()> {
    let (w, h) = terminal::size()?;
    let chars: Vec<char> = cli.group.unwrap_or(cli.chars).chars().collect();
    let char_width = chars.first().and_then(|c| c.width()).unwrap_or(1).max(1);
    let cfg = RainConfig {
        body: cli.color,
        head: cli.head,
        fade_to: cli.fade_to,
        shade: cli.shade,
        speed: cli.speed.clone(),
        chars,
        char_width,
    };
    let mut rain = Rain::new(w as usize, h as usize, cfg.clone());
    let mut term = Term::new()?;

    if let Some(bg) = cli.bg {
        execute!(term.out, SetBackgroundColor(to_color(bg)), Clear(ClearType::All))?;
    }

    loop {
        if event::poll(POLL_INTERVAL)? {
            match event::read()? {
                event::Event::Key(k) if is_exit(&k) => break,
                event::Event::Resize(nw, nh) => {
                    rain = Rain::new(nw as usize, nh as usize, cfg.clone());
                    execute!(term.out, Clear(ClearType::All))?;
                }
                _ => {}
            }
        }
        if rain.tick(Instant::now()) {
            draw(&rain, &mut term, cli.bg, cfg.char_width)?;
        }
    }
    Ok(())
}

fn main() {
    let cli = Cli::parse();
    if let Err(e) = run(cli) {
        eprintln!("erro: {e}");
        std::process::exit(1);
    }
}
