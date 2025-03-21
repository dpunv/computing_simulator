// file: regex.rs
// Project: Computing Simulator
// author: dp

use crate::turing_machine;
use std::iter::Peekable;
use std::str::Chars;

#[derive(Clone)]
pub enum Operation {
    Concat,
    Or,
    KleeneStar,
    KleneePlus,
    Optional,
    Symbol,
}

impl PartialEq for Operation {
    fn eq(&self, other: &Self) -> bool {
        matches!(
            (self, other),
            (Operation::Concat, Operation::Concat)
                | (Operation::Or, Operation::Or)
                | (Operation::KleeneStar, Operation::KleeneStar)
                | (Operation::KleneePlus, Operation::KleneePlus)
                | (Operation::Optional, Operation::Optional)
                | (Operation::Symbol, Operation::Symbol)
        )
    }
}

#[derive(Clone)]
pub struct Regex {
    pub operation: Operation,
    pub left: Option<Box<Regex>>,
    pub right: Option<Box<Regex>>,
    pub symbol: String,
}

impl Regex {
    pub fn symbol(symbol: &str) -> Self {
        Regex {
            operation: Operation::Symbol,
            left: None,
            right: None,
            symbol: symbol.to_string(),
        }
    }

    pub fn operation(op: Operation, left: Option<Box<Regex>>, right: Option<Box<Regex>>) -> Self {
        Regex {
            operation: op,
            left,
            right,
            symbol: String::new(),
        }
    }
}

pub fn build_regex_tree(input: &str) -> Result<Regex, String> {
    let mut chars = input.chars().peekable();
    parse_regex(&mut chars)
}

fn parse_regex(chars: &mut Peekable<Chars>) -> Result<Regex, String> {
    let mut left = parse_concat(chars)?;

    while let Some('|') = chars.peek() {
        chars.next();
        let right = parse_concat(chars)?;
        left = Regex::operation(Operation::Or, Some(Box::new(left)), Some(Box::new(right)));
    }

    Ok(left)
}

fn parse_concat(chars: &mut Peekable<Chars>) -> Result<Regex, String> {
    let mut left = parse_unary(chars)?;

    while let Some(&ch) = chars.peek() {
        if (ch == '(' || ch.is_alphanumeric() || ch == '\\') || (ch != ')' && ch != '|') {
            let right = parse_unary(chars)?;
            left = Regex::operation(
                Operation::Concat,
                Some(Box::new(left)),
                Some(Box::new(right)),
            );
        } else {
            break;
        }
    }

    Ok(left)
}

fn parse_unary(chars: &mut Peekable<Chars>) -> Result<Regex, String> {
    let mut expr = parse_primary(chars)?;

    while let Some(&ch) = chars.peek() {
        match ch {
            '*' => {
                chars.next();
                expr = Regex::operation(Operation::KleeneStar, Some(Box::new(expr)), None);
            }
            '+' => {
                chars.next();
                expr = Regex::operation(Operation::KleneePlus, Some(Box::new(expr)), None);
            }
            '?' => {
                chars.next();
                expr = Regex::operation(Operation::Optional, Some(Box::new(expr)), None);
            }
            _ => break,
        }
    }

    Ok(expr)
}

fn parse_primary(chars: &mut Peekable<Chars>) -> Result<Regex, String> {
    match chars.peek() {
        Some('(') => {
            chars.next();
            let expr = parse_regex(chars)?;

            if Some(')') != chars.next() {
                return Err("Expected closing parenthesis".to_string());
            }

            Ok(expr)
        }
        Some('\\') => {
            chars.next();
            if let Some(ch) = chars.next() {
                Ok(Regex::symbol(&format!("\\{}", ch)))
            } else {
                Err("Unexpected end of pattern after escape character".to_string())
            }
        }
        Some(&ch) if ch != '*' && ch != '+' && ch != '?' && ch != '|' && ch != ')' => {
            chars.next();
            Ok(Regex::symbol(&ch.to_string()))
        }
        Some(ch) => Err(format!("Unexpected character: {}", ch)),
        None => Err("Unexpected end of pattern".to_string()),
    }
}

pub fn regex_to_fsa(regex: &Regex) -> turing_machine::TuringMachine {
    let mut fsa = turing_machine::TuringMachine::new();
    fsa.blank_symbol = " ".to_string();

    let (start, end) = build_fsa(&mut fsa, regex);

    //fsa.end_on_final_state = true;
    fsa.tape_alphabet = fsa.input_alphabet.clone();
    fsa.tape_alphabet.push(fsa.blank_symbol.clone());

    for transition in fsa.transitions.clone().iter() {
        if transition.symbols[0] == fsa.blank_symbol {
            for symbol in fsa.input_alphabet.clone().iter() {
                fsa.add_transition(
                    transition.state.clone(),
                    vec![symbol.clone()],
                    transition.new_state.clone(),
                    vec![symbol.clone()],
                    transition.directions.clone(),
                );
            }
        }
    }

    let begin = fsa.add_state();
    fsa.initial_state = begin.clone();
    fsa.add_transition(
        begin.clone(),
        vec![" ".to_string()],
        start.clone(),
        vec![" ".to_string()],
        vec![turing_machine::Direction::Right],
    );
    let final_state = fsa.add_state();
    fsa.add_transition(
        end.clone(),
        vec![" ".to_string()],
        final_state.clone(),
        vec![" ".to_string()],
        vec![turing_machine::Direction::Stay],
    );
    fsa.final_states.push(final_state.clone());
    fsa.accept_state = final_state;

    fsa
}

