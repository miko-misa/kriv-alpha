mod scanner;
mod token;

fn run_prompt() {
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
  }
}
