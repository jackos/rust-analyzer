//! Holds notebook cells in memory, to reconstruct changes to the document when receiving a request from vscode-notebook-cell
//! and to retrieve the offset that should be applied depending on the cell index
use std::collections::HashMap;

use lsp_types::{Position, Range};
#[derive(Clone, Debug)]
pub(crate) struct Notebook {
    fragments: HashMap<i16, String>,
}

impl Default for Notebook {
    fn default() -> Self {
        let mut fragments: HashMap<i16, String> = HashMap::new();
        fragments.insert(-1, "fn main() {".to_string());
        Self { fragments }
    }
}

impl Notebook {
    pub fn insert_cell(&mut self, fragment: &str, text: String) {
        let index: i16 = fragment[2..fragment.len()].parse().unwrap();
        self.fragments.insert(index, text);
    }

    pub fn get_program(&self) -> String {
        let mut cells: Vec<_> = self.fragments.clone().into_iter().collect();
        cells.sort_by(|x, y| x.0.cmp(&y.0));
        let mut text =
            cells.iter().fold("".to_string(), |acc, item| format!("{}{}\n", acc, item.1));
        text.push('}');
        text
    }

    pub fn get_line_offset(&self, fragment: &str) -> u32 {
        let index: i16 = fragment[2..fragment.len()].parse().unwrap();
        let sum: usize = self
            .fragments
            .clone()
            .into_iter()
            .filter(|x| x.0 < index)
            .map(|x| x.1.lines().count())
            .sum();
        sum as u32
    }

    pub fn get_char_offset(&self, fragment: &str) -> u32 {
        let index: i16 = fragment[2..fragment.len()].parse().unwrap();
        let sum: usize =
            self.fragments.clone().into_iter().filter(|x| x.0 < index).map(|x| x.1.len()).sum();
        sum as u32
    }

    pub fn add_lines_to_position(&self, position: &mut Position, fragment: &str) {
        let offset = self.get_line_offset(fragment);
        position.line += offset;
    }

    pub fn add_lines_to_range(&self, range: &mut Range, fragment: &str) {
        let offset = self.get_line_offset(fragment);
        range.start.line += offset;
        range.end.line += offset;
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
