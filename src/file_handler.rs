// File: file_handler.rs
// Project: Computing Simulator
// author: dp

use crate::regex;
use crate::turing_machine;
use crate::ram_machine;
use crate::turing_machine::FromString;
use crate::options;
use crate::utils;

pub fn read_turing_machine_from_file(options: options::Options) -> turing_machine::TuringMachine {
    let mut tm = turing_machine::TuringMachine::new();

    let file = std::fs::read_to_string(options.file).expect("Error reading the file");

    let lines: Vec<&str> = file.lines().collect();

    let lines: Vec<&str> = lines
        .iter()
        .filter(|line| !line.starts_with("//"))
        .copied()
        .collect();

    tm.initial_state = lines[0].to_string();

    tm.accept_state = lines[1].to_string();

    tm.reject_state = lines[2].to_string();

    let final_states: Vec<&str> = lines[3].split(" ").collect();
    for final_state in final_states {
        tm.final_states.push(final_state.to_string());
    }

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
    let tape_count: usize = lines[8].parse().expect("Error parsing tape count");
    tm.tape_count = tape_count;

    for line in lines.iter().skip(9) {
        let transition: Vec<&str> = line.split(" ").collect();
        if line.len() < 2 + tape_count * 3 {
            panic!("Error parsing transition");
        }
        let mut symbols = Vec::new();
        let mut new_symbols = Vec::new();
        let mut directions = Vec::new();
        for i in 0..tape_count {
            symbols.push(transition[2 + i * 3].to_string());
            new_symbols.push(transition[3 + i * 3].to_string());
            directions.push(turing_machine::Direction::from_string(transition[4 + i * 3]));
        }
        tm.add_transition(transition[0].to_string(), symbols, transition[1].to_string(), new_symbols, directions);
    }

    tm
}

pub fn read_finite_state_machine_from_file(options: options::Options) -> turing_machine::TuringMachine {
    let mut tm = turing_machine::TuringMachine::new();
    tm.blank_symbol = " ".to_string();

    let file = std::fs::read_to_string(options.file).expect("Error reading the file");

    let lines: Vec<&str> = file.lines().collect();

    let lines: Vec<&str> = lines
        .iter()
        .filter(|line| !line.is_empty() && !line.starts_with("//"))
        .copied()
        .collect();

    tm.initial_state = lines[0].to_string();

    let final_states: Vec<&str> = lines[1].split(" ").collect();
    for final_state in final_states {
        tm.final_states.push(final_state.to_string());
    }

    let states: Vec<&str> = lines[2].split(" ").collect();
    for state in states {
        tm.states.push(state.to_string());
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
            panic!("Error: Transition format not valid: {}", line);
        }
    }
    tm.add_transition(
        tm.initial_state.clone(),
        vec![tm.blank_symbol.clone()],
        tm.initial_state.clone(),
        vec![tm.blank_symbol.clone()],
        vec![turing_machine::Direction::Right],
    );
    tm
}

pub fn read_pushdown_automaton_from_file(options: options::Options) -> turing_machine::TuringMachine {
    let mut tm = turing_machine::TuringMachine::new();
    tm.tape_count = 2;

    let file = std::fs::read_to_string(options.file).expect("Error reading the file");

    let lines: Vec<&str> = file.lines().collect();
    let lines: Vec<&str> = lines
        .iter()
        .filter(|line| !line.is_empty() && !line.starts_with("//"))
        .copied()
        .collect();
    tm.initial_state = lines[0].to_string();

    let final_states: Vec<&str> = lines[1].split(" ").collect();
    for final_state in final_states {
        tm.final_states.push(final_state.to_string());
    }

    let states: Vec<&str> = lines[2].split(" ").collect();
    for state in states {
        tm.states.push(state.to_string());
    }

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

    tm.blank_symbol = lines[5].to_string();

    tm.add_transition(
        tm.initial_state.clone(),
        vec![tm.blank_symbol.clone(), tm.blank_symbol.clone()],
        tm.initial_state.clone(),
        vec![tm.blank_symbol.clone(), tm.blank_symbol.clone()],
        vec![turing_machine::Direction::Right, turing_machine::Direction::Stay],
    );
    for line in lines.iter().skip(6) {
        let transition_data: Vec<&str> = line.split(" ").collect();
        if transition_data.len() < 5 {
            panic!("Error: Transition format not valid: {}", line);
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
                        vec![
                            symb.to_string(),
                            transition_data[2].to_string(),
                        ],
                        transition_data[3].to_string(),
                        vec![symb.clone(), transition_data[4].to_string()],
                        vec![turing_machine::Direction::Stay, dir.clone()],
                    );
                }
                tm.add_transition(
                    transition_data[0].to_string(),
                    vec![
                        tm.blank_symbol.clone(),
                        transition_data[2].to_string(),
                    ],
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
                        vec![
                            symb.clone(),
                            transition_data[2].to_string(),
                        ],
                        aux_state.clone(),
                        vec![symb.clone(), transition_data[4].to_string()],
                        vec![turing_machine::Direction::Stay, turing_machine::Direction::Right],
                    );
                    tm.add_transition(
                        aux_state.clone(),
                        vec![symb.clone(), tm.blank_symbol.clone()],
                        transition_data[3].to_string(),
                        vec![symb.clone(), transition_data[5].to_string()],
                        vec![turing_machine::Direction::Stay, turing_machine::Direction::Stay],
                    );
                }
                tm.add_transition(
                    transition_data[0].to_string(),
                    vec![
                        tm.blank_symbol.clone(),
                        transition_data[2].to_string(),
                    ],
                    aux_state.clone(),
                    vec![tm.blank_symbol.clone(), transition_data[4].to_string()],
                    vec![turing_machine::Direction::Stay, turing_machine::Direction::Right],
                );
                tm.add_transition(
                    aux_state.clone(),
                    vec![tm.blank_symbol.clone(), tm.blank_symbol.clone()],
                    transition_data[3].to_string(),
                    vec![tm.blank_symbol.clone(), transition_data[5].to_string()],
                    vec![turing_machine::Direction::Stay, turing_machine::Direction::Stay],
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
                    vec![turing_machine::Direction::Stay, turing_machine::Direction::Right],
                );
                tm.add_transition(
                    aux_state.clone(),
                    vec![tm.blank_symbol.clone(), tm.blank_symbol.clone()],
                    transition_data[3].to_string(),
                    vec![tm.blank_symbol.clone(), transition_data[5].to_string()],
                    vec![turing_machine::Direction::Right, turing_machine::Direction::Stay],
                );
            }
        } else {
            panic!("Error: Transition format not valid: {}", line);
        }
    }
    tm
}

