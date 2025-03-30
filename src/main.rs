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
