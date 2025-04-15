#[derive(Debug, Clone, PartialEq)]
pub enum TokenType {
  LeftParen,
  RightParen,
  LeftBrace,
  RightBrace,
  Comma,
  Dot,
  Minus,
  Plus,
  Semicolon,
  Slash,
  Star,
  Bang,
  BangEqual,
  Equal,
  EqualEqual,
  Greater,
  GreaterEqual,
  Less,
  LessEqual,
  Identifier,
  String,
  Number,
  And,
  Class,
  Else,
  False,
  Fun,
  For,
  If,
  Nil,
  Or,
  Print,
  Return,
  Super,
  This,
  True,
  Var,
  While,
  Eof,
}

#[derive(Debug, Clone)]
pub enum Object {
  Text(String), // Ensure this variant is used in your code or remove it if unnecessary
  Number(f64),
  Bool(bool),
  Nil,
  None,
}

#[derive(Clone)]
pub struct Token {
  token_type: TokenType,
  lexeme: String,
  literal: Object,
  line: usize,
  start: usize,
}

impl Token {
  pub fn new(
    token_type: TokenType,
    lexeme: String,
    literal: Object,
    line: usize,
    start: usize,
  ) -> Self {
    Token {
      token_type,
      lexeme,
      literal,
      line,
      start,
    }
  }

  pub fn to_string(&self) -> String {
    format!(
      "Token {{ type: {:?}, lexeme: {}, literal: {:?} }}",
      self.token_type, self.lexeme, self.literal,
    )
  }
}
