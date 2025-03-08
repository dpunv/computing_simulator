// file: automaton.rs
// Project: Computing Simulator
// author: dp
// date: 2025-03-06

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

pub trait Automaton: Clone {
    fn simulate(
        &self,
        input: Vec<String>,
        max_steps: i32,
    ) -> (String, Vec<Tape>, i32, Vec<Configuration>);
    fn is_deterministic(&self) -> bool;
    fn is_transition_total(&self) -> bool;
    fn is_ok(&self) -> bool;
    fn make_transition_map(&self) -> std::collections::HashMap<String, Vec<Transition>>;
    fn input_alphabet(&self) -> Vec<String>;
    fn to_encoding(
        &self,
    ) -> (
        String,
        std::collections::HashMap<String, String>,
        std::collections::HashMap<String, String>,
    );
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

impl Automaton for TuringMachine {
    fn simulate(
        &self,
        input: Vec<String>,
        max_steps: i32,
    ) -> (String, Vec<Tape>, i32, Vec<Configuration>) {
        #[derive(Clone)]
        struct TreeElement {
            state: String,
            tapes: Vec<Tape>,
            previous: usize,
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
                        new_level.push(TreeElement {
                            state: new_state,
                            tapes: new_tapes,
                            previous: ind,
                        });
                    }
                }
                if !found {
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
        (
            last_element.state.clone(),
            last_element.tapes.clone(),
            steps,
            computation,
        )
    }

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
                    break;
                }
            }
            for symbol in &transition.new_symbols {
                if !self.tape_alphabet.contains(symbol) {
                    is_transitions_valid = false;
                    break;
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

    fn to_encoding(
        &self,
    ) -> (
        String,
        std::collections::HashMap<String, String>,
        std::collections::HashMap<String, String>,
    ) {
        // find encoding for states: find the maximum number of bit needed to represent the number of states
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
        // encode transitions as (state;symbols;new_state;new_symbols;directions)
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

/* fn convert_multi_tape_to_single_tape_tm(tm: TuringMachine) -> TuringMachine {
    let mut new_tm = tm.clone();
    let mut new_transitions = Vec::new();
    for transition in tm.transitions {
        let mut new_symbols = Vec::new();
        let mut new_directions = Vec::new();
        for symbol in transition.symbols {
            new_symbols.push(symbol.clone());
        }
        for symbol in transition.new_symbols {
            new_symbols.push(symbol.clone());
        }
        for direction in transition.directions {
            new_directions.push(direction.clone());
        }
        new_transitions.push(Transition {
            state: transition.state,
            symbols: new_symbols.clone(),
            new_state: transition.new_state,
            new_symbols: new_symbols.clone(),
            directions: new_directions.clone(),
        });
    }
    new_tm.transitions = new_transitions;
    new_tm.tape_count = 1;
    new_tm
}
 */

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
    };
    if !tm.accept_state.is_empty() {
        orig_tm.accept_state = orig_state_encoding[&tm.accept_state].clone();
    }
    if !tm.reject_state.is_empty() {
        orig_tm.reject_state = orig_state_encoding[&tm.reject_state].clone();
    }
    orig_tm
}
