// File: file_handler.rs
// Project: Computing Simulator
// author: dp
// date: 2025-03-06

use crate::automaton;
use crate::automaton::FromString;
use crate::options;

pub fn read_turing_machine_from_file(options: options::Options) -> automaton::TuringMachine {
    let mut tm = automaton::TuringMachine {
        initial_state: "".to_string(),
        accept_state: "".to_string(),
        reject_state: "".to_string(),
        final_states: Vec::new(),
        blank_symbol: "".to_string(),
        states: Vec::new(),
        input_alphabet: Vec::new(),
        tape_alphabet: Vec::new(),
        transitions: Vec::new(),
        end_on_final_state: true,
        tape_count: 1,
    };

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
            directions.push(automaton::Direction::from_string(transition[4 + i * 3]));
        }
        let t = automaton::Transition {
            state: transition[0].to_string(),
            new_state: transition[1].to_string(),
            symbols,
            new_symbols,
            directions,
        };
        tm.transitions.push(t);
    }

    tm
}

pub fn read_finite_state_machine_from_file(options: options::Options) -> automaton::TuringMachine {
    let mut tm = automaton::TuringMachine {
        initial_state: "".to_string(),
        accept_state: "".to_string(),
        reject_state: "".to_string(),
        final_states: Vec::new(),
        states: Vec::new(),
        input_alphabet: Vec::new(),
        transitions: Vec::new(),
        blank_symbol: " ".to_string(),
        tape_alphabet: Vec::new(),
        end_on_final_state: false,
        tape_count: 1,
    };

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
        tm.transitions.push(automaton::Transition {
            state: transition_data[0].to_string(),
            symbols: vec![transition_data[1].to_string()],
            new_state: transition_data[2].to_string(),
            new_symbols: vec![" ".to_string()],
            directions: vec![automaton::Direction::Right],
        });
    }
    tm.transitions.push(automaton::Transition {
        state: tm.initial_state.clone(),
        symbols: vec![tm.blank_symbol.clone()],
        new_state: tm.initial_state.clone(),
        new_symbols: vec![tm.blank_symbol.clone()],
        directions: vec![automaton::Direction::Right],
    });
    tm
}

pub fn read_pushdown_automaton_from_file(options: options::Options) -> automaton::TuringMachine {
    let mut tm = automaton::TuringMachine {
        initial_state: "".to_string(),
        accept_state: "".to_string(),
        reject_state: "".to_string(),
        final_states: Vec::new(),
        states: Vec::new(),
        input_alphabet: Vec::new(),
        transitions: Vec::new(),
        blank_symbol: " ".to_string(),
        tape_alphabet: Vec::new(),
        end_on_final_state: false,
        tape_count: 2,
    };

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

    tm.transitions.push(automaton::Transition {
        state: tm.initial_state.clone(),
        symbols: vec![tm.blank_symbol.clone(), tm.blank_symbol.clone()],
        new_state: tm.initial_state.clone(),
        new_symbols: vec![tm.blank_symbol.clone(), tm.blank_symbol.clone()],
        directions: vec![automaton::Direction::Right, automaton::Direction::Stay],
    });
    for line in lines.iter().skip(6) {
        let transition_data: Vec<&str> = line.split(" ").collect();
        if transition_data.len() < 5 {
            panic!("Error: Transition format not valid: {}", line);
        } else if transition_data.len() == 5 {
            let dir = if transition_data[4] != tm.blank_symbol {
                automaton::Direction::Stay
            } else {
                automaton::Direction::Left
            };
            tm.transitions.push(automaton::Transition {
                state: transition_data[0].to_string(),
                symbols: vec![transition_data[1].to_string(), transition_data[2].to_string()],
                new_state: transition_data[3].to_string(),
                new_symbols: vec![tm.blank_symbol.clone(), transition_data[4].to_string()],
                directions: vec![automaton::Direction::Right, dir],
            });
        } else if transition_data.len() == 6 {
            let aux_state = format!("{}_aux_{}", transition_data[3], tm.transitions.len());
            tm.states.push(aux_state.clone());
            tm.transitions.push(automaton::Transition {
                state: transition_data[0].to_string(),
                symbols: vec![transition_data[1].to_string(), transition_data[2].to_string()],
                new_state: aux_state.clone(),
                new_symbols: vec![tm.blank_symbol.clone(), transition_data[4].to_string()],
                directions: vec![automaton::Direction::Stay, automaton::Direction::Right],
            });
            tm.transitions.push(automaton::Transition {
                state: aux_state.clone(),
                symbols: vec![tm.blank_symbol.clone(), tm.blank_symbol.clone()],
                new_state: transition_data[3].to_string(),
                new_symbols: vec![tm.blank_symbol.clone(), transition_data[5].to_string()],
                directions: vec![automaton::Direction::Right, automaton::Direction::Stay],
            });
        } else {
            panic!("Error: Transition format not valid: {}", line);
        }
    }
    tm
}

pub fn read_tm_from_encoding_file(options: options::Options) -> automaton::TuringMachine {
    let file = std::fs::read_to_string(options.file).expect("Error reading the file");

    let lines: Vec<&str> = file.lines().collect();
    let encoding = lines[0].to_string();
    if lines.len() < 2 {
        return automaton::encoding_to_tm(encoding);
    } else {
        let mut tape_encoding: std::collections::HashMap<String, String> = std::collections::HashMap::new();
        let mut state_encoding: std::collections::HashMap<String, String> = std::collections::HashMap::new();
        let lines_to_read: Vec<&&str> = lines.iter().skip(2).collect();
        let mut states: bool = false;
        for line in lines_to_read {
            if line.is_empty() && states{
                break;
            } else if line.is_empty() && !states{
                states = true;
                continue;
            }
            // println!("{}", line);
            let (key, value) = line.split_once(" ").unwrap();
            if states {
                state_encoding.insert(key.to_string(), value.to_string());
            } else {
                tape_encoding.insert(key.to_string(), value.to_string());
            }
        }
        let tm = automaton::encoding_to_orig(encoding, tape_encoding, state_encoding);
        tm
    }
}