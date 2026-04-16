use clap::Parser;
use std::ops::Range;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Rgb {
    pub r: u8,
    pub g: u8,
    pub b: u8,
}

impl Rgb {
    #[cfg(test)]
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
                "invalid color: '{s}' (use a name or 'R,G,B')"
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
        return Err(format!("invalid format: '{s}' (use min,max)"));
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
        return Err("min must be less than max".into());
    }
    Ok(min..max)
}

fn parse_chars(s: &str) -> Result<String, String> {
    if s.is_empty() {
        return Err("--chars must not be empty".into());
    }
    Ok(s.to_string())
}

fn chars_from_ranges(ranges: &[Range<u32>]) -> String {
    ranges
        .iter()
        .flat_map(|r| r.clone())
        .filter_map(char::from_u32)
        .collect()
}

fn parse_group(s: &str) -> Result<String, String> {
    let chars = match s.to_lowercase().as_str() {
        "all" => chars_from_ranges(&[
            // double-width first so char_width detection picks 2
            129_024..129_036, 129_040..129_096, 129_104..129_113,
            127_130..127_145, 127_148..127_162, 127_163..127_178, 127_179..127_240,
            128_336..128_360, 127_024..127_073, 127_074..127_123,
            129_292..129_401, 129_402..129_483, 129_484..129_536,
            65_382..65_437, 65_313..65_338,
            127_761..127_773, 127_757..127_760,
            9_312..9_332, 127_344..127_369, 127_793..127_827,
            128_512..128_518, 128_992..129_003,
            97..123, 65..91, 48..58,
        ]),
        "alphalow"  => chars_from_ranges(&[97..123]),
        "alphaup"   => chars_from_ranges(&[65..91]),
        "arrow"     => chars_from_ranges(&[129_024..129_036, 129_040..129_096, 129_104..129_113]),
        "bin"       => chars_from_ranges(&[48..50]),
        "braille"   => chars_from_ranges(&[10_241..10_252]),
        "cards"     => chars_from_ranges(&[127_130..127_145, 127_148..127_162, 127_163..127_178, 127_179..127_240]),
        "classic"   => chars_from_ranges(&[65_382..65_437, 48..58, 33..48, 58..64, 124..127]),
        "clock"     => chars_from_ranges(&[128_336..128_360]),
        "crab"      => chars_from_ranges(&[129_408..129_409]),
        "dominosh"  => chars_from_ranges(&[127_024..127_073]),
        "dominosv"  => chars_from_ranges(&[127_074..127_123]),
        "earth"     => chars_from_ranges(&[127_757..127_760]),
        "emojis"    => chars_from_ranges(&[129_292..129_401, 129_402..129_483, 129_484..129_536]),
        "jap" | "katakana" => chars_from_ranges(&[65_382..65_437]),
        "large-letters"    => chars_from_ranges(&[65_313..65_338]),
        "moon"      => chars_from_ranges(&[127_761..127_773]),
        "num" | "digits"   => chars_from_ranges(&[48..58]),
        "numbered-balls"   => chars_from_ranges(&[9_312..9_332]),
        "numbered-cubes" | "lettered-cubes" => chars_from_ranges(&[127_344..127_369]),
        "plants"    => chars_from_ranges(&[127_793..127_827]),
        "shapes"    => chars_from_ranges(&[128_992..129_003]),
        "smile"     => chars_from_ranges(&[128_512..128_518]),
        _ => {
            return Err(format!(
                "unknown group: '{s}'\n\
                 available: all, alphalow, alphaup, arrow, bin, braille, cards, classic, \
                 clock, crab, dominosh, dominosv, earth, emojis, jap, large-letters, \
                 lettered-cubes, moon, num, numbered-balls, numbered-cubes, plants, shapes, smile"
            ));
        }
    };
    Ok(chars)
}

