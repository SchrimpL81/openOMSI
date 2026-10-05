//! The Home page: what the launcher opens with. A line with three stops over the greeting,
//! the ways to drive and the workshop's tools as a row of picture cards that slides (the
//! wheel, the arrow keys, a click on a card at the side), and under it a row of chips: the
//! driver, the mods and the other pages.

use glam::Vec2;
use omsi_ui::paint::Align;
use omsi_ui::{Color, Rect, Weight};

use super::theme::*;
use super::ui::{id_of, Key};
use super::{Launcher, Page};

/// What a card leads to.
#[derive(Clone, Copy, PartialEq, Debug)]
enum Mode {
    Duty,
    Free,
    Online,
    Timetable,
    Gallery,
    Lines,
    Livery,
}

/// The picture a card draws (see `art`).
#[derive(Clone, Copy, PartialEq)]
enum Art {
    City,
    Country,
    Crowd,
    Board,
    Fleet,
    Plan,
    Paint,
}

struct Card {
    mode: Mode,
    title: &'static str,
    icon: &'static str,
    text: &'static str,
    art: Art,
}

const CARDS: [Card; 7] = [
    Card { mode: Mode::Duty, title: "Drive a duty", icon: "directions_bus", text: "A line and a tour from the timetable, its times kept stop by stop.", art: Art::City },
    Card { mode: Mode::Free, title: "Free drive", icon: "explore", text: "Only a map and a bus: go where the road takes you, nothing is booked.", art: Art::Country },
    Card { mode: Mode::Online, title: "Multiplayer", icon: "groups", text: "Drive with friends by a code, or join a server that is always on.", art: Art::Crowd },
    Card { mode: Mode::Timetable, title: "Timetable", icon: "departure_board", text: "The map's lines, their tours and when the buses go.", art: Art::Board },
    Card { mode: Mode::Gallery, title: "Bus gallery", icon: "photo_library", text: "Every bus you have, pictured: pick one and look at it from all sides.", art: Art::Fleet },
    Card { mode: Mode::Lines, title: "Line editor", icon: "route", text: "Lines of your own: click the stops, the way is found over the roads.", art: Art::Plan },
    Card { mode: Mode::Livery, title: "Livery studio", icon: "format_paint", text: "Paint a bus: colours, stripes, a name and a logo, seen on the model.", art: Art::Paint },
];

