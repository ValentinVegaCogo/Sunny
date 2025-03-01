macro_rules! debug {
	($($args:expr),+) => {{
		eprint!("[{}] ", crate::colors::yellow("debug"));
		eprintln!($($args),+);
	}};
}

macro_rules! Error {
	($name:ident, $($args:expr),+) => {{
		eprint!("{}: ", crate::colors::error(stringify!($name)));
		eprintln!($($args),+);
		::std::process::exit(1);
	}};
}

macro_rules! LoadError {
	($($args:expr),+) => {
		crate::errors::Error!(LoadError, $($args),+);
	};
}

macro_rules! ArgumentError {
	($($args:expr),+) => {
		crate::errors::Error!(ArgumentError, $($args),+);
	};
}

#[allow(unused)]
macro_rules! InternalError {
	($($args:expr),+) => {
		crate::errors::Error!(InternalError, $($args),+);
	};
}

macro_rules! SyntaxError {
	($($args:expr),+) => {
		crate::errors::Error!(SyntaxError, $($args),+);
	};
}

#[allow(unused)]
macro_rules! Warning {
	($($args:expr),+) => {{
		eprint!("{}: ", crate::colors::warning("Warning"));
		eprintln!($($args),+);
	}};
}

pub(crate) use debug;
pub(crate) use Error;
pub(crate) use LoadError;
pub(crate) use ArgumentError;
#[allow(unused)]
pub(crate) use InternalError;
pub(crate) use SyntaxError;
pub(crate) use Warning;

pub fn unexpected(chr: char) {
	match chr {
		| 'a'..='z' | '_' | 'A'..='Z'
		| '0'..='9'
		| '(' | ')' | '{' | '}' | '[' | ']'
		| '.' | ',' | ':' | ';'
		| '+' | '-' | '*' | '/' | '%'
		| '<' | '>'
		| '&' | '|' | '!'
		| '\'' | '"'
		| '=' | '?' | '@'
			=>
		{
			SyntaxError!("character '{chr}' unexpected here");
		}
		_ => {
			// U+{{{:06X}}}
			SyntaxError!("invalid character \\u{{{:x}}}", chr as u32);
		}
	}
}

pub fn unexpected_token(token: String) {
	match token.as_str() {
		| "fun"
		| "class" | "extends" | "static" | "private"
		| "var" | "const"
		| "match" | "case"
		| "if" | "else"
		| "for" | "in"
		| "import" | "from" | "as"
		| "while" | "until"
		| "loop" | "break" | "continue"
			=>
		{
			SyntaxError!("unexpected keyword {token:?} here");
		}
		_ => {
			SyntaxError!("unexpected identifier {token:?} here");
		}
	}
}