#[derive(Parser, Debug, Clone)]
#[command(
    name = "koko-matrix-rain",
    about = "Minimal Matrix-style rain CLI for the terminal",
    version
)]
pub struct Cli {
    /// Body color (name or "R,G,B")
    #[arg(short = 'c', long, default_value = "green", value_parser = Rgb::parse,
        long_help = "Set the body color of the rain trails.\n\
            Named colors: black, white, red, green, blue, yellow, cyan, magenta, orange, purple\n\
            Or an RGB tuple: \"R,G,B\" (e.g. \"0,255,70\")"
    )]
    pub color: Rgb,

    /// Head color (leading character in each column)
    #[arg(short = 'H', long, default_value = "white", value_parser = Rgb::parse,
        long_help = "Set the color of the leading (head) character in each column.\n\
            Accepts the same color formats as --color."
    )]
    pub head: Rgb,

    /// Background color (optional)
    #[arg(short = 'B', long, value_parser = Rgb::parse,
        long_help = "Set the background color.\n\
            When omitted the terminal's default background is used.\n\
            Accepts the same color formats as --color."
    )]
    pub bg: Option<Rgb>,

    /// Enable tail fade
    #[arg(short = 's', long,
        long_help = "Enable tail fade.\n\
            Each cell in the trail gradually blends from the body color toward the fade target (--fade-to)."
    )]
    pub shade: bool,

    /// Fade target color (where the tail fades to)
    #[arg(short = 'G', long, default_value = "black", value_parser = Rgb::parse,
        long_help = "Set the target color for the tail fade. Only visible when --shade is enabled.\n\
            Accepts the same color formats as --color."
    )]
    pub fade_to: Rgb,

    /// Speed range in ms, format "min,max"
    #[arg(short = 'S', long, default_value = "40,180", value_parser = parse_speed,
        long_help = "Set the tick interval range in milliseconds (format: \"min,max\").\n\
            Each column picks a random speed within this range.\n\
            Lower values = faster rain, higher values = slower rain.\n\
            Examples: \"20,80\" (fast), \"40,180\" (default), \"120,300\" (slow)"
    )]
    pub speed: Range<u64>,

    /// Character pool for the rain
    #[arg(long, default_value = "01", value_parser = parse_chars,
        long_help = "Set a custom character pool for the rain.\n\
            Each tick picks a random character from this string.\n\
            Supports ASCII, Unicode, and emoji.\n\
            Conflicts with --group."
    )]
    pub chars: String,

    /// Predefined character group (e.g. jap, emojis, cards)
    #[arg(short = 'g', long, value_parser = parse_group, conflicts_with = "chars",
        long_help = "Use a predefined character group instead of --chars.\n\
            Available groups:\n\
            \n  all             Most groups combined\
            \n  alphalow        Lowercase alphabet (a-z)\
            \n  alphaup         Uppercase alphabet (A-Z)\
            \n  arrow           Arrow symbols\
            \n  bin             Binary digits (0, 1)\
            \n  braille         Braille dot patterns\
            \n  cards           Playing card suits\
            \n  classic         Katakana + digits + symbols (cmatrix style)\
            \n  clock           Clock face emojis\
            \n  crab            🦀\
            \n  dominosh        Horizontal domino tiles\
            \n  dominosv        Vertical domino tiles\
            \n  earth           🌍🌎🌏\
            \n  emojis          Broad emoji set\
            \n  jap / katakana  Half-width Japanese katakana\
            \n  large-letters   Full-width Latin letters\
            \n  moon            Moon phase emojis\
            \n  num / digits    Digits (0-9)\
            \n  numbered-balls  Circled numbers\
            \n  numbered-cubes  Squared letters\
            \n  plants          Plant and fruit emojis\
            \n  shapes          Colored squares and circles\
            \n  smile           Smiley face emojis"
    )]
    pub group: Option<String>,
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

    #[test]
    fn group_returns_chars() {
        let jap = parse_group("jap").unwrap();
        assert!(jap.contains('ｱ'));
        assert!(jap.contains('ﾜ'));
        assert_eq!(parse_group("katakana").unwrap(), jap);
    }

    #[test]
    fn group_case_insensitive() {
        assert!(parse_group("JAP").is_ok());
        assert!(parse_group("Emojis").is_ok());
    }

    #[test]
    fn group_rejects_unknown() {
        let err = parse_group("nope").unwrap_err();
        assert!(err.contains("unknown group"));
        assert!(err.contains("available"));
    }

    #[test]
    fn all_groups_non_empty() {
        for name in [
            "all", "alphalow", "alphaup", "arrow", "bin", "braille", "cards", "classic",
            "clock", "crab", "dominosh", "dominosv", "earth", "emojis", "jap", "large-letters",
            "moon", "num", "numbered-balls", "numbered-cubes", "plants", "shapes", "smile",
        ] {
            let chars = parse_group(name).unwrap();
            assert!(!chars.is_empty(), "group '{name}' is empty");
        }
    }

    #[test]
    fn group_contents_are_correct() {
        let bin = parse_group("bin").unwrap();
        assert_eq!(bin, "01");

        let alpha = parse_group("alphalow").unwrap();
        assert_eq!(alpha.len(), 26);
        assert!(alpha.starts_with('a'));
        assert!(alpha.ends_with('z'));

        let num = parse_group("num").unwrap();
        assert_eq!(num, "0123456789");
        assert_eq!(parse_group("digits").unwrap(), num);

        let crab = parse_group("crab").unwrap();
        assert_eq!(crab, "🦀");

        let earth = parse_group("earth").unwrap();
        assert_eq!(earth, "🌍🌎🌏");
    }

    #[test]
    fn aliases_produce_same_result() {
        assert_eq!(parse_group("jap").unwrap(), parse_group("katakana").unwrap());
        assert_eq!(parse_group("num").unwrap(), parse_group("digits").unwrap());
        assert_eq!(parse_group("numbered-cubes").unwrap(), parse_group("lettered-cubes").unwrap());
    }
}
