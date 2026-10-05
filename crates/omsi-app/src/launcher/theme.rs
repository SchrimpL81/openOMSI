//! The launcher's looks: a palette each, chosen under Settings.
#![allow(non_snake_case)]

use omsi_ui::Color;
use std::sync::atomic::{AtomicUsize, Ordering};

pub struct Palette {
    /// The name in `settings.cfg` (`launcher_theme`) and the one shown.
    pub key: &'static str,
    pub name: &'static str,
    accent: Color,
    accent_2: Color,
    on_accent: Color,
    danger: Color,
    ok: Color,
    warn: Color,
    text: Color,
    text_soft: Color,
    text_dim: Color,
    text_faint: Color,
    backdrop: Color,
    rail: Color,
    panel: Color,
    field: Color,
    hover: Color,
    selected: Color,
    track: Color,
    knob: Color,
    popup: Color,
    edge: Color,
    shade: Color,
    shadow: Color,
    /// What a surface is tinted with under the mouse: white on a dark look, dark on a light.
    lift: Color,
    road: Color,
    road_main: Color,
    road_casing: Color,
}

const fn c(r: u8, g: u8, b: u8) -> Color {
    Color::rgba(r, g, b, 1.0)
}

pub const THEMES: [Palette; 3] = [
    Palette {
        key: "classic",
        name: "Classic",
        accent: c(232, 160, 48),
        accent_2: c(96, 160, 232),
        on_accent: c(18, 14, 8),
        danger: c(222, 78, 68),
        ok: c(104, 190, 118),
        warn: c(232, 170, 70),
        text: c(236, 236, 236),
        text_soft: c(200, 200, 200),
        text_dim: c(142, 142, 142),
        text_faint: c(96, 96, 96),
        backdrop: c(18, 18, 18),
        rail: c(18, 18, 18),
        panel: c(22, 22, 22),
        field: c(31, 31, 31),
        hover: c(38, 38, 38),
        selected: c(44, 44, 44),
        track: c(62, 62, 62),
        knob: c(240, 240, 240),
        popup: c(28, 28, 28),
        edge: Color::rgba(255, 255, 255, 0.06),
        shade: Color::rgba(0, 0, 0, 0.62),
        shadow: Color::rgba(0, 0, 0, 0.4),
        lift: c(255, 255, 255),
        road: c(92, 92, 92),
        road_main: c(112, 112, 112),
        road_casing: Color::rgba(30, 30, 30, 0.9),
    },
    Palette {
        key: "station",
        name: "Station",
        accent: c(255, 213, 0),
        accent_2: c(122, 214, 236),
        on_accent: c(9, 26, 46),
        danger: c(255, 112, 99),
        ok: c(98, 210, 136),
        warn: c(255, 158, 66),
        text: c(244, 247, 251),
        text_soft: c(206, 218, 232),
        text_dim: c(146, 168, 194),
        text_faint: c(98, 124, 154),
        backdrop: c(14, 37, 64),
        rail: c(10, 27, 48),
        panel: c(18, 46, 77),
        field: c(25, 58, 94),
        hover: c(33, 72, 113),
        selected: c(41, 86, 133),
        track: c(52, 88, 128),
        knob: c(246, 249, 252),
        popup: c(22, 54, 89),
        edge: Color::rgba(150, 195, 255, 0.14),
        shade: Color::rgba(4, 13, 25, 0.7),
        shadow: Color::rgba(2, 9, 18, 0.45),
        lift: c(255, 255, 255),
        road: c(74, 108, 146),
        road_main: c(112, 146, 184),
        road_casing: Color::rgba(7, 20, 37, 0.9),
    },
    Palette {
        key: "daylight",
        name: "Daylight",
        accent: c(0, 92, 169),
        accent_2: c(0, 122, 143),
        on_accent: c(255, 255, 255),
        danger: c(196, 43, 28),
        ok: c(22, 122, 66),
        warn: c(176, 90, 0),
        text: c(16, 32, 52),
        text_soft: c(48, 66, 90),
        text_dim: c(92, 108, 130),
        text_faint: c(140, 153, 171),
        backdrop: c(230, 235, 242),
        rail: c(245, 247, 250),
        panel: c(255, 255, 255),
        field: c(240, 244, 248),
        hover: c(226, 233, 241),
        selected: c(208, 223, 240),
        track: c(188, 200, 215),
        knob: c(255, 255, 255),
        popup: c(255, 255, 255),
        edge: Color::rgba(16, 32, 52, 0.16),
        shade: Color::rgba(16, 32, 52, 0.45),
        shadow: Color::rgba(16, 32, 52, 0.22),
        lift: c(16, 32, 52),
        road: c(168, 178, 192),
        road_main: c(124, 137, 155),
        road_casing: Color::rgba(255, 255, 255, 0.9),
    },
];

