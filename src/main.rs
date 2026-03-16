mod core;
mod parser;

use core::VM;
use parser::parse;
use std::env;
use std::fs;

const LINES: usize = 32;

fn main() {

    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        println!("Usage: cpvm <program.cp>");
        return;
    }

    let source = fs::read_to_string(&args[1])
        .expect("Failed to read program file");

    let instructions = match parse(&source) {
        Ok(i) => i,
        Err(e) => {
            println!("Parse error: {}", e);
            return;
        }
    };

    let mut vm = VM::new(LINES, instructions);

    vm.run();
}