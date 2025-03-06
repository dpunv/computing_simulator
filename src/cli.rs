// file: cli.rs
// Project: Computing Simulator
// author: dp
// date: 2025-03-06

use crate::automaton;
use crate::automaton::Automaton;
use crate::file_handler;
use crate::options;
use crate::utils;
use std::io::Write;

fn print_help() {
    println!("Usage: turing_machine [OPTIONS] [FILE] [INPUT]");
    println!();
    println!("Options:");
    println!("  --type: turing machine (tm), finite state machine (fsm)");
    println!("  --output-tape: print the final tape of the Turing Machine");
    println!("  --trimmed-tape: print the final tape (trimmed) of the Turing Machine");
    println!(
        "  --steps: print the number of steps taken by the Turing Machine to reach the final state"
    );
    println!("  --state: print the final state of the Turing Machine");
    println!("  --computation: print the full computation of the Turing Machine");
    println!("  --help: print the help message");
    println!("  --version: print the version of the Turing Machine Simulator");
    println!("  --max-steps: set the maximum number of steps for the Turing Machine");
    println!("  --input: provide the input string for the Turing Machine");
    println!("  --file: provide the file containing the description of the Turing Machine");
    println!("  --status: print informations about the Turing Machine");
    println!();
    println!("Default: read from a custom file, don't print the tape, print the trimmed tape, print the number of steps, print the final state, don't print the status, don't print the computation");
    println!();
    println!("Acknowledgements:");
    println!("  This program is made by dp. Licensed under the MIT License.");
    println!();
}

fn print_version() {
    println!("Turing Machine Simulator 0.1.0");
}

pub fn print_tape(tape: Vec<String>, tm: automaton::TuringMachine, trim: Option<bool>) {
    let should_trim = trim.unwrap_or(false);
    let mut tape = tape;
    if should_trim {
        tape = tape
            .iter()
            .skip_while(|symbol| *symbol == &tm.blank_symbol.clone())
            .cloned()
            .collect();
        tape = tape
            .iter()
            .rev()
            .skip_while(|symbol| *symbol == &tm.blank_symbol.clone())
            .cloned()
            .collect();
        tape = tape.iter().rev().cloned().collect();
    }
    for symbol in tape {
        print!("{}", symbol);
    }
}

pub fn print_status<T: automaton::Automaton>(tm: &T) {
    println!("Deterministic: {}", tm.is_deterministic());
    println!("Ok: {}", tm.is_ok());
    println!("Transition total: {}", tm.is_transition_total());
}

fn execute(tm: automaton::TuringMachine, input: String, opt: options::Options) {
    let input_tape = utils::input_string_to_vec(tm.input_alphabet(), input);
    let result = tm.clone().simulate(input_tape, opt.max_steps);
    if opt.output_tape {
        print!("Output Tape:");
        print_tape(result.1.clone(), tm.clone(), Some(false));
        println!();
    }
    if opt.trimmed_tape {
        print!("Trimmed Tape:");
        print_tape(result.1.clone(), tm.clone(), Some(true));
        println!();
    }
    if opt.steps {
        println!("Steps: {}", result.2);
    }
    if opt.state {
        println!("State: {}", result.0);
    }
    if opt.computation {
        println!("Computation:");
        print_computation(result.3.clone(), tm.clone(), true, false, true, true, true);
    }
}

fn interactive_tui(tm: automaton::TuringMachine, opt: options::Options) {
    let mut input = String::new();
    loop {
        print!("> ");
        // flush the output buffer
        std::io::stdout().flush().unwrap();
        input.clear();
        std::io::stdin().read_line(&mut input).unwrap();
        let trimmed_input = input.trim().to_string();
        if trimmed_input == "status" {
            print_status(&tm);
        } else if trimmed_input == "help" {
            print_help();
        } else if trimmed_input == "version" {
            print_version();
        } else if trimmed_input == "exit" {
            break;
        } else {
            execute(tm.clone(), trimmed_input.clone(), opt.clone());
        }
    }
}

pub fn print_computation(
    computation: Vec<automaton::Configuration>,
    tm: automaton::TuringMachine,
    output_tape: bool,
    trimmed_tape: bool,
    steps: bool,
    state: bool,
    head: bool,
) {
    for config in computation {
        if steps {
            print!("Step: {}     ", config.head);
        }
        if state {
            print!("State {}     ", config.state);
        }
        if output_tape {
            print_tape(config.tape.clone(), tm.clone(), Some(trimmed_tape));
        }
        if head {
            println!(" Head: {} ", config.head);
        }
    }
}

pub fn main_cli() {
    let options = options::get_options();
    if options.help {
        print_help();
        return;
    }
    if options.version {
        print_version();
        return;
    }
    if (options.type_ != "tm" && options.type_ != "fsm") || options.file.is_empty() {
        println!("Error: Invalid options. Use --help for more information.");
        return;
    }
    let tm;
    if options.type_ == "tm" {
        tm = file_handler::read_turing_machine_from_file(options.clone().file);
    } else if options.type_ == "fsm" {
        tm = file_handler::read_finite_state_machine_from_file(options.clone());
    } else {
        return;
    }
    if options.status {
        print_status(&tm);
    } else if options.clone().input.is_empty() {
        interactive_tui(tm.clone(), options.clone());
    } else {
        execute(tm.clone(), options.clone().input, options.clone());
    }
}
