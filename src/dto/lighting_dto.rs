use bevy_color::Color;
use serde::{Deserialize, Serialize};

/// Global lighting of a game (Roblox `Lighting` service). Additive v1
/// section: files without it (or with missing fields) load with the engine
/// defaults below.
#[derive(Serialize, Deserialize, Debug)]
#[cfg_attr(test, derive(PartialEq))]
pub struct LightingDTO {
    /// Flat light added to every surface (sRGB).
    #[serde(default = "default_ambient")]
    pub ambient: Color,
    /// Sun intensity multiplier (2.0 ≈ daylight).
    #[serde(default = "default_brightness")]
    pub brightness: f32,
    /// Time of day in hours, 0..24 (6 sunrise, 12 noon, 18 sunset).
    #[serde(default = "default_clock_time")]
    pub clock_time: f32,
    #[serde(default = "default_fog_color")]
    pub fog_color: Color,
    /// Distance where fog starts, in world units.
    #[serde(default)]
    pub fog_start: f32,
    /// Distance of full fog opacity. The default is far enough to disable
    /// fog entirely.
    #[serde(default = "default_fog_end")]
    pub fog_end: f32,
}

impl Default for LightingDTO {
    fn default() -> Self {
        Self {
            ambient: default_ambient(),
            brightness: default_brightness(),
            clock_time: default_clock_time(),
            fog_color: default_fog_color(),
            fog_start: 0.0,
            fog_end: default_fog_end(),
        }
    }
}

fn default_ambient() -> Color {
    Color::srgb(0.5, 0.5, 0.5)
}

fn default_brightness() -> f32 {
    2.0
}

fn default_clock_time() -> f32 {
    14.0
}

fn default_fog_color() -> Color {
    Color::srgb(0.75, 0.75, 0.75)
}

fn default_fog_end() -> f32 {
    100_000.0
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn empty_object_loads_the_defaults() {
        let lighting: LightingDTO = serde_json::from_str("{}").unwrap();
        assert_eq!(lighting, LightingDTO::default());
        assert_eq!(lighting.brightness, 2.0);
        assert_eq!(lighting.clock_time, 14.0);
        assert_eq!(lighting.fog_end, 100_000.0);
    }

    #[test]
    fn values_round_trip() {
        let lighting = LightingDTO {
            ambient: Color::srgb(0.1, 0.2, 0.3),
            brightness: 5.0,
            clock_time: 6.5,
            fog_color: Color::srgb(0.9, 0.8, 0.7),
            fog_start: 10.0,
            fog_end: 250.0,
        };
        let back: LightingDTO =
            serde_json::from_str(&serde_json::to_string(&lighting).unwrap()).unwrap();
        assert_eq!(lighting, back);
    }
}
