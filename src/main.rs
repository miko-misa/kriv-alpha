pub mod kriv;
pub mod scanner;
pub mod token;

fn main() {
  print!("Hello, Kriv!\n");
  kriv::run_prompt();
}
