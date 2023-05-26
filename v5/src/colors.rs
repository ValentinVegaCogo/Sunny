pub fn yellow(text: &str) -> String {
	format!("\x1B[33m{text}\x1B[39m")
}

pub fn gray(text: &str) -> String {
	format!("\x1B[90m{text}\x1B[39m")
}

pub fn error(text: &str) -> String {
	format!("\u{1b}[31m\u{1b}[1m{text}\u{1b}[22m\u{1b}[39m")
}

pub fn warning(text: &str) -> String {
	format!("\u{1b}[33m\u{1b}[1m{text}\u{1b}[22m\u{1b}[39m")
}