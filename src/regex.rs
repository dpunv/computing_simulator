//! # Regex Module
//!
//! This module provides functionality for parsing regular expressions and converting them into finite state automata (FSA).
//! It includes a recursive descent parser for building a syntax tree representation of a regular expression and a function
//! to convert this syntax tree into a Turing Machine representation of an FSA.
//!
//! ## Features
//!
//! - **Regex Parsing**: Supports parsing of regular expressions with operations such as concatenation, alternation (`|`),
//!   Kleene star (`*`), Kleene plus (`+`), and optional (`?`).
//! - **Regex Syntax Tree**: Represents a regular expression as a tree structure using the `Regex` struct and `Operation` enum.
//! - **Regex to FSA Conversion**: Converts a parsed regular expression into a finite state automaton represented as a Turing Machine.
//!
//! ## Supported Operations
//!
//! - **Concatenation**: Combines two expressions sequentially (e.g., `ab`).
//! - **Alternation**: Matches either of two expressions (e.g., `a|b`).
//! - **Kleene Star**: Matches zero or more repetitions of an expression (e.g., `a*`).
//! - **Kleene Plus**: Matches one or more repetitions of an expression (e.g., `a+`).
//! - **Optional**: Matches zero or one occurrence of an expression (e.g., `a?`).
//! - **Symbols**: Matches individual characters or escaped characters.
//!
//! ## Public API
//!
//! - `build_regex_tree(input: &str) -> Result<Regex, String>`: Parses a regular expression string and constructs a syntax tree.
//! - `regex_to_fsa(regex: &Regex) -> Result<turing_machine::TuringMachine, String>`: Converts a `Regex` syntax tree into a Turing Machine representation of an FSA.
//!
//! ## Internal Parsing Functions
//!
//! - `parse_regex(chars: &mut Peekable<Chars>)`: Parses alternation (`|`) operations.
//! - `parse_concat(chars: &mut Peekable<Chars>)`: Parses concatenation operations.
//! - `parse_unary(chars: &mut Peekable<Chars>)`: Parses unary operations like `*`, `+`, and `?`.
//! - `parse_primary(chars: &mut Peekable<Chars>)`: Parses primary expressions such as symbols and grouped expressions.
//!
//! ## Testing
//!
//! The module includes a comprehensive set of unit tests to verify the correctness of regex parsing and FSA conversion.
//! These tests cover various scenarios, including simple regexes, nested expressions, invalid inputs, and escaped characters.
//!
//!
//! ## Author
//!
//! - dp
//!
//! # License
//!
//! This project is licensed under the MIT License. See the LICENSE file for details.

use crate::turing_machine;
use std::iter::Peekable;
use std::str::Chars;

/// Represents the various operations that can be performed in a regular expression.
///
/// This enum is used to define the structure of a regular expression syntax tree.
/// Each variant corresponds to a specific operation or symbol in a regular expression.
///
/// # Variants
///
/// - `Concat`: Represents the concatenation of two expressions (e.g., `ab`).
/// - `Or`: Represents alternation between two expressions (e.g., `a|b`).
/// - `KleeneStar`: Represents zero or more repetitions of an expression (e.g., `a*`).
/// - `KleneePlus`: Represents one or more repetitions of an expression (e.g., `a+`).
/// - `Optional`: Represents zero or one occurrence of an expression (e.g., `a?`).
/// - `Symbol`: Represents an individual character or escaped character in the expression.
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

/// Represents a regular expression as a syntax tree.
///
/// The `Regex` struct is used to define the structure of a regular expression
/// in a tree-like format. Each node in the tree corresponds to an operation
/// or symbol in the regular expression. This struct supports various operations
/// such as concatenation, alternation, Kleene star, Kleene plus, optional, and symbols.
///
/// # Fields
///
/// - `operation`: The operation represented by this node in the syntax tree.
///   It is defined by the `Operation` enum and can represent operations like
///   concatenation, alternation, or unary operations.
/// - `left`: An optional boxed `Regex` representing the left child of the current node.
///   This is used for binary operations like concatenation and alternation.
/// - `right`: An optional boxed `Regex` representing the right child of the current node.
///   This is used for binary operations like concatenation and alternation.
/// - `symbol`: A string representing the symbol for this node. This is used
///   when the node represents a single character or an escaped character.
///
/// # Notes
///
/// - The `symbol` field is only relevant for nodes with the `Operation::Symbol` operation.
/// - For binary operations like `Concat` and `Or`, both `left` and `right` fields must be `Some`.
/// - For unary operations like `KleeneStar`, only the `left` field is used, and the `right` field is `None`.
///
/// This struct is primarily used in conjunction with the `build_regex_tree` function to parse
/// regular expressions into a syntax tree representation.
#[derive(Clone)]
pub struct Regex {
    pub operation: Operation,
    pub left: Option<Box<Regex>>,
    pub right: Option<Box<Regex>>,
    pub symbol: String,
}

