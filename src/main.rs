//! # Computing Simulator - Main Entry Point
//!
//! This is the main entry point for the Computing Simulator project.
//! It imports all the core modules and launches the command-line interface (CLI).
//!
//! ## Modules
//! - `cli`: Handles the command-line interface and user interaction.
//! - `computer`: Core computer simulation logic.
//! - `file_handler`: Utilities for file input/output.
//! - `lambda`: Lambda calculus interpreter and related structures.
//! - `options`: Command-line options and configuration parsing.
//! - `ram_machine`: RAM machine simulation.
//! - `regex`: Regular expression utilities and simulation.
//! - `turing_machine`: Turing machine simulation.
//! - `utils`: Miscellaneous utility functions.
//!
//! ## Usage
//! Run the binary to start the CLI for the Computing Simulator.
//!
//! ## Testing
//! This file includes integration tests to ensure that all modules are properly linked
//! and that the CLI can be invoked without panicking.
//!
//! ## Author
//!
//! - dp
//! 
//! # License
//! 
//! This project is licensed under the MIT License. See the LICENSE file for details.

mod cli;
mod computer;
mod file_handler;
mod lambda;
mod options;
mod ram_machine;
mod regex;
mod turing_machine;
mod utils;


/// The main function serves as the entry point for the application.
/// It initializes the command-line interface (CLI) for the Computing Simulator.
/// The CLI allows users to interact with the simulator, providing options
/// for different computing models and configurations.
/// The function is designed to be run in a terminal environment.
/// 
/// # Example Usage
/// ```bash
/// cargo run --bin computing_simulator
/// ```
/// This will start the CLI, allowing users to input commands and options.
/// 
fn main() {
    cli::main_cli();
}
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_main_cli_integration() {
        assert!(std::panic::catch_unwind(|| {
            let stdout = std::io::stdout();
            let _handle = stdout.lock();

            cli::main_cli();
        })
        .is_ok());
    }

    #[test]
    fn test_module_imports() {
        assert!(std::panic::catch_unwind(|| {
            let _cli = cli::main_cli;
            let _computer = computer::Computer::new();
            let _lambda = lambda::Lambda {
                expr: lambda::LambdaExpr::Var("a".to_string()),
                references: Vec::new(),
                name: "test".to_string(),
                force_currying: false,
            };
            let _ram = ram_machine::RamMachine {
                instructions: Vec::new(),
                labels_map: std::collections::HashMap::new(),
                translation_map: std::collections::HashMap::new()
            };
            let _turing = turing_machine::TuringMachine::new();
        })
        .is_ok());
    }
}
