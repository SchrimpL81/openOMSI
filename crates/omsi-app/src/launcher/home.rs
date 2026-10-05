//! The Home page: what the launcher opens with - the ways to drive, the driver's record and
//! the rest of the pages.

use glam::Vec2;
use omsi_ui::paint::Align;
use omsi_ui::{Color, Rect, Weight};

use super::theme::*;
use super::ui::{id_of, ButtonKind};
use super::{Launcher, Page};

#[derive(Clone, Copy, PartialEq)]
enum Mode {
    Duty,
    Free,
    Online,
    Timetable,
}

const MODES: [(Mode, &str, &str, &str); 4] = [
    (Mode::Duty, "Drive a duty", "directions_bus", "Take a line and a tour from the timetable and keep to its times, stop by stop."),
    (Mode::Free, "Free drive", "explore", "Any bus on any map, no timetable: go where the road takes you."),
    (Mode::Online, "Multiplayer", "groups", "Drive with friends by a code, or join a server that is always on."),
    (Mode::Timetable, "Timetable", "departure_board", "The map's lines, their tours and when the buses go."),
];

const TILES: [(Page, &str, &str, &str); 6] = [
    (Page::Mods, "Mods", "extension", "Buses, maps, scenery"),
    (Page::Sessions, "Sessions", "sports_esports", "Running games and logs"),
    (Page::Controls, "Controls", "keyboard", "Keys, wheels, gamepads"),
    (Page::Settings, "Settings", "tune", "Graphics, sound, look"),
    (Page::Tutorials, "Tutorials", "help", "Learn to drive the buses"),
    (Page::Setup, "Setup", "folder_open", "The OMSI 2 folder"),
];

/// The greeting for an hour of the day.
fn greeting(hour: i32) -> &'static str {
    match hour {
        5..=10 => "Good morning",
        11..=17 => "Good afternoon",
        18..=22 => "Good evening",
        _ => "Good night",
    }
}

fn hours(h: f64) -> String {
    let m = (h * 60.0).round().max(0.0) as i64;
    format!("{}h {:02}m", m / 60, m % 60)
}

pub fn draw(l: &mut Launcher, area: Rect) {
    if !l.home_asked {
        l.home_asked = true;
        l.state.load_profile();
    }
    let w = area.w.min(1180.0);
    let x = area.x + (area.w - w) * 0.5;
    // (a low window: the greeting gives way first)
    let tall = area.h >= 600.0;
    let mut y = area.y + if tall { (area.h * 0.05).min(36.0) } else { 0.0 };

    let name = l.state.profile.as_ref().map(|p| p.name.clone()).filter(|n| !n.is_empty()).unwrap_or_else(|| l.state.config.profile.clone());
    let hour = omsi_launcher_lib::local_now().map(|t| t.3).unwrap_or(12);
    let hello = if name.is_empty() { omsi_ui::tr(greeting(hour)).to_string() } else { format!("{}, {name}", omsi_ui::tr(greeting(hour))) };
    l.ui.text_in(&hello, Rect::new(x, y, w, 40.0), 30.0, Weight::Bold, TEXT(), Align::Center);
    l.ui.text_in("How do you want to drive today?", Rect::new(x, y + 40.0, w, 22.0), 14.0, Weight::Regular, TEXT_DIM(), Align::Center);
    y += if tall { 86.0 } else { 70.0 };

    // the ways to drive
    let gap = 16.0;
    let card_h = (area.bottom() - y - 250.0).clamp(150.0, 230.0);
    let cw = (w - gap * 3.0) / 4.0;
    for (k, (mode, title, icon, text)) in MODES.iter().enumerate() {
        let r = Rect::new(x + (cw + gap) * k as f32, y, cw, card_h);
        if mode_card(l, r, k, title, icon, text) {
            match mode {
                Mode::Duty | Mode::Free => {
                    let free = *mode == Mode::Free;
                    if l.state.choice.free != free {
                        l.state.choice.free = free;
                        l.state.touched();
                    }
                    l.drive.tab = 0;
                    l.go(Page::Drive);
                }
                Mode::Online => l.go(Page::Multiplayer),
                Mode::Timetable => l.go(Page::Timetable),
            }
        }
    }
    y += card_h + gap;

    // the driver's record beside the other pages
    let low = Rect::new(x, y, w, (area.bottom() - y).min(214.0));
    let left_w = (w * 0.4).max(320.0);
    record(l, Rect::new(low.x, low.y, left_w, low.h));
    let grid = Rect::new(low.x + left_w + gap, low.y, low.w - left_w - gap, low.h);
    let (tw, th) = ((grid.w - gap * 2.0) / 3.0, (grid.h - gap) / 2.0);
    for (k, (page, title, icon, text)) in TILES.iter().enumerate() {
        let r = Rect::new(grid.x + (tw + gap) * (k % 3) as f32, grid.y + (th + gap) * (k / 3) as f32, tw, th);
        if tile(l, r, title, icon, text) {
            l.go(*page);
        }
    }
}

