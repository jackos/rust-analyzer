//! Holds notebook cells in memory, to reconstruct changes to the document when receiving a request from vscode-notebook-cell
//! and to retrieve the offset that should be applied depending on the cell index
use std::collections::HashMap;

use lsp_types::{Position, Range};
#[derive(Default, Clone, Debug)]
pub(crate) struct Notebook {
	pub(crate) cells: HashMap<u16, String>,
	pub(crate) line_offset: HashMap<u16, u32>,
	pub(crate) character_offset: HashMap<u16, u32>,
}

impl Notebook {
	pub fn insert_cell(&mut self, fragment: &str, text: String) {
		let index: u16 = fragment[2..fragment.len()].parse().unwrap();
		self.cells.insert(index, text.clone());
		self.character_offset.insert(index, text.len() as u32);
		self.line_offset.insert(index, text.lines().count() as u32);
	}


	pub fn get_program(&self) -> String {
		let mut cells: Vec<_> = self.cells.clone().into_iter().collect();
		cells.sort_by(|x, y| x.0.cmp(&y.0));
		let mut text = cells.iter().fold("fn main() {\n".to_string(), |acc, item| {
			format!("{}{}\n", acc, item.1)
		});
		text.push('}');
		text
	}

	pub fn get_line_offset(&self, fragment: &str) -> u32 {
		let index: u16 = fragment[2..fragment.len()].parse().unwrap();
		self.line_offset.clone().into_iter().filter(|x| x.0 < index).map(|x| x.1).sum()
	}

	pub fn get_char_offset(&self, fragment: &str) -> u32 {
		let index: u16 = fragment[2..fragment.len()].parse().unwrap();
		self.character_offset.clone().into_iter().filter(|x| x.0 < index).map(|x| x.1).sum()
	}
}

pub(crate) fn offset_position(position: Position, offset: u32) -> Position {
    Position::new(position.line + offset + 1, position.character)
}

pub(crate) fn offset_range(range: Range, offset: u32) -> Range {
    let start = Position::new(range.start.line + offset + 1, range.start.character);
    let end = Position::new(range.end.line + offset + 1, range.end.character);
    Range::new(start, end)
}