static CURRENT: AtomicUsize = AtomicUsize::new(0);

/// Which of `THEMES` is worn.
pub fn current() -> usize {
    CURRENT.load(Ordering::Relaxed).min(THEMES.len() - 1)
}

/// Wear the look of this key; one nobody knows is the first.
pub fn set(key: &str) {
    let i = THEMES.iter().position(|t| t.key.eq_ignore_ascii_case(key.trim())).unwrap_or(0);
    CURRENT.store(i, Ordering::Relaxed);
}

fn p() -> &'static Palette {
    &THEMES[current()]
}

macro_rules! colors {
    ($($name:ident => $field:ident),* $(,)?) => {
        $(pub fn $name() -> Color { p().$field })*
    };
}

colors! {
    ACCENT => accent, ACCENT_2 => accent_2, ON_ACCENT => on_accent, DANGER => danger, OK => ok, WARN => warn,
    TEXT => text, TEXT_SOFT => text_soft, TEXT_DIM => text_dim, TEXT_FAINT => text_faint,
    BACKDROP => backdrop, RAIL => rail, PANEL => panel, FIELD => field, HOVER => hover, SELECTED => selected,
    TRACK => track, KNOB => knob, POPUP => popup, EDGE => edge, SHADE => shade, SHADOW => shadow, LIFT => lift,
    ROAD => road, ROAD_MAIN => road_main, ROAD_CASING => road_casing,
}

pub const RADIUS: f32 = 10.0;
pub const CTRL: f32 = 6.0;
/// Height of a control row.
pub const ROW: f32 = 36.0;
pub const GAP: f32 = 12.0;

/// What the window is cleared to.
pub fn backdrop() -> wgpu::Color {
    let lin = |v: f32| if v <= 0.04045 { v / 12.92 } else { ((v + 0.055) / 1.055).powf(2.4) } as f64;
    let [r, g, b, _] = BACKDROP().0;
    wgpu::Color { r: lin(r), g: lin(g), b: lin(b), a: 1.0 }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn a_look_nobody_knows_is_the_first() {
        set("Station");
        assert_eq!(THEMES[current()].key, "station");
        set("no such look");
        assert_eq!(current(), 0);
    }

    #[test]
    fn text_reads_on_every_surface_of_every_look() {
        let lum = |c: Color| {
            let l = |v: f32| if v <= 0.04045 { v / 12.92 } else { ((v + 0.055) / 1.055).powf(2.4) };
            0.2126 * l(c.0[0]) + 0.7152 * l(c.0[1]) + 0.0722 * l(c.0[2])
        };
        let contrast = |a: Color, b: Color| {
            let (x, y) = (lum(a), lum(b));
            (x.max(y) + 0.05) / (x.min(y) + 0.05)
        };
        for t in &THEMES {
            for surface in [t.backdrop, t.rail, t.panel, t.field, t.popup] {
                assert!(contrast(t.text, surface) >= 7.0, "{}: text", t.key);
                assert!(contrast(t.text_dim, surface) >= 4.0, "{}: dim text", t.key);
            }
            assert!(contrast(t.on_accent, t.accent) >= 4.5, "{}: text on the accent", t.key);
        }
    }
}
