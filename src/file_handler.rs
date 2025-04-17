// File: file_handler.rs
// Project: Computing Simulator
// author: dp

use crate::computer;
use crate::lambda;
use crate::ram_machine;
use crate::regex;
use crate::regex::regex_to_fsa;
use crate::turing_machine;
use crate::turing_machine::FromString;
use crate::utils;

pub fn handle_file_reads(
    file_name: String,
    context: &mut computer::Server,
) -> Result<computer::Computer, String> {
    let file = std::fs::read_to_string(file_name.clone())
        .map_err(|_| "Error reading the file".to_string())?;

    let mut lines: Vec<String> = file
        .lines()
        .filter(|line| !line.starts_with("//"))
        .map(|line| line.to_string())
        .collect();

    let line = lines[0].clone();

    lines = lines.into_iter().skip(1).map(|e| e.to_string()).collect();

    let binding = lines.clone();
    let mapping_raw = binding.iter().filter(|el| el.starts_with(": ")).map(|el| {
        let splitted: Vec<&str> = el.split(" ").collect();
        (
            splitted[1].to_string(),
            splitted
                .iter()
                .skip(2)
                .cloned()
                .collect::<Vec<&str>>()
                .join(" "),
        )
    });

    lines.retain(|e| !e.starts_with(": "));

    let mut c = computer::Computer::new();

    for (name, f) in mapping_raw {
        if f == file_name {
            c.add_mapping(name, f);
        } else if !context.contains(f.clone()) {
            let new_comp = handle_file_reads(f.clone(), context)?;
            context.add_computer(f.clone(), new_comp);
            c.add_mapping(name, f);
        } else if (c.get_mapping(name.clone())?).is_empty() {
            c.add_mapping(name.clone(), f.clone());
        }
    }
    match line.as_str() {
        "tm" => read_turing_machine(lines, &mut c),
        "tm_e" => read_tm_from_encoding(lines, &mut c),
        "pda" => read_pushdown_automaton(lines, &mut c),
        "fsm" => read_finite_state_machine(lines, &mut c),
        "regex" => read_regex(lines, &mut c),
        "ram" => read_ram_program(lines, &mut c),
        "ram_e" => read_ram_program_from_encoding(lines, &mut c),
        "lambda" => read_lambda(lines, &mut c),
        &_ => Err("No valid type to read".to_string()),
    }
}

pub fn read_turing_machine(
    lines: Vec<String>,
    computer: &mut computer::Computer,
) -> Result<computer::Computer, String> {
    let mut tm = turing_machine::TuringMachine::new();

    tm.initial_state = lines[0].to_string();

    tm.accept_state = lines[1].to_string();

    tm.reject_state = lines[2].to_string();

    tm.halt_state = lines[3].to_string();

    tm.blank_symbol = lines[4].to_string();

    let states: Vec<&str> = lines[5].split(" ").collect();
    for state in states {
        tm.states.push(state.to_string());
    }

    let input_alphabet: Vec<&str> = lines[6].split(" ").collect();
    for symbol in input_alphabet {
        tm.input_alphabet.push(symbol.to_string());
    }

    let tape_alphabet: Vec<&str> = lines[7].split(" ").collect();
    for symbol in tape_alphabet {
        tm.tape_alphabet.push(symbol.to_string());
    }
    let tape_count: usize = lines[8]
        .parse()
        .map_err(|_| "Error parsing tape count".to_string())?;
    tm.tape_count = tape_count;

    for line in lines.iter().skip(9) {
        let transition: Vec<&str> = line.split(" ").collect();
        if transition.len() < 2 + tape_count * 3 {
            return Err("Error parsing transition".to_string());
        }
        let mut symbols = Vec::new();
        let mut new_symbols = Vec::new();
        let mut directions = Vec::new();
        for i in 0..tape_count {
            symbols.push(transition[2 + i * 3].to_string());
            new_symbols.push(transition[3 + i * 3].to_string());
            directions.push(turing_machine::Direction::from_string(
                transition[4 + i * 3],
            ));
        }
        tm.add_transition(
            transition[0].to_string(),
            symbols,
            transition[1].to_string(),
            new_symbols,
            directions,
        );
    }
    computer.set_turing(tm);
    Ok(computer.clone())
}

