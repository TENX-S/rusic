pub mod board;
pub mod timeline;

use super::model::Model;
use board::Board;
use timeline::Timeline;
use tui::{Frame, backend::Backend, layout::Rect};

pub trait View {
    fn draw<B: Backend>(&self, f: &mut Frame<B>, area: Rect, model: &mut Model);
    fn on_tick(&self, model: &mut Model);
}

#[derive(Debug)]
pub struct Canvas {
    board: Board,
    timeline: Timeline,
}