impl Regex {
    /// Creates a new `Regex` instance representing a single symbol.
    ///
    /// # Arguments
    ///
    /// * `symbol` - A string slice that holds the symbol for the regex.
    ///
    /// # Returns
    ///
    /// A `Regex` variant with the specified symbol, and no left or right sub-expressions.
    pub fn symbol(symbol: &str) -> Self {
        Regex {
            operation: Operation::Symbol,
            left: None,
            right: None,
            symbol: symbol.to_string(),
        }
    }

    /// Creates a new `Regex` instance with the specified operation and optional left and right sub-expressions.
    ///
    /// # Arguments
    ///
    /// * `op` - The `Operation` to be associated with this `Regex`.
    /// * `left` - An optional boxed `Regex` representing the left sub-expression.
    /// * `right` - An optional boxed `Regex` representing the right sub-expression.
    ///
    /// # Returns
    ///
    /// A `Regex` variant with the given operation and sub-expressions, and an empty symbol string.
    pub fn operation(op: Operation, left: Option<Box<Regex>>, right: Option<Box<Regex>>) -> Self {
        Regex {
            operation: op,
            left,
            right,
            symbol: String::new(),
        }
    }
}

/// Parses a regular expression string and constructs its corresponding syntax tree representation.
///
/// This function takes a string slice representing a regular expression and parses it into a
/// `Regex` syntax tree using a recursive descent parser. The resulting syntax tree can be used
/// for further processing, such as conversion to a finite state automaton (FSA).
///
/// # Arguments
///
/// * `input` - A string slice containing the regular expression to be parsed.
///
/// # Returns
///
/// * `Ok(Regex)` - If the parsing is successful, returns the root node of the constructed syntax tree.
/// * `Err(String)` - If the input is invalid or contains a syntax error, returns an error message describing the issue.
///
/// # Errors
///
/// Returns an error if the input string contains invalid regular expression syntax, such as
/// unmatched parentheses or misplaced operators.
///
/// # See Also
///
/// - [`Regex`] struct for the syntax tree representation.
/// - [`Operation`] enum for supported regex operations.
pub fn build_regex_tree(input: &str) -> Result<Regex, String> {
    let mut chars = input.chars().peekable();
    parse_regex(&mut chars)
}

/// Converts a parsed regular expression syntax tree into a finite state automaton (FSA)
/// represented as a Turing Machine.
///
/// This function takes a reference to a `Regex` syntax tree and constructs a corresponding
/// Turing Machine that recognizes the same language as the regular expression. The resulting
/// Turing Machine uses the input alphabet derived from the symbols in the regex and creates
/// states and transitions according to the structure of the regex tree.
///
/// The conversion supports the following regex operations:
/// - Concatenation
/// - Alternation (`|`)
/// - Kleene star (`*`)
/// - Kleene plus (`+`)
/// - Optional (`?`)
/// - Symbols (including escaped characters)
///
/// # Arguments
///
/// * `regex` - A reference to a `Regex` syntax tree representing the regular expression to convert.
///
/// # Returns
///
/// * `Ok(turing_machine::TuringMachine)` - If the conversion is successful, returns a Turing Machine
///   that acts as a finite state automaton for the given regex.
/// * `Err(String)` - If the regex tree is malformed or contains unsupported constructs, returns an error message.
///
/// # Errors
///
/// Returns an error if the regex tree is invalid or if required operands for operations are missing.
///
/// # See Also
///
/// - [`build_regex_tree`] for parsing a regex string into a syntax tree.
/// - [`turing_machine::TuringMachine`] for the FSA representation.
fn parse_regex(chars: &mut Peekable<Chars>) -> Result<Regex, String> {
    let mut left = parse_concat(chars)?;

    while let Some('|') = chars.peek() {
        chars.next();
        let right = parse_concat(chars)?;
        left = Regex::operation(Operation::Or, Some(Box::new(left)), Some(Box::new(right)));
    }

    Ok(left)
}

/// Parses a regular expression from a stream of characters, handling alternation (`|`) operations.
///
/// This function is the entry point for the recursive descent parser. It attempts to parse a regular expression
/// by first parsing a concatenation, and then repeatedly checking for the alternation operator (`|`). If an
/// alternation is found, it recursively parses the right-hand side and constructs an `Or` operation node in the
/// syntax tree. This process continues until no more alternation operators are found.
///
/// # Arguments
///
/// * `chars` - A mutable reference to a `Peekable<Chars>` iterator over the input regular expression string.
///
/// # Returns
///
/// * `Ok(Regex)` - The root node of the parsed regular expression syntax tree if parsing succeeds.
/// * `Err(String)` - An error message if the input is invalid or a syntax error is encountered.
///
/// # Errors
///
/// Returns an error if the input contains invalid syntax or if a required operand is missing for an alternation.
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

