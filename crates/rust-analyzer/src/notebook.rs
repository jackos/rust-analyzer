//! Holds notebook cells in memory, to reconstruct changes to the document when receiving a request from vscode-notebook-cell
//! and to retrieve the offset that should be applied depending on the cell index
use std::collections::HashMap;

use lsp_types::{Position, Range};
#[derive(Clone, Debug)]
pub(crate) struct Notebook {
    pub(crate) prefix: String,
    pub(crate) cells: HashMap<u16, String>,
    pub(crate) line_offsets: HashMap<u16, u32>,
    pub(crate) character_offsets: HashMap<u16, u32>,
    pub(crate) suffix: String,
}

impl Default for Notebook {
    fn default() -> Self {
        Self {
            prefix: "fn main() {\n".to_string(),
            suffix: "}".to_string(),
            cells: Default::default(),
            line_offsets: Default::default(),
            character_offsets: Default::default(),
        }
    }
}

impl Notebook {
    pub fn insert_cell(&mut self, fragment: &str, text: String) {
        let index: u16 = fragment[2..fragment.len()].parse().unwrap();
        self.cells.insert(index, text.clone());
        self.character_offsets.insert(index, text.len() as u32);
        self.line_offsets.insert(index, text.lines().count() as u32);
    }

    pub fn get_program(&self) -> String {
        let mut cells: Vec<_> = self.cells.clone().into_iter().collect();
        cells.sort_by(|x, y| x.0.cmp(&y.0));
        let mut text =
            cells.iter().fold(self.prefix.clone(), |acc, item| format!("{}{}\n", acc, item.1));
        text.push_str(&self.suffix);
        text
    }

    pub fn get_line_offset(&self, fragment: &str) -> i32 {
        let index: u16 = fragment[2..fragment.len()].parse().unwrap();
        let sum: i32 =
            self.line_offsets.clone().into_iter().filter(|x| x.0 < index).map(|x| x.1 as i32).sum();
        sum + self.prefix.lines().count() as i32
    }

    pub fn get_char_offset(&self, fragment: &str) -> u32 {
        let index: u16 = fragment[2..fragment.len()].parse().unwrap();
        self.character_offsets.clone().into_iter().filter(|x| x.0 < index).map(|x| x.1).sum()
    }
}

pub(crate) fn offset_position(position: Position, offset: i32) -> Position {
    Position::new((position.line as i32 + offset) as u32, position.character)
}

pub(crate) fn offset_range(range: Range, offset: i32) -> Range {
    let start = Position::new((range.start.line as i32 + offset) as u32, range.start.character);
    let end = Position::new((range.end.line as i32 + offset) as u32, range.end.character);
    Range::new(start, end)
}
