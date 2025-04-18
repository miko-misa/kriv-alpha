use crate::{
  expr::{BinaryExpr, Expr, GroupingExpr, LiteralExpr, UnaryExpr, Visitor},
  token::{LiteralObject, Token, TokenType},
};

pub struct Interpreter;

impl Interpreter {
  pub fn evaluate(&mut self, expr: &dyn Expr<LiteralObject>) -> LiteralObject {
    expr.accept(self)
  }

  fn is_equal(&mut self, a: LiteralObject, b: LiteralObject) -> bool {
    if a == LiteralObject::Nil && b == LiteralObject::Nil {
      return true;
    } else if a == LiteralObject::Nil {
      return false;
    }
    a == b
  }

  fn binary_number(
    &mut self,
    left: LiteralObject,
    right: LiteralObject,
    operator: Token,
  ) -> LiteralObject {
    if let (LiteralObject::Number(l), LiteralObject::Number(r)) = (left, right) {
      match operator.token_type {
        TokenType::Plus => return LiteralObject::Number(l + r),
        TokenType::Minus => return LiteralObject::Number(l - r),
        TokenType::Star => return LiteralObject::Number(l * r),
        TokenType::Slash => return LiteralObject::Number(l / r),
        TokenType::Greater => return LiteralObject::Bool(l > r),
        TokenType::GreaterEqual => return LiteralObject::Bool(l >= r),
        TokenType::Less => return LiteralObject::Bool(l < r),
        TokenType::LessEqual => return LiteralObject::Bool(l <= r),
        TokenType::EqualEqual => return LiteralObject::Bool(l == r),
        TokenType::BangEqual => return LiteralObject::Bool(l != r),
        _ => {}
      }
    }
    panic!("Invalid binary operation");
  }
}

impl Visitor<LiteralObject> for Interpreter {
  fn visit_binary_expr(&mut self, expr: &BinaryExpr<LiteralObject>) -> LiteralObject {
    let right = self.evaluate(expr.right.as_ref());
    let left = self.evaluate(expr.left.as_ref());
    match expr.operator.token_type {
      TokenType::BangEqual => return LiteralObject::Bool(!self.is_equal(right, left)),
      TokenType::EqualEqual => return LiteralObject::Bool(self.is_equal(right, left)),
      TokenType::Plus => match (left.clone(), right.clone()) {
        (LiteralObject::Number(l), LiteralObject::Number(r)) => LiteralObject::Number(l + r),
        (LiteralObject::Text(l), LiteralObject::Text(r)) => LiteralObject::Text(l + &r),
        (LiteralObject::Text(l), LiteralObject::Number(r)) => {
          LiteralObject::Text(l + &r.to_string())
        }
        (LiteralObject::Number(l), LiteralObject::Text(r)) => {
          LiteralObject::Text(l.to_string() + &r)
        }
        _ => panic!("Invalid addition operation"),
      },
      TokenType::Greater
      | TokenType::GreaterEqual
      | TokenType::Less
      | TokenType::LessEqual
      | TokenType::Minus
      | TokenType::Star
      | TokenType::Slash => {
        return self.binary_number(left, right, expr.operator.clone());
      }
      TokenType::And | TokenType::Or => {
        if let (LiteralObject::Bool(l), LiteralObject::Bool(r)) = (left, right) {
          return LiteralObject::Bool(if expr.operator.token_type == TokenType::And {
            l && r
          } else {
            l || r
          });
        } else {
          panic!("Invalid boolean operation");
        }
      }
      _ => return LiteralObject::Nil,
    }
  }

  fn visit_grouping_expr(&mut self, expr: &GroupingExpr<LiteralObject>) -> LiteralObject {
    self.evaluate(expr.expression.as_ref())
  }

  fn visit_literal_expr(&mut self, expr: &LiteralExpr) -> LiteralObject {
    expr.value.clone()
  }

  fn visit_unary_expr(&mut self, expr: &UnaryExpr<LiteralObject>) -> LiteralObject {
    let right = self.evaluate(expr.right.as_ref());
    match expr.operator.token_type {
      TokenType::Minus => {
        if let LiteralObject::Number(n) = right {
          return LiteralObject::Number(-n);
        }
      }
      TokenType::Bang => {
        return LiteralObject::Bool(!self.is_equal(right, LiteralObject::Nil));
      }
      _ => {}
    }
    panic!("Invalid unary operation");
  }
}
