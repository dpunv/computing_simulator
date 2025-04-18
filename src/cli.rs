// file: cli.rs
// Project: Computing Simulator
// author: dp

use crate::lambda;
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
    println!(
        "  --convert-to-tm: convert a RAM Machine or a lambda expression into a Turing Machine"
    );
    println!(
        "  --convert-to-ram: convert a Turing Machine or a lambda expression into a RAM Machine"
    );
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
        Ok((state, _, tape, steps, computation)) => {
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
            if opt.verbose >= 2 {
                println!("Computation: ");
                for conf in computation {
                    println!("  {}", conf);
                }
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
        match std::io::stdout().flush() {
            Ok(_) => {}
            Err(error) => println!("Error: {}", error),
        }
        input.clear();
        match std::io::stdin()
            .read_line(&mut input)
            .map_err(|e| e.to_string())
        {
            Ok(_) => {}
            Err(error) => println!("Error: {}", error),
        };
        let mut new_opt = opt.clone();
        let trimmed_input = input.trim().to_string();
        new_opt.input = input.clone();
        if trimmed_input == "status" {
            match server
                .get_computer(server.computes_at(0).clone())
                .map(|c| c.element.clone())
            {
                Some(element) => match element {
                    computer::ComputingElem::Tm(m) => print_status_tm(&m),
                    computer::ComputingElem::Ram(m) => print_status_ram(&m),
                    computer::ComputingElem::Lambda(_) => {}
                },
                None => println!("Error: Could not get computer status"),
            }
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

pub fn print_lambda_as_tree(l: lambda::Lambda) {
    println!("NAME: {}", l.name);
    fn print_expr(expr: &lambda::LambdaExpr, indent: usize) {
        let padding = " ".repeat(indent);
        match expr {
            lambda::LambdaExpr::Var(v) => println!("{}Var({})", padding, v),
            lambda::LambdaExpr::Abs(params, body) => {
                println!("{}Function {}", padding, params.join(", "));
                print_expr(body, indent + 4);
            }
            lambda::LambdaExpr::App(exprs) => {
                println!("{}Application", padding);
                for e in exprs.iter() {
                    print_expr(e, indent + 4);
                }
            }
        }
    }
    print_expr(&l.expr, 0);
}

pub fn print_tm(tm: turing_machine::TuringMachine) {
    println!("{}", tm.initial_state);
    println!("{}", tm.accept_state);
    println!("{}", tm.reject_state);
    println!("{}", tm.halt_state);
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
    if !ram.translation_map.is_empty() {
        println!("translations: ");
        for (k, v) in ram.translation_map {
            println!("  {} --> {}", k, v);
        } 
    }
}

pub fn print_lambda(l: lambda::Lambda) {
    for lambda in l.references {
        println!("{}", lambda);
    }
}

fn print_status_ram(ram: &ram_machine::RamMachine) {
    println!("Number of instructions: {}", ram.instructions.len());
}

pub fn main_cli() {
    main_cli_with_options(options::get_options());
}

pub fn main_cli_with_options(mut options: options::Options) {
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
            match turing_machine::TuringMachine::nth_turing_machine((options.print_nth_tm) as u128)
            {
                Ok(res) => res,
                Err(error) => return println!("error: {}", error),
            };
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
    match c.element.clone() {
        computer::ComputingElem::Tm(m) => {
            if options.convert_to_singletape {
                match m.convert_multi_tape_to_single_tape_tm() {
                    Ok(m_st) => {
                        c.set_turing(m_st);
                    }
                    Err(error) => println!("error: {}", error),
                }
            }
            if options.print_number {
                println!(
                    "{}",
                    match m.number() {
                        Ok(res) => res.to_string(),
                        Err(error) => error,
                    }
                );
                return;
            }
            if options.convert_to_ram {
                match c.to_ram(options, &mut s) {
                    Ok(comp) => c = comp,
                    Err(error) => {
                        println!("Error: {}", error);
                        return;
                    }
                }
            }
            if options.convert_to_tm {
                println!("Error: invalid option --convert-to-tm on tm file");
            }
        }
        computer::ComputingElem::Ram(_) => {
            if options.convert_to_tm {
                match c.to_tm(options, &mut s) {
                    Ok(comp) => c = comp,
                    Err(error) => {
                        println!("Error: {}", error);
                        return;
                    }
                }
            }
            if options.convert_to_tm {
                println!("Error: invalid option --convert-to-ran on ram file");
            }
            if options.convert_to_singletape {
                println!("Error: invalid option --convert-to-singletape on non-tm file");
            }
            if options.print_number {
                println!("Error: invalid option --print-number on non-tm file");
            }
        }
        computer::ComputingElem::Lambda(_) => {
            if options.convert_to_singletape || options.print_number {
                println!("Error: invalid option on non-tm, non-ram file");
            } else if options.convert_to_tm {
                match c.to_tm(options, &mut s) {
                    Ok(comp) => c = comp,
                    Err(error) => {
                        println!("Error: {}", error);
                        return;
                    }
                }
            } else if options.convert_to_ram {
                match c.to_ram(options, &mut s) {
                    Ok(comp) => c = comp,
                    Err(error) => {
                        println!("Error: {}", error);
                        return;
                    }
                }
            }
        }
    }
    s.add_computer(options.file.clone(), c.clone());
    s.set_computation_order_at(0, options.file.clone());
    if options.print_computer {
        match c.element {
            computer::ComputingElem::Ram(m) => print_ram(m),
            computer::ComputingElem::Tm(m) => print_tm(*m),
            computer::ComputingElem::Lambda(l) => print_lambda(l),
        }
        return;
    }

    if options.print_encoding {
        print_encoding(&c);
        return;
    }

    if options.status {
        match c.element.clone() {
            computer::ComputingElem::Tm(m) => print_status_tm(&m),
            computer::ComputingElem::Ram(m) => print_status_ram(&m),
            computer::ComputingElem::Lambda(l) => print_lambda_as_tree(l),
        }
    } else if options.clone().input.is_empty() {
        interactive_tui(&mut s, options.clone());
    } else {
        process_results(s, options.clone());
    }
}
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_validate_options() {
        let mut opt = options::Options::default();

        opt.file = "test.tm".to_string();
        assert!(validate_options(&opt));

        opt.file = "".to_string();
        opt.print_nth_tm = 1;
        assert!(validate_options(&opt));
    }

    #[test]
    fn test_print_status_tm() {
        let tm = turing_machine::TuringMachine {
            states: vec!["q0".to_string(), "q1".to_string()],
            input_alphabet: vec!["0".to_string(), "1".to_string()],
            tape_alphabet: vec!["0".to_string(), "1".to_string(), "B".to_string()],
            initial_state: "q0".to_string(),
            accept_state: "qa".to_string(),
            reject_state: "qr".to_string(),
            halt_state: "qh".to_string(),
            blank_symbol: "B".to_string(),
            transitions: vec![],
            tape_count: 1,
            next_state_id: 10,
        };
        print_status_tm(&tm);
    }

    #[test]
    fn test_print_status_ram() {
        let ram = ram_machine::RamMachine {
            instructions: vec![],
            labels_map: std::collections::HashMap::new(),
            translation_map: std::collections::HashMap::new(),
        };
        print_status_ram(&ram);
    }

    #[test]
    fn test_process_results() {
        let mut server = computer::Server::new();
        let mut opt = options::Options::default();
        opt.verbose = 1;
        opt.input = "test".to_string();
        opt.max_steps = 100;

        let mut computer = computer::Computer::new();
        computer.set_turing(turing_machine::TuringMachine::new());
        server.add_computer("test".to_string(), computer);
        process_results(server, opt);
    }

    #[test]
    fn test_print_tm() {
        let tm = turing_machine::TuringMachine {
            states: vec!["q0".to_string()],
            input_alphabet: vec!["0".to_string()],
            tape_alphabet: vec!["0".to_string(), "B".to_string()],
            initial_state: "q0".to_string(),
            accept_state: "qa".to_string(),
            reject_state: "qr".to_string(),
            halt_state: "qh".to_string(),
            blank_symbol: "B".to_string(),
            transitions: vec![turing_machine::Transition {
                state: "q0".to_string(),
                new_state: "q1".to_string(),
                symbols: vec!["0".to_string()],
                new_symbols: vec!["1".to_string()],
                directions: vec![turing_machine::Direction::Right],
            }],
            tape_count: 1,
            next_state_id: 1,
        };
        print_tm(tm);
    }

    #[test]
    fn test_print_encoding() {
        let mut computer = computer::Computer::new();
        computer.set_turing(turing_machine::TuringMachine::new());
        print_encoding(&computer);
    }

    #[test]
    fn test_print_version() {
        print_version();
    }

    #[test]
    fn test_print_help() {
        print_help();
    }
    #[test]
    fn test_print_ram() {
        let ram = ram_machine::RamMachine {
            instructions: vec![
                ram_machine::Instruction {
                    opcode: "0101".to_string(),
                    operand: "1".to_string(),
                    label: "".to_string(),
                },
                ram_machine::Instruction {
                    opcode: "0110".to_string(),
                    operand: "10".to_string(),
                    label: "".to_string(),
                },
            ],
            labels_map: std::collections::HashMap::new(),
            translation_map: std::collections::HashMap::new(),
        };
        print_ram(ram);
    }

    #[test]
    fn test_print_lambda() {
        let lambda = lambda::Lambda {
            name: "test".to_string(),
            expr: lambda::LambdaExpr::Var("x".to_string()),
            references: vec![lambda::Lambda {
                name: "test2".to_string(),
                expr: lambda::parse_lambda("(\\x.(x))").unwrap(),
                references: vec![],
                force_currying: false,
            }],
            force_currying: false,
        };
        print_lambda(lambda);
    }

    #[test]
    fn test_print_lambda_as_tree() {
        let lambda = lambda::Lambda {
            name: "test".to_string(),
            expr: lambda::LambdaExpr::Abs(
                vec!["x".to_string()],
                Box::new(lambda::LambdaExpr::Var("x".to_string())),
            ),
            references: vec![],
            force_currying: false,
        };
        print_lambda_as_tree(lambda);
    }

    #[test]
    fn test_pseudo_invalid_verbose_level() {
        let server = computer::Server::new();
        let mut opt = options::Options::default();
        opt.verbose = -1;
        opt.input = "test".to_string();
        opt.max_steps = 100;

        let result = std::panic::catch_unwind(|| {
            process_results(server, opt);
        });
        assert!(result.is_ok());
    }

    #[test]
    fn test_handle_computation_tm_convert_to_singletape() {
        let mut opt = options::Options::default();
        opt.file = "test.tm".to_string();
        opt.convert_to_singletape = true;
        let mut s = computer::Server::new();
        let mut c = computer::Computer::new();
        let mut tm = turing_machine::TuringMachine::new();
        tm.tape_count = 2;
        c.set_turing(tm);
        s.add_computer(opt.file.clone(), c.clone());
        handle_computation(&mut opt);
    }

    #[test]
    fn test_handle_computation_tm_print_number() {
        let mut opt = options::Options::default();
        opt.file = "test.tm".to_string();
        opt.print_number = true;
        let mut s = computer::Server::new();
        let mut c = computer::Computer::new();
        c.set_turing(turing_machine::TuringMachine::new());
        s.add_computer(opt.file.clone(), c.clone());
        handle_computation(&mut opt);
    }

    #[test]
    fn test_handle_computation_tm_convert_to_ram() {
        let mut opt = options::Options::default();
        opt.file = "test.tm".to_string();
        opt.convert_to_ram = true;
        let mut s = computer::Server::new();
        let mut c = computer::Computer::new();
        c.set_turing(turing_machine::TuringMachine::new());
        s.add_computer(opt.file.clone(), c.clone());
        handle_computation(&mut opt);
    }

    #[test]
    fn test_handle_computation_tm_convert_to_tm() {
        let mut opt = options::Options::default();
        opt.file = "test.tm".to_string();
        opt.convert_to_tm = true;
        let mut s = computer::Server::new();
        let mut c = computer::Computer::new();
        c.set_turing(turing_machine::TuringMachine::new());
        s.add_computer(opt.file.clone(), c.clone());
        handle_computation(&mut opt);
    }

    #[test]
    fn test_handle_computation_ram_convert_to_tm() {
        let mut opt = options::Options::default();
        opt.file = "test.ram".to_string();
        opt.convert_to_tm = true;
        let mut s = computer::Server::new();
        let mut c = computer::Computer::new();
        c.set_ram(ram_machine::RamMachine {
            instructions: vec![],
            labels_map: std::collections::HashMap::new(),
            translation_map: std::collections::HashMap::new(),
        });
        s.add_computer(opt.file.clone(), c.clone());
        handle_computation(&mut opt);
    }

    #[test]
    fn test_handle_computation_ram_convert_to_tm_invalid() {
        let mut opt = options::Options::default();
        opt.file = "test.ram".to_string();
        opt.convert_to_tm = true;
        let mut s = computer::Server::new();
        let mut c = computer::Computer::new();
        c.set_ram(ram_machine::RamMachine {
            instructions: vec![],
            labels_map: std::collections::HashMap::new(),
            translation_map: std::collections::HashMap::new(),
        });
        s.add_computer(opt.file.clone(), c.clone());
        opt.convert_to_tm = true;
        handle_computation(&mut opt);
    }

    #[test]
    fn test_handle_computation_ram_convert_to_singletape() {
        let mut opt = options::Options::default();
        opt.file = "test.ram".to_string();
        opt.convert_to_singletape = true;
        let mut s = computer::Server::new();
        let mut c = computer::Computer::new();
        c.set_ram(ram_machine::RamMachine {
            instructions: vec![],
            labels_map: std::collections::HashMap::new(),
            translation_map: std::collections::HashMap::new(),
        });
        s.add_computer(opt.file.clone(), c.clone());
        handle_computation(&mut opt);
    }

    #[test]
    fn test_handle_computation_ram_print_number() {
        let mut opt = options::Options::default();
        opt.file = "test.ram".to_string();
        opt.print_number = true;
        let mut s = computer::Server::new();
        let mut c = computer::Computer::new();
        c.set_ram(ram_machine::RamMachine {
            instructions: vec![],
            labels_map: std::collections::HashMap::new(),
            translation_map: std::collections::HashMap::new(),
        });
        s.add_computer(opt.file.clone(), c.clone());
        handle_computation(&mut opt);
    }

    #[test]
    fn test_handle_computation_lambda_convert_to_tm() {
        let mut opt = options::Options::default();
        opt.file = "test.lambda".to_string();
        opt.convert_to_tm = true;
        let mut s = computer::Server::new();
        let mut c = computer::Computer::new();
        c.set_lambda(lambda::Lambda {
            name: "test".to_string(),
            expr: lambda::LambdaExpr::Var("x".to_string()),
            references: vec![],
            force_currying: false,
        });
        s.add_computer(opt.file.clone(), c.clone());
        handle_computation(&mut opt);
    }

    #[test]
    fn test_handle_computation_lambda_convert_to_ram() {
        let mut opt = options::Options::default();
        opt.file = "test.lambda".to_string();
        opt.convert_to_ram = true;
        let mut s = computer::Server::new();
        let mut c = computer::Computer::new();
        c.set_lambda(lambda::Lambda {
            name: "test".to_string(),
            expr: lambda::LambdaExpr::Var("x".to_string()),
            references: vec![],
            force_currying: false,
        });
        s.add_computer(opt.file.clone(), c.clone());
        handle_computation(&mut opt);
    }

    #[test]
    fn test_handle_computation_lambda_convert_to_singletape_print_number() {
        let mut opt = options::Options::default();
        opt.file = "test.lambda".to_string();
        opt.convert_to_singletape = true;
        opt.print_number = true;
        let mut s = computer::Server::new();
        let mut c = computer::Computer::new();
        c.set_lambda(lambda::Lambda {
            name: "test".to_string(),
            expr: lambda::LambdaExpr::Var("x".to_string()),
            references: vec![],
            force_currying: false,
        });
        s.add_computer(opt.file.clone(), c.clone());
        handle_computation(&mut opt);
    }

    #[test]
    fn test_handle_computation_print_computer_tm() {
        let mut opt = options::Options::default();
        opt.file = "test.tm".to_string();
        opt.print_computer = true;
        let mut s = computer::Server::new();
        let mut c = computer::Computer::new();
        c.set_turing(turing_machine::TuringMachine::new());
        s.add_computer(opt.file.clone(), c.clone());
        handle_computation(&mut opt);
    }

    #[test]
    fn test_handle_computation_print_computer_ram() {
        let mut opt = options::Options::default();
        opt.file = "test.ram".to_string();
        opt.print_computer = true;
        let mut s = computer::Server::new();
        let mut c = computer::Computer::new();
        c.set_ram(ram_machine::RamMachine {
            instructions: vec![],
            labels_map: std::collections::HashMap::new(),
            translation_map: std::collections::HashMap::new(),
        });
        s.add_computer(opt.file.clone(), c.clone());
        handle_computation(&mut opt);
    }

    #[test]
    fn test_handle_computation_print_computer_lambda() {
        let mut opt = options::Options::default();
        opt.file = "test.lambda".to_string();
        opt.print_computer = true;
        let mut s = computer::Server::new();
        let mut c = computer::Computer::new();
        c.set_lambda(lambda::Lambda {
            name: "test".to_string(),
            expr: lambda::LambdaExpr::Var("x".to_string()),
            references: vec![],
            force_currying: false,
        });
        s.add_computer(opt.file.clone(), c.clone());
        handle_computation(&mut opt);
    }

    #[test]
    fn test_handle_computation_print_encoding() {
        let mut opt = options::Options::default();
        opt.file = "test.tm".to_string();
        opt.print_encoding = true;
        let mut s = computer::Server::new();
        let mut c = computer::Computer::new();
        c.set_turing(turing_machine::TuringMachine::new());
        s.add_computer(opt.file.clone(), c.clone());
        handle_computation(&mut opt);
    }

    #[test]
    fn test_handle_computation_status_tm() {
        let mut opt = options::Options::default();
        opt.file = "test.tm".to_string();
        opt.status = true;
        let mut s = computer::Server::new();
        let mut c = computer::Computer::new();
        c.set_turing(turing_machine::TuringMachine::new());
        s.add_computer(opt.file.clone(), c.clone());
        handle_computation(&mut opt);
    }

    #[test]
    fn test_handle_computation_status_ram() {
        let mut opt = options::Options::default();
        opt.file = "test.ram".to_string();
        opt.status = true;
        let mut s = computer::Server::new();
        let mut c = computer::Computer::new();
        c.set_ram(ram_machine::RamMachine {
            instructions: vec![],
            labels_map: std::collections::HashMap::new(),
            translation_map: std::collections::HashMap::new(),
        });
        s.add_computer(opt.file.clone(), c.clone());
        handle_computation(&mut opt);
    }

    #[test]
    fn test_handle_computation_status_lambda() {
        let mut opt = options::Options::default();
        opt.file = "test.lambda".to_string();
        opt.status = true;
        let mut s = computer::Server::new();
        let mut c = computer::Computer::new();
        c.set_lambda(lambda::Lambda {
            name: "test".to_string(),
            expr: lambda::LambdaExpr::Var("x".to_string()),
            references: vec![],
            force_currying: false,
        });
        s.add_computer(opt.file.clone(), c.clone());
        handle_computation(&mut opt);
    }

    #[test]
    fn test_handle_computation_interactive_tui() {
        // This test is limited since interactive_tui waits for stdin.
        // We can only check that it doesn't panic when input is empty.
        // You may want to refactor interactive_tui for better testability.ù
    }

    #[test]
    fn test_main_cli_help() {
        let mut opt = options::Options::default();
        opt.help = true;
        main_cli_with_options(opt);
    }

    #[test]
    fn test_main_cli_version() {
        let mut opt = options::Options::default();
        opt.version = true;
        main_cli_with_options(opt);
    }

    #[test]
    fn test_main_cli_invalid_options() {
        let opt = options::Options::default();
        main_cli_with_options(opt);
    }

    #[test]
    fn test_main_cli_print_nth_tm() {
        let mut opt = options::Options::default();
        opt.print_nth_tm = 0;
        main_cli_with_options(opt);
    }
}
