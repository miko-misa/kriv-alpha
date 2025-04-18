use std::fmt;

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

#[derive(Debug, Clone, PartialEq)]
pub enum LiteralObject {
  Text(String), // Ensure this variant is used in your code or remove it if unnecessary
  Number(f64),
  Bool(bool),
  Nil,
}

impl fmt::Display for LiteralObject {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    match self {
      LiteralObject::Text(s) => write!(f, "{}", s),
      LiteralObject::Number(n) => write!(f, "{}", n),
      LiteralObject::Bool(b) => write!(f, "{}", b),
      LiteralObject::Nil => write!(f, "nil"),
    }
  }
}

#[derive(Clone)]
pub struct Token {
  pub token_type: TokenType,
  pub lexeme: String,
  pub literal: Option<LiteralObject>,
  pub line: usize,
  pub start: usize,
}

impl Token {
  pub fn new(
    token_type: TokenType,
    lexeme: String,
    literal: Option<LiteralObject>,
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
    let literal = match &self.literal {
      Some(literal) => match literal {
        LiteralObject::Text(text) => format!("\"{}\"", text),
        LiteralObject::Number(num) => num.to_string(),
        LiteralObject::Bool(b) => b.to_string(),
        LiteralObject::Nil => "nil".to_string(),
      },
      None => "None".to_string(),
    };
    format!(
      "Token {{ type: {:?}, lexeme: {}, literal: {:?} }}",
      self.token_type, self.lexeme, literal,
    )
  }
}
