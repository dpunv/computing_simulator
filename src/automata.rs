// file: automaton.rs
// Project: Computing Simulator
// author: dp
// date: 2025-03-06

use crate::utils;

#[derive(Clone)]
pub struct TuringMachine {
    pub initial_state: String,
    pub accept_state: String,
    pub reject_state: String,
    pub final_states: Vec<String>,
    pub blank_symbol: String,
    pub states: Vec<String>,
    pub input_alphabet: Vec<String>,
    pub tape_alphabet: Vec<String>,
    pub transitions: Vec<Transition>,
    pub end_on_final_state: bool,
    pub tape_count: usize,
    pub last_execution: (String, Vec<Tape>, i32, Vec<Configuration>),
}

#[derive(Clone)]
pub struct Tape {
    pub tape: Vec<String>,
    pub head: usize,
}

#[derive(Clone)]
pub struct Configuration {
    pub state: String,
    pub tapes: Vec<Tape>,
}

#[derive(Clone)]
pub struct Transition {
    pub state: String,
    pub symbols: Vec<String>,
    pub new_state: String,
    pub new_symbols: Vec<String>,
    pub directions: Vec<Direction>,
}

#[derive(Clone)]
pub struct RamMachine {
    pub instructions: Vec<Instruction>,
}

#[derive(Clone)]
pub struct Instruction {
    pub opcode: String,
    pub operand: String,
}

impl PartialEq for Transition {
    fn eq(&self, other: &Self) -> bool {
        if self.state != other.state {
            return false;
        }
        if self.new_state != other.new_state {
            return false;
        }
        if self.symbols.len() != other.symbols.len() {
            return false;
        }
        for (ind, symbol) in self.symbols.iter().enumerate() {
            if *symbol != other.symbols[ind] {
                return false;
            }
        }
        if self.new_symbols.len() != other.new_symbols.len() {
            return false;
        }
        for (ind, symbol) in self.new_symbols.iter().enumerate() {
            if *symbol != other.new_symbols[ind] {
                return false;
            }
        }
        if self.directions.len() != other.directions.len() {
            return false;
        }
        for (ind, direction) in self.directions.iter().enumerate() {
            if *direction != other.directions[ind] {
                return false;
            }
        }
        true
    }
}

pub trait Executable {
    fn simulate(&mut self, input: Vec<String>, max_steps: i32) -> (String, i32);
    fn to_encoding(
        &self,
    ) -> (
        String,
        std::collections::HashMap<String, String>,
        std::collections::HashMap<String, String>,
    );
}

pub trait Automaton: Clone {
    fn is_deterministic(&self) -> bool;
    fn is_transition_total(&self) -> bool;
    fn is_ok(&self) -> bool;
    fn make_transition_map(&self) -> std::collections::HashMap<String, Vec<Transition>>;
    fn input_alphabet(&self) -> Vec<String>;
}

#[derive(Clone)]
pub enum Direction {
    Left,
    Right,
    Stay,
}

pub trait FromString {
    fn from_string(s: &str) -> Self;
}

impl FromString for Direction {
    fn from_string(s: &str) -> Self {
        match s {
            "L" => Direction::Left,
            "R" => Direction::Right,
            _ => Direction::Stay,
        }
    }
}

impl PartialEq for Direction {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (Direction::Left, Direction::Left) => true,
            (Direction::Right, Direction::Right) => true,
            (Direction::Stay, Direction::Stay) => true,
            (Direction::Left, Direction::Right) => false,
            (Direction::Left, Direction::Stay) => false,
            (Direction::Right, Direction::Left) => false,
            (Direction::Right, Direction::Stay) => false,
            (Direction::Stay, Direction::Left) => false,
            (Direction::Stay, Direction::Right) => false,
        }
    }
}

