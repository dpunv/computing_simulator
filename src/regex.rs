// file: regex.rs
// Project: Computing Simulator
// author: dp

use crate::turing_machine;
use std::iter::Peekable;
use std::str::Chars;

#[derive(Clone, Debug)]
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

pub fn regex_to_fsa(regex: &Regex) -> Result<turing_machine::TuringMachine, String> {
    let mut fsa = turing_machine::TuringMachine::new();
    fsa.blank_symbol = " ".to_string();

    let (start, end) = build_fsa(&mut fsa, regex)?;

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
    fsa.halt_state = final_state.clone();
    fsa.accept_state = final_state;

    Ok(fsa)
}

fn build_fsa(
    fsa: &mut turing_machine::TuringMachine,
    regex: &Regex,
) -> Result<(String, String), String> {
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
            Ok((start, end))
        }

        Operation::Concat => {
            let left = regex.left.as_ref().ok_or("Concat must have left operand")?;
            let right = regex
                .right
                .as_ref()
                .ok_or("Concat must have right operand")?;

            let (start_left, end_left) = build_fsa(fsa, left)?;
            let (start_right, end_right) = build_fsa(fsa, right)?;

            fsa.add_transition(
                end_left,
                vec![" ".to_string()],
                start_right,
                vec![" ".to_string()],
                vec![turing_machine::Direction::Stay],
            );

            Ok((start_left, end_right))
        }

        Operation::Or => {
            let left = regex.left.as_ref().ok_or("Or must have left operand")?;
            let right = regex.right.as_ref().ok_or("Or must have right operand")?;

            let start = fsa.add_state();
            let end = fsa.add_state();

            let (start_left, end_left) = build_fsa(fsa, left)?;
            let (start_right, end_right) = build_fsa(fsa, right)?;

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

            Ok((start, end))
        }

        Operation::KleeneStar => {
            let operand = regex
                .left
                .as_ref()
                .ok_or("KleeneStar must have an operand")?;

            let start = fsa.add_state();
            let end = fsa.add_state();

            let (start_operand, end_operand) = build_fsa(fsa, operand)?;

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

            Ok((start, end))
        }

        Operation::KleneePlus => {
            let operand = regex
                .left
                .as_ref()
                .ok_or("KleenePlus must have an operand")?;

            let start = fsa.add_state();
            let end = fsa.add_state();

            let (start_operand, end_operand) = build_fsa(fsa, operand)?;

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

            Ok((start, end))
        }

        Operation::Optional => {
            let operand = regex.left.as_ref().ok_or("Optional must have an operand")?;

            let start = fsa.add_state();
            let end = fsa.add_state();

            let (start_operand, end_operand) = build_fsa(fsa, operand)?;

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

            Ok((start, end))
        }
    }
}
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_simple_regex() {
        let result = build_regex_tree("abc").unwrap();
        assert_eq!(result.operation, Operation::Concat);
    }

    #[test]
    fn test_alternation() {
        let result = build_regex_tree("a|b").unwrap();
        assert_eq!(result.operation, Operation::Or);
    }

    #[test]
    fn test_kleene_star() {
        let result = build_regex_tree("a*").unwrap();
        assert_eq!(result.operation, Operation::KleeneStar);
    }

    #[test]
    fn test_kleene_plus() {
        let result = build_regex_tree("a+").unwrap();
        assert_eq!(result.operation, Operation::KleneePlus);
    }

    #[test]
    fn test_optional() {
        let result = build_regex_tree("a?").unwrap();
        assert_eq!(result.operation, Operation::Optional);
    }

    #[test]
    fn test_nested_expressions() {
        let result = build_regex_tree("(a|b)*c").unwrap();
        assert_eq!(result.operation, Operation::Concat);
    }

    #[test]
    fn test_invalid_regex() {
        assert!(build_regex_tree(")").is_err());
        assert!(build_regex_tree("(").is_err());
        assert!(build_regex_tree("*").is_err());
    }

    #[test]
    fn test_escaped_characters() {
        let result = build_regex_tree("\\*").unwrap();
        assert_eq!(result.operation, Operation::Symbol);
        assert_eq!(result.symbol, "\\*");
    }
    #[test]
    fn test_complex_regex() {
        let result = build_regex_tree("(a|b)+(c|d)*").unwrap();
        assert_eq!(result.operation, Operation::Concat);
    }

    #[test]
    fn test_multiple_alternations() {
        let result = build_regex_tree("a|b|c|d").unwrap();
        assert_eq!(result.operation, Operation::Or);
    }

    #[test]
    fn test_nested_parentheses() {
        let result = build_regex_tree("((a|b)|c)").unwrap();
        assert_eq!(result.operation, Operation::Or);
    }

    #[test]
    fn test_regex_to_fsa_simple() {
        let regex = build_regex_tree("ab").unwrap();
        let fsa = regex_to_fsa(&regex).unwrap();
        assert!(fsa.input_alphabet.contains(&"a".to_string()));
        assert!(fsa.input_alphabet.contains(&"b".to_string()));
    }

    #[test]
    fn test_regex_to_fsa_alternation() {
        let regex = build_regex_tree("a|b").unwrap();
        let fsa = regex_to_fsa(&regex).unwrap();
        assert!(fsa.input_alphabet.contains(&"a".to_string()));
        assert!(fsa.input_alphabet.contains(&"b".to_string()));
    }

    #[test]
    fn test_regex_to_fsa_kleene_star() {
        let regex = build_regex_tree("a*").unwrap();
        let fsa = regex_to_fsa(&regex).unwrap();
        assert!(fsa.input_alphabet.contains(&"a".to_string()));
    }
}