pub fn read_finite_state_machine(
    lines: Vec<String>,
    computer: &mut computer::Computer,
) -> Result<computer::Computer, String> {
    let mut tm = turing_machine::TuringMachine::new();
    tm.blank_symbol = " ".to_string();

    let mut initial_state = lines[0].to_string() + "_init";
    while tm.states.contains(&initial_state) {
        initial_state += "_"
    }
    tm.initial_state = initial_state.clone();
    tm.add_transition(
        initial_state.clone(),
        vec![tm.blank_symbol.clone()],
        lines[0].to_string(),
        vec![tm.blank_symbol.clone()],
        vec![turing_machine::Direction::Right],
    );

    tm.states = lines[2]
        .split(" ")
        .map(|e| e.to_string())
        .collect::<Vec<String>>();

    let final_states: Vec<&str> = lines[1].split(" ").collect();
    let mut final_state = "final".to_string();
    while tm.states.contains(&final_state) {
        final_state += "_"
    }
    tm.halt_state = final_state.clone();
    tm.states.push(final_state.clone());
    tm.states.push(initial_state);
    for fs in final_states {
        tm.add_transition(
            fs.to_string(),
            vec![tm.blank_symbol.clone()],
            final_state.clone(),
            vec![tm.blank_symbol.clone()],
            vec![turing_machine::Direction::Stay],
        );
    }

    let input_alphabet: Vec<&str> = lines[3].split(" ").collect();
    for symbol in input_alphabet {
        tm.input_alphabet.push(symbol.to_string());
    }
    tm.tape_alphabet = tm.input_alphabet.clone();
    tm.tape_alphabet.push(tm.blank_symbol.clone());

    for line in lines.iter().skip(4) {
        let transition_data: Vec<&str> = line.split(" ").collect();
        if transition_data.len() == 2 {
            // epsilon moves
            for symbol in tm.input_alphabet.clone().iter() {
                tm.add_transition(
                    transition_data[0].to_string(),
                    vec![symbol.to_string()],
                    transition_data[1].to_string(),
                    vec![symbol.to_string()],
                    vec![turing_machine::Direction::Stay],
                );
            }
        } else if transition_data.len() == 3 {
            tm.add_transition(
                transition_data[0].to_string(),
                vec![transition_data[1].to_string()],
                transition_data[2].to_string(),
                vec![" ".to_string()],
                vec![turing_machine::Direction::Right],
            );
        } else {
            return Err(format!("Error: Transition format not valid: {}", line));
        }
    }
    computer.set_turing(tm);
    Ok(computer.clone())
}