impl Executable for TuringMachine {
    fn simulate(&mut self, input: Vec<String>, max_steps: i32) -> (String, i32) {
        #[derive(Clone)]
        struct TreeElement {
            state: String,
            tapes: Vec<Tape>,
            previous: usize,
        }
        impl PartialEq for TreeElement {
            fn eq(&self, other: &Self) -> bool {
                for (ind, tape) in self.tapes.iter().enumerate() {
                    if tape.tape != other.tapes[ind].tape {
                        return false;
                    }
                }
                self.state == other.state
            }
        }
        let transitions_map = self.make_transition_map();
        let mut tree = Vec::new();
        tree.push(Vec::new());
        let mut tape = Vec::new();
        tape.push(self.blank_symbol.clone());
        for symbol in input {
            tape.push(symbol);
        }
        let mut tapes = Vec::new();
        tapes.push(Tape {
            tape: tape.clone(),
            head: 0,
        });
        for _ in 1..self.tape_count {
            tapes.push(Tape {
                tape: vec![self.blank_symbol.clone()],
                head: 0,
            });
        }
        tree[0].push(TreeElement {
            state: self.initial_state.clone(),
            tapes: tapes.clone(),
            previous: 0,
        });
        let mut steps = 0;
        let mut halts = false;
        while steps < max_steps && !halts {
            steps += 1;
            let mut new_level = Vec::new();
            for (ind, element) in tree[tree.len() - 1].iter().enumerate() {
                let state = element.state.clone();
                if self.final_states.contains(&state) && self.end_on_final_state {
                    halts = true;
                    break;
                }
                let mut key = state.clone();
                for tapenum in 0..self.tape_count {
                    key += &element.tapes[tapenum].tape[element.tapes[tapenum].head];
                }
                let mut found = false;
                if transitions_map.contains_key(&key) {
                    found = true;
                    let possible_transitions = transitions_map.get(&key).unwrap().clone();
                    for transition in possible_transitions.iter() {
                        let mut new_tapes = Vec::new();
                        for tapenum in 0..self.tape_count {
                            let mut new_tape = element.tapes[tapenum].clone();
                            new_tape.tape[new_tape.head] = transition.new_symbols[tapenum].clone();
                            let new_head = match transition.directions[tapenum] {
                                Direction::Left => {
                                    if new_tape.head == 0 {
                                        new_tape.tape.insert(0, self.blank_symbol.clone());
                                        0
                                    } else {
                                        new_tape.head - 1
                                    }
                                }
                                Direction::Right => {
                                    if new_tape.head == new_tape.tape.len() - 1 {
                                        new_tape.tape.push(self.blank_symbol.clone());
                                    }
                                    new_tape.head + 1
                                }
                                Direction::Stay => new_tape.head,
                            };
                            new_tape.head = new_head;
                            new_tapes.push(new_tape);
                        }
                        let new_state = transition.new_state.clone();
                        let el = TreeElement {
                            state: new_state,
                            tapes: new_tapes,
                            previous: ind,
                        };
                        if !new_level.contains(&el) {
                            new_level.push(el);
                        }
                    }
                }
                if !found && new_level.is_empty() {
                    halts = true;
                    break;
                }
            }
            tree.push(new_level);
        }
        tree.pop();
        let mut computation = Vec::new();
        let mut index = (tree.len() - 1) as i32;
        let mut previous = 0;
        while index >= 0 {
            let element = &tree[(index) as usize][previous];
            previous = element.previous;
            index -= 1;
            let configuration = Configuration {
                state: element.state.clone(),
                tapes: element.tapes.clone(),
            };
            computation.push(configuration);
        }
        computation.reverse();
        let last_element = tree[tree.len() - 1][previous].clone();
        self.last_execution = (
            last_element.state.clone(),
            last_element.tapes.clone(),
            steps,
            computation,
        );
        (last_element.state.clone(), steps)
    }
    fn to_encoding(
        &self,
    ) -> (
        String,
        std::collections::HashMap<String, String>,
        std::collections::HashMap<String, String>,
    ) {
        let mut state_bits: usize = 0;
        let mut states = self.states.len();
        while states > 0 {
            states >>= 1;
            state_bits += 1;
        }
        let mut state_encoding: std::collections::HashMap<String, String> =
            std::collections::HashMap::new();
        for (index, state) in self.states.iter().enumerate() {
            if self.final_states.contains(state)
                && state != &self.accept_state
                && state != &self.reject_state
            {
                state_encoding.insert(
                    state.clone(),
                    format!("h{:0>width$b}", index, width = state_bits),
                );
            } else if state == &self.accept_state {
                state_encoding.insert(
                    state.clone(),
                    format!("y{:0>width$b}", index, width = state_bits),
                );
            } else if state == &self.reject_state {
                state_encoding.insert(
                    state.clone(),
                    format!("n{:0>width$b}", index, width = state_bits),
                );
            } else if state == &self.initial_state {
                state_encoding.insert(
                    state.clone(),
                    format!("i{:0>width$b}", index, width = state_bits),
                );
            } else {
                state_encoding.insert(
                    state.clone(),
                    format!("q{:0>width$b}", index, width = state_bits),
                );
            }
        }
        let mut tape_bits: usize = 0;
        let mut tape_symbols = self.tape_alphabet.len();
        while tape_symbols > 0 {
            tape_symbols >>= 1;
            tape_bits += 1;
        }
        let mut tape_encoding: std::collections::HashMap<String, String> =
            std::collections::HashMap::new();
        for (index, symbol) in self.tape_alphabet.iter().enumerate() {
            if self.input_alphabet.contains(symbol) {
                tape_encoding.insert(
                    symbol.clone(),
                    format!("a{:0>width$b}", index, width = tape_bits),
                );
            } else if symbol == &self.blank_symbol {
                tape_encoding.insert(
                    symbol.clone(),
                    format!("b{:0>width$b}", index, width = tape_bits),
                );
            } else {
                tape_encoding.insert(
                    symbol.clone(),
                    format!("t{:0>width$b}", index, width = tape_bits),
                );
            }
        }
        let mut transitions_encoding = String::new();
        for transition in &self.transitions {
            let mut transition_encoding = "(".to_string();
            transition_encoding.push_str(&state_encoding[&transition.state]);
            transition_encoding.push(';');
            for symbol in &transition.symbols {
                transition_encoding.push_str(&tape_encoding[symbol]);
                transition_encoding.push(';');
            }
            transition_encoding.push_str(&state_encoding[&transition.new_state]);
            transition_encoding.push(';');
            for symbol in &transition.new_symbols {
                transition_encoding.push_str(&tape_encoding[symbol]);
                transition_encoding.push(';');
            }
            for direction in &transition.directions {
                match direction {
                    Direction::Left => transition_encoding.push('L'),
                    Direction::Right => transition_encoding.push('R'),
                    Direction::Stay => transition_encoding.push('S'),
                }
                transition_encoding.push(';');
            }
            transition_encoding.pop();
            transition_encoding.push(')');
            transitions_encoding.push_str(&transition_encoding);
        }
        (transitions_encoding, tape_encoding, state_encoding)
    }
}

impl Automaton for TuringMachine {
    fn input_alphabet(&self) -> Vec<String> {
        self.input_alphabet.clone()
    }

    fn make_transition_map(&self) -> std::collections::HashMap<String, Vec<Transition>> {
        let mut transition_map: std::collections::HashMap<String, Vec<Transition>> =
            std::collections::HashMap::new();
        for transition in &self.transitions {
            let mut key = transition.state.clone();
            for symbol in &transition.symbols {
                key += symbol;
            }
            if transition_map.contains_key(&key) {
                transition_map
                    .get_mut(&key)
                    .unwrap()
                    .push(transition.clone());
            } else {
                transition_map.insert(key.clone(), vec![transition.clone()]);
            }
        }
        transition_map
    }

    fn is_ok(&self) -> bool {
        let mut is_input_subset_of_tape = true;
        let mut is_blank_in_tape = true;
        let mut is_blank_not_in_input = true;
        let mut is_transitions_valid = true;
        let mut is_final_states_valid = true;
        let mut is_initial_state_valid = true;

        for symbol in &self.input_alphabet {
            if !self.tape_alphabet.contains(symbol) {
                is_input_subset_of_tape = false;
                break;
            }
        }

        if !self.tape_alphabet.contains(&self.blank_symbol) {
            is_blank_in_tape = false;
        }

        if self.input_alphabet.contains(&self.blank_symbol) {
            is_blank_not_in_input = false;
        }

        for transition in &self.transitions {
            for symbol in &transition.symbols {
                if !self.tape_alphabet.contains(symbol) {
                    is_transitions_valid = false;
                    //break;
                }
            }
            for symbol in &transition.new_symbols {
                if !self.tape_alphabet.contains(symbol) {
                    is_transitions_valid = false;
                    //break;
                }
            }
            for direction in &transition.directions {
                if !matches!(
                    direction,
                    Direction::Left | Direction::Right | Direction::Stay
                ) {
                    is_transitions_valid = false;
                    break;
                }
            }
        }

        for final_state in &self.final_states {
            if !self.states.contains(final_state) {
                is_final_states_valid = false;
                break;
            }
        }

        if !self.states.contains(&self.initial_state) {
            is_initial_state_valid = false;
        }
        is_blank_in_tape
            && is_blank_not_in_input
            && is_final_states_valid
            && is_input_subset_of_tape
            && is_initial_state_valid
            && is_transitions_valid
    }

