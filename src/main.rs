pub mod expr;
pub mod interpreter;
pub mod kriv;
pub mod parser;
pub mod scanner;
pub mod token;

fn main() {
  print!("Hello, Kriv!\n");
  kriv::run_prompt();
}