/// The chips under the cards: a page, its icon and its name.
const CHIPS: [(Page, &str, &str); 5] = [
    (Page::Mods, "extension", "Mods"),
    (Page::Settings, "tune", "Settings"),
    (Page::Controls, "keyboard", "Controls"),
    (Page::Sessions, "sports_esports", "Sessions"),
    (Page::Tutorials, "help", "Tutorials"),
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

/// The card shown in the middle: kept between frames (the wheel and the arrow keys move it).
pub struct HomeView {
    pub focus: usize,
}

impl Default for HomeView {
    /// (the second card in the middle: the first stands beside it, the row reads both ways)
    fn default() -> Self {
        HomeView { focus: 1 }
    }
}

/// Where the row is: the focused card's left edge put so that it stands in the middle.
fn row_x(centre: f32, card_w: f32, gap: f32, at: f32) -> f32 {
    centre - card_w * 0.5 - at * (card_w + gap)
}

pub fn draw(l: &mut Launcher, area: Rect) {
    if !l.home_asked {
        l.home_asked = true;
        l.state.load_profile();
    }
    let size = l.ui.size;
    let tall = area.h >= 620.0;
    let card_h = if tall { (area.h * 0.3).clamp(190.0, 250.0) } else { 170.0 };
    let card_w = (card_h * 1.12).min(area.w * 0.42);
    let gap = 18.0;

    // the stage: a glow behind the cards and a line through it that runs the window's width
    // (the whole of it - the line, the greeting, the cards, the chips - stands a little above
    // the middle of the page)
    let head = if tall { 136.0 } else { 104.0 };
    let block = head + card_h + 112.0;
    let band_y = area.y + ((area.h - block) * 0.42).max(0.0) + head;
    let mid = band_y + card_h * 0.5;
    l.ui.p().gradient(Rect::new(0.0, area.y - 30.0, size.x, mid - area.y + 30.0), BACKDROP(), BACKDROP().mix(ACCENT(), 0.10));
    l.ui.p().gradient(Rect::new(0.0, mid, size.x, (area.bottom() - mid).max(0.0)), BACKDROP().mix(ACCENT(), 0.10), BACKDROP());
    l.ui.p().gradient_h(Rect::new(0.0, mid - 1.0, size.x * 0.5, 2.0), ACCENT().alpha(0.0), ACCENT().alpha(0.85));
    l.ui.p().gradient_h(Rect::new(size.x * 0.5, mid - 1.0, size.x * 0.5, 2.0), ACCENT().alpha(0.85), ACCENT().alpha(0.0));
    l.ui.p().gradient_h(Rect::new(0.0, mid - 4.0, size.x * 0.5, 8.0), ACCENT().alpha(0.0), ACCENT().alpha(0.16));
    l.ui.p().gradient_h(Rect::new(size.x * 0.5, mid - 4.0, size.x * 0.5, 8.0), ACCENT().alpha(0.16), ACCENT().alpha(0.0));

    // the line with its three stops, the greeting under it
    let cx = area.x + area.w * 0.5;
    let line_y = band_y - head + 18.0;
    stop_line(l, Vec2::new(cx, line_y), (area.w * 0.2).clamp(150.0, 260.0));
    let name = l.state.profile.as_ref().map(|p| p.name.clone()).filter(|n| !n.is_empty()).unwrap_or_else(|| l.state.config.profile.clone());
    let hour = omsi_launcher_lib::local_now().map(|t| t.3).unwrap_or(12);
    let hello = if name.is_empty() { omsi_ui::tr(greeting(hour)).to_string() } else { format!("{} {name}", omsi_ui::tr(greeting(hour))) };
    l.ui.text_in(&hello, Rect::new(area.x, line_y + 22.0, area.w, 40.0), 30.0, Weight::Bold, TEXT(), Align::Center);
    l.ui.text_in("How do you want to drive today?", Rect::new(area.x, line_y + 62.0, area.w, 22.0), 14.0, Weight::Regular, TEXT_DIM(), Align::Center);

    // the row of cards: the focused one in the middle, the wheel and the arrow keys move it
    let row = Rect::new(0.0, band_y - 10.0, size.x, card_h + 20.0);
    if l.ui.hover(row) && l.ui.input.wheel != Vec2::ZERO {
        let w = if l.ui.input.wheel.x.abs() > l.ui.input.wheel.y.abs() { -l.ui.input.wheel.x } else { -l.ui.input.wheel.y };
        if w > 0.0 {
            l.home.focus = (l.home.focus + 1).min(CARDS.len() - 1);
        } else if w < 0.0 {
            l.home.focus = l.home.focus.saturating_sub(1);
        }
    }
    let keys = l.ui.input.keys.clone();
    if l.ui.focus.is_none() {
        for k in keys {
            match k {
                Key::Right => l.home.focus = (l.home.focus + 1).min(CARDS.len() - 1),
                Key::Left => l.home.focus = l.home.focus.saturating_sub(1),
                Key::Enter => open(l, CARDS[l.home.focus].mode),
                _ => {}
            }
        }
    }
    let at = l.ui.anim(id_of("home-row"), l.home.focus as f32, 0.12);
    let x0 = row_x(cx, card_w, gap, at);
    for (k, c) in CARDS.iter().enumerate() {
        let r = Rect::new(x0 + k as f32 * (card_w + gap), band_y, card_w, card_h);
        if r.right() < -20.0 || r.x > size.x + 20.0 {
            continue;
        }
        // how far from the middle: the far ones fade into the backdrop
        let off = ((r.center().x - cx) / (card_w + gap)).abs();
        let fade = (1.0 - (off - 1.2).max(0.0) * 0.55).clamp(0.0, 1.0);
        if fade <= 0.02 {
            continue;
        }
        if card(l, r, k, c, k == l.home.focus, fade) {
            if k == l.home.focus {
                open(l, c.mode);
            } else {
                l.home.focus = k;
            }
        }
    }
    // the dots under the row: which card is in the middle
    let dots_y = band_y + card_h + 22.0;
    let dw = 18.0;
    let dx = cx - dw * (CARDS.len() as f32 - 1.0) * 0.5;
    for k in 0..CARDS.len() {
        let c = Vec2::new(dx + k as f32 * dw, dots_y);
        let r = Rect::new(c.x - 8.0, c.y - 8.0, 16.0, 16.0);
        let (h, _, clicked) = l.ui.interact(id_of(&format!("home-dot-{k}")), r);
        if clicked {
            l.home.focus = k;
        }
        let on = k == l.home.focus;
        l.ui.p().circle(c, if on { 4.5 } else { 3.2 }, if on { ACCENT() } else if h { TEXT_DIM() } else { TRACK() });
    }

    // the chips: the driver first, then the other pages
    let chips_y = dots_y + 24.0;
    chips(l, Rect::new(area.x, chips_y, area.w, 44.0));
}

/// The way a card goes.
fn open(l: &mut Launcher, mode: Mode) {
    match mode {
        Mode::Duty | Mode::Free => {
            let free = mode == Mode::Free;
            if l.state.choice.free != free {
                l.state.choice.free = free;
                l.state.touched();
            }
            l.drive.tab = 0;
            l.go(Page::Drive);
        }
        Mode::Online => l.go(Page::Multiplayer),
        Mode::Timetable => l.go(Page::Timetable),
        Mode::Gallery => l.go(Page::Buses),
        Mode::Lines => l.go(Page::Lines),
        Mode::Livery => l.go(Page::Livery),
    }
}

/// A line along the top with three stops on it, the middle one the brightest.
fn stop_line(l: &mut Launcher, c: Vec2, half: f32) {
    let y = c.y;
    l.ui.p().gradient_h(Rect::new(c.x - half - 40.0, y - 2.5, 40.0, 5.0), ACCENT().alpha(0.0), ACCENT());
    l.ui.p().rect(Rect::new(c.x - half, y - 2.5, half * 2.0, 5.0), ACCENT());
    l.ui.p().gradient_h(Rect::new(c.x + half, y - 2.5, 40.0, 5.0), ACCENT(), ACCENT().alpha(0.0));
    let t = l.ui.time;
    for (k, col) in [DANGER(), ACCENT_2(), OK()].into_iter().enumerate() {
        let p = Vec2::new(c.x + (k as f32 - 1.0) * half, y);
        let pulse = 0.5 + 0.5 * (t * 1.6 - k as f32 * 0.9).sin();
        l.ui.p().circle(p, 15.0 + 3.0 * pulse, col.alpha(0.14));
        l.ui.p().circle(p, 12.0, RAIL());
        l.ui.p().circle(p, 10.0, col);
        l.ui.p().circle(p, 5.0, RAIL());
    }
}

/// One card: its picture, the icon and the name over it, a line about it. True when clicked.
fn card(l: &mut Launcher, r: Rect, k: usize, c: &Card, focused: bool, fade: f32) -> bool {
    let id = id_of(&format!("home-card-{k}"));
    let (h, held, clicked) = l.ui.interact(id, r);
    let t = l.ui.anim(id, if h { 1.0 } else { 0.0 }, 0.08);
    let f = l.ui.anim(id ^ 9, if focused { 1.0 } else { 0.0 }, 0.12);
    let lift = 6.0 * f + 3.0 * t;
    let r = if held { r.inset(1.0) } else { Rect::new(r.x, r.y - lift, r.w, r.h) };
    l.ui.solid(r);
    if f > 0.01 {
        l.ui.p().shadow(Rect::new(r.x, r.y + 10.0, r.w, r.h), RADIUS + 4.0, 34.0, SHADOW().alpha(f * fade));
        l.ui.p().shadow(r, RADIUS + 4.0, 22.0, ACCENT().alpha(0.28 * f * fade));
    }
    l.ui.p().rounded(r, RADIUS + 4.0, PANEL().alpha(fade));
    l.ui.push_clip(Rect::new(r.x + 1.0, r.y + 1.0, r.w - 2.0, r.h - 2.0), RADIUS + 3.0);
    art(l, r, c.art, fade, t);
    // the words over the foot of the picture, on a shade that keeps them readable
    let foot = Rect::new(r.x, r.y + r.h * 0.42, r.w, r.h * 0.58);
    l.ui.p().gradient(foot, PANEL().alpha(0.0), PANEL().alpha(0.94 * fade));
    l.ui.pop_clip();

    let badge = Rect::new(r.x + 16.0, r.y + r.h * 0.5, 30.0, 30.0);
    l.ui.p().rounded(badge, 8.0, LIFT().alpha(0.16 * fade));
    l.ui.p().rounded_border(badge, 8.0, 1.0, LIFT().alpha(0.22 * fade));
    l.ui.icon(c.icon, badge.center(), 17.0, TEXT().alpha(fade));
    let ty = badge.bottom() + 8.0;
    l.ui.text_in(c.title, Rect::new(r.x + 16.0, ty, r.w - 32.0, 24.0), 17.0, Weight::Bold, TEXT().alpha(fade), Align::Left);
    if r.bottom() - ty > 54.0 {
        l.ui.push_clip(Rect::new(r.x, ty + 26.0, r.w, r.bottom() - ty - 32.0), 0.0);
        l.ui.paragraph(c.text, Vec2::new(r.x + 16.0, ty + 27.0), r.w - 32.0, 11.5, Weight::Regular, TEXT_SOFT().alpha(fade));
        l.ui.pop_clip();
    }
    let edge = EDGE().mix(ACCENT(), (f + t * 0.6).min(1.0));
    l.ui.p().rounded_border(r, RADIUS + 4.0, 1.0 + f, edge.alpha(edge.0[3].max(0.12) * fade));
    clicked
}

/// The sky of a card: the look's own colours, light at the horizon.
fn sky(l: &mut Launcher, r: Rect, top: Color, bottom: Color) {
    l.ui.p().gradient(Rect::new(r.x, r.y, r.w, r.h * 0.62), top, bottom);
    l.ui.p().rect(Rect::new(r.x, r.y + r.h * 0.62, r.w, r.h * 0.38), bottom.mix(PANEL(), 0.5));
}

/// A row of houses and towers along `base`, their heights from a seed.
fn skyline(l: &mut Launcher, r: Rect, base: f32, max_h: f32, c: Color, seed: u32) {
    let mut x = r.x - 6.0;
    let mut s = seed.wrapping_mul(2654435761);
    while x < r.right() {
        s = s.wrapping_mul(1103515245).wrapping_add(12345);
        let w = 10.0 + (s >> 16 & 15) as f32 * 1.4;
        let h = max_h * (0.3 + (s >> 8 & 255) as f32 / 255.0 * 0.7);
        l.ui.p().rect(Rect::new(x, base - h, w - 2.0, h), c);
        if s & 7 == 0 {
            // a tower with a spire
            l.ui.p().convex(&[Vec2::new(x + 2.0, base - h), Vec2::new(x + w * 0.5 - 1.0, base - h - max_h * 0.35), Vec2::new(x + w - 4.0, base - h)], c);
        }
        x += w;
    }
}

/// A bus seen from the side, `len` long, its front to the right.
fn bus(l: &mut Launcher, at: Vec2, len: f32, body: Color, glass: Color) {
    let h = len * 0.28;
    let b = Rect::new(at.x, at.y - h, len, h);
    l.ui.p().rounded(b, h * 0.14, body);
    let wy = b.y + h * 0.16;
    let wh = h * 0.36;
    let mut x = b.x + len * 0.05;
    while x < b.right() - len * 0.12 {
        l.ui.p().rounded(Rect::new(x, wy, len * 0.11, wh), 1.5, glass);
        x += len * 0.125;
    }
    l.ui.p().rounded(Rect::new(b.right() - len * 0.075, wy, len * 0.06, h * 0.62), 1.5, glass);
    for wx in [0.2, 0.78] {
        let c = Vec2::new(b.x + len * wx, b.bottom());
        l.ui.p().circle(c, h * 0.17, Color::rgba(14, 16, 22, 1.0));
        l.ui.p().circle(c, h * 0.08, Color::rgba(120, 124, 132, 1.0));
    }
}

/// The picture of a card, drawn in the look's colours (no photographs: shapes).
fn art(l: &mut Launcher, r: Rect, art: Art, fade: f32, hover: f32) {
    let a = |c: Color| c.alpha(c.0[3] * fade);
    let pic = Rect::new(r.x, r.y, r.w, r.h * 0.72);
    let ground = pic.y + pic.h * 0.66;
    match art {
        Art::City => {
            sky(l, pic, a(ACCENT().mix(BACKDROP(), 0.55)), a(ACCENT().mix(TEXT(), 0.25).mix(PANEL(), 0.35)));
            skyline(l, pic, ground, pic.h * 0.5, a(PANEL().mix(ACCENT(), 0.25)), 3);
            skyline(l, pic, ground, pic.h * 0.32, a(PANEL().mix(ACCENT(), 0.12)), 11);
            // the route in the sky with its stops, as on the line map
            let pts = [(0.0, 0.62), (0.24, 0.46), (0.5, 0.52), (0.78, 0.28), (1.0, 0.22)];
            let p = |q: (f32, f32)| Vec2::new(pic.x + pic.w * q.0, pic.y + pic.h * q.1);
            for s in pts.windows(2) {
                l.ui.p().line(p(s[0]), p(s[1]), 3.0, a(ACCENT_2().alpha(0.85)));
            }
            for q in &pts[1..4] {
                l.ui.p().circle(p(*q), 5.0, a(ACCENT_2()));
                l.ui.p().circle(p(*q), 2.4, a(PANEL()));
            }
            bus(l, Vec2::new(pic.x + pic.w * (0.08 + 0.04 * hover), ground + 2.0), pic.w * 0.5, a(TEXT().mix(ACCENT(), 0.1)), a(PANEL().mix(ACCENT(), 0.3)));
        }
        Art::Country => {
            sky(l, pic, a(OK().mix(BACKDROP(), 0.6)), a(OK().mix(TEXT(), 0.35).mix(PANEL(), 0.3)));
            // hills, a road winding to the horizon
            let hill = |l: &mut Launcher, y: f32, amp: f32, c: Color, ph: f32| {
                let n = 24;
                for i in 0..n {
                    let x0 = pic.x + pic.w * i as f32 / n as f32;
                    let x1 = pic.x + pic.w * (i + 1) as f32 / n as f32;
                    let y0 = y - amp * ((i as f32 * 0.5 + ph).sin() * 0.5 + 0.5);
                    let y1 = y - amp * (((i + 1) as f32 * 0.5 + ph).sin() * 0.5 + 0.5);
                    l.ui.p().convex(&[Vec2::new(x0, y0), Vec2::new(x1, y1), Vec2::new(x1, pic.bottom()), Vec2::new(x0, pic.bottom())], c);
                }
            };
            hill(l, ground - pic.h * 0.12, pic.h * 0.16, a(OK().mix(PANEL(), 0.62)), 0.0);
            hill(l, ground, pic.h * 0.10, a(OK().mix(PANEL(), 0.45)), 2.0);
            let road = [Vec2::new(pic.x + pic.w * 0.62, ground - pic.h * 0.2), Vec2::new(pic.x + pic.w * 0.66, ground - pic.h * 0.2), Vec2::new(pic.x + pic.w * 0.95, pic.bottom()), Vec2::new(pic.x + pic.w * 0.35, pic.bottom())];
            l.ui.p().convex(&road, a(ROAD()));
            for k in 0..4 {
                let t0 = k as f32 / 4.0 + 0.05;
                let lerp = |t: f32| road[0].lerp(road[3], t).lerp(road[1].lerp(road[2], t), 0.5);
                l.ui.p().line(lerp(t0), lerp(t0 + 0.1), 1.5, a(TEXT().alpha(0.6)));
            }
        }
        Art::Crowd => {
            sky(l, pic, a(ACCENT_2().mix(BACKDROP(), 0.6)), a(ACCENT_2().mix(TEXT(), 0.2).mix(PANEL(), 0.35)));
            skyline(l, pic, ground, pic.h * 0.38, a(PANEL().mix(ACCENT_2(), 0.18)), 7);
            // three buses on three lines, linked
            let ys = [0.30, 0.48, 0.66];
            for (k, y) in ys.iter().enumerate() {
                let x = pic.x + pic.w * (0.12 + k as f32 * 0.24);
                bus(l, Vec2::new(x, pic.y + pic.h * y + 8.0), pic.w * 0.3, a([ACCENT(), ACCENT_2(), OK()][k].mix(TEXT(), 0.2)), a(PANEL().alpha(0.8)));
            }
        }
        Art::Board => {
            sky(l, pic, a(DANGER().mix(BACKDROP(), 0.62)), a(DANGER().mix(TEXT(), 0.15).mix(PANEL(), 0.4)));
            // a departure board
            let b = Rect::new(pic.x + pic.w * 0.12, pic.y + pic.h * 0.14, pic.w * 0.76, pic.h * 0.58);
            l.ui.p().rounded(b, 6.0, a(RAIL()));
            l.ui.p().rounded_border(b, 6.0, 1.0, a(EDGE()));
            for i in 0..4 {
                let y = b.y + 10.0 + i as f32 * (b.h - 20.0) / 4.0;
                l.ui.p().rounded(Rect::new(b.x + 10.0, y, 22.0, 9.0), 2.0, a(ACCENT_2()));
                l.ui.p().rect(Rect::new(b.x + 40.0, y + 2.0, b.w * (0.35 + 0.08 * (i % 2) as f32), 5.0), a(TEXT_SOFT().alpha(0.6)));
                l.ui.p().rect(Rect::new(b.right() - 40.0, y + 2.0, 28.0, 5.0), a(ACCENT_2().alpha(0.75)));
            }
        }
        Art::Fleet => {
            sky(l, pic, a(ACCENT().mix(BACKDROP(), 0.7)), a(TEXT_DIM().mix(PANEL(), 0.3)));
            // a hall with buses in a row
            l.ui.p().rect(Rect::new(pic.x, ground - 2.0, pic.w, 2.0), a(TEXT_FAINT()));
            for k in 0..3 {
                let len = pic.w * 0.42;
                let x = pic.x + pic.w * (-0.08 + k as f32 * 0.36);
                bus(l, Vec2::new(x, ground - 2.0 - k as f32 * 0.0), len, a([TEXT(), ACCENT_2(), DANGER()][k].mix(PANEL(), 0.15)), a(PANEL().mix(ACCENT(), 0.25)));
            }
            for k in 0..6 {
                let x = pic.x + pic.w * k as f32 / 5.0;
                l.ui.p().line(Vec2::new(x, pic.y), Vec2::new(pic.x + pic.w * 0.5 + (x - pic.x - pic.w * 0.5) * 1.6, ground - pic.h * 0.5), 1.0, a(LIFT().alpha(0.06)));
            }
        }
        Art::Plan => {
            l.ui.p().rect(pic, a(RAIL()));
            // a street grid and a line drawn over it
            let step = 22.0;
            let mut x = pic.x + 8.0;
            while x < pic.right() {
                l.ui.p().rect(Rect::new(x, pic.y, 1.0, pic.h), a(ROAD().alpha(0.7)));
                x += step;
            }
            let mut y = pic.y + 6.0;
            while y < pic.bottom() {
                l.ui.p().rect(Rect::new(pic.x, y, pic.w, 1.0), a(ROAD().alpha(0.7)));
                y += step;
            }
            let pts = [(0.06, 0.82), (0.06, 0.5), (0.38, 0.5), (0.38, 0.22), (0.72, 0.22), (0.72, 0.62), (0.96, 0.62)];
            let p = |q: (f32, f32)| Vec2::new(pic.x + pic.w * q.0, pic.y + pic.h * q.1);
            for s in pts.windows(2) {
                l.ui.p().line(p(s[0]), p(s[1]), 4.0, a(ACCENT()));
            }
            for q in [pts[0], pts[2], pts[4], pts[6]] {
                l.ui.p().circle(p(q), 6.0, a(TEXT()));
                l.ui.p().circle(p(q), 3.5, a(ACCENT()));
            }
        }
        Art::Paint => {
            sky(l, pic, a(TEXT_DIM().mix(BACKDROP(), 0.5)), a(TEXT_SOFT().mix(PANEL(), 0.25)));
            // a bus half painted, the brush's stroke still on it
            let len = pic.w * 0.82;
            let at = Vec2::new(pic.x + pic.w * 0.09, ground + 4.0);
            bus(l, at, len, a(TEXT().mix(PANEL(), 0.1)), a(PANEL().mix(ACCENT(), 0.25)));
            let h = len * 0.28;
            let split = at.x + len * (0.45 + 0.25 * hover);
            l.ui.p().rect(Rect::new(at.x + 3.0, at.y - h * 0.42, split - at.x - 3.0, h * 0.30), a(ACCENT()));
            l.ui.p().rect(Rect::new(at.x + 3.0, at.y - h * 0.12, split - at.x - 3.0, h * 0.06), a(ACCENT_2()));
            l.ui.p().circle(Vec2::new(split, at.y - h * 0.27), 6.0, a(ACCENT()));
        }
    }
}

/// The chips under the cards: the driver (level and hours with them), then the other pages.
fn chips(l: &mut Launcher, r: Rect) {
    let p = l.state.profile.clone().filter(|p| p.exists);
    let driver = match &p {
        Some(p) => format!("{}: {}  ·  {} {}  ·  {}", omsi_ui::tr("Driver"), p.name, omsi_ui::tr("Level"), p.level, hours(p.hours)),
        None => omsi_ui::tr("Create a driver").to_string(),
    };
    let mut items: Vec<(Page, &str, String)> = vec![(Page::Profile, "account_circle", driver)];
    for (page, icon, name) in CHIPS {
        items.push((page, icon, omsi_ui::tr(name).to_string()));
    }
    let widths: Vec<f32> = items.iter().map(|(_, _, t)| l.ui.width(t, 13.0, Weight::Medium) + 54.0).collect();
    let gap = 10.0;
    let total: f32 = widths.iter().sum::<f32>() + gap * (items.len() - 1) as f32;
    // (a narrow window: as many as fit, the driver always)
    let mut x = r.x + ((r.w - total) * 0.5).max(0.0);
    for (k, ((page, icon, text), w)) in items.iter().zip(widths).enumerate() {
        if x + w > r.right() && k > 0 {
            break;
        }
        let c = Rect::new(x, r.y, w, r.h);
        x += w + gap;
        let id = id_of(&format!("home-chip-{k}"));
        let (h, _, clicked) = l.ui.interact(id, c);
        let t = l.ui.anim(id, if h { 1.0 } else { 0.0 }, 0.07);
        l.ui.solid(c);
        l.ui.p().rounded(c, RADIUS, PANEL().mix(HOVER(), t));
        l.ui.p().rounded_border(c, RADIUS, 1.0, EDGE().mix(ACCENT(), t * 0.8));
        l.ui.icon(icon, Vec2::new(c.x + 22.0, c.center().y), 18.0, if k == 0 { ACCENT_2() } else { TEXT_SOFT().mix(TEXT(), t) });
        l.ui.text_in(text, Rect::new(c.x + 40.0, c.y, c.w - 48.0, c.h), 13.0, Weight::Medium, TEXT(), Align::Left);
        if clicked {
            l.go(*page);
        }
    }
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

    #[test]
    fn the_focused_card_stands_in_the_middle() {
        let (w, g) = (200.0, 20.0);
        for at in [0.0, 1.0, 3.0] {
            let x = row_x(700.0, w, g, at) + at * (w + g);
            assert!((x + w * 0.5 - 700.0).abs() < 1e-3);
        }
    }

    #[test]
    fn every_card_leads_somewhere_of_its_own() {
        for (i, a) in CARDS.iter().enumerate() {
            for b in &CARDS[i + 1..] {
                assert_ne!(a.mode, b.mode);
            }
        }
    }
}