    fn is_deterministic(&self) -> bool {
        let transition_map = self.make_transition_map();
        for transitions in transition_map.values() {
            if transitions.len() > 1 {
                return false;
            }
        }
        true
    }

    fn is_transition_total(&self) -> bool {
        let transition_map = self.make_transition_map();
        for state in &self.states {
            for symbol in &self.tape_alphabet {
                let key = state.clone() + symbol;
                if !transition_map.contains_key(&key) {
                    return false;
                }
            }
        }
        true
    }
}

pub fn convert_multi_tape_to_single_tape_tm(tm: TuringMachine) -> TuringMachine {
    let mut new_tm = TuringMachine {
        initial_state: tm.initial_state.clone(),
        accept_state: tm.accept_state.clone(),
        reject_state: tm.reject_state.clone(),
        final_states: tm.final_states.clone(),
        blank_symbol: tm.blank_symbol.clone(),
        states: Vec::new(),
        input_alphabet: tm.input_alphabet.clone(),
        tape_alphabet: Vec::new(),
        transitions: Vec::new(),
        end_on_final_state: tm.end_on_final_state,
        tape_count: 1,
        last_execution: ("".to_string(), Vec::new(), 0, Vec::new()),
    };
    let head_symbols = vec!["^".to_string(), "_".to_string()];
    let mut new_compound_symbols = Vec::new();
    for symbol in &tm.tape_alphabet {
        for head_symbol in &head_symbols {
            new_compound_symbols.push(symbol.clone() + head_symbol);
        }
    }
    let mut new_tape_alphabet = new_compound_symbols.clone();
    for tape_symbol in &tm.tape_alphabet {
        new_tape_alphabet.push(tape_symbol.clone());
    }
    let tape_sep_symbol = "#".to_string();
    new_tape_alphabet.push(tape_sep_symbol.clone());
    new_tm.tape_alphabet = new_tape_alphabet.clone();
    let mut new_states = Vec::new();
    let mut new_transitions = Vec::new();
    for tapenum in 0..tm.tape_count {
        let initial_state_tape =
            tm.initial_state.clone() + "<INIT_TP" + &tapenum.to_string() + "_START>";
        let end_state_tape = tm.initial_state.clone() + "<INIT_TP" + &tapenum.to_string() + "_END>";
        new_states.push(initial_state_tape.clone());
        new_states.push(end_state_tape.clone());
        if tapenum == 0 {
            for symbol in &tm.tape_alphabet {
                let new_transition = Transition {
                    state: tm.initial_state.clone(),
                    symbols: vec![symbol.clone()],
                    new_state: initial_state_tape.clone(),
                    new_symbols: vec![symbol.clone() + "^"],
                    directions: vec![Direction::Right],
                };
                if !new_transitions.contains(&new_transition) {
                    new_transitions.push(new_transition);
                }
                if *symbol != tm.blank_symbol.clone() {
                    let new_transition = Transition {
                        state: initial_state_tape.clone(),
                        symbols: vec![symbol.clone()],
                        new_state: initial_state_tape.clone(),
                        new_symbols: vec![symbol.clone() + "_"],
                        directions: vec![Direction::Right],
                    };
                    if !new_transitions.contains(&new_transition) {
                        new_transitions.push(new_transition);
                    }
                }
                let new_transition = Transition {
                    state: initial_state_tape.clone(),
                    symbols: vec![tm.blank_symbol.clone()],
                    new_state: end_state_tape.clone(),
                    new_symbols: vec![tm.blank_symbol.clone()],
                    directions: vec![Direction::Stay],
                };
                if !new_transitions.contains(&new_transition) {
                    new_transitions.push(new_transition);
                }
            }
        } else {
            let new_transition = Transition {
                state: tm.initial_state.clone() + "<INIT_TP" + &(tapenum - 1).to_string() + "_END>",
                symbols: vec![tm.blank_symbol.clone()],
                new_state: initial_state_tape.clone(),
                new_symbols: vec![tape_sep_symbol.clone()],
                directions: vec![Direction::Right],
            };
            if !new_transitions.contains(&new_transition) {
                new_transitions.push(new_transition);
            }
            let new_transition = Transition {
                state: initial_state_tape.clone(),
                symbols: vec![tm.blank_symbol.clone()],
                new_state: end_state_tape.clone(),
                new_symbols: vec![tm.blank_symbol.clone() + "^"],
                directions: vec![Direction::Right],
            };
            if !new_transitions.contains(&new_transition) {
                new_transitions.push(new_transition);
            }
        }
    }
    let setup_state = tm.initial_state.clone() + "<SETUP>";
    new_states.push(setup_state.clone());
    let new_start_state = tm.initial_state.clone() + "<START>";
    new_states.push(new_start_state.clone());
    for symbol in new_tape_alphabet.clone() {
        if symbol != tm.blank_symbol {
            let new_transition = Transition {
                state: setup_state.clone(),
                symbols: vec![symbol.clone()],
                new_state: setup_state.clone(),
                new_symbols: vec![symbol.clone()],
                directions: vec![Direction::Left],
            };
            if !new_transitions.contains(&new_transition) {
                new_transitions.push(new_transition);
            }
        } else {
            let new_transition = Transition {
                state: tm.initial_state.clone()
                    + "<INIT_TP"
                    + (tm.tape_count - 1).to_string().as_str()
                    + "_END>",
                symbols: vec![tm.blank_symbol.clone()],
                new_state: setup_state.clone(),
                new_symbols: vec![tm.blank_symbol.clone()],
                directions: vec![Direction::Left],
            };
            if !new_transitions.contains(&new_transition) {
                new_transitions.push(new_transition);
            }
            let new_transition = Transition {
                state: setup_state.clone(),
                symbols: vec![tm.blank_symbol.clone()],
                new_state: new_start_state.clone(),
                new_symbols: vec![tm.blank_symbol.clone()],
                directions: vec![Direction::Right],
            };
            if !new_transitions.contains(&new_transition) {
                new_transitions.push(new_transition);
            }
        }
    }
    let mut states_to_process = Vec::new();
    for state in &tm.states {
        if state != &tm.initial_state {
            states_to_process.push(state.clone());
        } else {
            states_to_process.push(new_start_state.clone());
        }
    }
    let mut map_states: std::collections::HashMap<String, Vec<String>> =
        std::collections::HashMap::new();
    let mut states_vec = states_to_process.clone();
    for state in tm.final_states.clone() {
        if states_to_process.contains(&state) {
            states_to_process.retain(|x| x != &state);
        }
    }
    let mut states_to_copy = Vec::new();
    for state in states_to_process {
        map_states.insert(state.clone() + "0", vec![state.clone()]);
        for tapenum in 0..tm.tape_count {
            let mut this_state_vec = Vec::new();
            for symbol in &new_compound_symbols {
                for actual_state in map_states[&(state.clone() + &tapenum.to_string())].clone() {
                    let state_tape = actual_state.clone() + "<R_TP" + &tapenum.to_string() + ">";
                    let new_state = actual_state.clone()
                        + "<R_TP"
                        + &tapenum.to_string()
                        + "_S_"
                        + symbol
                        + ">";
                    let end_state = actual_state.clone()
                        + "<R_TP"
                        + &tapenum.to_string()
                        + "_S_"
                        + symbol
                        + "_END>";
                    if !states_vec.contains(&new_state) {
                        states_vec.push(new_state.clone());
                    }
                    if !states_vec.contains(&end_state) {
                        states_vec.push(end_state.clone());
                    }
                    if !states_vec.contains(&state_tape) {
                        states_vec.push(state_tape.clone());
                    }
                    this_state_vec.push(end_state.clone());
                    let new_transition = Transition {
                        state: new_state.clone(),
                        symbols: vec![tm.blank_symbol.clone()],
                        new_state: end_state.clone(),
                        new_symbols: vec![tm.blank_symbol.clone()],
                        directions: vec![Direction::Stay],
                    };
                    if !new_transitions.contains(&new_transition) {
                        new_transitions.push(new_transition);
                    }
                    if tapenum == 0 {
                        let new_transition = Transition {
                            state: state.clone(),
                            symbols: vec![symbol.clone()],
                            new_state: state_tape.clone(),
                            new_symbols: vec![symbol.clone()],
                            directions: vec![Direction::Stay],
                        };
                        if !new_transitions.contains(&new_transition) {
                            new_transitions.push(new_transition);
                        }
                    } else {
                        let new_transition = Transition {
                            state: actual_state.clone(),
                            symbols: vec![symbol.clone()],
                            new_state: state_tape.clone(),
                            new_symbols: vec![symbol.clone()],
                            directions: vec![Direction::Stay],
                        };
                        if !new_transitions.contains(&new_transition) {
                            new_transitions.push(new_transition);
                        }
                    }
                    if symbol.ends_with("^") {
                        let new_transition = Transition {
                            state: state_tape.clone(),
                            symbols: vec![symbol.clone()],
                            new_state: new_state.clone(),
                            new_symbols: vec![symbol.clone()],
                            directions: vec![Direction::Right],
                        };
                        if !new_transitions.contains(&new_transition) {
                            new_transitions.push(new_transition);
                        }
                    } else {
                        let new_transition = Transition {
                            state: state_tape.clone(),
                            symbols: vec![symbol.clone()],
                            new_state: state_tape.clone(),
                            new_symbols: vec![symbol.clone()],
                            directions: vec![Direction::Right],
                        };
                        if !new_transitions.contains(&new_transition) {
                            new_transitions.push(new_transition);
                        }
                    }
                    for symb in new_compound_symbols.clone() {
                        if !symb.ends_with("^") {
                            let new_transition = Transition {
                                state: new_state.clone(),
                                symbols: vec![symb.clone()],
                                new_state: new_state.clone(),
                                new_symbols: vec![symb.clone()],
                                directions: vec![Direction::Right],
                            };
                            if !new_transitions.contains(&new_transition) {
                                new_transitions.push(new_transition);
                            }
                        }
                    }
                    let new_transition = Transition {
                        state: new_state.clone(),
                        symbols: vec![tape_sep_symbol.clone()],
                        new_state: end_state.clone(),
                        new_symbols: vec![tape_sep_symbol.clone()],
                        directions: vec![Direction::Right],
                    };
                    if !new_transitions.contains(&new_transition) {
                        new_transitions.push(new_transition);
                    }
                }
            }
            map_states.insert(
                state.clone() + &(tapenum + 1).to_string(),
                this_state_vec.clone(),
            );
        }
        let old_transition_map = tm.make_transition_map();
        let mut states_done = Vec::new();
        for actual_state in map_states[&(state.clone() + &tm.tape_count.to_string())].clone() {
            let splitted0: Vec<&str> = actual_state.split("<R_TP").collect();
            let key = state
                .clone()
                .strip_suffix("<START>")
                .unwrap_or(&state)
                .to_string()
                + &splitted0
                    .iter()
                    .skip(1)
                    .map(|elem| {
                        let parts: Vec<&str> = elem.split("_S_").collect();
                        let mut part = parts.get(1).unwrap_or(&"").to_string();
                        part = part.strip_suffix("__END>").unwrap_or(&part).to_string();
                        part = part.strip_suffix("^_END>").unwrap_or(&part).to_string();
                        part
                    })
                    .collect::<Vec<String>>()
                    .join("");
            if old_transition_map.contains_key(&key) && !states_done.contains(&key) {
                states_done.push(key.clone());
                let transitions = old_transition_map[&key].clone();
                for (ind, t) in transitions.iter().enumerate() {
                    for tapenum in 0..tm.tape_count {
                        let state_init_tape = actual_state.clone()
                            + "<WRITE_TR"
                            + &ind.to_string()
                            + "_TP_"
                            + &tapenum.to_string()
                            + "_START>";
                        let state_mid_tape = actual_state.clone()
                            + "<WRITE_TR"
                            + &ind.to_string()
                            + "_TP_"
                            + &tapenum.to_string()
                            + "_^FOUND>";
                        let state_mid_mid_tape = actual_state.clone()
                            + "<WRITE_TR"
                            + &ind.to_string()
                            + "_TP_"
                            + &tapenum.to_string()
                            + "_COPY>";
                        let state_end_tape = actual_state.clone()
                            + "<WRITE_TR"
                            + &ind.to_string()
                            + "_TP_"
                            + &tapenum.to_string()
                            + "_END>";
                        if !states_vec.contains(&state_init_tape) {
                            states_vec.push(state_init_tape.clone());
                        }
                        if !states_vec.contains(&state_mid_tape) {
                            states_vec.push(state_mid_tape.clone());
                        }
                        if !states_vec.contains(&state_mid_mid_tape) {
                            states_vec.push(state_mid_mid_tape.clone());
                        }
                        if !states_vec.contains(&state_end_tape) {
                            states_vec.push(state_end_tape.clone());
                        }
                        for symbol in new_compound_symbols.clone() {
                            if symbol.ends_with("^") {
                                if t.directions[tapenum] == Direction::Right {
                                    let new_transition = Transition {
                                        state: state_init_tape.clone(),
                                        symbols: vec![t.symbols[tapenum].clone() + "^"],
                                        new_state: state_mid_tape.clone(),
                                        new_symbols: vec![t.new_symbols[tapenum].clone() + "_"],
                                        directions: vec![Direction::Right],
                                    };
                                    if !new_transitions.contains(&new_transition) {
                                        new_transitions.push(new_transition);
                                    }
                                    let new_transition = Transition {
                                        state: state_mid_tape.clone(),
                                        symbols: vec![tape_sep_symbol.clone()],
                                        new_state: state_mid_mid_tape.clone(),
                                        new_symbols: vec![tape_sep_symbol.clone()],
                                        directions: vec![Direction::Left],
                                    };
                                    if !new_transitions.contains(&new_transition) {
                                        new_transitions.push(new_transition);
                                    }
                                    let new_transition = Transition {
                                        state: state_mid_tape.clone(),
                                        symbols: vec![tm.blank_symbol.clone()],
                                        new_state: state_mid_mid_tape.clone(),
                                        new_symbols: vec![tm.blank_symbol.clone()],
                                        directions: vec![Direction::Left],
                                    };
                                    if !new_transitions.contains(&new_transition) {
                                        new_transitions.push(new_transition);
                                    }
                                    for symb in new_compound_symbols.clone() {
                                        let new_transition = Transition {
                                            state: state_mid_mid_tape.clone(),
                                            symbols: vec![symb.clone()],
                                            new_state: state_mid_mid_tape.clone()
                                                + "<COPY_CYCLE_RIGHT>",
                                            new_symbols: vec![symb.clone() + "<COPY>"],
                                            directions: vec![Direction::Right],
                                        };
                                        states_to_copy.push(state_mid_mid_tape.clone());
                                        if !new_transitions.contains(&new_transition) {
                                            new_transitions.push(new_transition);
                                        }
                                        if !symb.ends_with("^") {
                                            let new_transition = Transition {
                                                state: state_mid_tape.clone(),
                                                symbols: vec![
                                                    symb.clone()
                                                        .strip_suffix("_")
                                                        .unwrap_or(&symb)
                                                        .to_string()
                                                        + "_",
                                                ],
                                                new_state: state_end_tape.clone(),
                                                new_symbols: vec![
                                                    symb.clone()
                                                        .strip_suffix("_")
                                                        .unwrap_or(&symb)
                                                        .to_string()
                                                        + "^",
                                                ],
                                                directions: vec![Direction::Left],
                                            };
                                            if !new_transitions.contains(&new_transition) {
                                                new_transitions.push(new_transition);
                                            }
                                        }
                                    }
                                } else if t.directions[tapenum] == Direction::Left {
                                    let new_transition = Transition {
                                        state: state_init_tape.clone(),
                                        symbols: vec![t.symbols[tapenum].clone() + "^"],
                                        new_state: state_mid_tape.clone(),
                                        new_symbols: vec![t.new_symbols[tapenum].clone() + "_"],
                                        directions: vec![Direction::Left],
                                    };
                                    if !new_transitions.contains(&new_transition) {
                                        new_transitions.push(new_transition);
                                    }
                                    let new_transition = Transition {
                                        state: state_mid_tape.clone(),
                                        symbols: vec![tape_sep_symbol.clone()],
                                        new_state: state_mid_tape.clone() + "<COPY_CYCLE_RIGHT>",
                                        new_symbols: vec![tape_sep_symbol.clone() + "<COPY>"],
                                        directions: vec![Direction::Right],
                                    };
                                    states_to_copy.push(state_mid_tape.clone());
                                    if !new_transitions.contains(&new_transition) {
                                        new_transitions.push(new_transition);
                                    }
                                    for symb in new_compound_symbols.clone() {
                                        if !symb.ends_with("^") {
                                            let new_transition = Transition {
                                                state: state_mid_tape.clone(),
                                                symbols: vec![
                                                    symb.clone()
                                                        .strip_suffix("_")
                                                        .unwrap_or(&symb)
                                                        .to_string()
                                                        + "_",
                                                ],
                                                new_state: state_end_tape.clone(),
                                                new_symbols: vec![
                                                    symb.clone()
                                                        .strip_suffix("_")
                                                        .unwrap_or(&symb)
                                                        .to_string()
                                                        + "^",
                                                ],
                                                directions: vec![Direction::Left],
                                            };
                                            if !new_transitions.contains(&new_transition) {
                                                new_transitions.push(new_transition);
                                            }
                                        }
                                    }
                                } else {
                                    let new_transition = Transition {
                                        state: state_init_tape.clone(),
                                        symbols: vec![t.symbols[tapenum].clone() + "^"],
                                        new_state: state_end_tape.clone(),
                                        new_symbols: vec![t.new_symbols[tapenum].clone() + "^"],
                                        directions: vec![Direction::Left],
                                    };
                                    if !new_transitions.contains(&new_transition) {
                                        new_transitions.push(new_transition);
                                    }
                                }
                            } else {
                                let new_transition = Transition {
                                    state: state_init_tape.clone(),
                                    symbols: vec![symbol.clone()],
                                    new_state: state_init_tape.clone(),
                                    new_symbols: vec![symbol.clone()],
                                    directions: vec![Direction::Left],
                                };
                                if !new_transitions.contains(&new_transition) {
                                    new_transitions.push(new_transition);
                                }
                                let new_transition = Transition {
                                    state: state_end_tape.clone(),
                                    symbols: vec![symbol.clone()],
                                    new_state: state_end_tape.clone(),
                                    new_symbols: vec![symbol.clone()],
                                    directions: vec![Direction::Left],
                                };
                                if !new_transitions.contains(&new_transition) {
                                    new_transitions.push(new_transition);
                                }
                            }
                        }
                        if tapenum == 0 {
                            let new_transition = Transition {
                                state: state_end_tape.clone(),
                                symbols: vec![tm.blank_symbol.clone()],
                                new_state: t.new_state.clone(),
                                new_symbols: vec![tm.blank_symbol.clone()],
                                directions: vec![Direction::Right],
                            };
                            if !new_transitions.contains(&new_transition) {
                                new_transitions.push(new_transition);
                            }
                        } else {
                            let new_transition = Transition {
                                state: state_end_tape.clone(),
                                symbols: vec![tape_sep_symbol.clone()],
                                new_state: actual_state.clone()
                                    + "<WRITE_TR"
                                    + &ind.to_string()
                                    + "_TP_"
                                    + &(tapenum - 1).to_string()
                                    + "_START>",
                                new_symbols: vec![tape_sep_symbol.clone()],
                                directions: vec![Direction::Left],
                            };
                            if !new_transitions.contains(&new_transition) {
                                new_transitions.push(new_transition);
                            }
                        }
                        if tapenum == tm.tape_count - 1 {
                            let new_transition = Transition {
                                state: actual_state.clone(),
                                symbols: vec![tm.blank_symbol.clone()],
                                new_state: state_init_tape.clone(),
                                new_symbols: vec![tm.blank_symbol.clone()],
                                directions: vec![Direction::Left],
                            };
                            if !new_transitions.contains(&new_transition) {
                                new_transitions.push(new_transition);
                            }
                        }
                    }
                }
            }
        }
    }
    new_tape_alphabet.push(tape_sep_symbol.clone() + "<COPY>");
    for state in states_to_copy {
        let state_copy_a = state.clone() + "<COPY_CYCLE_RIGHT>";
        let state_copy_b = state.clone() + "<COPY_BLANK_FOUND>";
        let state_copy_c = state.clone() + "<COPY_GO_LEFT_1>";
        let state_copy_e = state.clone() + "<COPY_FINISHED>";
        if !states_vec.contains(&state_copy_a) {
            states_vec.push(state_copy_a.clone());
        }
        if !states_vec.contains(&state_copy_b) {
            states_vec.push(state_copy_b.clone());
        }
        if !states_vec.contains(&state_copy_c) {
            states_vec.push(state_copy_c.clone());
        }
        if !states_vec.contains(&state_copy_e) {
            states_vec.push(state_copy_e.clone());
        }
        let mut symbols_to_cycle = new_compound_symbols.clone();
        symbols_to_cycle.push(tape_sep_symbol.clone());
        for symbol in &symbols_to_cycle {
            let new_transition = Transition {
                state: state_copy_a.clone(),
                symbols: vec![symbol.clone()],
                new_state: state_copy_a.clone(),
                new_symbols: vec![symbol.clone()],
                directions: vec![Direction::Right],
            };
            if !new_transitions.contains(&new_transition) {
                new_transitions.push(new_transition);
            }
            let new_transition = Transition {
                state: state_copy_b.clone(),
                symbols: vec![tm.blank_symbol.clone()],
                new_state: state_copy_c.clone(),
                new_symbols: vec![tm.blank_symbol.clone()],
                directions: vec![Direction::Left],
            };
            if !new_transitions.contains(&new_transition) {
                new_transitions.push(new_transition);
            }
            let state_copy_d = state.clone() + "<COPY_SYMBOL_" + &symbol.clone() + ">";
            if !states_vec.contains(&state_copy_d) {
                states_vec.push(state_copy_d.clone());
            }
            let new_transition = Transition {
                state: state_copy_c.clone(),
                symbols: vec![symbol.clone()],
                new_state: state_copy_d.clone(),
                new_symbols: vec![tm.blank_symbol.clone()],
                directions: vec![Direction::Right],
            };
            if !new_transitions.contains(&new_transition) {
                new_transitions.push(new_transition);
            }
            let new_transition = Transition {
                state: state_copy_d.clone(),
                symbols: vec![tm.blank_symbol.clone()],
                new_state: state_copy_b.clone(),
                new_symbols: vec![symbol.clone()],
                directions: vec![Direction::Left],
            };
            if !new_transitions.contains(&new_transition) {
                new_transitions.push(new_transition);
            }
            let symbol_with_copy = symbol.clone() + "<COPY>";
            let new_transition = Transition {
                state: state_copy_c.clone(),
                symbols: vec![symbol_with_copy.clone()],
                new_state: state_copy_e.clone(),
                new_symbols: vec![symbol.clone()],
                directions: vec![Direction::Right],
            };
            if !new_tape_alphabet.contains(&symbol_with_copy) {
                new_tape_alphabet.push(symbol_with_copy);
            }
            if !new_transitions.contains(&new_transition) {
                new_transitions.push(new_transition);
            }
            if state.ends_with("COPY>") {
                let new_transition = Transition {
                    state: state_copy_e.clone(),
                    symbols: vec![tm.blank_symbol.clone()],
                    new_state: state
                        .clone()
                        .strip_suffix("_COPY>")
                        .unwrap_or(&state)
                        .to_string()
                        + "_^FOUND>",
                    new_symbols: vec![tm.blank_symbol.clone() + "_"],
                    directions: vec![Direction::Stay],
                };
                if !new_transitions.contains(&new_transition) {
                    new_transitions.push(new_transition);
                }
            } else {
                let new_transition = Transition {
                    state: state_copy_e.clone(),
                    symbols: vec![tm.blank_symbol.clone()],
                    new_state: state.clone(),
                    new_symbols: vec![tm.blank_symbol.clone() + "_"],
                    directions: vec![Direction::Stay],
                };
                if !new_transitions.contains(&new_transition) {
                    new_transitions.push(new_transition);
                }
            }
        }
        let new_transition = Transition {
            state: state_copy_a.clone(),
            symbols: vec![tm.blank_symbol.clone()],
            new_state: state_copy_b.clone(),
            new_symbols: vec![tm.blank_symbol.clone()],
            directions: vec![Direction::Stay],
        };
        if !new_transitions.contains(&new_transition) {
            new_transitions.push(new_transition);
        }
    }
    let mut real_final_states = Vec::new();
    for state in tm.final_states {
        let state_final_1 = state.clone() + "<OTHER_TP>";
        let state_final_2 = state.clone() + "<END>";
        if !states_vec.contains(&state_final_1) {
            states_vec.push(state_final_1.clone());
        }
        if !states_vec.contains(&state_final_2) {
            states_vec.push(state_final_2.clone());
        }
        real_final_states.push(state_final_2.clone());
        for symbol in new_compound_symbols.clone() {
            let new_transition = Transition {
                state: state.clone(),
                symbols: vec![symbol.clone()],
                new_state: state.clone(),
                new_symbols: vec![symbol
                    .clone()
                    .strip_suffix("^")
                    .unwrap_or(&symbol)
                    .to_string()
                    .strip_suffix("_")
                    .unwrap_or(symbol.clone().strip_suffix("^").unwrap_or(&symbol))
                    .to_string()],
                directions: vec![Direction::Right],
            };
            if !new_transitions.contains(&new_transition) {
                new_transitions.push(new_transition);
            }
            let new_transition = Transition {
                state: state_final_1.clone(),
                symbols: vec![symbol.clone()],
                new_state: state_final_1.clone(),
                new_symbols: vec![tm.blank_symbol.clone()],
                directions: vec![Direction::Right],
            };
            if !new_transitions.contains(&new_transition) {
                new_transitions.push(new_transition);
            }
        }
        let new_transition = Transition {
            state: state.clone(),
            symbols: vec![tm.blank_symbol.clone()],
            new_state: state_final_1.clone(),
            new_symbols: vec![tm.blank_symbol.clone()],
            directions: vec![Direction::Stay],
        };
        if !new_transitions.contains(&new_transition) {
            new_transitions.push(new_transition);
        }
        let new_transition = Transition {
            state: state.clone(),
            symbols: vec![tape_sep_symbol.clone()],
            new_state: state_final_1.clone(),
            new_symbols: vec![tm.blank_symbol.clone()],
            directions: vec![Direction::Right],
        };
        if !new_transitions.contains(&new_transition) {
            new_transitions.push(new_transition);
        }
        let new_transition = Transition {
            state: state_final_1.clone(),
            symbols: vec![tape_sep_symbol.clone()],
            new_state: state_final_1.clone(),
            new_symbols: vec![tm.blank_symbol.clone()],
            directions: vec![Direction::Right],
        };
        if !new_transitions.contains(&new_transition) {
            new_transitions.push(new_transition);
        }
        let new_transition = Transition {
            state: state_final_1.clone(),
            symbols: vec![tm.blank_symbol.clone()],
            new_state: state_final_2.clone(),
            new_symbols: vec![tm.blank_symbol.clone()],
            directions: vec![Direction::Right],
        };
        if !new_transitions.contains(&new_transition) {
            new_transitions.push(new_transition);
        }
    }
    if !states_vec.contains(&tm.initial_state) {
        states_vec.push(tm.initial_state.clone());
    }
    for state in &new_states {
        if !states_vec.contains(state) {
            states_vec.push(state.clone());
        }
    }
    new_tm.states = states_vec.clone();
    new_tm.tape_alphabet = new_tape_alphabet.clone();
    new_tm.transitions = new_transitions.clone();
    new_tm.final_states = real_final_states.clone();
    new_tm
}

