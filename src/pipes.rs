//! Pipes screensaver simulation. No I/O, no terminal.

use crate::cli::Rgb;
use crate::rain::RenderCell;
use rand::{rngs::StdRng, RngExt, SeedableRng};
use std::collections::VecDeque;
use std::time::{Duration, Instant};

const H:  char = '═';
const V:  char = '║';
const TL: char = '╔';
const TR: char = '╗';
const BL: char = '╚';
const BR: char = '╝';

const TURN_CHANCE:  f32 = 0.15;
const TRAIL_LEN:    usize = 12;
/// Per-tick decay multiplier applied to all live cells.
const DECAY_FACTOR: f32  = 0.95;
/// Cells below this brightness on all channels are cleared.
const DECAY_FLOOR:  u8   = 6;

#[derive(Clone, Copy, PartialEq, Eq)]
enum Dir { Up, Down, Left, Right }

impl Dir {
    fn delta(self) -> (i32, i32) {
        match self {
            Dir::Up    => ( 0, -1),
            Dir::Down  => ( 0,  1),
            Dir::Left  => (-1,  0),
            Dir::Right => ( 1,  0),
        }
    }

    fn perpendiculars(self) -> [Dir; 2] {
        match self {
            Dir::Up | Dir::Down    => [Dir::Left, Dir::Right],
            Dir::Left | Dir::Right => [Dir::Up,   Dir::Down],
        }
    }

    fn straight_char(self) -> char {
        match self { Dir::Left | Dir::Right => H, _ => V }
    }

    fn corner_char(self, next: Dir) -> char {
        match (self, next) {
            (Dir::Right, Dir::Down) | (Dir::Up,   Dir::Left)  => TR,
            (Dir::Right, Dir::Up)   | (Dir::Down, Dir::Left)  => BR,
            (Dir::Left,  Dir::Down) | (Dir::Up,   Dir::Right) => TL,
            (Dir::Left,  Dir::Up)   | (Dir::Down, Dir::Right) => BL,
            _ => self.straight_char(),
        }
    }
}

struct Pipe {
    x:         i32,
    y:         i32,
    dir:       Dir,
    last_tick: Instant,
    trail:     VecDeque<(i32, i32)>,
}

pub struct PipesConfig {
    pub tick_ms:   u64,
    pub num_pipes: usize,
    pub color:     Rgb,
    pub head:      Rgb,
}

pub struct Pipes {
    width:  u16,
    height: u16,
    cfg:    PipesConfig,
    pipes:  Vec<Pipe>,
    grid:   Vec<Vec<Option<RenderCell>>>,
    rng:    StdRng,
}

/// A single cell update: position + new content (None = clear to blank).
pub type CellUpdate = (u16, u16, Option<RenderCell>);

impl Pipes {
    pub fn new(width: u16, height: u16, cfg: PipesConfig) -> Self {
        let seed: u64 = rand::rng().random();
        let mut rng = StdRng::seed_from_u64(seed);
        let pipes = (0..cfg.num_pipes).map(|_| spawn(&mut rng, width, height)).collect();
        Self { width, height, cfg, pipes, grid: empty_grid(width, height), rng }
    }

    pub fn resize(&mut self, width: u16, height: u16) {
        self.width  = width;
        self.height = height;
        self.grid   = empty_grid(width, height);
        self.pipes  = (0..self.cfg.num_pipes).map(|_| spawn(&mut self.rng, width, height)).collect();
    }

    /// Returns the full grid for an initial or post-resize redraw.
    pub fn grid(&self) -> &[Vec<Option<RenderCell>>] {
        &self.grid
    }