pub fn read_pushdown_automaton(
    lines: Vec<String>,
    computer: &mut computer::Computer,
) -> Result<computer::Computer, String> {
    let mut tm = turing_machine::TuringMachine::new();
    tm.tape_count = 2;
    tm.blank_symbol = lines[5].to_string();

    let mut initial_state = lines[0].to_string() + "_init";
    while tm.states.contains(&initial_state) {
        initial_state += "_"
    }
    tm.initial_state = initial_state.clone();
    tm.add_transition(
        initial_state.clone(),
        vec![tm.blank_symbol.clone(), tm.blank_symbol.clone()],
        lines[0].to_string(),
        vec![tm.blank_symbol.clone(), tm.blank_symbol.clone()],
        vec![
            turing_machine::Direction::Right,
            turing_machine::Direction::Stay,
        ],
    );

    tm.states = lines[2].split(" ").map(|e| e.to_string()).collect();
    tm.states.push(initial_state);

    let final_states: Vec<&str> = lines[1].split(" ").collect();
    let mut final_state = "final".to_string();
    while tm.states.contains(&final_state) {
        final_state += "_"
    }
    tm.halt_state = final_state.clone();
    tm.states.push(final_state.clone());

    for fs in final_states {
        tm.add_transition(
            fs.to_string(),
            vec![tm.blank_symbol.clone(), tm.blank_symbol.clone()],
            final_state.clone(),
            vec![tm.blank_symbol.clone(), tm.blank_symbol.clone()],
            vec![
                turing_machine::Direction::Stay,
                turing_machine::Direction::Stay,
            ],
        );
    }

    tm.states.push(final_state);

    let input_alphabet: Vec<&str> = lines[3].split(" ").collect();
    for symbol in input_alphabet {
        tm.input_alphabet.push(symbol.to_string());
    }

    let stack_alphabet: Vec<&str> = lines[4].split(" ").collect();
    tm.tape_alphabet = tm.input_alphabet.clone();
    for symbol in stack_alphabet {
        if !tm.tape_alphabet.contains(&symbol.to_string()) {
            tm.tape_alphabet.push(symbol.to_string());
        }
    }

    for line in lines.iter().skip(6) {
        let transition_data: Vec<&str> = line.split(" ").collect();
        if transition_data.len() < 5 {
            return Err(format!("Error: Transition format not valid: {}", line));
        } else if transition_data.len() == 5 {
            let dir = if transition_data[4] != tm.blank_symbol {
                turing_machine::Direction::Stay
            } else {
                turing_machine::Direction::Left
            };
            if transition_data[1] == tm.blank_symbol {
                // epsilon move
                for symb in tm.input_alphabet.clone().iter() {
                    tm.add_transition(
                        transition_data[0].to_string(),
                        vec![symb.to_string(), transition_data[2].to_string()],
                        transition_data[3].to_string(),
                        vec![symb.clone(), transition_data[4].to_string()],
                        vec![turing_machine::Direction::Stay, dir.clone()],
                    );
                }
                tm.add_transition(
                    transition_data[0].to_string(),
                    vec![tm.blank_symbol.clone(), transition_data[2].to_string()],
                    transition_data[3].to_string(),
                    vec![tm.blank_symbol.clone(), transition_data[4].to_string()],
                    vec![turing_machine::Direction::Stay, dir.clone()],
                );
            } else {
                tm.add_transition(
                    transition_data[0].to_string(),
                    vec![
                        transition_data[1].to_string(),
                        transition_data[2].to_string(),
                    ],
                    transition_data[3].to_string(),
                    vec![tm.blank_symbol.clone(), transition_data[4].to_string()],
                    vec![turing_machine::Direction::Right, dir],
                );
            }
        } else if transition_data.len() == 6 {
            let aux_state = format!("{}_aux_{}", transition_data[3], tm.transitions.len());
            tm.states.push(aux_state.clone());
            if transition_data[1] == tm.blank_symbol {
                // epsilon move
                for symb in tm.input_alphabet.clone().iter() {
                    tm.add_transition(
                        transition_data[0].to_string(),
                        vec![symb.clone(), transition_data[2].to_string()],
                        aux_state.clone(),
                        vec![symb.clone(), transition_data[4].to_string()],
                        vec![
                            turing_machine::Direction::Stay,
                            turing_machine::Direction::Right,
                        ],
                    );
                    tm.add_transition(
                        aux_state.clone(),
                        vec![symb.clone(), tm.blank_symbol.clone()],
                        transition_data[3].to_string(),
                        vec![symb.clone(), transition_data[5].to_string()],
                        vec![
                            turing_machine::Direction::Stay,
                            turing_machine::Direction::Stay,
                        ],
                    );
                }
                tm.add_transition(
                    transition_data[0].to_string(),
                    vec![tm.blank_symbol.clone(), transition_data[2].to_string()],
                    aux_state.clone(),
                    vec![tm.blank_symbol.clone(), transition_data[4].to_string()],
                    vec![
                        turing_machine::Direction::Stay,
                        turing_machine::Direction::Right,
                    ],
                );
                tm.add_transition(
                    aux_state.clone(),
                    vec![tm.blank_symbol.clone(), tm.blank_symbol.clone()],
                    transition_data[3].to_string(),
                    vec![tm.blank_symbol.clone(), transition_data[5].to_string()],
                    vec![
                        turing_machine::Direction::Stay,
                        turing_machine::Direction::Stay,
                    ],
                );
            } else {
                tm.add_transition(
                    transition_data[0].to_string(),
                    vec![
                        transition_data[1].to_string(),
                        transition_data[2].to_string(),
                    ],
                    aux_state.clone(),
                    vec![tm.blank_symbol.clone(), transition_data[4].to_string()],
                    vec![
                        turing_machine::Direction::Stay,
                        turing_machine::Direction::Right,
                    ],
                );
                tm.add_transition(
                    aux_state.clone(),
                    vec![tm.blank_symbol.clone(), tm.blank_symbol.clone()],
                    transition_data[3].to_string(),
                    vec![tm.blank_symbol.clone(), transition_data[5].to_string()],
                    vec![
                        turing_machine::Direction::Right,
                        turing_machine::Direction::Stay,
                    ],
                );
            }
        } else {
            return Err(format!("Error: Transition format not valid: {}", line));
        }
    }
    computer.set_turing(tm);
    Ok(computer.clone())
}

