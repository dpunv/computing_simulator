// file: options.rs
// Project: Computing Simulator
// author: dp

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
    let args = tests::ARGS.with(|args| args.borrow().clone()).into_iter().skip(1);
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
                _ => {}
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
        pub static ARGS: RefCell<Vec<String>> = RefCell::new(Vec::new());
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
        assert_eq!(options.convert_to_tm, true);
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
        assert_eq!(options.convert_to_tm, false);
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
        assert_eq!(options.print_computer, true);
        assert_eq!(options.status, true);
        assert_eq!(options.print_encoding, true);
    }
}
