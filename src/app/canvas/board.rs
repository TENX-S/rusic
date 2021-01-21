use super::View;

use tui::widgets::{Table, TableState};

#[derive(Debug)]
pub struct Board {
    pub state: TableState,
    pub items: Vec<Vec<String>>,
}

// impl View for Board {
//
// }

