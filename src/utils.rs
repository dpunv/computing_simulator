//! # Utilities Module
//!
//! This module provides a collection of utility functions for string manipulation,
//! binary conversions, and custom alphabet-based encoding. These functions are
//! designed to be reusable and cover a variety of common operations in computing
//! simulations and data processing.
//!
//! ## Functions
//!
//! - `input_string_to_vec`: Converts an input string into a vector of strings based on a provided input alphabet.
//! - `int2bin`: Converts an integer to its binary representation as a string, with optional zero-padding.
//! - `bin2int`: Converts a binary string to an integer, returning a `Result` to handle invalid inputs.
//! - `uint2str`: Converts an unsigned integer to a string representation using a custom alphabet.
//! - `bin2alphabet`: Converts a binary string to a string representation using a custom alphabet.
//! - `is_numeric`: Checks if a string contains only numeric characters.
//!
//! ## Error Handling
//!
//! Functions that involve conversions or custom alphabets return `Result` types
//! to handle errors gracefully. For instance, invalid binary strings or empty
//! alphabets will result in descriptive error messages.
//!
//! ## Testing
//!
//! The module includes a robust set of unit tests to ensure correctness and
//! reliability. These tests cover normal usage, edge cases, and invalid inputs
//! to verify the behavior of each function.
//!
//! ## Author
//!
//! - dp
//!
//! # License
//!
//! This project is licensed under the MIT License. See the LICENSE file for details.

/// Converts an input string into a vector of strings based on the provided input alphabet.
///
/// # Arguments
///
/// * `input_alphabet` - A vector of strings representing the valid symbols.
/// * `input` - The input string to be converted.
///
/// # Returns
///
/// A vector of strings where each element is a symbol from the input alphabet.
pub fn input_string_to_vec(input_alphabet: Vec<String>, input: String) -> Vec<String> {
    let mut vec = Vec::new();
    let mut current_symbol = String::new();
    for c in input.chars() {
        current_symbol.push(c);
        if input_alphabet.contains(&current_symbol) {
            vec.push(current_symbol.clone());
            current_symbol = String::new();
        }
    }
    vec
}

/// Converts an integer to its binary representation as a string, with optional zero-padding.
///
/// # Arguments
///
/// * `n` - The integer to convert.
/// * `bitnum` - The number of bits to pad the binary representation to. If 0, no padding is applied.
///
/// # Returns
///
/// A string representing the binary representation of the integer.
pub fn int2bin(n: i32, bitnum: usize) -> String {
    if bitnum > 0 {
        let s = format!("{:0>width$b}", n, width = bitnum);
        s[s.len() - bitnum..s.len()].to_string()
    } else {
        format!("{:b}", n)
    }
}

/// Converts a binary string to an integer.
///
/// # Arguments
///
/// * `s` - A string representing a binary number.
///
/// # Returns
///
/// A `Result` containing the integer value if successful, or an error message if the input is invalid.
pub fn bin2int(s: String) -> Result<i32, String> {
    if s.is_empty() || s.starts_with('-') {
        return Err(format!("invalid input string: {}", s));
    }
    i32::from_str_radix(s.as_str(), 2).map_err(|e| e.to_string())
}

/// Converts an unsigned integer to a string representation using a custom alphabet.
///
/// # Arguments
///
/// * `n` - The unsigned integer to convert.
/// * `alphabet` - A vector of strings representing the custom alphabet.
///
/// # Returns
///
/// A `Result` containing the string representation if successful, or an error message if the alphabet is empty.
pub fn uint2str(n: usize, alphabet: Vec<String>) -> Result<String, String> {
    if alphabet.is_empty() {
        return Err("void alphabet, cannot convert int2str".to_string());
    }
    let mut i = 1;
    let mut p = 0;
    let mut u;
    loop {
        let x = int2bin(i + 1, 0);
        let m = x.len();
        let y = x[1..m].to_string();
        p += 1;
        let bitnum = (alphabet.len() as f64).log2().ceil() as usize;
        let padding = if y.len() % bitnum != 0 {
            bitnum - (y.len() % bitnum)
        } else {
            0
        };
        let y = format!("{:0>width$}", y, width = y.len() + padding);
        u = bin2alphabet(y, alphabet.clone())?;
        if p == n {
            break;
        }
        i += 1;
    }
    Ok(u)
}

/// Converts a binary string to a string representation using a custom alphabet.
///
/// # Arguments
///
/// * `s` - A binary string to convert.
/// * `alphabet` - A vector of strings representing the custom alphabet.
///
/// # Returns
///
/// A `Result` containing the string representation if successful, or an error message if the input is invalid.
pub fn bin2alphabet(s: String, alphabet: Vec<String>) -> Result<String, String> {
    let bitnum: usize = (alphabet.len() as f64).log2().ceil() as usize;
    if s.is_empty() || (s.len() % bitnum != 0) {
        return Err(format!("wrong input string length: {}", s.len()));
    }
    let mut result = String::new();
    for i in 0..(s.len() / bitnum) {
        let symbol = &s.get(i * bitnum..((i + 1) * bitnum)).ok_or(format!(
            "char not founds in range: {} - {}",
            i * bitnum,
            ((i + 1) * bitnum)
        ))?;
        result.push_str(&alphabet[bin2int(symbol.to_string())? as usize]);
    }
    Ok(result)
}

