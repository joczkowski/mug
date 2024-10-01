pub mod token;
pub mod tokenizer;

use std::fmt::{self};
use std::{env, fs};
use crate::tokenizer::{Tokenizer, Tokenize};

#[derive(Clone, Debug)]
pub struct InputError {}

impl fmt::Display for InputError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Input error")
    }
}

fn main() -> Result<(), String> {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        return Err(String::from("Missing arg, pass file name as arg"));
    }
    let filename = &args[1];

    let source = fs::read_to_string(filename).expect("Error reading input file");

    let mut tokenizer = Tokenizer::new(source);
    let tokens = tokenizer.tokenize().expect("Tokenize error");

    for token in tokens.into_iter() {
        println!("Token: {:?}", token);
    }

    Ok(())
}