pub fn encoding_to_tm(encoding: String) -> TuringMachine {
    let mut tm = TuringMachine {
        initial_state: "".to_string(),
        accept_state: "".to_string(),
        reject_state: "".to_string(),
        final_states: Vec::new(),
        states: Vec::new(),
        input_alphabet: Vec::new(),
        transitions: Vec::new(),
        blank_symbol: " ".to_string(),
        tape_alphabet: Vec::new(),
        end_on_final_state: false,
        tape_count: 1,
        last_execution: ("".to_string(), Vec::new(), 0, Vec::new()),
    };
    let mut transitions: Vec<&str> = encoding.split(")").collect();
    transitions.pop();
    for transition in transitions {
        let transition = transition.trim();
        let transition = transition.strip_prefix("(").unwrap();
        let mut transition = transition.split(";");
        let state = transition.next().unwrap().to_string();
        let mut new_state = String::new();
        let mut symbols = Vec::new();
        let mut found_all = false;
        while !found_all {
            let symbol = transition.next().unwrap().to_string();
            if symbol.starts_with("a") || symbol.starts_with("t") || symbol.starts_with("b") {
                symbols.push(symbol);
            } else {
                found_all = true;
                new_state = symbol.to_string();
            }
        }
        tm.tape_count = symbols.len();
        let mut new_symbols = Vec::new();
        for _ in 0..tm.tape_count {
            new_symbols.push(transition.next().unwrap().to_string());
        }
        let mut directions = Vec::new();
        for _ in 0..tm.tape_count {
            let direction = transition.next().unwrap();
            match direction {
                "L" => directions.push(Direction::Left),
                "R" => directions.push(Direction::Right),
                "S" => directions.push(Direction::Stay),
                _ => (),
            }
        }
        let new_transition = Transition {
            state: state.to_string(),
            symbols: symbols.clone(),
            new_state: new_state.to_string(),
            new_symbols: new_symbols.clone(),
            directions: directions.clone(),
        };
        if !tm.states.contains(&state.to_string()) {
            tm.states.push(state.to_string());
        }
        if state.starts_with("y") {
            tm.accept_state = state.to_string();
            tm.final_states.push(state.to_string());
        } else if state.starts_with("n") {
            tm.reject_state = state.to_string();
            tm.final_states.push(state.to_string());
        } else if state.starts_with("h") {
            tm.final_states.push(state.to_string());
        } else if state.starts_with("i") {
            tm.initial_state = state.to_string();
        }
        if !tm.states.contains(&new_state.to_string()) {
            tm.states.push(new_state.to_string());
        }
        if state.starts_with("y") {
            tm.accept_state = state.to_string();
            tm.final_states.push(state.to_string());
        } else if state.starts_with("n") {
            tm.reject_state = state.to_string();
            tm.final_states.push(state.to_string());
        } else if state.starts_with("h") {
            tm.final_states.push(state.to_string());
        }
        for symbol in symbols {
            if !tm.tape_alphabet.contains(&symbol) {
                tm.tape_alphabet.push(symbol.clone());
            }
            if symbol.starts_with("a") && !tm.input_alphabet.contains(&symbol) {
                tm.input_alphabet.push(symbol.clone());
            } else if symbol.starts_with("b") {
                tm.blank_symbol = symbol.clone();
            }
        }
        for symbol in new_symbols {
            if !tm.tape_alphabet.contains(&symbol) {
                tm.tape_alphabet.push(symbol.clone());
            }
            if symbol.starts_with("a") && !tm.input_alphabet.contains(&symbol) {
                tm.input_alphabet.push(symbol.clone());
            } else if symbol.starts_with("b") {
                tm.blank_symbol = symbol.clone();
            }
        }
        tm.transitions.push(new_transition);
    }
    tm
}

