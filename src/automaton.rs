// file: automaton.rs
// Project: Computing Simulator
// author: dp
// date: 2025-03-06

#[derive(Clone)]
pub struct TuringMachine {
    pub initial_state: String,
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
        max_steps: i32
    ) -> (String, Vec<Tape>, i32, Vec<Configuration>);
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

impl Automaton for TuringMachine {
    fn simulate(
        &self,
        input: Vec<String>,
        max_steps: i32
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
                tape: Vec::new(),
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
                for tapenum in 0..self.tape_count{
                    key += &element.tapes[tapenum].tape[element.tapes[tapenum].head];
                }
                let mut found = false;
                if transitions_map.contains_key(&key) {
                    found = true;
                    let possible_transitions = transitions_map.get(&key).unwrap().clone();
                    for transition in possible_transitions.iter() {
                        let mut new_tapes = Vec::new();
                        for tapenum in 0..self.tape_count{
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
                if !matches!(direction, Direction::Left | Direction::Right | Direction::Stay) {
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
