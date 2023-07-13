use std::fmt::Display;
use crate::{
	errors::SyntaxError,
	dict::Id
};

pub struct Context<'a> {
	pub id: String,
	pub char_vec: Vec<char>, // for debugging
	pub chars: std::str::Chars<'a>,
	pub ch: char,
	pub char_count: usize,
	pub idx: usize,
	pub line: usize,
	pub column: usize
}

impl<'a> Display for Context<'a> {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		write!(f, "{}:{}:{} - chars[{}] = {:?}", self.id, self.line, self.column, self.idx, self.ch)
	}
}

impl<'a> Context<'a> {
	pub fn new(id: String, string: &'a String) -> Self {
		Context {
			id,
			ch: string.chars().next().unwrap(),
			char_count: string.len(),
			chars: string.chars(),
			char_vec: string.chars().collect::<Vec<char>>(),
			idx: 0,
			line: 0, // reserved for namespace name
			column: 1
		}
	}
	pub fn ignore_spaces(&mut self) {
		while self.idx < self.char_count {
			match self.ch {
				' ' | '\t' | '\n' | '\r' => {}
				// also ignore comments
				'#' => {
					while self.idx < self.char_count && self.ch != '\n' {
						self.next_char();
					}
				}
				_ => {
					break;
				}
			}
			self.next_char();
		}
	}
	pub fn next_char(&mut self) {
		match self.ch {
			'\n' => {
				self.column = 0;
				self.line += 1;
			}
			'\t' => {
				self.column += 4;
			}
			_ => {
				self.column += 1;
			}
		}
		self.idx += 1;
		self.ch = match self.chars.next() {
			None => {
				SyntaxError!(self, "unexpected end of file");
			}
			Some(ch) => ch
		};
		// println!("next_char: {:?}, :{}:{}", self.ch, self.line, self.column);
	}
	pub fn collect_word(&mut self) -> Id {
		let mut word = String::new();
		match self.ch {
			'a'..='z' | 'A'..='Z' | '_' => {
				word.push(self.ch);
			}
			_ => {
				SyntaxError!(self, "expected an identifier, got {:?}", self.ch);
			}
		}
		self.next_char();
		while self.idx < self.char_count {
			match self.ch {
				'a'..='z' | 'A'..='Z' | '_' | '0'..='9' => {
					word.push(self.ch);
				}
				_ => {
					break;
				}
			}
			self.next_char();
		}
		if word.is_empty() {
			SyntaxError!(self, "expected a word, got {:?}", self.ch);
		}
		Id::from(word.as_str())
	}
}