//! Pure simulation. No I/O, no terminal. Everything here is testable with a seed.

use crate::cli::Rgb;
use rand::{rngs::StdRng, Rng, SeedableRng};
use std::{
    collections::VecDeque,
    ops::Range,
    time::{Duration, Instant},
};

/// Runtime configuration for the rain effect.
#[derive(Clone, Debug)]
pub struct RainConfig {
    pub body: Rgb,
    pub head: Rgb,
    pub fade_to: Rgb,
    pub shade: bool,
    pub speed: Range<u64>,
    pub chars: Vec<char>,
}

/// A rendered cell. `None` means empty (no drop here).
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct RenderCell {
    pub ch: char,
    pub color: Rgb,
}

/// One falling drop, one per column.
#[derive(Debug, Clone)]
struct Drop {
    /// Row of the head. Can be negative (drop is still above the screen).
    head_y: i32,
    /// Total visible length including the head.
    visible_len: usize,
    /// Chars currently visible. Index 0 = head, index N = row head_y - N.
    chars: VecDeque<char>,
    /// Milliseconds between advances for this drop.
    tick_ms: u64,
    /// Last time this drop advanced (used only by real-time tick()).
    last_tick: Instant,
}

impl Drop {
    fn spawn(rng: &mut StdRng, height: usize, speed: &Range<u64>, now: Instant) -> Self {
        // Trail length bounded by screen height, with a sane floor.
        let max_len = height.saturating_sub(2).max(4);
        let min_len = 4.min(max_len);
        let visible_len = if min_len == max_len {
            min_len
        } else {
            rng.gen_range(min_len..=max_len)
        };

        // Stagger spawn above the screen so drops enter at different times.
        let head_y = -(rng.gen_range(0..=height.max(1) as i32));
        let tick_ms = rng.gen_range(speed.clone());

        Drop {
            head_y,
            visible_len,
            chars: VecDeque::with_capacity(visible_len),
            tick_ms,
            last_tick: now,
        }
    }

    /// True once the whole trail has scrolled past the bottom row.
    fn is_dead(&self, height: usize) -> bool {
        // tail_y = head_y - (visible_len - 1). Dead when tail_y >= height.
        self.head_y >= height as i32 + self.visible_len as i32 - 1
    }
}

pub struct Rain {
    width: usize,
    height: usize,
    cfg: RainConfig,
    drops: Vec<Drop>,
    rng: StdRng,
}

impl Rain {
    /// Create a Rain with entropy-seeded RNG (production use).
    pub fn new(width: usize, height: usize, cfg: RainConfig) -> Self {
        let seed: u64 = rand::thread_rng().gen();
        Self::new_seeded(width, height, cfg, seed)
    }

    /// Create a Rain with a fixed seed (for tests and reproducibility).
    pub fn new_seeded(width: usize, height: usize, cfg: RainConfig, seed: u64) -> Self {
        let mut rng = StdRng::seed_from_u64(seed);
        let now = Instant::now();
        let drops = (0..width)
            .map(|_| Drop::spawn(&mut rng, height, &cfg.speed, now))
            .collect();
        Self {
            width,
            height,
            cfg,
            drops,
            rng,
        }
    }

    pub fn width(&self) -> usize {
        self.width
    }
    pub fn height(&self) -> usize {
        self.height
    }

    /// Real-time tick: advance only drops whose per-column timer has elapsed.
    /// Returns true if any drop advanced.
    pub fn tick(&mut self, now: Instant) -> bool {
        let mut advanced = false;
        for i in 0..self.drops.len() {
            let elapsed = now.duration_since(self.drops[i].last_tick);
            if elapsed >= Duration::from_millis(self.drops[i].tick_ms) {
                self.drops[i].last_tick = now;
                self.step_drop(i);
                advanced = true;
            }
        }
        advanced
    }

    /// Advance every drop exactly one step, ignoring timing. Used by tests.
    pub fn step_all(&mut self) {
        for i in 0..self.drops.len() {
            self.step_drop(i);
        }
    }

    fn step_drop(&mut self, i: usize) {
        let height = self.height;
        let pool_len = self.cfg.chars.len();
        let ch = self.cfg.chars[self.rng.gen_range(0..pool_len)];

        let needs_respawn = {
            let drop = &mut self.drops[i];
            drop.head_y += 1;
            drop.chars.push_front(ch);
            while drop.chars.len() > drop.visible_len {
                drop.chars.pop_back();
            }
            drop.is_dead(height)
        };

        if needs_respawn {
            let now = self.drops[i].last_tick;
            self.drops[i] = Drop::spawn(&mut self.rng, height, &self.cfg.speed, now);
        }
    }

