// file: cli.rs
// Project: Computing Simulator
// author: dp

use crate::turing_machine;

use crate::computer;
use crate::file_handler;
use crate::options;
use crate::ram_machine;
use std::io::Write;

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

/* pub fn print_tape(
    tape: turing_machine::Tape,
    tm: turing_machine::TuringMachine,
    trim: Option<bool>,
) {
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
 */

pub fn print_status_tm(tm: &turing_machine::TuringMachine) {
    println!("Deterministic: {}", tm.is_deterministic());
    println!("Ok: {}", tm.is_ok());
    println!("Transition total: {}", tm.is_transition_total());
}

fn process_results(server: computer::Server, opt: options::Options) {
    let result = server.clone().execute(opt.input.clone(), opt.max_steps);
    match result {
        Ok((state, _, tape, steps)) => {
            if opt.verbose < 0 {
                panic!("Invalid verbose level");
            }
            if opt.verbose >= 0 {
                println!("{}", state);
                println!("{}", tape);
            }
            if opt.verbose >= 1 {
                println!("Steps: {}", steps);
            }
        }
        Err(error) => {
            println!("An error occurred: {}", error);
        }
    }
}

fn interactive_tui(server: &mut computer::Server, opt: options::Options) {
    let mut input = String::new();
    loop {
        print!("> ");
        std::io::stdout().flush().unwrap();
        input.clear();
        std::io::stdin().read_line(&mut input).unwrap();
        let mut new_opt = opt.clone();
        let trimmed_input = input.trim().to_string();
        new_opt.input = input.clone();
        if trimmed_input == "status" {
            if server
                .get_computer(server.computes_at(0).clone())
                .unwrap()
                .is_turing()
            {
                print_status_tm(
                    server
                        .get_computer(server.computes_at(0).clone())
                        .unwrap()
                        .turing_machine
                        .as_ref()
                        .unwrap(),
                );
            } else {
                print_status_ram(
                    server
                        .get_computer(server.computes_at(0).clone())
                        .unwrap()
                        .ram_machine
                        .as_ref()
                        .unwrap(),
                )
            }
        } else if trimmed_input == "help" {
            print_help();
        } else if trimmed_input == "version" {
            print_version();
        } else if trimmed_input == "exit" {
            break;
        } else {
            process_results(server.clone(), new_opt.clone());
        }
    }
}

pub fn print_encoding(c: &computer::Computer) {
    let encoded: (
        String,
        std::collections::HashMap<String, String>,
        std::collections::HashMap<String, String>,
    ) = match c.to_encoding() {
        Ok(res) => res,
        Err(error) => {
            println!("Error: {}", error);
            return;
        }
    };
    println!("{}", encoded.0);
    println!();
    for (k, v) in encoded.1.iter() {
        println!("{} {}", v, k);
    }
    println!();
    for (k, v) in encoded.2.iter() {
        println!("{} {}", v, k);
    }
}
/*
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
 */
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
    let mut options = options::get_options();

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
        let tm_encoding =
            turing_machine::TuringMachine::nth_turing_machine((options.print_nth_tm) as u128);
        println!("{}", tm_encoding);
        return;
    }

    handle_computation(&mut options);
}

fn validate_options(options: &options::Options) -> bool {
    !options.file.is_empty() || options.print_nth_tm != -1
}

fn handle_computation(options: &mut options::Options) {
    let mut s = computer::Server::new();
    let mut c;
    match file_handler::handle_file_reads(options.file.clone(), &mut s) {
        Ok(comp) => {
            c = comp;
        }
        Err(error) => {
            println!("Error: {}", error);
            return;
        }
    }
    if c.is_ram() {
        if options.convert_to_tm {
            match c.ram_to_tm(options, &mut s) {
                Ok(comp) => c = comp,
                Err(error) => {
                    println!("Error: {}", error);
                    return;
                }
            }
        }
        if options.convert_to_singletape {
            println!("Error: invalid option --convert-to-singletape on non-tm file");
        }
        if options.print_number {
            println!("Error: invalid option --print-number on non-tm file");
        }
    } else {
        if options.convert_to_singletape {
            c.turing_machine = std::option::Option::Some(
                c.turing_machine
                    .as_ref()
                    .unwrap()
                    .convert_multi_tape_to_single_tape_tm(),
            );
        }
        if options.print_number {
            println!("{}", c.turing_machine.as_ref().unwrap().number());
            return;
        }
        if options.convert_to_tm {
            println!("Error: invalid option --convert-to-tm on non-ram file");
        }
    }
    s.add_computer(options.file.clone(), c.clone());
    s.set_computation_order_at(0, options.file.clone());
    if options.print_computer {
        if c.is_ram() {
            print_ram(c.ram_machine.unwrap());
        } else {
            print_tm(c.turing_machine.unwrap());
        }
        return;
    }

    if options.print_encoding {
        print_encoding(&c);
        return;
    }

    if options.status {
        if c.is_turing() {
            print_status_tm(c.turing_machine.as_ref().unwrap());
        } else {
            print_status_ram(c.ram_machine.as_ref().unwrap())
        }
    } else if options.clone().input.is_empty() {
        interactive_tui(&mut s, options.clone());
    } else {
        process_results(s, options.clone());
    }
}
