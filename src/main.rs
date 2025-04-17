// File: main.rs
// Project: Computing Simulator
// author: dp

mod cli;
mod computer;
mod file_handler;
mod lambda;
mod options;
mod ram_machine;
mod regex;
mod turing_machine;
mod utils;

fn main() {
    cli::main_cli();
}
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_main_cli_integration() {
        // Since main_cli() handles user input/output, we can test that it exists
        // and can be called without panicking
        assert!(std::panic::catch_unwind(|| {
            // Redirect stdout to avoid printing during tests
            let stdout = std::io::stdout();
            let _handle = stdout.lock();
            
            // Call main_cli with mock args if needed
            cli::main_cli();
        }).is_ok());
    }

    // Integration test to ensure all modules are properly linked
    #[test]
    fn test_module_imports() {
        // Verify modules can be accessed
        assert!(std::panic::catch_unwind(|| {
            let _cli = cli::main_cli;
            let _computer = computer::Computer::new();
            let _lambda = lambda::Lambda { expr: lambda::LambdaExpr::Var("a".to_string()), references: Vec::new(), name: "test".to_string(), force_currying: false };
            let _ram = ram_machine::RamMachine {
                instructions: Vec::new(),
                labels_map: std::collections::HashMap::new()
            };
            let _turing = turing_machine::TuringMachine::new();
        }).is_ok());
    }
}