/// Parses a primary expression from a stream of characters, handling symbols, escaped characters,
/// and grouped sub-expressions (parentheses).
///
/// This function is responsible for parsing the most basic units of a regular expression:
/// - Single symbols (alphanumeric or other allowed characters)
/// - Escaped characters (e.g., `\*`, `\+`)
/// - Grouped expressions within parentheses (e.g., `(a|b)`)
///
/// # Arguments
///
/// * `chars` - A mutable reference to a `Peekable<Chars>` iterator over the input regular expression string.
///
/// # Returns
///
/// * `Ok(Regex)` - The parsed primary expression as a `Regex` node if successful.
/// * `Err(String)` - An error message if the input is invalid or a syntax error is encountered.
///
/// # Errors
///
/// Returns an error if:
/// - There is an unmatched parenthesis.
/// - An escape character is not followed by a valid character.
/// - An unexpected character is encountered.
///
/// # See Also
/// - [`parse_concat`] for parsing concatenation expressions.
///
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

/// Parses a concatenation expression from a stream of characters in a regular expression.
///
/// This function attempts to parse a sequence of unary expressions that are implicitly concatenated,
/// such as `ab` or `a(bc)`. It repeatedly parses unary expressions and combines them into a
/// concatenation operation node in the syntax tree until it encounters a character that cannot
/// start a new concatenated expression (such as `|`, `)`, or the end of input).
///
/// # Arguments
///
/// * `chars` - A mutable reference to a `Peekable<Chars>` iterator over the input regular expression string.
///
/// # Returns
///
/// * `Ok(Regex)` - The root node of the parsed concatenation expression as a `Regex` syntax tree.
/// * `Err(String)` - An error message if the input is invalid or a syntax error is encountered.
///
/// # Errors
///
/// Returns an error if a unary expression cannot be parsed or if the input contains invalid syntax.
///
/// # See Also
///
/// - [`parse_unary`] for parsing unary expressions.
///
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

/// Converts a parsed regular expression syntax tree into a finite state automaton (FSA)
/// represented as a Turing Machine.
///
/// This function takes a reference to a `Regex` syntax tree and constructs a corresponding
/// Turing Machine that recognizes the same language as the regular expression.
/// The resulting Turing Machine uses the input alphabet derived from the symbols in the regex
/// and creates states and transitions according to the structure of the regex tree.
///
/// The conversion supports the following regex operations:
/// - Concatenation
/// - Alternation (`|`)
/// - Kleene star (`*`)
/// - Kleene plus (`+`)
/// - Optional (`?`)
/// - Symbols (including escaped characters)
///
/// # Arguments
///
/// * `regex` - A reference to a `Regex` syntax tree representing the regular expression to convert.
///
/// # Returns
///
/// * `Ok(turing_machine::TuringMachine)` - If the conversion is successful, returns a Turing Machine that acts as a finite state automaton for the given regex.
/// * `Err(String)` - If the regex tree is malformed or contains unsupported constructs, returns an error message.
///
/// # Errors
///
/// Returns an error if the regex tree is invalid or if required operands for operations are missing.
/// # See Also
/// - [`build_regex_tree`] for parsing a regex string into a syntax tree.
/// - [`turing_machine::TuringMachine`] for the FSA representation.
///
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

/// Recursively builds a finite state automaton (FSA) from a regular expression syntax tree.
///
/// This function takes a mutable reference to a `TuringMachine` and a `Regex` syntax tree,
/// and constructs the corresponding FSA by adding states and transitions based on the
/// operations defined in the regex tree.
///
/// # Arguments
///
/// * `fsa` - A mutable reference to a `TuringMachine` instance that will be modified to represent the FSA.
/// * `regex` - A reference to a `Regex` syntax tree representing the regular expression to convert.
///
/// # Returns
///
/// * `Ok((String, String))` - If the conversion is successful, returns a tuple containing the start and end states of the FSA.
/// * `Err(String)` - If the regex tree is malformed or contains unsupported constructs, returns an error message.
///
/// # Errors
///
/// Returns an error if the regex tree is invalid or if required operands for operations are missing.
///
/// # See Also
///
/// - [`build_regex_tree`] for parsing a regex string into a syntax tree.
/// - [`turing_machine::TuringMachine`] for the FSA representation.
///
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