pub fn read_tm_from_encoding(
    lines: Vec<String>,
    computer: &mut computer::Computer,
) -> Result<computer::Computer, String> {
    let encoding = lines[0].to_string();
    if lines.len() < 2 {
        computer.set_turing(turing_machine::TuringMachine::encoding_to_tm(encoding)?);
        Ok(computer.clone())
    } else {
        let mut tape_encoding: std::collections::HashMap<String, String> =
            std::collections::HashMap::new();
        let mut state_encoding: std::collections::HashMap<String, String> =
            std::collections::HashMap::new();
        let lines_to_read: Vec<&String> = lines.iter().skip(2).collect();
        let mut states: bool = false;
        for line in lines_to_read {
            if line.is_empty() && states {
                break;
            } else if line.is_empty() && !states {
                states = true;
                continue;
            }
            let (key, value) = line
                .split_once(" ")
                .ok_or_else(|| "cannot split".to_string())?;
            if states {
                state_encoding.insert(key.to_string(), value.to_string());
            } else {
                tape_encoding.insert(key.to_string(), value.to_string());
            }
        }
        computer.set_turing(turing_machine::TuringMachine::encoding_to_orig(
            encoding,
            tape_encoding,
            state_encoding,
        )?);
        Ok(computer.clone())
    }
}

