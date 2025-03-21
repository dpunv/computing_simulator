// file: cli.rs
// Project: Computing Simulator
// author: dp

use crate::turing_machine;

use crate::file_handler;
use crate::options;
use crate::regex;
use crate::ram_machine;
use crate::utils;
use core::panic;
use std::io::Write;
use std::vec;

fn print_help() {
    println!("Usage: turing_machine [OPTIONS] [FILE] [INPUT]");
    println!();
    println!("Options:");
    println!("  --type: turing machine (tm), finite state machine (fsm), pushdown automaton (pda), random access machine (ram)");
    println!("  --convert-to-tm: convert a RAM machine into a Turing Machine");
    println!("  --from-encoding: read the Turing Machine from an encoding file");
    println!("  --help: print the help message");
    println!("  --version: print the version of the Turing Machine Simulator");
    println!("  --print-nth-tm: print the nth Turing Machine");
    println!("  --print-tm: print the Turing Machine");
    println!("  --print-number: print the number of the Turing Machine");
    println!("  --verbose: set the verbosity level of the Turing Machine");
    println!("  --max-steps: set the maximum number of steps for the Turing Machine");
    println!(
        "  --convert-to-singletape: convert multitape machines into single tape Turing Machines"
    );
    println!("  --input: provide the input string for the Turing Machine");
    println!("  --file: provide the file containing the description of the Turing Machine");
    println!("  --regex: read the regular expression from the file (works only with FSM type)");
    println!("  --status: print informations about the Turing Machine");
    println!("  --print-encoding: print the encoding of the Turing Machine");
    println!();
    println!("Acknowledgements:");
    println!("  This program is made by dp. Licensed under the MIT License.");
    println!();
}

fn print_version() {
    println!("Turing Machine Simulator 0.1.0");
}

