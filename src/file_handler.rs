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

    for line in lines.iter().skip(6) {
        let transition: Vec<&str> = line.split(" ").collect();
        let t = automaton::Transition {
            state: transition[0].to_string(),
            symbol: transition[1].to_string(),
            new_state: transition[2].to_string(),
            new_symbol: transition[3].to_string(),
            direction: automaton::Direction::from_string(transition[4]),
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
            symbol: transition_data[1].to_string(),
            new_state: transition_data[2].to_string(),
            new_symbol: tm.blank_symbol.clone(),
            direction: automaton::Direction::Right,
        });
    }
    tm.transitions.push(automaton::Transition {
        state: tm.initial_state.clone(),
        symbol: tm.blank_symbol.clone(),
        new_state: tm.initial_state.clone(),
        new_symbol: tm.blank_symbol.clone(),
        direction: automaton::Direction::Right,
    });
    tm
}
