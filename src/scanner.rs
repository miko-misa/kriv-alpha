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
      TokenType::EOF,
      String::new(),
      Object::Text("".to_string()),
      self.line,
    ));
    self.tokens.clone()
  }

  fn is_at_end(&self) -> bool {
    self.current >= self.source.len()
  }

  fn scan_token(&mut self) {
    let c = self.advance();
    match c {
      '(' => self.add_token(TokenType::LEFT_PAREN),
      ')' => self.add_token(TokenType::RIGHT_PAREN),
      '{' => self.add_token(TokenType::LEFT_BRACE),
      '}' => self.add_token(TokenType::RIGHT_BRACE),
      ',' => self.add_token(TokenType::COMMA),
      '.' => self.add_token(TokenType::DOT),
      '-' => self.add_token(TokenType::MINUS),
      '+' => self.add_token(TokenType::PLUS),
      ';' => self.add_token(TokenType::SEMICOLON),
      '*' => self.add_token(TokenType::STAR),
      '!' => {
        let token_type = if self.match_next('=') {
          TokenType::BANG_EQUAL
        } else {
          TokenType::BANG
        };
        self.add_token(token_type);
      }
      '=' => {
        let token_type = if self.match_next('=') {
          TokenType::EQUAL_EQUAL
        } else {
          TokenType::EQUAL
        };
        self.add_token(token_type);
      }
      '<' => {
        let token_type = if self.match_next('=') {
          TokenType::LESS_EQUAL
        } else {
          TokenType::LESS
        };
        self.add_token(token_type);
      }
      '>' => {
        let token_type = if self.match_next('=') {
          TokenType::GREATER_EQUAL
        } else {
          TokenType::GREATER
        };
        self.add_token(token_type);
      }
      '/' => {
        if self.match_next('/') {
          while !self.is_at_end() && !self.peek().is_whitespace() && self.peek() != '\n' {
            self.advance();
          }
        } else {
          self.add_token(TokenType::SLASH);
        }
      }
      ' ' | '\r' | '\t' => {}
      '\n' => {
        self.line += 1;
      }
      '"' => {
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
        self.add_token_with_literal(TokenType::STRING, Object::Text(value.to_string()));
      }
      _ => {
        if c.to_string().parse::<i32>().is_ok() {
          self.number();
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
    self.add_token_with_literal(token_type, Object::Nil);
  }

  fn add_token_with_literal(&mut self, token_type: TokenType, literal: Object) {
    let text = &self.source[self.start..self.current];
    self
      .tokens
      .push(Token::new(token_type, text.to_string(), literal, self.line));
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
    self.add_token_with_literal(TokenType::NUMBER, Object::Number(number));
  }
}
