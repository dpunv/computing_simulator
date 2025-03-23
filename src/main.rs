// File: main.rs
// Project: Computing Simulator
// author: dp

mod cli;
mod file_handler;
mod options;
mod ram_machine;
mod regex;
mod turing_machine;
mod utils;
mod computer;

fn main() {
    cli::main_cli();
}
