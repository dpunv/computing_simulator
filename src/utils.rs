// file: utils.rs
// Project: Computing Simulator
// author: dp
// date: 2025-03-05

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

/* pub fn int2bin(n: i32, bitnum: usize) -> String {
    format!("{:0>width$b}", n, width = bitnum)
} */

pub fn invert_hashmap<K, V>(hashmap: &std::collections::HashMap<K, V>) -> std::collections::HashMap<V, K>
where
    K: Eq + std::hash::Hash + Clone,
    V: Eq + std::hash::Hash + Clone,
{
    hashmap.iter().map(|(k, v)| (v.clone(), k.clone())).collect()
}