fn build_fsa(fsa: &mut turing_machine::TuringMachine, regex: &Regex) -> (String, String) {
    match regex.operation {
        Operation::Symbol => {
            let start = fsa.add_state();
            let end = fsa.add_state();
            fsa.add_transition(
                start.clone(),
                vec![regex.symbol.to_string()],
                end.clone(),
                vec![" ".to_string()],
                vec![turing_machine::Direction::Right],
            );
            if !fsa.input_alphabet.contains(&regex.symbol) {
                fsa.input_alphabet.push(regex.symbol.clone());
            }
            (start, end)
        }

        Operation::Concat => {
            let left = regex.left.as_ref().expect("Concat must have left operand");
            let right = regex
                .right
                .as_ref()
                .expect("Concat must have right operand");

            let (start_left, end_left) = build_fsa(fsa, left);
            let (start_right, end_right) = build_fsa(fsa, right);

            fsa.add_transition(
                end_left,
                vec![" ".to_string()],
                start_right,
                vec![" ".to_string()],
                vec![turing_machine::Direction::Stay],
            );

            (start_left, end_right)
        }

        Operation::Or => {
            let left = regex.left.as_ref().expect("Or must have left operand");
            let right = regex.right.as_ref().expect("Or must have right operand");

            let start = fsa.add_state();
            let end = fsa.add_state();

            let (start_left, end_left) = build_fsa(fsa, left);
            let (start_right, end_right) = build_fsa(fsa, right);

            fsa.add_transition(
                start.clone(),
                vec![" ".to_string()],
                start_left.clone(),
                vec![" ".to_string()],
                vec![turing_machine::Direction::Stay],
            );
            fsa.add_transition(
                start.clone(),
                vec![" ".to_string()],
                start_right.clone(),
                vec![" ".to_string()],
                vec![turing_machine::Direction::Stay],
            );

            fsa.add_transition(
                end_left.clone(),
                vec![" ".to_string()],
                end.clone(),
                vec![" ".to_string()],
                vec![turing_machine::Direction::Stay],
            );
            fsa.add_transition(
                end_right.clone(),
                vec![" ".to_string()],
                end.clone(),
                vec![" ".to_string()],
                vec![turing_machine::Direction::Stay],
            );

            (start, end)
        }

        Operation::KleeneStar => {
            let operand = regex
                .left
                .as_ref()
                .expect("KleeneStar must have an operand");

            let start = fsa.add_state();
            let end = fsa.add_state();

            let (start_operand, end_operand) = build_fsa(fsa, operand);

            fsa.add_transition(
                start.clone(),
                vec![" ".to_string()],
                end.clone(),
                vec![" ".to_string()],
                vec![turing_machine::Direction::Stay],
            );

            fsa.add_transition(
                start.clone(),
                vec![" ".to_string()],
                start_operand.clone(),
                vec![" ".to_string()],
                vec![turing_machine::Direction::Stay],
            );

            fsa.add_transition(
                end_operand.clone(),
                vec![" ".to_string()],
                start_operand.clone(),
                vec![" ".to_string()],
                vec![turing_machine::Direction::Stay],
            );

            fsa.add_transition(
                end_operand.clone(),
                vec![" ".to_string()],
                end.clone(),
                vec![" ".to_string()],
                vec![turing_machine::Direction::Stay],
            );

            (start, end)
        }

        Operation::KleneePlus => {
            let operand = regex
                .left
                .as_ref()
                .expect("KleenePlus must have an operand");

            let start = fsa.add_state();
            let end = fsa.add_state();

            let (start_operand, end_operand) = build_fsa(fsa, operand);

            fsa.add_transition(
                start.clone(),
                vec![" ".to_string()],
                start_operand.clone(),
                vec![" ".to_string()],
                vec![turing_machine::Direction::Stay],
            );

            fsa.add_transition(
                end_operand.clone(),
                vec![" ".to_string()],
                start_operand.clone(),
                vec![" ".to_string()],
                vec![turing_machine::Direction::Stay],
            );

            fsa.add_transition(
                end_operand.clone(),
                vec![" ".to_string()],
                end.clone(),
                vec![" ".to_string()],
                vec![turing_machine::Direction::Stay],
            );

            (start, end)
        }

        Operation::Optional => {
            let operand = regex.left.as_ref().expect("Optional must have an operand");

            let start = fsa.add_state();
            let end = fsa.add_state();

            let (start_operand, end_operand) = build_fsa(fsa, operand);

            fsa.add_transition(
                start.clone(),
                vec![" ".to_string()],
                end.clone(),
                vec![" ".to_string()],
                vec![turing_machine::Direction::Stay],
            );

            fsa.add_transition(
                start.clone(),
                vec![" ".to_string()],
                start_operand.clone(),
                vec![" ".to_string()],
                vec![turing_machine::Direction::Stay],
            );

            fsa.add_transition(
                end_operand.clone(),
                vec![" ".to_string()],
                end.clone(),
                vec![" ".to_string()],
                vec![turing_machine::Direction::Stay],
            );

            (start, end)
        }
    }
}
