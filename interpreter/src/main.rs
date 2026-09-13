use crate::interpreting::NackInterpreter;
use crate::lexing::tokenize_source_string;
use crate::parsing::parser::NackParser;
use std::process::ExitCode;
use std::{env, fs::read_to_string};

mod interpreting;
mod lexing;
mod parsing;
#[cfg(test)]
mod test;

fn main() -> ExitCode {
    let args = env::args().collect::<Vec<String>>();

    if args.len() != 2 {
        println!("Usage: nack <input file>");
        return ExitCode::from(1);
    }

    let source_string = match read_to_string(&args[1])
        .map_err(|err| format!("Failed to open {}: {err}", args[1]))
    {
        Ok(source_string) => source_string,
        Err(msg) => {
            println!("Failed to read {}: {msg}", args[1]);
            return ExitCode::from(2);
        }
    };
    let tokens = match tokenize_source_string(&source_string) {
        Ok(tokens) => tokens,
        Err(e) => {
            println!(
                "Syntax error at ({},{}): {}",
                e.position.line, e.position.column, e.message
            );
            return ExitCode::from(3);
        }
    };
    let ast_root = match NackParser::new(tokens).parse() {
        Ok(ast_root) => ast_root,
        Err(e) => {
            println!(
                "Syntax error at ({},{}): {}",
                e.position.line, e.position.column, e.message
            );
            return ExitCode::from(3);
        }
    };

    println!("{:?}", ast_root);

    let interpreter = NackInterpreter::new();

    interpreter.interpret_ast(&ast_root);

    ExitCode::SUCCESS
}