pub fn read_tm_from_encoding_file(options: options::Options) -> turing_machine::TuringMachine {
    let file = std::fs::read_to_string(options.file).expect("Error reading the file");

    let lines: Vec<&str> = file.lines().collect();
    let encoding = lines[0].to_string();
    if lines.len() < 2 {
        turing_machine::encoding_to_tm(encoding)
    } else {
        let mut tape_encoding: std::collections::HashMap<String, String> =
            std::collections::HashMap::new();
        let mut state_encoding: std::collections::HashMap<String, String> =
            std::collections::HashMap::new();
        let lines_to_read: Vec<&&str> = lines.iter().skip(2).collect();
        let mut states: bool = false;
        for line in lines_to_read {
            if line.is_empty() && states {
                break;
            } else if line.is_empty() && !states {
                states = true;
                continue;
            }
            let (key, value) = line.split_once(" ").unwrap();
            if states {
                state_encoding.insert(key.to_string(), value.to_string());
            } else {
                tape_encoding.insert(key.to_string(), value.to_string());
            }
        }

        turing_machine::encoding_to_orig(encoding, tape_encoding, state_encoding)
    }
}

pub fn read_ram_program_from_file(options: options::Options) -> ram_machine::RamMachine {
    let file = std::fs::read_to_string(options.file).expect("Error reading the file");

    let lines: Vec<&str> = file.lines().collect();

    let mut instr = Vec::new();
    for line in lines.iter() {
        if line.starts_with("//") {
            continue;
        } else {
            let instruction: Vec<&str> = line.split(" ").collect();
            if instruction.len() == 1 {
                instr.push(ram_machine::Instruction {
                    opcode: ram_machine::RamMachine::ram_instruction_lookup(instruction[0].to_string()),
                    operand: "0000000000000000".to_string(),
                });
            } else if instruction.len() == 2 {
                instr.push(ram_machine::Instruction {
                    opcode: ram_machine::RamMachine::ram_instruction_lookup(instruction[0].to_string()),
                    operand: utils::int2bin(
                        instruction[1].parse().expect("Error parsing operand"),
                        16,
                    ),
                });
            } else {
                panic!("Error parsing instruction");
            }
        }
    }
    ram_machine::RamMachine {
        instructions: instr,
    }
}

pub fn read_ram_program_from_encoding_file(options: options::Options) -> ram_machine::RamMachine {
    let file = std::fs::read_to_string(options.file).expect("Error reading the file");

    let lines: Vec<&str> = file.lines().collect();

    let line = lines[0].strip_prefix("#").unwrap().strip_suffix("#").unwrap();
    let mut instr = Vec::new();
    for elem in line.split("#") {
        let splitted = elem.split(",").collect::<Vec<&str>>();
        if !splitted.is_empty() {
            instr.insert(
                utils::bin2int(splitted[0].to_string()) as usize,
                ram_machine::Instruction {
                    opcode: splitted[1][0..4].to_string(),
                    operand: splitted[1][4..].to_string(),
                }
            );
        }
    }

    let ram = ram_machine::RamMachine {
        instructions: instr,
    };
    ram
}

pub fn read_regex_from_file(options: options::Options) -> Result<regex::Regex, String> {
    let file = std::fs::read_to_string(options.clone().file).expect("Error reading the file");

    let lines: Vec<&str> = file.lines().collect();
    let mut i = 0;
    let mut line = lines[i];
    while line.starts_with("//") {
        i+=1;
        line = lines[i];
    }
    regex::build_regex_tree(line)
}