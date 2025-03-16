// file: cli.rs
// Project: Computing Simulator
// author: dp
// date: 2025-03-06

use crate::automata;
use crate::automata::Automaton;
use crate::automata::Executable;

use crate::file_handler;
use crate::options;
use crate::utils;
use core::panic;
use std::io::Write;
use std::vec;

fn print_help() {
    println!("Usage: turing_machine [OPTIONS] [FILE] [INPUT]");
    println!();
    println!("Options:");
    println!("  --type: turing machine (tm), finite state machine (fsm)");
    println!("  --from-encoding: read the Turing Machine from an encoding file");
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
    println!(
        "  --convert-to-singletape: convert multitape machines into single tape Turing Machines"
    );
    println!("  --input: provide the input string for the Turing Machine");
    println!("  --file: provide the file containing the description of the Turing Machine");
    println!("  --status: print informations about the Turing Machine");
    println!("  --print-encoding: print the encoding of the Turing Machine");
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

pub fn print_tape(tape: automata::Tape, tm: automata::TuringMachine, trim: Option<bool>) {
    let should_trim = trim.unwrap_or(false);
    let mut tape = tape;
    if should_trim {
        tape.tape = tape
            .tape
            .iter()
            .skip_while(|symbol| *symbol == &tm.blank_symbol.clone())
            .cloned()
            .collect();
        tape.tape = tape
            .tape
            .iter()
            .rev()
            .skip_while(|symbol| *symbol == &tm.blank_symbol.clone())
            .cloned()
            .collect();
        tape.tape = tape.tape.iter().rev().cloned().collect();
    }
    for symbol in tape.tape {
        print!("{}", symbol);
    }
}

pub fn print_status<T: automata::Automaton>(tm: &T) {
    println!("Deterministic: {}", tm.is_deterministic());
    println!("Ok: {}", tm.is_ok());
    println!("Transition total: {}", tm.is_transition_total());
}

fn execute_tm(mut tm: automata::TuringMachine, opt: options::Options) {
    let input_tape = utils::input_string_to_vec(tm.input_alphabet(), opt.input.clone());
    let result = tm.simulate(input_tape, opt.max_steps);
    if opt.verbose == 0 {
        println!("{}", result.0);
        print_tape(tm.last_execution.1[0].clone(), tm.clone(), Some(true));
    } else if opt.verbose == 1 {
        println!("State: {}", result.0);
        println!("Steps: {}", result.1);
        print!("Output Tape: ");
        print_tape(tm.last_execution.1[0].clone(), tm.clone(), Some(true));
        println!();
    } else if opt.verbose == 2 {
        println!("State: {}", result.0);
        println!("Steps: {}", result.1);
        print!("Output Tape: ");
        print_tape(tm.last_execution.1[0].clone(), tm.clone(), Some(true));
        println!();
        println!("Computation:");
        print_computation(
            tm.last_execution.3.clone(),
            tm.clone(),
            true,
            false,
            true,
            true,
            true,
        );
    } else {
        panic!("Invalid verbose level");
    }
}

fn execute_ram(ram: automata::RamMachine, opt: options::Options) {
    let input_tape = vec![opt.input.clone()];
    let result: (String, i32) = ram.clone().simulate(input_tape, opt.max_steps);
    if opt.verbose == 0 {
        println!("{}", result.0);
    } else if opt.verbose >= 1 {
        println!("Execution complete:");
        println!("  Output: {}", result.0);
        println!("  Steps: {}", result.1);
    } else {
        panic!("Invalid verbose level");
    }
}

fn interactive_tui_tm(tm: automata::TuringMachine, opt: options::Options) {
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
            execute_tm(tm.clone(), opt.clone());
        }
    }
}

fn interactive_tui_ram(ram: automata::RamMachine, opt: options::Options) {
    let mut input = String::new();
    loop {
        print!("> ");
        // flush the output buffer
        std::io::stdout().flush().unwrap();
        input.clear();
        std::io::stdin().read_line(&mut input).unwrap();
        let trimmed_input = input.trim().to_string();
        if trimmed_input == "status" {
            // print_status(&ram); // todo
        } else if trimmed_input == "help" {
            print_help();
        } else if trimmed_input == "version" {
            print_version();
        } else if trimmed_input == "exit" {
            break;
        } else {
            execute_ram(ram.clone(), opt.clone());
        }
    }
}

pub fn print_encoding_tm(tm: &automata::TuringMachine) {
    let encoded: (
        String,
        std::collections::HashMap<String, String>,
        std::collections::HashMap<String, String>,
    ) = tm.to_encoding();
    println!("{}", encoded.0);
    println!("Alphabet:");
    for (k, v) in encoded.1.iter() {
        println!("{} {}", v, k);
    }
    println!("States: ");
    for (k, v) in encoded.2.iter() {
        println!("{} {}", v, k);
    }
}

