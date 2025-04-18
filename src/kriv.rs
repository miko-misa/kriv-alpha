use colored::Colorize;
use rustyline::{error::ReadlineError, DefaultEditor};

use crate::{
  ast_printer::AstPrinter,
  interpreter::{self, Interpreter},
  parser::Parser,
  scanner::Scanner,
  token::LiteralObject,
};

pub fn start_prompt() -> Result<(), ReadlineError> {
  println!("Enter a command (or 'exit' to quit):");
  let mut rl = DefaultEditor::new()?;
  loop {
    let readline = rl.readline(&format!("{}  ", "Kriv >".red()).to_string());
    match readline {
      Ok(line) => {
        let _ = rl.add_history_entry(line.as_str());
        let input = line.trim().to_string();
        if input.is_empty() {
          continue;
        } else if input == "exit" {
          break;
        }
        println!("");
        run_prompt(input.clone());
      }
      Err(ReadlineError::Interrupted) => {
        println!("CTRL-C");
        break;
      }
      Err(ReadlineError::Eof) => {
        println!("CTRL-D");
        break;
      }
      Err(err) => {
        println!("Error: {:?}", err);
        break;
      }
    }
    println!("");
  }
  rl.save_history("history.txt")?;
  Ok(())
}

fn run_prompt(input: String) {
  let mut scanner = Scanner::new(input.clone(), 1);
  let tokens = scanner.scan_tokens();

  /*
  let parser = Parser::new(tokens.clone()).parse();
  let mut printer = AstPrinter {};
  let print = printer.print(parser.as_ref());
  println!("{}", print);
  */

  let parser = Parser::new(tokens.clone()).parse();
  let mut interpreter = Interpreter {};
  let result = interpreter.evaluate(parser.as_ref());
  println!("{}", result);
}
