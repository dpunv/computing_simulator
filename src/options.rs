//! # options.rs
//!
//! This module provides the `Options` struct and related functionality for parsing and storing
//! command-line options for the Computing Simulator project.
//!
//! ## Overview
//!
//! - Defines the `Options` struct, which holds all configurable parameters and flags
//!   that can be set via command-line arguments.
//! - Implements the `get_options` function, which parses command-line arguments and
//!   returns an `Options` instance populated with the appropriate values.
//! - Includes a test module for verifying the correct parsing and handling of options.
//!
//! ## Supported Command-Line Arguments
//!
//! - `--convert-to-tm`: Enable conversion to Turing Machine.
//! - `--convert-to-ram`: Enable conversion to RAM machine.
//! - `--convert-to-singletape`: Enable conversion to single-tape Turing Machine.
//! - `--print-computer`: Print the computer configuration.
//! - `--print-number`: Print the number representation.
//! - `--print-nth-tm=<i128>`: Print the nth Turing Machine.
//! - `--help`: Show help information.
//! - `--version`: Show version information.
//! - `--max-steps=<usize>`: Set the maximum number of steps (default: 1000).
//! - `--input=<String>`: Specify the input string.
//! - `--file=<String>`: Specify the input file.
//! - `--status`: Print status information.
//! - `--print-encoding`: Print the encoding used.
//! - `--verbose=<i32>`: Set verbosity level (default: 1).
//!
//! Any unrecognized argument is treated as a file name, with optional surrounding quotes removed.
//!
//! ## Testing
//!
//! The module includes a test suite that mocks command-line arguments to verify the correct
//! parsing and population of the `Options` struct under various scenarios.
//!
//! ## Author
//!
//! - dp
//! 
//! # License
//! 
//! This project is licensed under the MIT License. See the LICENSE file for details.

/// Represents the set of configurable command-line options for the Computing Simulator.
///
/// Each field corresponds to a specific command-line flag or parameter that can be set
/// by the user when running the application. This struct is populated by the `get_options`
/// function, which parses the command-line arguments and assigns values accordingly.
///
/// # Fields
///
/// - `convert_to_tm`: Enables conversion to a Turing Machine when set to `true`.
/// - `convert_to_ram`: Enables conversion to a RAM machine when set to `true`.
/// - `convert_to_singletape`: Enables conversion to a single-tape Turing Machine when set to `true`.
/// - `print_computer`: Prints the computer configuration if `true`.
/// - `print_number`: Prints the number representation if `true`.
/// - `print_nth_tm`: If set to a non-negative value, prints the nth Turing Machine.
/// - `help`: Shows help information when set to `true`.
/// - `version`: Shows version information when set to `true`.
/// - `max_steps`: Sets the maximum number of steps for the simulation (default: 1000).
/// - `input`: Specifies the input string for the simulation.
/// - `file`: Specifies the input file name for the simulation.
/// - `status`: Prints status information if `true`.
/// - `print_encoding`: Prints the encoding used if `true`.
/// - `verbose`: Sets the verbosity level (default: 1).
#[derive(Clone, Default)]
pub struct Options {
    pub convert_to_tm: bool,
    pub convert_to_ram: bool,
    pub convert_to_singletape: bool,
    pub print_computer: bool,
    pub print_number: bool,
    pub print_nth_tm: i128,
    pub help: bool,
    pub version: bool,
    pub max_steps: usize,
    pub input: String,
    pub file: String,
    pub status: bool,
    pub print_encoding: bool,
    pub verbose: i32,
}

