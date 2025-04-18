pub mod ast_printer;
pub mod expr;
pub mod interpreter;
pub mod kriv;
pub mod parser;
pub mod scanner;
pub mod tests;
pub mod token;

fn main() {
  print!("Hello, Kriv!\n");
  let _ = kriv::start_prompt();
}
