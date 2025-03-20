// File: main.rs
// Project: Computing Simulator
// author: dp

mod turing_machine;
mod cli;
mod file_handler;
mod options;
mod utils;
mod regex;
mod ram_machine;

fn main() {
    cli::main_cli();
}