/// One way to drive: a picture of its own, its name and a line about it. True when clicked.
fn mode_card(l: &mut Launcher, r: Rect, k: usize, title: &str, icon: &str, text: &str) -> bool {
    let id = id_of(&format!("home-mode-{k}"));
    let (h, held, clicked) = l.ui.interact(id, r);
    let t = l.ui.anim(id, if h { 1.0 } else { 0.0 }, 0.08);
    let r = if held { r.inset(1.0) } else { Rect::new(r.x, r.y - 3.0 * t, r.w, r.h) };
    l.ui.solid(r);
    let tint = [ACCENT(), OK(), ACCENT_2(), DANGER()][k % 4];
    if t > 0.01 {
        l.ui.p().shadow(Rect::new(r.x, r.y + 8.0, r.w, r.h), RADIUS + 4.0, 26.0, SHADOW().alpha(t));
    }
    l.ui.p().rounded(r, RADIUS + 4.0, PANEL());

    // the picture: the mode's colour over the card, and a line with its stops
    let art = Rect::new(r.x, r.y, r.w, (r.h * 0.5).max(70.0));
    l.ui.push_clip(Rect::new(r.x + 1.0, r.y + 1.0, r.w - 2.0, r.h - 2.0), RADIUS + 3.0);
    l.ui.p().gradient(art, PANEL().mix(tint, 0.34 + 0.12 * t), PANEL());
    let base = art.y + art.h * 0.62;
    let pts: [(f32, f32); 5] = [(-0.02, 0.10), (0.22, 0.10), (0.40, -0.20), (0.66, -0.20), (1.02, 0.16)];
    let at = |p: (f32, f32)| Vec2::new(art.x + art.w * p.0, base + art.h * p.1);
    for s in pts.windows(2) {
        l.ui.p().line(at(s[0]), at(s[1]), 3.0, tint.alpha(0.55 + 0.3 * t));
    }
    for (n, p) in pts[1..4].iter().enumerate() {
        l.ui.p().circle(at(*p), 6.0, tint);
        l.ui.p().circle(at(*p), 3.4, if n == 1 { TEXT() } else { PANEL() });
    }
    l.ui.icon(icon, Vec2::new(art.right() - 44.0, art.y + 40.0), 56.0, tint.alpha(0.22 + 0.2 * t));
    l.ui.pop_clip();

    let badge = Vec2::new(r.x + 34.0, art.y + 34.0);
    l.ui.p().circle(badge, 18.0, tint);
    l.ui.icon(icon, badge, 20.0, if k == 2 { Color::rgba(18, 14, 8, 1.0) } else { ON_ACCENT() });

    let ty = art.bottom() + 4.0;
    l.ui.text_in(title, Rect::new(r.x + 18.0, ty, r.w - 60.0, 26.0), 16.5, Weight::Bold, TEXT(), Align::Left);
    l.ui.icon("chevron_right", Vec2::new(r.right() - 24.0 + 3.0 * t, ty + 13.0), 22.0, TEXT_DIM().mix(tint, t));
    if r.bottom() - ty > 62.0 {
        l.ui.push_clip(Rect::new(r.x, ty + 26.0, r.w, r.bottom() - ty - 34.0), 0.0);
        l.ui.paragraph(text, Vec2::new(r.x + 18.0, ty + 26.0), r.w - 36.0, 12.5, Weight::Regular, TEXT_DIM());
        l.ui.pop_clip();
    }
    l.ui.p().rounded_border(r, RADIUS + 4.0, 1.0 + t, EDGE().mix(tint, t));
    clicked
}