pub fn print_tape(tape: turing_machine::Tape, tm: turing_machine::TuringMachine, trim: Option<bool>) {
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

pub fn print_status_tm(tm: &turing_machine::TuringMachine) {
    println!("Deterministic: {}", tm.is_deterministic());
    println!("Ok: {}", tm.is_ok());
    println!("Transition total: {}", tm.is_transition_total());
}

fn execute_tm(mut tm: turing_machine::TuringMachine, opt: options::Options) {
    let input_tape = utils::input_string_to_vec(tm.input_alphabet(), opt.input.clone());
    let result = tm.simulate(input_tape, opt.max_steps);
    if opt.verbose == 0 {
        println!("{}", result.0);
        print_tape(tm.last_execution.1[0].clone(), tm.clone(), Some(true));
    } else if opt.verbose >= 1 {
        println!("State: {}", result.0);
        println!("Steps: {}", result.1);
        print!("Output Tape: ");
        print_tape(tm.last_execution.1[0].clone(), tm.clone(), Some(true));
        println!();
        if tm.final_states.contains(&result.0) && (tm.accept_state == result.0 || !tm.end_on_final_state) && (tm.end_on_final_state || tm.last_execution.1[0].tape.iter().all(|x| x == &tm.blank_symbol)) {
            println!("Accept");
        } else if tm.final_states.contains(&result.0) && tm.reject_state != result.0 {
            println!("Halt");
        } else {
            println!("Reject");
        }
    } else if opt.verbose < 0 || opt.verbose > 2 {
        panic!("Invalid verbose level");
    }
    if opt.verbose >= 2 {
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
    }
}

fn execute_ram(ram: ram_machine::RamMachine, opt: options::Options) {
    let input_tape = vec![opt.input.clone()];
    let result: (String, i32) = ram.clone().simulate(input_tape, opt.max_steps);
    if opt.verbose == 0 {
        println!("{}", result.0);
    } else if opt.verbose >= 1 {
        println!("Output: {}", result.0);
        println!("Steps: {}", result.1);
    } else {
        panic!("Invalid verbose level");
    }
}

fn interactive_tui_tm(tm: turing_machine::TuringMachine, opt: options::Options) {
    let mut input = String::new();
    loop {
        print!("> ");
        // flush the output buffer
        std::io::stdout().flush().unwrap();
        input.clear();
        std::io::stdin().read_line(&mut input).unwrap();
        let trimmed_input = input.trim().to_string();
        let mut new_opt = opt.clone();
        new_opt.input = input.clone();
        if trimmed_input == "status" {
            print_status_tm(&tm);
        } else if trimmed_input == "help" {
            print_help();
        } else if trimmed_input == "version" {
            print_version();
        } else if trimmed_input == "exit" {
            break;
        } else {
            execute_tm(tm.clone(), new_opt.clone());
        }
    }
}

fn interactive_tui_ram(ram: ram_machine::RamMachine, opt: options::Options) {
    let mut input = String::new();
    loop {
        print!("> ");
        std::io::stdout().flush().unwrap();
        input.clear();
        std::io::stdin().read_line(&mut input).unwrap();
        let mut new_opt = opt.clone();
        new_opt.input = input.clone();
        let trimmed_input = input.trim().to_string();
        if trimmed_input == "status" {
            print_status_ram(&ram);
        } else if trimmed_input == "help" {
            print_help();
        } else if trimmed_input == "version" {
            print_version();
        } else if trimmed_input == "exit" {
            break;
        } else {
            execute_ram(ram.clone(), new_opt.clone());
        }
    }
}

pub fn print_encoding_tm(tm: &turing_machine::TuringMachine) {
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

pub fn print_encoding_ram(ram: &ram_machine::RamMachine) {
    let encoded: (
        String,
        std::collections::HashMap<String, String>,
        std::collections::HashMap<String, String>,
    ) = ram.to_encoding();
    println!("{}", encoded.0);
}

pub fn print_computation(
    computation: Vec<turing_machine::Configuration>,
    tm: turing_machine::TuringMachine,
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

pub fn print_tm(tm: turing_machine::TuringMachine) {
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
                    if *x == turing_machine::Direction::Left {
                        "L".to_string()
                    } else if *x == turing_machine::Direction::Right {
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

pub fn print_ram(ram: ram_machine::RamMachine) {
    for instruction in ram.instructions.iter() {
        print!("OPCODE: {} ", instruction.opcode);
        print!("ARGUMENTS: {} ", instruction.operand);
        println!();
    }
}

fn print_status_ram(ram: &ram_machine::RamMachine) {
    println!("Number of instructions: {}", ram.instructions.len());
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

    if !validate_options(&options) {
        println!("Error: Invalid options. Use --help for more information.");
        return;
    }

    if options.print_nth_tm != -1 {
        let tm_encoding = turing_machine::nth_turing_machine((options.print_nth_tm) as u128);
        println!("{}", tm_encoding);
        //automata::test_turing_machines((options.print_nth_tm) as u64);
        //print_tm(automata::encoding_to_tm(tm_encoding));
        return;
    }

    match options.type_.as_str() {
        "ram" => handle_ram_machine(options),
        "tm" | "fsm" | "pda" => handle_automaton(options),
        _ => println!("Error: Unsupported machine type"),
    }
}

fn validate_options(options: &options::Options) -> bool {
    let valid_types = ["tm", "fsm", "pda", "ram"];
    valid_types.contains(&options.type_.as_str())
        && (!options.file.is_empty() || options.print_nth_tm != -1)
}

fn handle_ram_machine(options: options::Options) {
    let ram: ram_machine::RamMachine;
    if options.from_encoding {
        ram = file_handler::read_ram_program_from_encoding_file(options.clone())
    } else {
        ram = file_handler::read_ram_program_from_file(options.clone());
    }

    if options.convert_to_tm {
        handle_ram_to_tm_conversion(ram, options);
        return;
    }

    if options.print_tm {
        print_ram(ram);
        return;
    }

    if options.print_encoding {
        print_encoding_ram(&ram);
        return;
    }

    if options.status {
        print_status_ram(&ram); // TODO: Implementare questa funzione
    } else if options.clone().input.is_empty() {
        interactive_tui_ram(ram.clone(), options);
    } else {
        execute_ram(ram, options);
    }
}

fn handle_ram_to_tm_conversion(ram: ram_machine::RamMachine, options: options::Options) {
    let mut options_new = options.clone();
    options_new.type_ = "tm".to_string();
    options_new.file = "src/standard/ram over tm.tm".to_string();
    options_new.input = options.input + &ram.to_encoding().0;

    handle_automaton(options_new);
}

fn handle_automaton(options: options::Options) {
    let mut tm = load_automaton(options.clone());

    if options.convert_to_singletape {
        tm = tm.convert_multi_tape_to_single_tape_tm();
    }

    if options.print_number {
        println!("{}", tm.number());
        return;
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
        print_status_tm(&tm);
    } else if options.input.is_empty() {
        interactive_tui_tm(tm.clone(), options);
    } else {
        execute_tm(tm, options);
    }
}

fn load_automaton(options: options::Options) -> turing_machine::TuringMachine {
    match options.type_.as_str() {
        "tm" => {
            if options.from_encoding {
                file_handler::read_tm_from_encoding_file(options)
            } else {
                file_handler::read_turing_machine_from_file(options)
            }
        }
        "fsm" => {
            if options.regex {
                let result = file_handler::read_regex_from_file(options);
                match result {
                    Ok(regex) => {
                        regex::regex_to_fsa(&regex)
                    }
                    Err(e) => {
                        panic!("Error parsing the regex: {}", e);
                    }
                }
            } else {
                file_handler::read_finite_state_machine_from_file(options)
            }
        }
        "pda" => file_handler::read_pushdown_automaton_from_file(options),
        _ => panic!("Tipo di automa non supportato"),
    }
}
