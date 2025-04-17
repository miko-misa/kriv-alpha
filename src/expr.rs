use crate::token::{LiteralObject, Token};

pub trait Expr<T> {
  fn accept(&self, visitor: &mut dyn Visitor<T>) -> T;
}

pub trait Visitor<T> {
  fn visit_binary_expr(&mut self, expr: &BinaryExpr<T>) -> T;
  fn visit_grouping_expr(&mut self, expr: &GroupingExpr<T>) -> T;
  fn visit_literal_expr(&mut self, expr: &LiteralExpr) -> T;
  fn visit_unary_expr(&mut self, expr: &UnaryExpr<T>) -> T;
}

pub struct BinaryExpr<T> {
  pub left: Box<dyn Expr<T>>,
  pub operator: Token,
  pub right: Box<dyn Expr<T>>,
}

impl<T> Expr<T> for BinaryExpr<T> {
  fn accept(&self, visitor: &mut dyn Visitor<T>) -> T {
    visitor.visit_binary_expr(self)
  }
}

pub struct GroupingExpr<T> {
  pub expression: Box<dyn Expr<T>>,
}

impl<T> Expr<T> for GroupingExpr<T> {
  fn accept(&self, visitor: &mut dyn Visitor<T>) -> T {
    visitor.visit_grouping_expr(self)
  }
}

pub struct LiteralExpr {
  pub value: LiteralObject,
}

impl<T> Expr<T> for LiteralExpr {
  fn accept(&self, visitor: &mut dyn Visitor<T>) -> T {
    visitor.visit_literal_expr(self)
  }
}

pub struct UnaryExpr<T> {
  pub operator: Token,
  pub right: Box<dyn Expr<T>>,
}

impl<T> Expr<T> for UnaryExpr<T> {
  fn accept(&self, visitor: &mut dyn Visitor<T>) -> T {
    visitor.visit_unary_expr(self)
  }
}
