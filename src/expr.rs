pub trait Expr<T> {
  fn accept(&self, visitor: &dyn Visitor<T>) -> T;
}

pub trait Visitor<T> {
  fn visit_binary_expr(&self, expr: &dyn Expr<T>) -> T;
  fn visit_grouping_expr(&self, expr: &dyn Expr<T>) -> T;
  fn visit_literal_expr(&self, expr: &dyn Expr<T>) -> T;
  fn visit_unary_expr(&self, expr: &dyn Expr<T>) -> T;
}

pub struct BinaryExpr {
  left: Box<dyn Expr<T>>,
  operator: Token,
  right: Box<dyn Expr<T>>,
}

impl Expr<T> for BinaryExpr {
  fn accept(&self, visitor: &dyn Visitor<T>) -> T {
    visitor.visit_binary_expr(self)
  }
}

pub struct GroupingExpr {
  expression: Box<dyn Expr<T>>,
}

impl Expr<T> for GroupingExpr {
  fn accept(&self, visitor: &dyn Visitor<T>) -> T {
    visitor.visit_grouping_expr(self)
  }
}

pub struct LiteralExpr {
  value: LiteralObject,
}

impl Expr<T> for LiteralExpr {
  fn accept(&self, visitor: &dyn Visitor<T>) -> T {
    visitor.visit_literal_expr(self)
  }
}

pub struct UnaryExpr {
  operator: Token,
  right: Box<dyn Expr<T>>,
}

impl Expr<T> for UnaryExpr {
  fn accept(&self, visitor: &dyn Visitor<T>) -> T {
    visitor.visit_unary_expr(self)
  }
}