    /// Build a 2D grid of cells. `None` = empty.
    pub fn render(&self) -> Vec<Vec<Option<RenderCell>>> {
        let mut grid = vec![vec![None; self.width]; self.height];
        for (col, drop) in self.drops.iter().enumerate() {
            for (offset, &ch) in drop.chars.iter().enumerate() {
                let y = drop.head_y - offset as i32;
                if y < 0 || y as usize >= self.height {
                    continue;
                }
                let color = if offset == 0 {
                    self.cfg.head
                } else if self.cfg.shade {
                    let t = offset as f32 / drop.visible_len.max(1) as f32;
                    self.cfg.body.lerp(self.cfg.fade_to, t)
                } else {
                    self.cfg.body
                };
                grid[y as usize][col] = Some(RenderCell { ch, color });
            }
        }
        grid
    }

    /// Render as plain ASCII for snapshot testing. '.' = empty.
    pub fn render_ascii(&self) -> String {
        let mut out = String::with_capacity((self.width + 1) * self.height);
        for row in self.render() {
            for cell in row {
                match cell {
                    Some(c) => out.push(c.ch),
                    None => out.push('.'),
                }
            }
            out.push('\n');
        }
        out
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn cfg() -> RainConfig {
        RainConfig {
            body: Rgb::new(0, 255, 70),
            head: Rgb::new(255, 255, 255),
            fade_to: Rgb::new(0, 0, 0),
            shade: true,
            speed: 40..180,
            chars: "01".chars().collect(),
        }
    }

    #[test]
    fn dimensions_are_stored() {
        let rain = Rain::new_seeded(20, 10, cfg(), 42);
        assert_eq!(rain.width(), 20);
        assert_eq!(rain.height(), 10);
    }

    #[test]
    fn same_seed_produces_same_output() {
        let mut a = Rain::new_seeded(12, 6, cfg(), 1234);
        let mut b = Rain::new_seeded(12, 6, cfg(), 1234);
        for _ in 0..20 {
            a.step_all();
            b.step_all();
        }
        assert_eq!(a.render_ascii(), b.render_ascii());
    }

    #[test]
    fn different_seeds_produce_different_output() {
        let mut a = Rain::new_seeded(20, 10, cfg(), 1);
        let mut b = Rain::new_seeded(20, 10, cfg(), 999);
        for _ in 0..30 {
            a.step_all();
            b.step_all();
        }
        assert_ne!(a.render_ascii(), b.render_ascii());
    }

    #[test]
    fn grid_shape_is_stable_across_steps() {
        let mut rain = Rain::new_seeded(8, 4, cfg(), 7);
        for _ in 0..50 {
            rain.step_all();
            let g = rain.render();
            assert_eq!(g.len(), 4);
            for row in &g {
                assert_eq!(row.len(), 8);
            }
        }
    }

    #[test]
    fn only_uses_chars_from_pool() {
        let pool: Vec<char> = "AB".chars().collect();
        let mut c = cfg();
        c.chars = pool.clone();
        let mut rain = Rain::new_seeded(10, 5, c, 42);
        for _ in 0..20 {
            rain.step_all();
        }
        for row in rain.render() {
            for cell in row {
                if let Some(c) = cell {
                    assert!(pool.contains(&c.ch), "char {:?} not in pool", c.ch);
                }
            }
        }
    }

    #[test]
    fn head_is_head_color_body_is_body_color_without_shade() {
        let mut c = cfg();
        c.shade = false;
        let mut rain = Rain::new_seeded(4, 8, c.clone(), 42);
        // Step enough that at least one drop has a visible head and body.
        for _ in 0..40 {
            rain.step_all();
        }
        let grid = rain.render();
        let mut saw_head = false;
        let mut saw_body = false;
        for row in &grid {
            for cell in row {
                if let Some(cell) = cell {
                    if cell.color == c.head {
                        saw_head = true;
                    }
                    if cell.color == c.body {
                        saw_body = true;
                    }
                }
            }
        }
        assert!(saw_head, "esperava ver ao menos uma cabeça");
        assert!(saw_body, "esperava ver ao menos um corpo");
    }

    #[test]
    fn snapshot_frame_50_30x10() {
        let mut rain = Rain::new_seeded(30, 10, cfg(), 42);
        for _ in 0..50 {
            rain.step_all();
        }
        insta::assert_snapshot!(rain.render_ascii());
    }

    #[test]
    fn snapshot_frame_100_20x8_no_shade() {
        let mut c = cfg();
        c.shade = false;
        let mut rain = Rain::new_seeded(20, 8, c, 7);
        for _ in 0..100 {
            rain.step_all();
        }
        insta::assert_snapshot!(rain.render_ascii());
    }
}
