use crate::lexing::tokenize_source_string;
use std::{env, fs::read_to_string};

mod lexing;

fn main() -> Result<(), String> {
    let args = env::args().collect::<Vec<String>>();

    if args.len() != 2 {
        return Err("Usage: nack <input file>".to_owned());
    }

    let source_string =
        read_to_string(&args[1]).map_err(|err| format!("Failed to open {}: {err}", args[1]))?;
    let tokens = tokenize_source_string(&source_string);

    println!("{tokens:?}");

    Ok(())
}
