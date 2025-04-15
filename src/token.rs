#[derive(Debug, Clone)]
pub enum TokenType {
  LEFT_PAREN,
  RIGHT_PAREN,
  LEFT_BRACE,
  RIGHT_BRACE,
  COMMA,
  DOT,
  MINUS,
  PLUS,
  SEMICOLON,
  SLASH,
  STAR,
  BANG,
  BANG_EQUAL,
  EQUAL,
  EQUAL_EQUAL,
  GREATER,
  GREATER_EQUAL,
  LESS,
  LESS_EQUAL,
  IDENTIFIER,
  STRING,
  NUMBER,
  AND,
  CLASS,
  ELSE,
  FALSE,
  FUN,
  FOR,
  IF,
  NIL,
  OR,
  PRINT,
  RETURN,
  SUPER,
  THIS,
  TRUE,
  VAR,
  WHILE,
  EOF,
}

#[derive(Debug, Clone)]
pub enum Object {
  Text(String), // Ensure this variant is used in your code or remove it if unnecessary
  Number(f64),
  Bool(bool),
  Nil,
}

#[derive(Clone)]
pub struct Token {
  token_type: TokenType,
  lexeme: String,
  literal: Object,
  line: usize,
}

impl Token {
  pub fn new(token_type: TokenType, lexeme: String, literal: Object, line: usize) -> Self {
    Token {
      token_type,
      lexeme,
      literal,
      line,
    }
  }

  pub fn to_string(&self) -> String {
    format!(
      "Token {{ type: {:?}, lexeme: {}, literal: {:?} }}",
      self.token_type, self.lexeme, self.literal,
    )
  }
}