pub fn print_encoding_ram(ram: &automata::RamMachine) {
    let encoded: (
        String,
        std::collections::HashMap<String, String>,
        std::collections::HashMap<String, String>,
    ) = ram.to_encoding();
    println!("{}", encoded.0);
}

pub fn print_computation(
    computation: Vec<automata::Configuration>,
    tm: automata::TuringMachine,
    output_tape: bool,
    trimmed_tape: bool,
    steps: bool,
    state: bool,
    head: bool,
) {
    for (ind, config) in computation.iter().enumerate() {
        if steps {
            print!("Step: {}     ", ind);
        }
        if state {
            print!("State {}     ", config.state);
        }
        if output_tape {
            print_tape(config.tapes[0].clone(), tm.clone(), Some(trimmed_tape));
        }
        if head {
            println!(" Head: {} ", config.tapes[0].head);
        }
    }
}

pub fn print_tm(tm: automata::TuringMachine) {
    println!("{}", tm.initial_state);
    println!("{}", tm.accept_state);
    println!("{}", tm.reject_state);
    println!("{} ", tm.final_states.join(" "));
    println!("{}", tm.blank_symbol);
    println!("{}", tm.states.join(" "));
    println!("{}", tm.input_alphabet.join(" "));
    println!("{}", tm.tape_alphabet.join(" "));
    println!("{}", tm.tape_count);
    for transition in tm.transitions.iter() {
        print!("{} ", transition.state);
        print!("{} ", transition.new_state);
        print!("{} ", transition.symbols.join(" "));
        print!("{} ", transition.new_symbols.join(" "));
        print!(
            "{} ",
            transition
                .directions
                .iter()
                .map(|x| {
                    if *x == automata::Direction::Left {
                        "L".to_string()
                    } else if *x == automata::Direction::Right {
                        "R".to_string()
                    } else {
                        "S".to_string()
                    }
                })
                .collect::<Vec<String>>()
                .join(" ")
        );
        println!();
    }
}

pub fn print_ram(ram: automata::RamMachine) {
    for instruction in ram.instructions.iter() {
        print!("OPCODE: {} ", instruction.opcode);
        print!("ARGUMENTS: {} ", instruction.operand);
        println!();
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
    if (options.type_ != "tm"
        && options.type_ != "fsm"
        && options.type_ != "pda"
        && options.type_ != "ram")
        || options.file.is_empty()
    {
        println!("Error: Invalid options. Use --help for more information.");
        return;
    }
    if options.type_ == "ram" {
        let ram: automata::RamMachine = file_handler::read_ram_progran_from_file(options.clone());
        if options.convert_to_tm{
            let mut options_new = options.clone();
            options_new.type_ = "tm".to_string();
            options_new.file = "src/standard/ram over tm.tm".to_string();
            options_new.input = options.input + &ram.to_encoding().0;
            let tm = file_handler::read_turing_machine_from_file(options_new.clone());
            if options_new.print_tm {
                print_tm(tm);
                return;
            }
            if options_new.print_encoding {
                print_encoding_tm(&tm);
                return;
            }
            if options.status {
                print_status(&tm);
            } else if options_new.clone().input.is_empty() {
                interactive_tui_tm(tm.clone(), options_new.clone());
            } else {
                execute_tm(tm.clone(), options_new.clone());
            }
        } else {
            if options.print_tm {
                print_ram(ram);
                return;
            }
            if options.print_encoding {
                print_encoding_ram(&ram);
    
                return;
            }
            if options.status {
                // print_status_ram(&ram); todo
            } else if options.clone().input.is_empty() {
                interactive_tui_ram(ram.clone(), options.clone());
            } else {
                execute_ram(ram.clone(), options.clone());
            }
        }
    } else {
        let mut tm;
        if options.type_ == "tm" {
            if options.from_encoding {
                tm = file_handler::read_tm_from_encoding_file(options.clone());
            } else {
                tm = file_handler::read_turing_machine_from_file(options.clone());
            }
        } else if options.type_ == "fsm" {
            tm = file_handler::read_finite_state_machine_from_file(options.clone());
        } else if options.type_ == "pda" {
            tm = file_handler::read_pushdown_automaton_from_file(options.clone());
        } else {
            return;
        }
        if options.convert_to_singletape {
            tm = automata::convert_multi_tape_to_single_tape_tm(tm);
        }
        if options.print_tm {
            print_tm(tm);
            return;
        }
        if options.print_encoding {
            print_encoding_tm(&tm);
            return;
        }
        if options.status {
            print_status(&tm);
        } else if options.clone().input.is_empty() {
            interactive_tui_tm(tm.clone(), options.clone());
        } else {
            execute_tm(tm.clone(), options.clone());
        }
    }
}