pub fn read_ram_program(
    lines: Vec<String>,
    computer: &mut computer::Computer,
) -> Result<computer::Computer, String> {
    let mut instr = Vec::new();
    let mut labels_map: std::collections::HashMap<String, String> =
        std::collections::HashMap::new();
    for (index, line) in lines.iter().enumerate() {
        let instruction: Vec<&str> = line.split(" ").collect();
        if instruction.len() == 1 {
            if !ram_machine::RamMachine::is_instruction(instruction[0]) {
                instr.push(ram_machine::Instruction {
                    opcode: "".to_string(),
                    operand: "0".to_string(),
                    label: instruction[0].to_string(),
                });
                labels_map.insert(instruction[0].to_string(), utils::int2bin(index as i32, 0));
            } else {
                instr.push(ram_machine::Instruction {
                    opcode: ram_machine::RamMachine::ram_instruction_lookup(
                        instruction[0].to_string(),
                    ),
                    operand: "0".to_string(),
                    label: "".to_string(),
                });
            }
        } else if instruction.len() == 2 {
            if ram_machine::RamMachine::is_instruction(instruction[0]) {
                if utils::is_numeric(instruction[1].to_string()) {
                    instr.push(ram_machine::Instruction {
                        opcode: ram_machine::RamMachine::ram_instruction_lookup(
                            instruction[0].to_string(),
                        ),
                        operand: utils::int2bin(
                            instruction[1].parse().map_err(|_| {
                                format!("Error parsing operand '{}'", instruction[1])
                            })?,
                            0,
                        ),
                        label: "".to_string(),
                    });
                } else {
                    instr.push(ram_machine::Instruction {
                        opcode: ram_machine::RamMachine::ram_instruction_lookup(
                            instruction[0].to_string(),
                        ),
                        operand: "".to_string(),
                        label: instruction[1].to_string(),
                    });
                }
            } else {
                if !ram_machine::RamMachine::is_instruction(instruction[1]) {
                    return Err(format!("invalid instruction: {}", instruction[1]));
                }
                instr.push(ram_machine::Instruction {
                    opcode: ram_machine::RamMachine::ram_instruction_lookup(
                        instruction[1].to_string(),
                    ),
                    operand: "".to_string(),
                    label: "".to_string(),
                });
                labels_map.insert(instruction[0].to_string(), utils::int2bin(index as i32, 0));
            }
        } else if instruction.len() == 3 {
            if !ram_machine::RamMachine::is_instruction(instruction[1]) {
                return Err(format!("invalid instruction: {}", instruction[1]));
            }
            if utils::is_numeric(instruction[2].to_string()) {
                instr.push(ram_machine::Instruction {
                    opcode: ram_machine::RamMachine::ram_instruction_lookup(
                        instruction[1].to_string(),
                    ),
                    operand: utils::int2bin(
                        instruction[2]
                            .parse()
                            .map_err(|_| format!("Error parsing operand '{}'", instruction[2]))?,
                        0,
                    ),
                    label: "".to_string(),
                });
            } else {
                instr.push(ram_machine::Instruction {
                    opcode: ram_machine::RamMachine::ram_instruction_lookup(
                        instruction[1].to_string(),
                    ),
                    operand: "".to_string(),
                    label: instruction[2].to_string(),
                });
            }
            labels_map.insert(instruction[0].to_string(), utils::int2bin(index as i32, 0));
        } else {
            return Err("Error parsing instruction".to_string());
        }
    }
    computer.set_ram(ram_machine::RamMachine {
        instructions: instr,
        labels_map,
    });
    Ok(computer.clone())
}

pub fn read_ram_program_from_encoding(
    lines: Vec<String>,
    computer: &mut computer::Computer,
) -> Result<computer::Computer, String> {
    let line = lines[0]
        .strip_prefix("#")
        .ok_or_else(|| "cannot strip prefix #".to_string())?
        .strip_suffix("#")
        .ok_or_else(|| "cannot strip suffix #".to_string())?;
    let mut instr = Vec::new();
    for elem in line.split("#") {
        let splitted = elem.split(",").collect::<Vec<&str>>();
        if !splitted.is_empty() {
            instr.insert(
                (utils::bin2int(splitted[0].to_string())?) as usize,
                ram_machine::Instruction {
                    opcode: splitted[1][0..4].to_string(),
                    operand: splitted[1][4..].to_string(),
                    label: "".to_string(),
                },
            );
        }
    }

    computer.set_ram(ram_machine::RamMachine {
        instructions: instr,
        labels_map: std::collections::HashMap::new(),
    });
    Ok(computer.clone())
}

pub fn read_regex(
    lines: Vec<String>,
    computer: &mut computer::Computer,
) -> Result<computer::Computer, String> {
    computer.set_turing(regex_to_fsa(&regex::build_regex_tree(&lines[0])?)?);
    Ok(computer.clone())
}

