use crate::scanner::Scanner;

pub fn run_prompt() {
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
    for token in tokens {
      println!("{}", token.to_string());
    }
  }
}
