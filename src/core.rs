use std::collections::HashMap;

pub const BOXES_PER_LINE: usize = 16;

#[derive(Clone, Debug)]
pub enum Value {
    Int(i64),
    Float(f64),
    Text(String),
}

#[derive(Debug)]
pub struct Memory {
    pub grid: Vec<Vec<Value>>,
}

impl Memory {
    pub fn new(lines: usize) -> Self {
        Self {
            grid: vec![vec![Value::Int(0); BOXES_PER_LINE]; lines],
        }
    }
}

#[derive(Clone, Debug)]
pub struct Pointer {
    pub line: usize,
    pub box_index: usize,
}

#[derive(Clone, Debug)]
pub enum Instruction {
    MoveLeft,
    MoveRight,
    Add,
    Sub,
    Input(usize),
    Output(usize),
    Store(Value),
    CrossAccess { line: usize, box: usize },
    LoopStart(usize),   // contains matching LoopEnd index
    LoopEnd(usize),     // contains matching LoopStart index
    Jump(usize),
}

#[derive(Debug)]
pub struct Core {
    pub ic: usize,
    pub pointer: Pointer,
}

pub struct VM {
    pub memory: Memory,
    pub instructions: Vec<Instruction>,
    pub cores: [Core; 2],
    pub active_core: usize,
}

impl VM {
    pub fn new(lines: usize, instructions: Vec<Instruction>) -> Self {
        Self {
            memory: Memory::new(lines),
            instructions,
            cores: [
                Core { ic: 0, pointer: Pointer { line: 0, box_index: 0 } },
                Core { ic: 0, pointer: Pointer { line: 0, box_index: 0 } },
            ],
            active_core: 0,
        }
    }

    fn current_value(&self) -> &Value {
        let core = &self.cores[self.active_core];
        &self.memory.grid[core.pointer.line][core.pointer.box_index]
    }

    fn current_value_mut(&mut self) -> &mut Value {
        let core = &self.cores[self.active_core];
        &mut self.memory.grid[core.pointer.line][core.pointer.box_index]
    }

    pub fn step(&mut self) {
        let core = &mut self.cores[self.active_core];

        if core.ic >= self.instructions.len() {
            return;
        }

        match self.instructions[core.ic].clone() {
            Instruction::MoveLeft => {
                if core.pointer.box_index > 0 {
                    core.pointer.box_index -= 1;
                }
            }

            Instruction::MoveRight => {
                if core.pointer.box_index < BOXES_PER_LINE - 1 {
                    core.pointer.box_index += 1;
                }
            }

            Instruction::Add => {
                if let Value::Int(v) = self.current_value_mut() {
                    *v += 1;
                }
            }

            Instruction::Sub => {
                if let Value::Int(v) = self.current_value_mut() {
                    *v -= 1;
                }
            }

            Instruction::Store(val) => {
                *self.current_value_mut() = val;
            }

            Instruction::CrossAccess { line, box } => {
                if line < self.memory.grid.len() && box < BOXES_PER_LINE {
                    let value = self.memory.grid[line][box].clone();
                    *self.current_value_mut() = value;
                }
            }

            Instruction::LoopStart(end_index) => {
                if let Value::Int(v) = self.current_value() {
                    if *v == 0 {
                        core.ic = end_index;
                    }
                }
            }

            Instruction::LoopEnd(start_index) => {
                if let Value::Int(v) = self.current_value() {
                    if *v != 0 {
                        core.ic = start_index;
                    }
                }
            }

            Instruction::Jump(target) => {
                core.ic = target;
                return;
            }

            Instruction::Input(box_index) => {
    use std::io::{self, Write};

    print!("Input: ");
    io::stdout().flush().unwrap();

    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();

    if let Ok(num) = input.trim().parse::<i64>() {
        if box_index < BOXES_PER_LINE {
            self.memory.grid[core.pointer.line][box_index] = Value::Int(num);
        }
    }
}

Instruction::Output(box_index) => {
    if box_index < BOXES_PER_LINE {
        println!("{:?}", self.memory.grid[core.pointer.line][box_index]);
    }
}
        }

        core.ic += 1;
        self.active_core = (self.active_core + 1) % 2;
    }

    pub fn run(&mut self) {
        while self.cores[0].ic < self.instructions.len()
            || self.cores[1].ic < self.instructions.len()
        {
            self.step();
        }
    }
}