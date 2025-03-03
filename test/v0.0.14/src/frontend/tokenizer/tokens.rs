
pub enum Token {
  // #[flatten]
  Keyword(Keyword),
  Identifier(Box<str>),
  // #[flatten]
  // Number(Number),
}