pub fn encoding_to_orig(
    encoding: String,
    orig_alphabet_encoding: std::collections::HashMap<String, String>,
    orig_state_encoding: std::collections::HashMap<String, String>,
) -> TuringMachine {
    let tm = encoding_to_tm(encoding);
    let mut orig_tm: TuringMachine = TuringMachine {
        initial_state: orig_state_encoding[&tm.initial_state].clone(),
        accept_state: "".to_string(),
        reject_state: "".to_string(),
        final_states: tm
            .final_states
            .iter()
            .map(|state| orig_state_encoding[state].clone())
            .collect(),
        states: tm
            .states
            .iter()
            .map(|state| orig_state_encoding[state].clone())
            .collect(),
        input_alphabet: tm
            .input_alphabet
            .iter()
            .map(|symbol| orig_alphabet_encoding[symbol].clone())
            .collect(),
        transitions: tm
            .transitions
            .iter()
            .map(|transition| Transition {
                state: orig_state_encoding[&transition.state].clone(),
                symbols: transition
                    .symbols
                    .iter()
                    .map(|symbol| orig_alphabet_encoding[symbol].clone())
                    .collect(),
                new_state: orig_state_encoding[&transition.new_state].clone(),
                new_symbols: transition
                    .new_symbols
                    .iter()
                    .map(|symbol| orig_alphabet_encoding[symbol].clone())
                    .collect(),
                directions: transition.directions.clone(),
            })
            .collect(),
        blank_symbol: orig_alphabet_encoding[&tm.blank_symbol].clone(),
        tape_alphabet: tm
            .tape_alphabet
            .iter()
            .map(|symbol| orig_alphabet_encoding[symbol].clone())
            .collect(),
        end_on_final_state: tm.end_on_final_state,
        tape_count: tm.tape_count,
        last_execution: ("".to_string(), Vec::new(), 0, Vec::new()),
    };
    if !tm.accept_state.is_empty() {
        orig_tm.accept_state = orig_state_encoding[&tm.accept_state].clone();
    }
    if !tm.reject_state.is_empty() {
        orig_tm.reject_state = orig_state_encoding[&tm.reject_state].clone();
    }
    orig_tm
}