    /// Advance simulation. Returns only the cells that changed this tick.
    pub fn tick(&mut self, now: Instant) -> Vec<CellUpdate> {
        let mut changed: Vec<CellUpdate> = Vec::new();
        let w = self.width as i32;
        let h = self.height as i32;

        // ── pipe movement ────────────────────────────────────────────────────
        for i in 0..self.pipes.len() {
            if now.duration_since(self.pipes[i].last_tick) < Duration::from_millis(self.cfg.tick_ms) {
                continue;
            }
            self.pipes[i].last_tick = now;

            let Pipe { x, y, dir, .. } = self.pipes[i];

            let (dx, dy) = dir.delta();
            let must_turn = {
                let (nx, ny) = (x + dx, y + dy);
                nx < 0 || nx >= w || ny < 0 || ny >= h
            };
            let want_turn = self.rng.random::<f32>() < TURN_CHANCE;

            let new_dir = if must_turn || want_turn {
                let [a, b] = dir.perpendiculars();
                let (adx, ady) = a.delta();
                let a_ok = x+adx >= 0 && x+adx < w && y+ady >= 0 && y+ady < h;
                let (bdx, bdy) = b.delta();
                let b_ok = x+bdx >= 0 && x+bdx < w && y+bdy >= 0 && y+bdy < h;
                match (a_ok, b_ok) {
                    (true, true)   => if self.rng.random::<bool>() { a } else { b },
                    (true, false)  => a,
                    (false, true)  => b,
                    (false, false) => {
                        self.pipes[i] = spawn(&mut self.rng, self.width, self.height);
                        continue;
                    }
                }
            } else {
                dir
            };

            // Paint current position as body.
            if x >= 0 && x < w && y >= 0 && y < h {
                let ch = if new_dir != dir { dir.corner_char(new_dir) } else { dir.straight_char() };
                let cell = Some(RenderCell { ch, color: self.cfg.color });
                self.grid[y as usize][x as usize] = cell;
                changed.push((x as u16, y as u16, cell));
            }

            self.pipes[i].trail.push_front((x, y));
            if self.pipes[i].trail.len() > TRAIL_LEN {
                self.pipes[i].trail.pop_back();
            }

            // Repaint trail gradient (head → color).
            for (j, &(tx, ty)) in self.pipes[i].trail.iter().enumerate() {
                if tx < 0 || tx >= w || ty < 0 || ty >= h { continue; }
                let t = j as f32 / (TRAIL_LEN - 1) as f32;
                let color = self.cfg.head.lerp(self.cfg.color, t);
                if let Some(cell) = &mut self.grid[ty as usize][tx as usize] {
                    cell.color = color;
                    changed.push((tx as u16, ty as u16, Some(RenderCell { ch: cell.ch, color })));
                }
            }

            // Advance and draw new head.
            let (dx, dy) = new_dir.delta();
            let (nx, ny) = (x + dx, y + dy);
            if nx >= 0 && nx < w && ny >= 0 && ny < h {
                let cell = Some(RenderCell { ch: new_dir.straight_char(), color: self.cfg.head });
                self.grid[ny as usize][nx as usize] = cell;
                changed.push((nx as u16, ny as u16, cell));
            }

            self.pipes[i].x   = nx;
            self.pipes[i].y   = ny;
            self.pipes[i].dir = new_dir;
        }

        // ── global decay ─────────────────────────────────────────────────────
        // Runs every tick with a gentle factor so the fade is smooth, not pulsed.
        for y in 0..self.height as usize {
            for x in 0..self.width as usize {
                if let Some(c) = &mut self.grid[y][x] {
                    let r = (c.color.r as f32 * DECAY_FACTOR) as u8;
                    let g = (c.color.g as f32 * DECAY_FACTOR) as u8;
                    let b = (c.color.b as f32 * DECAY_FACTOR) as u8;
                    if r < DECAY_FLOOR && g < DECAY_FLOOR && b < DECAY_FLOOR {
                        self.grid[y][x] = None;
                        changed.push((x as u16, y as u16, None));
                    } else {
                        c.color = Rgb { r, g, b };
                        changed.push((x as u16, y as u16, Some(*c)));
                    }
                }
            }
        }

        changed
    }
}

fn empty_grid(w: u16, h: u16) -> Vec<Vec<Option<RenderCell>>> {
    vec![vec![None; w as usize]; h as usize]
}

fn spawn(rng: &mut StdRng, width: u16, height: u16) -> Pipe {
    let dirs = [Dir::Up, Dir::Down, Dir::Left, Dir::Right];
    Pipe {
        x:         rng.random_range(0..width  as i32),
        y:         rng.random_range(0..height as i32),
        dir:       dirs[rng.random_range(0..4usize)],
        last_tick: Instant::now(),
        trail:     VecDeque::with_capacity(TRAIL_LEN),
    }
}
