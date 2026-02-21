mod core;
mod parser;

use core::{Instruction, Value, BOXES_PER_LINE};
use parser::parse;
use std::io::{self, Write};

const LINES: usize = 32; // configurable

struct VM {
    memory: Vec<[Value; BOXES_PER_LINE]>,
    current_line: usize,
    pointer: usize,
    ic: usize,
    program: Vec<Instruction>,
}

impl VM {
    fn new(program: Vec<Instruction>) -> Self {
        Self {
            memory: vec![[Value::Int(0); BOXES_PER_LINE]; LINES],
            current_line: 0,
            pointer: 0,
            ic: 0,
            program,
        }
    }

    fn run(&mut self) {
        while self.ic < self.program.len() {
            match self.program[self.ic].clone() {

                Instruction::MoveLeft => {
                    if self.pointer > 0 {
                        self.pointer -= 1;
                    }
                }

                Instruction::MoveRight => {
                    if self.pointer < BOXES_PER_LINE - 1 {
                        self.pointer += 1;
                    }
                }

                Instruction::Add => {
                    if let Value::Int(ref mut v) =
                        self.memory[self.current_line][self.pointer]
                    {
                        *v += 1;
                    }
                }

                Instruction::Sub => {
                    if let Value::Int(ref mut v) =
                        self.memory[self.current_line][self.pointer]
                    {
                        *v -= 1;
                    }
                }

                Instruction::Input(n) => {
                    print!("Input for box {}: ", n);
                    io::stdout().flush().unwrap();

                    let mut input = String::new();
                    io::stdin().read_line(&mut input).unwrap();
                    let trimmed = input.trim();

                    if let Ok(v) = trimmed.parse::<i64>() {
                        self.memory[self.current_line][n] = Value::Int(v);
                    } else if let Ok(v) = trimmed.parse::<f64>() {
                        self.memory[self.current_line][n] = Value::Float(v);
                    } else {
                        self.memory[self.current_line][n] =
                            Value::Text(trimmed.to_string());
                    }
                }

                Instruction::Output(n) => {
                    println!(
                        "Output: {:?}",
                        self.memory[self.current_line][n]
                    );
                }

                Instruction::Store(val) => {
                    self.memory[self.current_line][self.pointer] = val;
                }

                Instruction::CrossAccess { line, box: b } => {
                    if line < self.memory.len() {
                        let val = self.memory[line][b].clone();
                        self.memory[self.current_line][self.pointer] = val;
                    }
                }

                Instruction::LoopStart(target) => {
                    if let Value::Int(v) =
                        self.memory[self.current_line][self.pointer]
                    {
                        if v == 0 {
                            self.ic = target;
                            continue;
                        }
                    }
                }

                Instruction::LoopEnd(target) => {
                    if let Value::Int(v) =
                        self.memory[self.current_line][self.pointer]
                    {
                        if v != 0 {
                            self.ic = target;
                            continue;
                        }
                    }
                }

                Instruction::Jump(target) => {
                    if target < self.program.len() {
                        self.ic = target;
                        continue;
                    }
                }
            }

            self.ic += 1;
        }
    }
}

fn main() {
    let source = std::fs::read_to_string("program.cp")
        .expect("Could not read program.cp");

    let program = parse(&source).expect("Parsing failed");

    let mut vm = VM::new(program);
    vm.run();
}