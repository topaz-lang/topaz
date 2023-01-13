use std::path::Path;

mod ast;
mod lexer;
mod parser;

use crate::parser::Parser;
use ast::token::RawToken;
use lexer::Lexer;
use std::{env, fs, process::exit};

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() != 2 {
        eprintln!("usage: sigma <filename>");
        exit(1);
    }

    let filename = args[1].as_str();
    match fs::read_to_string(filename) {
        Ok(content) => {
            let mut parser = Parser::new(filename, content.as_str());
            println!("{:?}", parser.parse());
        }
        Err(_) => {
            eprintln!("unable to read file");
            exit(1);
        }
    }
}
