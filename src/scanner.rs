use crate::token::{Object, Token, TokenType};

pub struct Scanner {
  source: String,
  tokens: Vec<Token>,
  start: usize,
  current: usize,
  line: usize,
}

impl Scanner {
  pub fn new(source: String, line: usize) -> Self {
    Scanner {
      source,
      tokens: Vec::new(),
      start: 0,
      current: 0,
      line,
    }
  }

  pub fn scan_tokens(&mut self) -> Vec<Token> {
    while !self.is_at_end() {
      self.start = self.current;
      self.scan_token();
    }
    self.tokens.push(Token::new(
      TokenType::Eof,
      String::new(),
      Object::None,
      self.line,
      self.start,
    ));
    self.tokens.clone()
  }

  fn is_at_end(&self) -> bool {
    self.current >= self.source.len()
  }

  fn scan_token(&mut self) {
    let c = self.advance();
    match c {
      '(' => self.add_token(TokenType::LeftParen),
      ')' => self.add_token(TokenType::RightParen),
      '{' => self.add_token(TokenType::LeftBrace),
      '}' => self.add_token(TokenType::RightBrace),
      ',' => self.add_token(TokenType::Comma),
      '.' => self.add_token(TokenType::Dot),
      '-' => self.add_token(TokenType::Minus),
      '+' => self.add_token(TokenType::Plus),
      ';' => self.add_token(TokenType::Semicolon),
      '*' => self.add_token(TokenType::Star),
      '!' => {
        let token_type = if self.match_next('=') {
          TokenType::BangEqual
        } else {
          TokenType::Bang
        };
        self.add_token(token_type);
      }
      '=' => {
        let token_type = if self.match_next('=') {
          TokenType::EqualEqual
        } else {
          TokenType::Equal
        };
        self.add_token(token_type);
      }
      '<' => {
        let token_type = if self.match_next('=') {
          TokenType::LessEqual
        } else {
          TokenType::Less
        };
        self.add_token(token_type);
      }
      '>' => {
        let token_type = if self.match_next('=') {
          TokenType::GreaterEqual
        } else {
          TokenType::Greater
        };
        self.add_token(token_type);
      }
      '/' => {
        if self.match_next('/') {
          while !self.is_at_end() && !self.peek().is_whitespace() && self.peek() != '\n' {
            self.advance();
          }
        } else {
          self.add_token(TokenType::Slash);
        }
      }
      ' ' | '\r' | '\t' => {}
      '\n' => {
        self.line += 1;
      }
      '"' => {
        self.string();
      }
      _ => {
        if c.to_string().parse::<i32>().is_ok() {
          self.number();
        } else if c.is_alphabetic() {
          self.identifier();
        }
      }
    }
  }

  fn advance(&mut self) -> char {
    let c = self.source.chars().nth(self.current).unwrap();
    self.current += 1;
    c
  }

  fn match_next(&mut self, expected: char) -> bool {
    if self.is_at_end() {
      return false;
    }
    if self.source.chars().nth(self.current).unwrap() != expected {
      return false;
    }
    self.current += 1;
    true
  }

  fn peek(&self) -> char {
    if self.is_at_end() {
      return '\0';
    }
    self.source.chars().nth(self.current).unwrap()
  }

  fn peek_next(&self) -> char {
    if self.current + 1 >= self.source.len() {
      return '\0';
    }
    self.source.chars().nth(self.current + 1).unwrap()
  }

  fn add_token(&mut self, token_type: TokenType) {
    self.add_token_with_literal(token_type, Object::None);
  }

  fn add_token_with_literal(&mut self, token_type: TokenType, literal: Object) {
    let text = &self.source[self.start..self.current];
    self.tokens.push(Token::new(
      token_type,
      text.to_string(),
      literal,
      self.line,
      self.start,
    ));
  }

  fn string(&mut self) {
    while !self.is_at_end() && self.peek() != '"' {
      if self.peek() == '\n' {
        self.line += 1;
      }
      self.advance();
    }
    if self.is_at_end() {
      // Handle error: unterminated string
      return;
    }
    self.advance();
    let value = &self.source[self.start + 1..self.current - 1];
    self.add_token_with_literal(TokenType::String, Object::Text(value.to_string()));
  }

  fn number(&mut self) {
    while self.peek().is_digit(10) {
      self.advance();
    }
    if self.peek() == '.' && self.peek_next().is_digit(10) {
      self.advance();
      while self.peek().is_digit(10) {
        self.advance();
      }
    }
    let value = &self.source[self.start..self.current];
    let number: f64 = value.parse().unwrap();
    self.add_token_with_literal(TokenType::Number, Object::Number(number));
  }

  fn identifier(&mut self) {
    while self.peek().is_alphanumeric() || self.peek() == '_' {
      self.advance();
    }
    let text = &self.source[self.start..self.current];
    let token_type = match text {
      "and" => TokenType::And,
      "class" => TokenType::Class,
      "else" => TokenType::Else,
      "false" => TokenType::False,
      "for" => TokenType::For,
      "fun" => TokenType::Fun,
      "if" => TokenType::If,
      "nil" => TokenType::Nil,
      "or" => TokenType::Or,
      "print" => TokenType::Print,
      "return" => TokenType::Return,
      "super" => TokenType::Super,
      "this" => TokenType::This,
      "true" => TokenType::True,
      "var" => TokenType::Var,
      "while" => TokenType::While,
      _ => TokenType::Identifier,
    };
    if token_type == TokenType::Nil {
      self.add_token_with_literal(token_type, Object::Nil);
      return;
    }
    self.add_token(token_type);
  }
}
