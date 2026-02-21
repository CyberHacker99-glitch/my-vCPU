use crate::core::{Instruction, Value, BOXES_PER_LINE};

pub fn parse(source: &str) -> Result<Vec<Instruction>, String> {
    let mut instructions: Vec<Instruction> = Vec::new();
    let mut loop_stack: Vec<usize> = Vec::new();

    let tokens: Vec<&str> = source.split_whitespace().collect();
    let mut i = 0;

    while i < tokens.len() {
        let token = tokens[i];

        match token {

            // Basic Instructions
            "L" => instructions.push(Instruction::MoveLeft),
            "R" => instructions.push(Instruction::MoveRight),
            "A" => instructions.push(Instruction::Add),
            "S" => instructions.push(Instruction::Sub),

            // Input
            t if t.starts_with(":(") && t.ends_with(")") => {
                let inside = &t[2..t.len()-1];
                let index: usize = inside.parse()
                    .map_err(|_| "Invalid input index".to_string())?;
                instructions.push(Instruction::Input(index));
            }

            // Output
            t if t.starts_with(";(") && t.ends_with(")") => {
                let inside = &t[2..t.len()-1];
                let index: usize = inside.parse()
                    .map_err(|_| "Invalid output index".to_string())?;
                instructions.push(Instruction::Output(index));
            }

            // Store hidden value
            t if t.starts_with(".(") && t.ends_with(")") => {
                let inside = &t[2..t.len()-1];

                let value = if let Ok(v) = inside.parse::<i64>() {
                    Value::Int(v)
                } else if let Ok(v) = inside.parse::<f64>() {
                    Value::Float(v)
                } else {
                    if inside.chars().any(|c| c.is_control()) {
                        return Err("Invalid hidden value".to_string());
                    }
                    Value::Text(inside.to_string())
                };

                instructions.push(Instruction::Store(value));
            }

            // Cross-line access ,[x,n]
            t if t.starts_with(",[") && t.ends_with("]") => {
                let inside = &t[2..t.len()-1];
                let parts: Vec<&str> = inside.split(',').collect();
                if parts.len() != 2 {
                    return Err("Invalid cross access syntax".to_string());
                }

                let line: usize = parts[0].parse()
                    .map_err(|_| "Invalid line number".to_string())?;
                let box_index: usize = parts[1].parse()
                    .map_err(|_| "Invalid box number".to_string())?;

                if box_index >= BOXES_PER_LINE {
                    return Err("Box index out of range".to_string());
                }

                instructions.push(Instruction::CrossAccess {
                    line,
                    box: box_index,
                });
            }

            // Loop Start {[N]B}
            t if t.starts_with("{[") && t.ends_with("]B}") => {
                loop_stack.push(instructions.len());
                instructions.push(Instruction::LoopStart(0)); // temp
            }

            // Loop End {E}
            "{E}" => {
                let start_index = loop_stack.pop()
                    .ok_or("Loop end without start")?;
                let end_index = instructions.len();

                instructions.push(Instruction::LoopEnd(start_index));

                // Fix start index
                if let Instruction::LoopStart(ref mut target) = instructions[start_index] {
                    *target = end_index;
                }
            }

            // Jump IC == n_J(x)
            t if t.starts_with("IC") => {
                // Format: IC == n_J(x)
                if i + 2 >= tokens.len() {
                    return Err("Invalid jump syntax".to_string());
                }

                let eq = tokens[i + 1];
                let jump_token = tokens[i + 2];

                if eq != "==" {
                    return Err("Invalid jump syntax".to_string());
                }

                if jump_token.starts_with("n_J(") && jump_token.ends_with(")") {
                    let inside = &jump_token[4..jump_token.len()-1];
                    let target: usize = inside.parse()
                        .map_err(|_| "Invalid jump target".to_string())?;
                    instructions.push(Instruction::Jump(target));
                } else {
                    return Err("Invalid jump format".to_string());
                }

                i += 2;
            }

            _ => return Err(format!("Unknown token: {}", token)),
        }

        i += 1;
    }

    if !loop_stack.is_empty() {
        return Err("Unclosed loop detected".to_string());
    }

    Ok(instructions)
}