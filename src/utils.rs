// file: utils.rs
// Project: Computing Simulator
// author: dp

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

pub fn int2bin(n: i32, bitnum: usize) -> String {
    if bitnum > 0 {
        let s = format!("{:0>width$b}", n, width = bitnum);
        s[s.len()-bitnum..s.len()].to_string()
    } else {
        format!("{:b}", n)
    }
}

pub fn bin2int(s: String) -> Result<i32, String> {
    if s.is_empty() || s.chars().nth(0) == Some('-') {
        return Err(format!("invalid input string: {}",  s))
    }
    i32::from_str_radix(s.as_str(), 2).map_err(|e| e.to_string())
}

/* pub fn invert_hashmap<K, V>(hashmap: &std::collections::HashMap<K, V>) -> std::collections::HashMap<V, K>
where
    K: Eq + std::hash::Hash + Clone,
    V: Eq + std::hash::Hash + Clone,
{
    hashmap.iter().map(|(k, v)| (v.clone(), k.clone())).collect()
} */

pub fn uint2str(n: usize, alphabet: Vec<String>) -> Result<String, String> {
    if alphabet.len() < 1 {
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

pub fn bin2alphabet(s: String, alphabet: Vec<String>) -> Result<String, String> {
    // number of bit needed to encode the alphabet
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

/* pub fn int2pair(n: i32) -> (i32, i32) {
    assert!(n>0);
    let d = ((((8 * n + 1) as f32).sqrt() + (1 as f32)) / (2 as f32)).floor() as i32;
    (n - (d * (d+1)/2), d + (d * (d+1) / 2) - n)
} */

pub fn is_numeric(s: String) -> bool {
    if s.len() < 1 {
        return false
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

        // Test with multi-char symbols
        let alphabet2 = vec!["aa".to_string(), "bb".to_string(), "c".to_string()];
        assert_eq!(
            input_string_to_vec(alphabet2, "aabbc".to_string()),
            vec!["aa", "bb", "c"]
        );
        
        // Test empty input
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
        // Additional cases
        assert_eq!(int2bin(0, 4), "0000");
        assert_eq!(int2bin(15, 4), "1111");
        assert_eq!(int2bin(8, 5), "01000");
    }

    #[test]
    fn test_bin2int() {
        assert_eq!(bin2int("101".to_string()), Ok(5));
        assert!(bin2int("abc".to_string()).is_err());
        // Additional cases
        assert_eq!(bin2int("0000".to_string()), Ok(0));
        assert_eq!(bin2int("1111".to_string()), Ok(15));
        assert!(bin2int("21".to_string()).is_err());
        assert!(bin2int("".to_string()).is_err());
    }

    #[test]
    fn test_uint2str() {
        let alphabet = vec!["a".to_string(), "b".to_string()];
        assert_eq!(uint2str(1, alphabet.clone()), Ok("a".to_string()));
        
        // Test with larger alphabet
        let alphabet2 = vec!["x".to_string(), "y".to_string(), "z".to_string()];
        assert_eq!(uint2str(2, alphabet2.clone()), Ok("y".to_string()));
    }

    #[test]
    fn test_bin2alphabet() {
        let alphabet = vec!["a".to_string(), "b".to_string()];
        assert_eq!(bin2alphabet("0".to_string(), alphabet.clone()), Ok("a".to_string()));
        assert!(bin2alphabet("".to_string(), alphabet.clone()).is_err());
        
        // Additional cases
        let large_alphabet = vec!["a".to_string(), "b".to_string(), "c".to_string(), "d".to_string()];
        assert_eq!(bin2alphabet("10".to_string(), large_alphabet.clone()), Ok("c".to_string()));
        assert!(bin2alphabet("111".to_string(), large_alphabet).is_err()); // Invalid bit length
    }

    #[test]
    fn test_is_numeric() {
        assert!(is_numeric("123".to_string()));
        assert!(!is_numeric("12a".to_string()));
        // Additional cases
        assert!(is_numeric("0".to_string()));
        assert!(is_numeric("9999999".to_string()));
        assert!(!is_numeric("".to_string()));
        assert!(!is_numeric(" 123".to_string()));
        assert!(!is_numeric("12.3".to_string()));
    }
    #[test]
    fn test_input_string_to_vec_extended() {
        // Test with overlapping symbols
        let alphabet = vec!["a".to_string(), "aa".to_string()];
        assert_eq!(
            input_string_to_vec(alphabet, "aaa".to_string()),
            vec!["a", "a", "a"]
        );

        // Test with longer input
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
        let alphabet = vec!["00".to_string(), "11".to_string(), "22".to_string(), "33".to_string()];
        assert_eq!(bin2alphabet("0010".to_string(), alphabet.clone()), Ok("0022".to_string()));
        assert!(bin2alphabet("0".to_string(), alphabet).is_err()); // Insufficient bits
    }

    #[test]
    fn test_is_numeric_special_cases() {
        assert!(!is_numeric("12 3".to_string()));
        assert!(!is_numeric("-123".to_string()));
        assert!(!is_numeric("+123".to_string()));
        assert!(!is_numeric("12_3".to_string()));
    }
}