/// Parses command-line arguments and returns an `Options` struct populated with the corresponding values.
///
/// This function processes the command-line arguments provided to the program and sets the fields
/// of the `Options` struct according to the recognized flags and parameters. Supported arguments include
/// flags for enabling conversions, printing information, setting simulation parameters, and specifying
/// input sources. Unrecognized arguments are treated as file names, with optional surrounding quotes removed.
///
/// # Returns
///
/// An `Options` instance with fields set according to the parsed command-line arguments.
///
/// # Supported Arguments
///
/// - `--convert-to-tm`
/// - `--convert-to-ram`
/// - `--convert-to-singletape`
/// - `--print-computer`
/// - `--print-number`
/// - `--print-nth-tm=<i128>`
/// - `--help`
/// - `--version`
/// - `--max-steps=<usize>`
/// - `--input=<String>`
/// - `--file=<String>`
/// - `--status`
/// - `--print-encoding`
/// - `--verbose=<i32>`
///
/// # Note
///
/// In test mode, command-line arguments are mocked for testing purposes.
pub fn get_options() -> Options {
    let mut convert_to_tm = false;
    let mut convert_to_ram = false;
    let mut convert_to_singletape = false;
    let mut print_computer = false;
    let mut print_nth_tm: i128 = -1;
    let mut print_number = false;
    let mut help = false;
    let mut version = false;
    let mut max_steps = 1000;
    let mut input = String::new();
    let mut file = String::new();
    let mut status = false;
    let mut print_encoding = false;
    let mut verbose = 1;

    #[cfg(test)]
    let args = tests::ARGS
        .with(|args| args.borrow().clone())
        .into_iter()
        .skip(1);
    #[cfg(not(test))]
    let args = std::env::args().skip(1);
    for arg in args {
        if arg.starts_with("--input=") {
            input = arg.strip_prefix("--input=").unwrap_or("").to_string();
        } else if arg.starts_with("--file=") {
            file = arg.strip_prefix("--file=").unwrap_or("").to_string();
            if file.starts_with('"') && file.ends_with('"') {
                file = file[1..file.len() - 1].to_string();
            }
        } else if arg.starts_with("--print-nth-tm=") {
            if let Ok(value) = arg.strip_prefix("--print-nth-tm=").unwrap_or("-1").parse() {
                print_nth_tm = value;
            }
        } else if arg.starts_with("--max-steps=") {
            if let Ok(value) = arg.strip_prefix("--max-steps=").unwrap_or("1000").parse() {
                max_steps = value;
            }
        } else if arg.starts_with("--verbose=") {
            if let Ok(value) = arg.strip_prefix("--verbose=").unwrap_or("1").parse() {
                verbose = value;
            }
        } else {
            match arg.as_str() {
                "--convert-to-tm" => convert_to_tm = true,
                "--convert-to-ram" => convert_to_ram = true,
                "--convert-to-singletape" => convert_to_singletape = true,
                "--print-computer" => print_computer = true,
                "--print-number" => print_number = true,
                "--help" => help = true,
                "--version" => version = true,
                "--status" => status = true,
                "--print-encoding" => print_encoding = true,
                _ => {
                    file = arg.clone();
                    if file.starts_with('"') && file.ends_with('"') {
                        file = file[1..file.len() - 1].to_string();
                    }
                }
            }
        }
    }

    Options {
        print_computer,
        print_number,
        print_nth_tm,
        convert_to_tm,
        convert_to_ram,
        convert_to_singletape,
        help,
        version,
        max_steps,
        input,
        file,
        status,
        print_encoding,
        verbose,
    }
}

#[cfg(test)]
mod tests {
    use std::cell::RefCell;
    thread_local! {
        pub static ARGS: RefCell<Vec<String>> = const { RefCell::new(Vec::new()) };
    }
    use super::*;

    #[test]
    fn test_command_line_options() {
        ARGS.with(|args| {
            *args.borrow_mut() = vec![
                "program".to_string(),
                "--convert-to-tm".to_string(),
                "--input=test_input".to_string(),
                "--file=test.txt".to_string(),
                "--max-steps=500".to_string(),
                "--verbose=2".to_string(),
            ];
        });

        let options = get_options();
        assert!(options.convert_to_tm);
        assert_eq!(options.input, "test_input");
        assert_eq!(options.file, "test.txt");
        assert_eq!(options.max_steps, 500);
        assert_eq!(options.verbose, 2);
    }

    #[test]
    fn test_default_options() {
        ARGS.with(|args| {
            *args.borrow_mut() = Vec::new();
        });

        let options = get_options();
        assert!(!options.convert_to_tm);
        assert_eq!(options.max_steps, 1000);
        assert_eq!(options.verbose, 1);
        assert_eq!(options.input, "");
        assert_eq!(options.file, "");
    }

    #[test]
    fn test_flag_options() {
        ARGS.with(|args| {
            *args.borrow_mut() = vec![
                "program".to_string(),
                "--print-computer".to_string(),
                "--status".to_string(),
                "--print-encoding".to_string(),
            ];
        });

        let options = get_options();
        assert!(options.print_computer);
        assert!(options.status);
        assert!(options.print_encoding);
    }

    #[test]
    fn test_file_option() {
        ARGS.with(|args| {
            *args.borrow_mut() = vec![
                "program".to_string(),
                "--file=\"prova.file\"".to_string(),
            ];
        });

        let options = get_options();
        assert_eq!(options.file, "prova.file");
    }

    #[test]
    fn test_nth_machine() {
        ARGS.with(|args| {
            *args.borrow_mut() = vec![
                "program".to_string(),
                "--print-nth-tm=10".to_string(),
            ];
        });

        let options = get_options();
        assert_eq!(options.print_nth_tm, 10);
    }
    #[test]
    fn test_all_flags() {
        ARGS.with(|args| {
            *args.borrow_mut() = vec![
                "program".to_string(),
                "--convert-to-tm".to_string(),
                "--convert-to-ram".to_string(),
                "--convert-to-singletape".to_string(),
                "--print-computer".to_string(),
                "--print-number".to_string(),
                "--help".to_string(),
                "--version".to_string(),
                "--status".to_string(),
                "--print-encoding".to_string(),
            ];
        });

        let options = get_options();
        assert!(options.convert_to_tm);
        assert!(options.convert_to_ram);
        assert!(options.convert_to_singletape);
        assert!(options.print_computer);
        assert!(options.print_number);
        assert!(options.help);
        assert!(options.version);
        assert!(options.status);
        assert!(options.print_encoding);
    }

    #[test]
    fn test_random_string() {
        ARGS.with(|args| {
            *args.borrow_mut() = vec![
                "program".to_string(),
                "\"testfile.tm\"".to_string(),
            ];
        });

        let options = get_options();
        assert_eq!(options.file, "testfile.tm");
    }
}
