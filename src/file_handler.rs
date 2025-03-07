// File: file_handler.rs
// Project: Computing Simulator
// author: dp
// date: 2025-03-06

use crate::automaton;
use crate::automaton::FromString;
use crate::options;

pub fn read_turing_machine_from_file(file_path: String) -> automaton::TuringMachine {
    let mut tm = automaton::TuringMachine {
        initial_state: "".to_string(),
        final_states: Vec::new(),
        blank_symbol: "".to_string(),
        states: Vec::new(),
        input_alphabet: Vec::new(),
        tape_alphabet: Vec::new(),
        transitions: Vec::new(),
        end_on_final_state: true,
        tape_count: 1,
    };

    let file = std::fs::read_to_string(file_path).expect("Error reading the file");

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

    tm.blank_symbol = lines[2].to_string();

    let states: Vec<&str> = lines[3].split(" ").collect();
    for state in states {
        tm.states.push(state.to_string());
    }

    let input_alphabet: Vec<&str> = lines[4].split(" ").collect();
    for symbol in input_alphabet {
        tm.input_alphabet.push(symbol.to_string());
    }

    let tape_alphabet: Vec<&str> = lines[5].split(" ").collect();
    for symbol in tape_alphabet {
        tm.tape_alphabet.push(symbol.to_string());
    }
    let tape_count: usize = lines[6].parse().expect("Error parsing tape count");
    tm.tape_count = tape_count;

    for line in lines.iter().skip(7) {
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