fn read_lambda(
    lines: Vec<String>,
    computer: &mut computer::Computer,
) -> Result<computer::Computer, String> {
    let mut readed: Vec<lambda::Lambda> = Vec::new();
    for line in lines {
        if line.trim() != "" {
            let splitted: Vec<&str> = line.split(": ").collect();
            let name = splitted[0].to_string();
            let lambda = splitted[1..].join(": ");
            readed.push(lambda::Lambda {
                expr: lambda::parse_lambda(lambda.as_str())?,
                references: Vec::new(),
                name,
                force_currying: false,
            });
        }
    }
    readed = readed
        .clone()
        .iter()
        .map(|l| lambda::Lambda {
            expr: l.expr.clone(),
            references: readed.clone(),
            name: l.name.clone(),
            force_currying: false,
        })
        .collect();
    computer.set_lambda(readed[0].clone());
    Ok(computer.clone())
}
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_handle_file_reads_invalid_type() {
        let mut context = computer::Server::new();
        let result = handle_file_reads("test.txt".to_string(), &mut context);
        assert!(result.is_err());
    }

    #[test]
    fn test_read_turing_machine_invalid_transition() {
        let mut computer = computer::Computer::new();
        let lines = vec![
            "q0".to_string(),
            "qa".to_string(), 
            "qr".to_string(),
            "qh".to_string(),
            "_".to_string(),
            "q0 qa qr qh".to_string(),
            "0 1".to_string(),
            "0 1 _".to_string(),
            "1".to_string(),
            "invalid transition".to_string()
        ];
        let result = read_turing_machine(lines, &mut computer);
        assert!(result.is_err());
    }

    #[test]
    fn test_read_finite_state_machine_invalid_transition() {
        let mut computer = computer::Computer::new();
        let lines = vec![
            "q0".to_string(),
            "qa".to_string(),
            "q0 qa".to_string(), 
            "0 1".to_string(),
            "q0 0 1 2 qa".to_string() // Invalid transition format
        ];
        let result = read_finite_state_machine(lines, &mut computer);
        assert!(result.is_err());
    }

    #[test]
    fn test_read_pushdown_automaton_invalid_transition() {
        let mut computer = computer::Computer::new();
        let lines = vec![
            "q0".to_string(),
            "qa".to_string(),
            "q0 qa".to_string(),
            "0 1".to_string(), 
            "Z $".to_string(),
            "_".to_string(),
            "invalid pda transition".to_string()
        ];
        let result = read_pushdown_automaton(lines, &mut computer);
        assert!(result.is_err());
    }

    #[test]
    fn test_read_tm_from_encoding_invalid() {
        let mut computer = computer::Computer::new();
        let lines = vec![
            "invalid_encoding".to_string()
        ];
        let result = read_tm_from_encoding(lines, &mut computer);
        assert!(result.is_err());
    }

    #[test]
    fn test_read_ram_program_invalid_instruction() {
        let mut computer = computer::Computer::new();
        let lines = vec![
            "INVALID 123".to_string()
        ];
        let result = read_ram_program(lines, &mut computer);
        assert!(result.is_err());
    }

    #[test]
    fn test_read_ram_program_from_encoding_invalid() {
        let mut computer = computer::Computer::new();
        let lines = vec![
            "invalid_encoding".to_string()
        ];
        let result = read_ram_program_from_encoding(lines, &mut computer);
        assert!(result.is_err());
    }

    #[test]
    fn test_read_regex_invalid() {
        let mut computer = computer::Computer::new();
        let lines = vec![
            "(((".to_string() // Invalid regex
        ];
        let result = read_regex(lines, &mut computer);
        assert!(result.is_err());
    }

    #[test]
    fn test_read_lambda_invalid() {
        let mut computer = computer::Computer::new();
        let lines = vec![
            "invalid lambda expression".to_string()
        ];
        let result = read_lambda(lines, &mut computer);
        assert!(result.is_err());
    }
    #[test]
    fn test_read_turing_machine_valid() {
        let mut computer = computer::Computer::new();
        let lines = vec![
            "q0".to_string(),
            "qa".to_string(),
            "qr".to_string(), 
            "qh".to_string(),
            "_".to_string(),
            "q0 qa qr qh".to_string(),
            "0 1".to_string(),
            "0 1 _".to_string(),
            "1".to_string(),
            "q0 0 qa 1 R".to_string()
        ];
        let result = read_turing_machine(lines, &mut computer);
        assert!(result.is_ok());
    }

    #[test]
    fn test_read_finite_state_machine_valid() {
        let mut computer = computer::Computer::new();
        let lines = vec![
            "q0".to_string(),
            "qa".to_string(), 
            "q0 qa".to_string(),
            "0 1".to_string(),
            "q0 0 qa".to_string()
        ];
        let result = read_finite_state_machine(lines, &mut computer);
        assert!(result.is_ok());
    }

    #[test]
    fn test_handle_file_reads_empty_file() {
        let mut context = computer::Server::new();
        let file = "".to_string();
        let result = handle_file_reads(file, &mut context);
        assert!(result.is_err());
    }

    #[test]
    fn test_read_pushdown_automaton_valid() {
        let mut computer = computer::Computer::new();
        let lines = vec![
            "q0".to_string(),
            "qa".to_string(),
            "q0 qa".to_string(),
            "0 1".to_string(),
            "Z $".to_string(),
            "_".to_string(),
            "q0 0 Z qa Z Z".to_string()
        ];
        let result = read_pushdown_automaton(lines, &mut computer);
        assert!(result.is_ok());
    }

    #[test]
    fn test_read_ram_program_valid() {
        let mut computer = computer::Computer::new();
        let lines = vec![
            "START L 1".to_string(),
            "ST 2".to_string(),
            "H".to_string()
        ];
        let result = read_ram_program(lines, &mut computer);
        assert!(result.is_ok());
    }

    #[test]
    fn test_read_regex_valid() {
        let mut computer = computer::Computer::new();
        let lines = vec![
            "(a|b)*".to_string()
        ];
        let result = read_regex(lines, &mut computer);
        assert!(result.is_ok());
    }
    #[test]
    fn test_read_tm_from_encoding_valid() {
        let mut computer = computer::Computer::new();
        let lines = vec![
            "(i00;b10;q01;a01;R)(q01;a01;h10;a00;R)".to_string(),
            "".to_string(),
            "a00 1".to_string(), 
            "a01 0".to_string(),
            "b10 _".to_string(),
            "".to_string(),
            "i00 s0".to_string(),
            "q01 s0".to_string(),
            "h10 s1".to_string()
        ];
        let result = read_tm_from_encoding(lines, &mut computer);
        assert!(result.is_ok());
    }

    #[test]
    fn test_read_lambda_valid() {
        let mut computer = computer::Computer::new();
        let lines = vec![
            "id: (\\x.(x))".to_string(),
            "const: (\\x.((\\y.(x))))".to_string()
        ];
        let result = read_lambda(lines, &mut computer);
        assert!(result.is_ok());
    }

    #[test] 
    fn test_read_ram_program_from_encoding_valid() {
        let mut computer = computer::Computer::new();
        let lines = vec![
            "#0,00001#1,00010#10,00100#".to_string()
        ];
        let result = read_ram_program_from_encoding(lines, &mut computer);
        assert!(result.is_ok());
    }

    #[test]
    fn test_handle_file_reads_valid_tm() {
        let mut context = computer::Server::new();
        let mut computer = computer::Computer::new();
        computer.set_turing(turing_machine::TuringMachine::new());
        context.add_computer("test.tm".to_string(), computer);
        
        let lines = [
            "tm",
            "q0",
            "qa",
            "qr",
            "qh",
            "_",
            "q0 qa qr qh",
            "0 1",
            "0 1 _",
            "1",
            "q0 0 qa 1 R",
            ": test test.tm",
        ].join("\n");
        
        let temp_file = std::fs::write("temp.txt", lines);
        assert!(temp_file.is_ok());
        
        let result = handle_file_reads("temp.txt".to_string(), &mut context);
        std::fs::remove_file("temp.txt").unwrap();
        assert!(result.is_ok());
    }
}