pub fn ram_instruction_lookup(instruction: String) -> String {
    let opcode = match instruction.as_str() {
        "R" => "0000",
        "MIR" => "0001",
        "MIL" => "0010",
        "W" => "0011",
        "L" => "0100",
        "A" => "0101",
        "S" => "0110",
        "INIT" => "0111",
        "ST" => "1000",
        "JUMP" => "1001",
        "CJUMP" => "1010",
        "H" => "1011",
        _ => "0000",
    };
    opcode.to_string()
}

impl Executable for RamMachine {
    fn simulate(&mut self, input: Vec<String>, max_steps: i32) -> (String, i32) {
        let mut ir;
        let r#in: String = input.join("");
        let mut out: String = "".to_string();
        let mut pc: String = "0000000000000000".to_string();
        let mut acc: String = "0000000000000000".to_string();
        let mut ar: String;
        let mut input_head = 0;
        let mut memory: std::collections::HashMap<String, String> =
            std::collections::HashMap::new();
        for (index, instr) in self.instructions.clone().into_iter().enumerate() {
            memory.insert(
                utils::int2bin(index as i32, 16),
                instr.opcode.clone() + &instr.operand.clone(),
            );
        }
        let mut steps = 0;
        while steps < max_steps {
            steps += 1;
            ir = memory[&pc].clone()[0..4].to_string();
            ar = memory[&pc].clone()[4..].to_string();
            pc = utils::int2bin(utils::bin2int(pc) + 1, 16);
            match ir.as_str() {
                "0000" => {
                    // R: Read [operands] bit from input
                    let end = input_head + (utils::bin2int(ar) as usize);
                    acc = r#in[input_head..end].to_string();
                }
                "0001" => {
                    // MIR: move input head [operands] bits to the right
                    input_head += utils::bin2int(ar) as usize;
                }
                "0010" => {
                    // MIL: move input head [operands] bits to the left
                    input_head -= utils::bin2int(ar) as usize;
                }
                "0011" => {
                    // W: Write ACC to output
                    out = out + &acc.clone();
                }
                "0100" => {
                    // L: Load AR to ACC
                    acc = memory[&ar].clone();
                }
                "0101" => {
                    // A: Add AR to ACC
                    acc = utils::int2bin(
                        utils::bin2int(acc) + utils::bin2int(memory[&ar].clone()),
                        16,
                    );
                }
                "0110" => {
                    // S: Subtract AR from ACC
                    acc = utils::int2bin(
                        utils::bin2int(acc) - (utils::bin2int(memory[&ar].clone())),
                        16,
                    );
                }
                "0111" => {
                    // INIT: Initialize ACC to [operands]
                    acc = ar.clone();
                }
                "1000" => {
                    // ST: Store ACC to AR
                    memory.insert(ar.clone(), acc.clone());
                }
                "1001" => {
                    // JUMP: Jump to AR
                    pc = ar.clone();
                }
                "1010" => {
                    // CJUMP: Conditional jump to AR if ACC is 0000
                    if !acc.contains("1") {
                        pc = ar.clone();
                    }
                }
                "1011" => {
                    // HALT: Halt
                    break;
                }
                _ => {
                    // default: Halt
                    break;
                }
            }
        }
        (out, steps)
    }

    fn to_encoding(&self) -> (String, std::collections::HashMap<String, String>, std::collections::HashMap<String, String>) {
        let mut encoding = "#".to_string();
        let mut counter = 0;
        for instr in self.instructions.clone() {
            let counter_number_bits = if counter > 0 { (counter as f32).log2().ceil() as i32 } else { 1 };
            if instr.opcode == "1011" || instr.opcode == "0011" { // Write and Halt does not have operands
                encoding = encoding + &utils::int2bin(counter, (counter_number_bits) as usize) + "," + &instr.opcode + "#";
            } else {
                encoding = encoding + &utils::int2bin(counter, (counter_number_bits) as usize) + "," + &instr.opcode + &(utils::int2bin(utils::bin2int(instr.operand), 0)) + "#";
            }
            counter += 1;
        }
        (encoding, std::collections::HashMap::new(), std::collections::HashMap::new())
    }
}
