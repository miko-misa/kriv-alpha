use crate::token::{LiteralObject, Token};

pub trait Expr {
  fn accept(&self, visitor: &mut dyn Visitor) -> LiteralObject;
}

pub trait Visitor {
  fn visit_binary_expr(&self, expr: &BinaryExpr) -> LiteralObject;
  fn visit_grouping_expr(&self, expr: &GroupingExpr) -> LiteralObject;
  fn visit_literal_expr(&self, expr: &LiteralExpr) -> LiteralObject;
  fn visit_unary_expr(&self, expr: &UnaryExpr) -> LiteralObject;
}

pub struct BinaryExpr {
  pub left: Box<dyn Expr>,
  pub operator: Token,
  pub right: Box<dyn Expr>,
}

impl Expr for BinaryExpr {
  fn accept(&self, visitor: &mut dyn Visitor) -> LiteralObject {
    visitor.visit_binary_expr(self)
  }
}

pub struct GroupingExpr {
  pub expression: Box<dyn Expr>,
}

impl Expr for GroupingExpr {
  fn accept(&self, visitor: &mut dyn Visitor) -> LiteralObject {
    visitor.visit_grouping_expr(self)
  }
}

pub struct LiteralExpr {
  pub value: LiteralObject,
}

impl Expr for LiteralExpr {
  fn accept(&self, visitor: &mut dyn Visitor) -> LiteralObject {
    visitor.visit_literal_expr(self)
  }
}

pub struct UnaryExpr {
  pub operator: Token,
  pub right: Box<dyn Expr>,
}

impl Expr for UnaryExpr {
  fn accept(&self, visitor: &mut dyn Visitor) -> LiteralObject {
    visitor.visit_unary_expr(self)
  }
}
