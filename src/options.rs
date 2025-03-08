// file: options.rs
// Project: Computing Simulator
// author: dp
// date: 2025-03-06

#[derive(Clone)]
pub struct Options {
    pub type_: String,
    pub from_encoding: bool,
    pub output_tape: bool,
    pub trimmed_tape: bool,
    pub steps: bool,
    pub state: bool,
    pub computation: bool,
    pub help: bool,
    pub version: bool,
    pub max_steps: i32,
    pub input: String,
    pub file: String,
    pub status: bool,
    pub print_encoding: bool,
}

pub fn get_options() -> Options {
    let mut type_ = "tm".to_string();
    let mut from_encoding = false;
    let mut output_tape = false;
    let mut trimmed_tape = false;
    let mut steps = false;
    let mut state = false;
    let mut computation = false;
    let mut help = false;
    let mut version = false;
    let mut max_steps = 1000;
    let mut input = String::new();
    let mut file = String::new();
    let mut status = false;
    let mut print_encoding = false;

    let mut default_out = true;

    let mut args = std::env::args().skip(1);
    while let Some(arg) = args.next() {
        if arg.starts_with("--input=") {
            input = arg.strip_prefix("--input=").unwrap_or("").to_string();
        } else if arg.starts_with("--file=") {
            file = arg.strip_prefix("--file=").unwrap_or("").to_string();
            if file.starts_with('"') && file.ends_with('"') {
                file = file[1..file.len() - 1].to_string();
            }
        } else if arg.starts_with("--max-steps=") {
            if let Ok(value) = arg.strip_prefix("--max-steps=").unwrap_or("1000").parse() {
                max_steps = value;
            }
        } else if arg.starts_with("--type=") {
            type_ = arg.strip_prefix("--type=").unwrap_or("turing").to_string();
            if type_ != "tm" && type_ != "fsm" && type_ != "pda" {
                type_ = "tm".to_string();
            }
        } else {
            match arg.as_str() {
                "--output-tape" => {
                    output_tape = true;
                    default_out = false;
                }
                "--trimmed-tape" => {
                    trimmed_tape = true;
                    default_out = false;
                }
                "--steps" => {
                    steps = true;
                    default_out = false;
                }
                "--state" => {
                    state = true;
                    default_out = false;
                }
                "--computation" => {
                    computation = true;
                    default_out = false;
                }
                "--help" => help = true,
                "--version" => version = true,
                "--from-encoding" => from_encoding = true,
                "--print-encoding" => print_encoding = true,
                "--status" => {
                    status = true;
                    default_out = false;
                }
                _ => {
                    file = arg;
                    input = args.next().unwrap();
                }
            }
        }
    }

    if default_out {
        trimmed_tape = true;
        steps = true;
        state = true;
    }

    Options {
        type_,
        from_encoding,
        output_tape,
        trimmed_tape,
        steps,
        state,
        computation,
        help,
        version,
        max_steps,
        input,
        file,
        status,
        print_encoding,
    }
}
