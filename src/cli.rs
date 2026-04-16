use clap::Parser;
use std::ops::Range;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Rgb {
    pub r: u8,
    pub g: u8,
    pub b: u8,
}

impl Rgb {
    pub const fn new(r: u8, g: u8, b: u8) -> Self {
        Self { r, g, b }
    }

    /// Parse a color from either a named preset or "R,G,B".
    pub fn parse(s: &str) -> Result<Self, String> {
        let lower = s.to_lowercase();
        let named: Option<(u8, u8, u8)> = match lower.as_str() {
            "black"   => Some((0, 0, 0)),
            "white"   => Some((255, 255, 255)),
            "red"     => Some((255, 0, 0)),
            "green"   => Some((0, 255, 70)),
            "blue"    => Some((40, 80, 255)),
            "yellow"  => Some((255, 255, 0)),
            "cyan"    => Some((0, 255, 255)),
            "magenta" => Some((255, 0, 255)),
            "orange"  => Some((255, 140, 0)),
            "purple"  => Some((160, 40, 220)),
            _ => None,
        };
        if let Some((r, g, b)) = named {
            return Ok(Rgb { r, g, b });
        }
        let parts: Vec<&str> = s.split(',').collect();
        if parts.len() != 3 {
            return Err(format!(
                "cor inválida: '{s}' (use um nome ou 'R,G,B')"
            ));
        }
        let r = parts[0].trim().parse::<u8>().map_err(|e| format!("R: {e}"))?;
        let g = parts[1].trim().parse::<u8>().map_err(|e| format!("G: {e}"))?;
        let b = parts[2].trim().parse::<u8>().map_err(|e| format!("B: {e}"))?;
        Ok(Rgb { r, g, b })
    }

    /// Linear interpolation between two colors. t in [0,1].
    pub fn lerp(self, other: Self, t: f32) -> Self {
        let t = t.clamp(0.0, 1.0);
        let s = 1.0 - t;
        Rgb {
            r: (self.r as f32 * s + other.r as f32 * t).round() as u8,
            g: (self.g as f32 * s + other.g as f32 * t).round() as u8,
            b: (self.b as f32 * s + other.b as f32 * t).round() as u8,
        }
    }
}

/// Parse "min,max" into an exclusive Range<u64>.
fn parse_speed(s: &str) -> Result<Range<u64>, String> {
    let parts: Vec<&str> = s.split(',').collect();
    if parts.len() != 2 {
        return Err(format!("formato inválido: '{s}' (use min,max)"));
    }
    let min: u64 = parts[0]
        .trim()
        .parse()
        .map_err(|e: std::num::ParseIntError| format!("min: {e}"))?;
    let max: u64 = parts[1]
        .trim()
        .parse()
        .map_err(|e: std::num::ParseIntError| format!("max: {e}"))?;
    if min >= max {
        return Err("min deve ser menor que max".into());
    }
    Ok(min..max)
}

fn parse_chars(s: &str) -> Result<String, String> {
    if s.is_empty() {
        return Err("--chars não pode ser vazio".into());
    }
    Ok(s.to_string())
}

#[derive(Parser, Debug, Clone)]
#[command(
    name = "rain",
    about = "Efeito de chuva Matrix minimalista para o terminal",
    version
)]
pub struct Cli {
    /// Cor do corpo (nome ou "R,G,B")
    #[arg(short = 'c', long, default_value = "green", value_parser = Rgb::parse)]
    pub color: Rgb,

    /// Cor da cabeça (primeiro caractere da coluna)
    #[arg(short = 'H', long, default_value = "white", value_parser = Rgb::parse)]
    pub head: Rgb,

    /// Cor de fundo (opcional)
    #[arg(short = 'B', long, value_parser = Rgb::parse)]
    pub bg: Option<Rgb>,

    /// Ativa o fade da cauda
    #[arg(short = 's', long)]
    pub shade: bool,

    /// Cor-alvo do fade (pra onde a cauda desbota)
    #[arg(short = 'G', long, default_value = "black", value_parser = Rgb::parse)]
    pub fade_to: Rgb,

    /// Velocidade em ms, formato "min,max"
    #[arg(short = 'S', long, default_value = "40,180", value_parser = parse_speed)]
    pub speed: Range<u64>,

    /// Pool de caracteres a usar na chuva
    #[arg(long, default_value = "01", value_parser = parse_chars)]
    pub chars: String,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parses_named_colors() {
        assert_eq!(Rgb::parse("green").unwrap(), Rgb::new(0, 255, 70));
        assert_eq!(Rgb::parse("WHITE").unwrap(), Rgb::new(255, 255, 255));
        assert_eq!(Rgb::parse("Black").unwrap(), Rgb::new(0, 0, 0));
    }

    #[test]
    fn parses_rgb_tuples() {
        assert_eq!(Rgb::parse("10,20,30").unwrap(), Rgb::new(10, 20, 30));
        assert_eq!(Rgb::parse("  0 , 128 , 255 ").unwrap(), Rgb::new(0, 128, 255));
    }

    #[test]
    fn rejects_bad_colors() {
        assert!(Rgb::parse("not-a-color").is_err());
        assert!(Rgb::parse("1,2").is_err());
        assert!(Rgb::parse("999,0,0").is_err());
        assert!(Rgb::parse("").is_err());
    }

    #[test]
    fn lerp_endpoints_and_midpoint() {
        let a = Rgb::new(0, 0, 0);
        let b = Rgb::new(200, 100, 50);
        assert_eq!(a.lerp(b, 0.0), a);
        assert_eq!(a.lerp(b, 1.0), b);
        assert_eq!(a.lerp(b, 0.5), Rgb::new(100, 50, 25));
    }

    #[test]
    fn lerp_clamps_out_of_range() {
        let a = Rgb::new(10, 10, 10);
        let b = Rgb::new(90, 90, 90);
        assert_eq!(a.lerp(b, -1.0), a);
        assert_eq!(a.lerp(b, 2.0), b);
    }

    #[test]
    fn speed_parses_valid() {
        assert_eq!(parse_speed("40,180").unwrap(), 40..180);
    }

    #[test]
    fn speed_rejects_invalid() {
        assert!(parse_speed("100,50").is_err());
        assert!(parse_speed("bad").is_err());
        assert!(parse_speed("1,2,3").is_err());
    }

    #[test]
    fn chars_rejects_empty() {
        assert!(parse_chars("").is_err());
        assert_eq!(parse_chars("01").unwrap(), "01");
    }
}
