//! The Line editor.

use super::Launcher;
use omsi_ui::Rect;

#[derive(Default)]
pub struct LinesView {}

pub fn draw(l: &mut Launcher, area: Rect) {
    l.page_title(area, "Line editor", "Lines of your own.");
}
