use std::fmt;
use super::CharsIter;
use super::tokens::Token;

pub fn parse_word(chars: &mut CharsIter, ch: char) -> (Token, usize) {
  let mut word = String::from(ch);
  while let Some(ch) = chars.peek() {
    if word.len() > 30 {
      syntax_err!("this identifier is too long");
    }
    match ch {
      'a'..='z' => {
        word.push(ch);
        chars.next();
      }
      // Can't be a keyword, no need to check
      'A'..='Z' | '0'..='9' | '_' => {
        word.push(ch);
        chars.next();
        while let Some(ch @ ('a'..='z' | 'A'..='Z' | '0'..='9' | '_')) = chars.peek() {
          if word.len() > 30 {
            syntax_err!("this identifier is too long");
          }
          word.push(ch);
          chars.next();
        }
        let len = word.len();
        return (Token::Ident(word), len);
      },
      _ => break
    }
  }
  let len = word.len();
  let token = match Keyword::parse(&word) {
    Some(kw) => Token::Keyword(kw),
    None => Token::Ident(word)
  };
  (token, len)
}

#[allow(unused)]
#[derive(Debug, Clone, PartialEq)]
pub enum Keyword {
  Use,
  As,

  All,
  Shared,
  Hidden,

  Const,
  State,
  Let,
  Var,

  TypeDef,

  Struct,
  Enum,
  BitSet,
  Idea,

  Class,
  Extends,
  Where,
  Impl,

  ArgStruct,
  Fun,
  Defer,
  Return,

  Unsafe,

  Async,
  Await,

  Loop,
  While,
  For,
  In,
  Break,
  Continue,

  If,
  Else,

  Match,
  Is,

  Macro,
  Case,

  SelfType,
  SuperType,
  Never
}

impl fmt::Display for Keyword {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    let word = match self {
      Self::Use => "use",
      Self::As => "as",
      Self::Const => "const",
      Self::State => "state",
      Self::Let => "let",
      Self::Var => "var",
      Self::TypeDef => "typedef",
      Self::Struct => "struct",
      Self::Enum => "enum",
      Self::BitSet => "bitset",
      Self::Idea => "idea",
      Self::Class => "class",
      Self::Extends => "extends",
      Self::Where => "where",
      Self::Impl => "impl",
      Self::ArgStruct => "argstruct",
      Self::Fun => "fun",
      Self::Defer => "defer",
      Self::Return => "return",
      Self::Unsafe => "unsafe",
      Self::Async => "async",
      Self::Await => "await",
      Self::Loop => "loop",
      Self::While => "while",
      Self::For => "for",
      Self::In => "in",
      Self::Break => "break",
      Self::Continue => "continue",
      Self::If => "if",
      Self::Else => "else",
      Self::Match => "match",
      Self::Is => "is",
      Self::Macro => "macro",
      Self::Case => "case",
      Self::Shared => "shared",
      Self::Hidden => "hidden",
      Self::All => "all",
      Self::SelfType => "Self",
      Self::SuperType => "Super",
      Self::Never => "never",
    };
    write!(f, "{word:?}")
  }
}

impl Keyword {
  pub fn parse(input: &str) -> Option<Self> {
    let keyword = match input {
      "use" => Self::Use,
      "as" => Self::As,
      "const" => Self::Const,
      "state" => Self::State,
      "var" => Self::Var,
      "let" => Self::Let,
      "class" => Self::Class,
      "struct" => Self::Struct,
      "enum" => Self::Enum,
      "idea" => Self::Idea,
      "impl" => Self::Impl,
      "argstruct" => Self::ArgStruct,
      "where" => Self::Where,
      "fun" => Self::Fun,
      "return" => Self::Return,
      "loop" => Self::Loop,
      "while" => Self::While,
      "for" => Self::For,
      "in" => Self::In,
      "break" => Self::Break,
      "continue" => Self::Continue,
      "if" => Self::If,
      "else" => Self::Else,
      "match" => Self::Match,
      "is" => Self::Is,
      "macro" => Self::Macro,
      "extends" => Self::Extends,
      "bitset" => Self::BitSet,
      "defer" => Self::Defer,
      "unsafe" => Self::Unsafe,
      "async" => Self::Async,
      "await" => Self::Await,
      "case" => Self::Case,
      "typedef" => Self::TypeDef,
      "shared" => Self::Shared,
      "hidden" => Self::Hidden,
      "all" => Self::All,
      "Self" => Self::SelfType,
      "Super" => Self::SuperType,
      "never" => Self::Never,
      _ => return None,
    };
    Some(keyword)
  }
}