/// Checks if a string contains only numeric characters.
///
/// # Arguments
///
/// * `s` - The string to check.
///
/// # Returns
///
/// `true` if the string contains only numeric characters, `false` otherwise.
pub fn is_numeric(s: String) -> bool {
    if s.is_empty() {
        return false;
    }
    for ch in s.chars() {
        if !ch.is_ascii_digit() {
            return false;
        }
    }
    true
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_input_string_to_vec() {
        let alphabet = vec!["a".to_string(), "b".to_string()];
        assert_eq!(
            input_string_to_vec(alphabet, "abb".to_string()),
            vec!["a", "b", "b"]
        );

        let alphabet2 = vec!["aa".to_string(), "bb".to_string(), "c".to_string()];
        assert_eq!(
            input_string_to_vec(alphabet2, "aabbc".to_string()),
            vec!["aa", "bb", "c"]
        );

        let alphabet3 = vec!["a".to_string(), "b".to_string()];
        assert_eq!(
            input_string_to_vec(alphabet3, "".to_string()),
            Vec::<String>::new()
        );
    }

    #[test]
    fn test_int2bin() {
        assert_eq!(int2bin(5, 4), "0101");
        assert_eq!(int2bin(3, 0), "11");
        assert_eq!(int2bin(0, 4), "0000");
        assert_eq!(int2bin(15, 4), "1111");
        assert_eq!(int2bin(8, 5), "01000");
    }

    #[test]
    fn test_bin2int() {
        assert_eq!(bin2int("101".to_string()), Ok(5));
        assert!(bin2int("abc".to_string()).is_err());
        assert_eq!(bin2int("0000".to_string()), Ok(0));
        assert_eq!(bin2int("1111".to_string()), Ok(15));
        assert!(bin2int("21".to_string()).is_err());
        assert!(bin2int("".to_string()).is_err());
    }

    #[test]
    fn test_uint2str() {
        let alphabet = vec!["a".to_string(), "b".to_string()];
        assert_eq!(uint2str(1, alphabet.clone()), Ok("a".to_string()));

        let alphabet2 = vec!["x".to_string(), "y".to_string(), "z".to_string()];
        assert_eq!(uint2str(2, alphabet2.clone()), Ok("y".to_string()));
    }

    #[test]
    fn test_bin2alphabet() {
        let alphabet = vec!["a".to_string(), "b".to_string()];
        assert_eq!(
            bin2alphabet("0".to_string(), alphabet.clone()),
            Ok("a".to_string())
        );
        assert!(bin2alphabet("".to_string(), alphabet.clone()).is_err());

        let large_alphabet = vec![
            "a".to_string(),
            "b".to_string(),
            "c".to_string(),
            "d".to_string(),
        ];
        assert_eq!(
            bin2alphabet("10".to_string(), large_alphabet.clone()),
            Ok("c".to_string())
        );
        assert!(bin2alphabet("111".to_string(), large_alphabet).is_err());
    }

    #[test]
    fn test_is_numeric() {
        assert!(is_numeric("123".to_string()));
        assert!(!is_numeric("12a".to_string()));
        assert!(is_numeric("0".to_string()));
        assert!(is_numeric("9999999".to_string()));
        assert!(!is_numeric("".to_string()));
        assert!(!is_numeric(" 123".to_string()));
        assert!(!is_numeric("12.3".to_string()));
    }
    #[test]
    fn test_input_string_to_vec_extended() {
        let alphabet = vec!["a".to_string(), "aa".to_string()];
        assert_eq!(
            input_string_to_vec(alphabet, "aaa".to_string()),
            vec!["a", "a", "a"]
        );

        let alphabet2 = vec!["00".to_string(), "11".to_string(), "22".to_string()];
        assert_eq!(
            input_string_to_vec(alphabet2, "001122".to_string()),
            vec!["00", "11", "22"]
        );
    }

    #[test]
    fn test_int2bin_negative() {
        assert_eq!(int2bin(-5, 5), "11011");
        assert_eq!(int2bin(-1, 8), "11111111");
    }

    #[test]
    fn test_bin2int_edge_cases() {
        assert_eq!(bin2int("10000000".to_string()), Ok(128));
        assert!(bin2int("1a1".to_string()).is_err());
        assert!(bin2int("-101".to_string()).is_err());
    }

    #[test]
    fn test_uint2str_edge_cases() {
        let empty_alphabet: Vec<String> = vec![];
        assert!(uint2str(1, empty_alphabet).is_err());
    }

    #[test]
    fn test_bin2alphabet_complex() {
        let alphabet = vec![
            "00".to_string(),
            "11".to_string(),
            "22".to_string(),
            "33".to_string(),
        ];
        assert_eq!(
            bin2alphabet("0010".to_string(), alphabet.clone()),
            Ok("0022".to_string())
        );
        assert!(bin2alphabet("0".to_string(), alphabet).is_err());
    }

    #[test]
    fn test_is_numeric_special_cases() {
        assert!(!is_numeric("12 3".to_string()));
        assert!(!is_numeric("-123".to_string()));
        assert!(!is_numeric("+123".to_string()));
        assert!(!is_numeric("12_3".to_string()));
    }
}
