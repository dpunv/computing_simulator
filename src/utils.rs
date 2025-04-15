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
        format!("{:0>width$b}", n, width = bitnum)
    } else {
        format!("{:b}", n)
    }
}

pub fn bin2int(s: String) -> Result<i32, String> {
    i32::from_str_radix(s.as_str(), 2).map_err(|e| e.to_string())
}

/* pub fn invert_hashmap<K, V>(hashmap: &std::collections::HashMap<K, V>) -> std::collections::HashMap<V, K>
where
    K: Eq + std::hash::Hash + Clone,
    V: Eq + std::hash::Hash + Clone,
{
    hashmap.iter().map(|(k, v)| (v.clone(), k.clone())).collect()
} */

pub fn int2str(n: i32, alphabet: Vec<String>) -> Result<String, String> {
    let mut i = 1;
    let mut p = 0;
    let mut u;
    loop {
        let x = int2bin(i + 1, 0);
        let m = x.len();
        let y = x[2..m].to_string();
        p += 1;
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
    if s.len() % bitnum != 0 || s.is_empty() {
        return Err(" ERROR ".to_string());
    }
    let mut result = String::new();
    for i in 0..(s.len() / bitnum) {
        let symbol = &s.get(i * bitnum..((i + 1) * bitnum - 1)).ok_or(format!(
            "char not founds in range: {} - {}",
            i * bitnum,
            ((i + 1) * bitnum - 1)
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
    for ch in s.chars() {
        if !ch.is_ascii_digit() {
            return false;
        }
    }
    true
}