/// The driver's record: level, hours, kilometres, stops.
fn record(l: &mut Launcher, r: Rect) {
    l.ui.panel(r);
    let inner = Rect::new(r.x + 20.0, r.y + 14.0, r.w - 40.0, r.h - 28.0);
    l.ui.heading(inner, "Your service record", None);
    let p = l.state.profile.clone().filter(|p| p.exists);
    let Some(p) = p else {
        l.ui.paragraph("No driver yet. Create one and the launcher keeps your hours, kilometres and punctuality.", Vec2::new(inner.x, inner.y + 34.0), inner.w, 12.5, Weight::Regular, TEXT_DIM());
        if l.ui.button("home-driver", Rect::new(inner.x, inner.bottom() - ROW, 170.0, ROW), "Create a driver", Some("person"), ButtonKind::Normal) {
            l.go(Page::Profile);
        }
        return;
    };
    let cells = [(format!("{}", p.level), "Level"), (hours(p.hours), "At the wheel"), (format!("{:.0} km", p.km), "Driven"), (format!("{}", p.stops), "Stops served")];
    let cw = inner.w / cells.len() as f32;
    for (k, (value, label)) in cells.iter().enumerate() {
        let c = Rect::new(inner.x + cw * k as f32, inner.y + 34.0, cw - 8.0, 46.0);
        l.ui.text_in(value, Rect::new(c.x, c.y, c.w, 28.0), 22.0, Weight::Bold, if k == 0 { ACCENT_2() } else { TEXT() }, Align::Left);
        l.ui.text_in(label, Rect::new(c.x, c.y + 28.0, c.w, 16.0), 11.5, Weight::Regular, TEXT_DIM(), Align::Left);
    }
    if inner.h > 150.0 {
        let need = p.next_level_xp.max(1) as f32;
        let bar = Rect::new(inner.x, inner.y + 100.0, inner.w, 6.0);
        l.ui.progress(bar, (p.xp as f32 / need).clamp(0.0, 1.0), false);
        l.ui.text_in(&format!("{} / {} XP", p.xp, p.next_level_xp), Rect::new(inner.x, bar.bottom() + 4.0, inner.w, 16.0), 11.0, Weight::Regular, TEXT_FAINT(), Align::Left);
    }
    if l.ui.button("home-record", Rect::new(inner.right() - 190.0, inner.bottom() - 30.0, 190.0, 30.0), "Open the service record", Some("badge"), ButtonKind::Ghost) {
        l.go(Page::Profile);
    }
}

/// One of the other pages: an icon, its name and a word about it. True when clicked.
fn tile(l: &mut Launcher, r: Rect, title: &str, icon: &str, text: &str) -> bool {
    let id = id_of(&format!("home-tile-{title}"));
    let (h, held, clicked) = l.ui.interact(id, r);
    let t = l.ui.anim(id, if h { 1.0 } else { 0.0 }, 0.07);
    let r = if held { r.inset(1.0) } else { r };
    l.ui.solid(r);
    l.ui.p().rounded(r, RADIUS, PANEL().mix(HOVER(), t));
    l.ui.p().rounded_border(r, RADIUS, 1.0, EDGE().mix(ACCENT(), t * 0.8));
    let c = Vec2::new(r.x + 32.0, r.center().y);
    l.ui.p().circle(c, 18.0, FIELD().mix(ACCENT(), t));
    l.ui.icon(icon, c, 19.0, TEXT_SOFT().mix(ON_ACCENT(), t));
    let tx = r.x + 62.0;
    l.ui.text_in(title, Rect::new(tx, r.center().y - 19.0, r.right() - tx - 10.0, 20.0), 14.0, Weight::Bold, TEXT(), Align::Left);
    l.ui.text_in(text, Rect::new(tx, r.center().y + 1.0, r.right() - tx - 10.0, 18.0), 11.5, Weight::Regular, TEXT_DIM(), Align::Left);
    clicked
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn every_hour_has_a_greeting() {
        assert_eq!(greeting(7), "Good morning");
        assert_eq!(greeting(13), "Good afternoon");
        assert_eq!(greeting(21), "Good evening");
        assert_eq!(greeting(2), "Good night");
        assert_eq!(hours(1.5), "1h 30m");
    }
}
