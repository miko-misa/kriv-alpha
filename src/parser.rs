use crate::{
  expr::{Expr, *},
  token::{LiteralObject, Token, TokenType},
};

pub struct Parser {
  current: usize,
  tokens: Vec<Token>,
}

impl Parser {
  pub fn new(tokens: Vec<Token>) -> Self {
    Parser { current: 0, tokens }
  }

  pub fn parse<T: 'static>(&mut self) -> Box<dyn Expr<T>> {
    match self.expression() {
      Ok(expr) => expr,
      Err(err) => panic!("Parse error: {}", err),
    }
  }

  fn expression<T: 'static>(&mut self) -> Result<Box<dyn Expr<T>>, String> {
    self.equality()
  }

  fn equality<T: 'static>(&mut self) -> Result<Box<dyn Expr<T>>, String> {
    let mut expr = self.comparison()?;

    while self.match_tokens(&[TokenType::BangEqual, TokenType::EqualEqual]) {
      let operator = self.previous().clone();
      let right = self.comparison()?;
      expr = Box::new(BinaryExpr {
        left: expr,
        operator,
        right,
      });
    }

    Ok(expr)
  }

  fn comparison<T: 'static>(&mut self) -> Result<Box<dyn Expr<T>>, String> {
    let mut expr = self.term()?;

    while self.match_tokens(&[
      TokenType::Greater,
      TokenType::GreaterEqual,
      TokenType::Less,
      TokenType::LessEqual,
    ]) {
      let operator = self.previous().clone();
      let right = self.term()?;
      expr = Box::new(BinaryExpr {
        left: expr,
        operator,
        right,
      });
    }

    Ok(expr)
  }

  fn term<T: 'static>(&mut self) -> Result<Box<dyn Expr<T>>, String> {
    let mut expr = self.factor()?;

    while self.match_tokens(&[TokenType::Minus, TokenType::Plus]) {
      let operator = self.previous().clone();
      let right = self.factor()?;
      expr = Box::new(BinaryExpr {
        left: expr,
        operator,
        right,
      });
    }

    Ok(expr)
  }

  fn factor<T: 'static>(&mut self) -> Result<Box<dyn Expr<T>>, String> {
    let mut expr = self.unary()?;

    while self.match_tokens(&[TokenType::Slash, TokenType::Star]) {
      let operator = self.previous().clone();
      let right = self.unary()?;
      expr = Box::new(BinaryExpr {
        left: expr,
        operator,
        right,
      });
    }

    Ok(expr)
  }

  fn unary<T: 'static>(&mut self) -> Result<Box<dyn Expr<T>>, String> {
    if self.match_tokens(&[TokenType::Bang, TokenType::Minus]) {
      let operator = self.previous().clone();
      let right = self.unary()?;
      return Ok(Box::new(UnaryExpr { operator, right }));
    }

    self.primary()
  }

  fn primary<T: 'static>(&mut self) -> Result<Box<dyn Expr<T>>, String> {
    if self.match_tokens(&[TokenType::False]) {
      return Ok(Box::new(LiteralExpr {
        value: LiteralObject::Bool(false),
      }));
    }
    if self.match_tokens(&[TokenType::True]) {
      return Ok(Box::new(LiteralExpr {
        value: LiteralObject::Bool(true),
      }));
    }
    if self.match_tokens(&[TokenType::Nil]) {
      return Ok(Box::new(LiteralExpr {
        value: LiteralObject::Nil,
      }));
    }

    if self.match_tokens(&[TokenType::Number, TokenType::String]) {
      return Ok(Box::new(LiteralExpr {
        value: self.previous().literal.clone().unwrap(),
      }));
    }

    if self.match_tokens(&[TokenType::LeftParen]) {
      let expr = self.expression()?;
      self.consume(TokenType::RightParen, "Expect ')' after expression.")?;
      return Ok(Box::new(GroupingExpr { expression: expr }));
    }

    Err("Expect expression.".to_string())
  }

  fn consume(&mut self, token_type: TokenType, message: &str) -> Result<&Token, String> {
    if self.check(token_type) {
      return Ok(self.advance());
    }
    Err(format!("{} at line {}", message, self.peek().line))
  }

  fn previous(&self) -> &Token {
    &self.tokens[self.current - 1]
  }

  fn synchronize(&mut self) {
    self.advance();
    while !self.is_at_end() {
      if self.previous().token_type == TokenType::Semicolon {
        return;
      }
      match self.peek().token_type {
        TokenType::Class
        | TokenType::Fun
        | TokenType::Var
        | TokenType::For
        | TokenType::If
        | TokenType::While
        | TokenType::Print
        | TokenType::Return => return,
        _ => {}
      }
      self.advance();
    }
  }

  fn check(&mut self, token_type: TokenType) -> bool {
    if self.is_at_end() {
      return false;
    }
    self.peek().token_type == token_type
  }

  fn match_tokens(&mut self, tokens: &[TokenType]) -> bool {
    if !self.is_at_end() && tokens.contains(&self.peek().token_type) {
      self.advance();
      return true;
    }
    false
  }

  fn advance(&mut self) -> &Token {
    if !self.is_at_end() {
      self.current += 1;
    }
    &self.tokens[self.current - 1]
  }

  fn is_at_end(&self) -> bool {
    self.peek().token_type == TokenType::Eof
  }

  fn peek(&self) -> Token {
    self.tokens[self.current].clone()
  }
}
