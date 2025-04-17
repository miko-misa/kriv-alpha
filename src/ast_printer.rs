use crate::{
  expr::{BinaryExpr, Expr, GroupingExpr, LiteralExpr, UnaryExpr, Visitor},
  parser::Parser,
  scanner::Scanner,
  token::LiteralObject,
};

pub fn ast_printer() {
  loop {
    let mut input = String::new();
    println!("Enter a command (or 'exit' to quit):");
    std::io::stdin().read_line(&mut input).unwrap();
    let input = input.trim().to_string();
    if input.is_empty() {
      continue;
    } else if input == "exit" {
      break;
    }

    let mut scanner = Scanner::new(input.clone(), 1);
    let tokens = scanner.scan_tokens();
    let parser = Parser::new(tokens).parse::<String>();
    let mut printer = AstPrinter {};
    let result = printer.print(parser.as_ref());
    println!("{}", result);
  }
}

pub struct AstPrinter {}

impl AstPrinter {
  pub fn print(&mut self, expr: &dyn Expr<String>) -> String {
    expr.accept(self)
  }

  pub fn parse(&mut self, name: String, exprs: &[&dyn Expr<String>]) -> String {
    let mut res = "( ".to_string() + &name;
    for expr in exprs {
      res += &" ";
      res += &expr.accept(self);
    }
    res += &" )";
    return res.to_string();
  }
}

impl Visitor<String> for AstPrinter {
  fn visit_binary_expr(&mut self, expr: &BinaryExpr<String>) -> String {
    self.parse(
      expr.operator.lexeme.clone(),
      &[expr.left.as_ref(), expr.right.as_ref()],
    )
  }

  fn visit_grouping_expr(&mut self, expr: &GroupingExpr<String>) -> String {
    self.parse("group".to_string(), &[expr.expression.as_ref()])
  }

  fn visit_literal_expr(&mut self, expr: &LiteralExpr) -> String {
    match expr.value {
      LiteralObject::Text(ref s) => s.clone(),
      LiteralObject::Number(ref n) => n.to_string(),
      LiteralObject::Nil => "nil".to_string(),
      LiteralObject::Bool(ref b) => b.to_string(),
    }
  }

  fn visit_unary_expr(&mut self, expr: &UnaryExpr<String>) -> String {
    self.parse(expr.operator.lexeme.clone(), &[expr.right.as_ref()])
  }
}
