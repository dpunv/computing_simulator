// file: automaton.rs
// Project: Computing Simulator
// author: dp

use crate::computer;
use crate::utils;

#[derive(Clone)]
pub struct TuringMachine {
    pub initial_state: String,
    pub accept_state: String,
    pub reject_state: String,
    pub halt_state: String,
    pub blank_symbol: String,
    pub states: Vec<String>,
    pub input_alphabet: Vec<String>,
    pub tape_alphabet: Vec<String>,
    pub transitions: Vec<Transition>,
    pub tape_count: usize,
    pub next_state_id: usize,
}

#[derive(Clone)]
pub struct Tape {
    pub tape: Vec<String>,
    pub head: usize,
}

#[derive(Clone, Debug)]
pub struct Transition {
    pub state: String,
    pub symbols: Vec<String>,
    pub new_state: String,
    pub new_symbols: Vec<String>,
    pub directions: Vec<Direction>,
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

#[derive(Clone, Debug)]
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

#[derive(Clone)]
struct TreeElement {
    state: String,
    tapes: Vec<Tape>,
    computation: Vec<String>,
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

/* fn print_tree(tree: &Vec<Vec<TreeElement>>) {
    for (ind, level) in tree.iter().enumerate() {
        println!("Level {}", ind);
        for (ind2, element) in level.iter().enumerate() {
            println!("{} {} {}", ind2, element.state, element.tapes[0].tape[element.tapes[0].head]);
        }
    }
} */

impl TuringMachine {
    pub fn new() -> Self {
        TuringMachine {
            initial_state: "".to_string(),
            accept_state: "".to_string(),
            reject_state: "".to_string(),
            halt_state: "".to_string(),
            blank_symbol: "".to_string(),
            states: Vec::new(),
            input_alphabet: Vec::new(),
            tape_alphabet: Vec::new(),
            transitions: Vec::new(),
            tape_count: 1,
            next_state_id: 0,
        }
    }
    pub fn add_state(&mut self) -> String {
        let state = format!("state {}", self.next_state_id);
        self.states.push(state.clone());
        self.next_state_id += 1;
        state
    }

    pub fn is_final(&self, state: &String) -> bool {
        *state == self.accept_state || *state == self.reject_state || *state == self.halt_state
    }

    pub fn final_states(&self) -> Vec<String> {
        vec![
            self.accept_state.clone(),
            self.reject_state.clone(),
            self.halt_state.clone(),
        ]
    }

    pub fn add_transition(
        &mut self,
        state: String,
        symbols: Vec<String>,
        new_state: String,
        new_symbols: Vec<String>,
        directions: Vec<Direction>,
    ) {
        let transition = Transition {
            state,
            symbols,
            new_state,
            new_symbols,
            directions,
        };
        if !self.transitions.contains(&transition) {
            self.transitions.push(transition);
        }
    }

    pub fn simulate(
        self,
        input: Vec<String>,
        max_steps: usize,
        this_computer_object: computer::Computer,
        context: computer::Server,
        prev_head: usize,
    ) -> Result<computer::SimulationResult, String> {
        if max_steps == 0 {
            return Err("max steps should be greater than 0".to_string());
        }
        let transitions_map = self.make_transition_map();
        /* if transitions_map.is_empty(){
            return Err("empty transition function".to_string());
        } */
        let mut tree = Vec::new();
        tree.push(Vec::new());
        let det = self.is_deterministic();
        let mut tape = Vec::new();
        if input.is_empty() || input[0] != self.blank_symbol {
            tape.push(self.blank_symbol.clone());
        }
        for symbol in input {
            tape.push(symbol);
        }
        let mut tapes = Vec::new();
        tapes.push(Tape {
            tape: tape.clone(),
            head: prev_head,
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
            computation: vec![
                "tm;".to_string()
                    + &self.initial_state.clone()
                    + ";"
                    + &tapes[0].tape.clone().join(""),
            ],
        });
        let mut steps = 0;
        let mut halts = false;
        while steps < max_steps && !halts {
            steps += 1;
            let mut new_level = Vec::new();
            for (ind, element) in tree[tree.len() - 1].iter().enumerate() {
                let state = element.state.clone();
                if self.is_final(&state) && (self.is_deterministic() || state == self.accept_state)
                {
                    halts = true;
                    break;
                }
                let mut key = state.clone();
                for tapenum in 0..self.tape_count {
                    // println!("tapes[{}] = {}      Head: {}", tapenum, element.tapes[tapenum].tape.join(""), element.tapes[tapenum].head);
                    key += &element.tapes[tapenum].tape[element.tapes[tapenum].head];
                }
                let mut found = false;
                // println!("STEP: {}, key: {}", steps, key);
                if transitions_map.contains_key(&key) {
                    found = true;
                    let possible_transitions =
                        transitions_map.get(&key).unwrap_or(&Vec::new()).clone();
                    for transition in possible_transitions.iter() {
                        let mut this_computation = element.computation.clone();
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
                        this_computation.push(
                            "tm;".to_string()
                                + &new_state.clone()
                                + ";"
                                + &new_tapes[0].tape.clone().join(""),
                        );
                        let subroutine_name: String = this_computer_object
                            .clone()
                            .get_mapping(new_state.clone())?;
                        if subroutine_name != *"" {
                            let remaining_steps = max_steps - steps;
                            let subroutine = context
                                .clone()
                                .get_computer(subroutine_name.clone())
                                .ok_or_else(|| {
                                    format!("cannot get computer with name '{}'", subroutine_name)
                                })?
                                .clone();
                            let new_tape_input = if subroutine.is_ram() {
                                new_tapes[0]
                                    .tape
                                    .clone()
                                    .into_iter()
                                    .filter(|symb| *symb != self.blank_symbol.clone())
                                    .collect::<Vec<String>>()
                                    .join("")
                            } else {
                                new_tapes[0].tape.clone().join("")
                            };
                            let (_, head_result, tape_result, steps_result, sub_computation) =
                                subroutine.clone().simulate(
                                    new_tape_input,
                                    remaining_steps,
                                    context.clone(),
                                    new_tapes[0].head,
                                )?;
                            this_computation.extend(sub_computation);
                            if subroutine.is_ram() {
                                new_tapes[0].tape = [
                                    vec![self.blank_symbol.clone()],
                                    utils::input_string_to_vec(
                                        self.input_alphabet.clone(),
                                        tape_result[0].clone(),
                                    ),
                                ]
                                .concat();
                            } else {
                                new_tapes[0].tape = tape_result;
                            }
                            new_tapes[0].head = head_result;
                            steps += steps_result;
                        }
                        let el = TreeElement {
                            state: new_state,
                            tapes: new_tapes,
                            computation: this_computation,
                        };
                        if !new_level.contains(&el) {
                            new_level.push(el);
                        }
                    }
                }
                if !found && new_level.is_empty() && ind == tree[tree.len() - 1].len() - 1 {
                    halts = true;
                    break;
                }
            }
            if det && !halts && steps < max_steps {
                tree.pop();
            }
            tree.push(new_level);
        }
        tree.pop();
        let mut previous = 0;
        let mut changed = false;
        if !det {
            for (ind, element) in tree[tree.len() - 1].clone().into_iter().enumerate() {
                if element.state == self.accept_state {
                    previous = ind;
                    break;
                } else if element.state == self.reject_state && !changed {
                    previous = ind;
                } else if self.is_final(&element.state) {
                    previous = ind;
                    changed = true;
                }
            }
        }
        let last_element = tree[tree.len() - 1][previous].clone();
        if self.accept_state == last_element.state.clone() {
            Ok((
                "accept".to_string(),
                last_element.tapes[0].head,
                last_element.tapes[0].tape.clone(),
                steps,
                last_element.computation,
            ))
        } else if self.reject_state == last_element.state.clone() {
            Ok((
                "reject".to_string(),
                last_element.tapes[0].head,
                last_element.tapes[0].tape.clone(),
                steps,
                last_element.computation,
            ))
        } else if self.is_final(&last_element.state.clone()) {
            Ok((
                "halt".to_string(),
                last_element.tapes[0].head,
                last_element.tapes[0].tape.clone(),
                steps,
                last_element.computation,
            ))
        } else {
            Ok((
                last_element.state.clone(),
                last_element.tapes[0].head,
                last_element.tapes[0].tape.clone(),
                steps,
                last_element.computation,
            ))
        }
    }

    pub fn to_encoding(&self) -> Result<computer::EncodingResult, String> {
        let mut state_bits: usize = 0;
        let mut states = self.states.len();
        while states > 0 {
            states >>= 1;
            state_bits += 1;
        }
        let mut state_encoding: std::collections::HashMap<String, String> =
            std::collections::HashMap::new();
        for (index, state) in self.states.iter().enumerate() {
            if self.is_final(state) && state != &self.accept_state && state != &self.reject_state {
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
            transition_encoding.push_str(
                state_encoding
                    .get(&transition.state)
                    .ok_or(format!("key not found: {}", transition.state))?,
            );
            transition_encoding.push(';');
            for symbol in &transition.symbols {
                transition_encoding.push_str(
                    tape_encoding
                        .get(symbol)
                        .ok_or(format!("key not found: {}", symbol))?,
                );
                transition_encoding.push(';');
            }
            transition_encoding.push_str(
                state_encoding
                    .get(&transition.new_state)
                    .ok_or(format!("key not found: {}", transition.new_state))?,
            );
            transition_encoding.push(';');
            for symbol in &transition.new_symbols {
                transition_encoding.push_str(
                    tape_encoding
                        .get(symbol)
                        .ok_or(format!("key not found: {}", symbol))?,
                );
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
        Ok((transitions_encoding, tape_encoding, state_encoding))
    }

    pub fn number(&self) -> Result<i32, String> {
        let alphabet = vec![
            "0".to_string(),
            "1".to_string(),
            ";".to_string(),
            "(".to_string(),
            ")".to_string(),
            "a".to_string(),
            "b".to_string(),
            "t".to_string(),
            "y".to_string(),
            "n".to_string(),
            "h".to_string(),
            "i".to_string(),
            "R".to_string(),
            "L".to_string(),
            "S".to_string(),
        ];
        let mut p = 0;
        let mut i = 0;
        let mut tm_string = "".to_string();
        let encoding = self.to_encoding()?.0;
        while tm_string != encoding {
            i += 1;
            tm_string = utils::uint2str(i, alphabet.clone())?;
            if TuringMachine::check_tm_encoding(tm_string.clone())? {
                p += 1;
            }
        }
        Ok(p)
    }

    /* pub fn input_alphabet(&self) -> Vec<String> {
        self.input_alphabet.clone()
    } */

    pub fn make_transition_map(&self) -> std::collections::HashMap<String, Vec<Transition>> {
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
                    .unwrap_or(&mut Vec::new())
                    .push(transition.clone());
            } else {
                transition_map.insert(key.clone(), vec![transition.clone()]);
            }
        }
        transition_map
    }

    pub fn is_ok(&self) -> bool {
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

        if !(self.accept_state.is_empty() || self.states.contains(&self.accept_state))
            || !(self.reject_state.is_empty() || self.states.contains(&self.reject_state))
            || !(self.halt_state.is_empty() || self.states.contains(&self.halt_state))
        {
            is_final_states_valid = false;
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

    pub fn is_deterministic(&self) -> bool {
        let transition_map = self.make_transition_map();
        for transitions in transition_map.values() {
            if transitions.len() > 1 {
                return false;
            }
        }
        true
    }

    pub fn is_transition_total(&self) -> bool {
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

    pub fn convert_multi_tape_to_single_tape_tm(&self) -> Result<TuringMachine, String> {
        let initial_state_fake = self.initial_state.clone() + "<FAKE>";
        let mut new_tm = TuringMachine {
            initial_state: initial_state_fake.clone(),
            accept_state: self.accept_state.clone(),
            reject_state: self.reject_state.clone(),
            halt_state: self.halt_state.clone(),
            blank_symbol: self.blank_symbol.clone(),
            states: Vec::new(),
            input_alphabet: self.input_alphabet.clone(),
            tape_alphabet: Vec::new(),
            transitions: Vec::new(),
            tape_count: 1,
            next_state_id: 0,
        };
        let head_symbols = vec!["^".to_string(), "_".to_string()];
        let mut new_compound_symbols = Vec::new();
        for symbol in &self.tape_alphabet {
            for head_symbol in &head_symbols {
                new_compound_symbols.push(symbol.clone() + head_symbol);
            }
        }
        let mut new_tape_alphabet = new_compound_symbols.clone();
        for tape_symbol in &self.tape_alphabet {
            new_tape_alphabet.push(tape_symbol.clone());
        }
        let tape_sep_symbol = "#".to_string();
        new_tape_alphabet.push(tape_sep_symbol.clone());
        new_tm.tape_alphabet = new_tape_alphabet.clone();
        let mut new_states = Vec::new();
        for tapenum in 0..self.tape_count {
            let initial_state_tape =
                initial_state_fake.clone() + "<INIT_TP" + &tapenum.to_string() + "_START>";
            let end_state_tape =
                initial_state_fake.clone() + "<INIT_TP" + &tapenum.to_string() + "_END>";
            new_states.push(initial_state_tape.clone());
            new_states.push(end_state_tape.clone());
            if tapenum == 0 {
                for symbol in &self.tape_alphabet {
                    new_tm.add_transition(
                        initial_state_fake.clone(),
                        vec![symbol.clone()],
                        initial_state_tape.clone(),
                        vec![symbol.clone() + "^"],
                        vec![Direction::Right],
                    );
                    if *symbol != self.blank_symbol.clone() {
                        new_tm.add_transition(
                            initial_state_tape.clone(),
                            vec![symbol.clone()],
                            initial_state_tape.clone(),
                            vec![symbol.clone() + "_"],
                            vec![Direction::Right],
                        );
                    }
                    new_tm.add_transition(
                        initial_state_tape.clone(),
                        vec![self.blank_symbol.clone()],
                        end_state_tape.clone(),
                        vec![self.blank_symbol.clone()],
                        vec![Direction::Stay],
                    );
                }
            } else {
                new_tm.add_transition(
                    initial_state_fake.clone() + "<INIT_TP" + &(tapenum - 1).to_string() + "_END>",
                    vec![self.blank_symbol.clone()],
                    initial_state_tape.clone(),
                    vec![tape_sep_symbol.clone()],
                    vec![Direction::Right],
                );
                new_tm.add_transition(
                    initial_state_tape.clone(),
                    vec![self.blank_symbol.clone()],
                    end_state_tape.clone(),
                    vec![self.blank_symbol.clone() + "^"],
                    vec![Direction::Right],
                );
            }
        }
        let setup_state = initial_state_fake.clone() + "<SETUP>";
        new_states.push(setup_state.clone());
        // let new_start_state = initial_state_fake.clone() + "<START>";
        // new_states.push(self.initial_state.clone());
        for symbol in new_tape_alphabet.clone() {
            if symbol != self.blank_symbol {
                new_tm.add_transition(
                    setup_state.clone(),
                    vec![symbol.clone()],
                    setup_state.clone(),
                    vec![symbol.clone()],
                    vec![Direction::Left],
                );
            } else {
                new_tm.add_transition(
                    initial_state_fake.clone()
                        + "<INIT_TP"
                        + (self.tape_count - 1).to_string().as_str()
                        + "_END>",
                    vec![self.blank_symbol.clone()],
                    setup_state.clone(),
                    vec![self.blank_symbol.clone()],
                    vec![Direction::Left],
                );
                new_tm.add_transition(
                    setup_state.clone(),
                    vec![self.blank_symbol.clone()],
                    self.initial_state.clone(),
                    vec![self.blank_symbol.clone()],
                    vec![Direction::Right],
                );
            }
        }
        let mut states_to_process = Vec::new();
        for state in &self.states {
            if state != &initial_state_fake {
                states_to_process.push(state.clone());
            } else {
                states_to_process.push(self.initial_state.clone());
            }
        }
        let mut map_states: std::collections::HashMap<String, Vec<String>> =
            std::collections::HashMap::new();
        let mut states_vec = states_to_process.clone();
        for state in self.final_states() {
            if states_to_process.contains(&state) {
                states_to_process.retain(|x| x != &state);
            }
        }
        let mut states_to_copy: Vec<String> = Vec::new();
        for state in states_to_process {
            map_states.insert(state.clone() + "0", vec![state.clone()]);
            for tapenum in 0..self.tape_count {
                let mut this_state_vec = Vec::new();
                for symbol in &new_compound_symbols {
                    for actual_state in map_states
                        .get(&(state.clone() + &tapenum.to_string()))
                        .ok_or(format!(
                            "key not found: {}",
                            (state.clone() + &tapenum.to_string())
                        ))?
                        .clone()
                    {
                        let state_tape =
                            actual_state.clone() + "<R_TP" + &tapenum.to_string() + ">";
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
                        new_tm.add_transition(
                            new_state.clone(),
                            vec![self.blank_symbol.clone()],
                            end_state.clone(),
                            vec![self.blank_symbol.clone()],
                            vec![Direction::Stay],
                        );
                        if tapenum == 0 {
                            new_tm.add_transition(
                                state.clone(),
                                vec![symbol.clone()],
                                state_tape.clone(),
                                vec![symbol.clone()],
                                vec![Direction::Stay],
                            );
                        } else {
                            new_tm.add_transition(
                                actual_state.clone(),
                                vec![symbol.clone()],
                                state_tape.clone(),
                                vec![symbol.clone()],
                                vec![Direction::Stay],
                            );
                        }
                        if symbol.ends_with("^") {
                            new_tm.add_transition(
                                state_tape.clone(),
                                vec![symbol.clone()],
                                new_state.clone(),
                                vec![symbol.clone()],
                                vec![Direction::Right],
                            );
                        } else {
                            new_tm.add_transition(
                                state_tape.clone(),
                                vec![symbol.clone()],
                                state_tape.clone(),
                                vec![symbol.clone()],
                                vec![Direction::Right],
                            );
                        }
                        for symb in new_compound_symbols.clone() {
                            if !symb.ends_with("^") {
                                new_tm.add_transition(
                                    new_state.clone(),
                                    vec![symb.clone()],
                                    new_state.clone(),
                                    vec![symb.clone()],
                                    vec![Direction::Right],
                                );
                            }
                        }
                        new_tm.add_transition(
                            new_state.clone(),
                            vec![tape_sep_symbol.clone()],
                            end_state.clone(),
                            vec![tape_sep_symbol.clone()],
                            vec![Direction::Right],
                        );
                    }
                }
                map_states.insert(
                    state.clone() + &(tapenum + 1).to_string(),
                    this_state_vec.clone(),
                );
            }
            let old_transition_map = self.make_transition_map();
            let mut states_done = Vec::new();
            for actual_state in map_states
                .get(&(state.clone() + &self.tape_count.to_string()))
                .ok_or(format!(
                    "key not found: {}",
                    (state.clone() + &self.tape_count.to_string())
                ))?
                .clone()
            {
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
                    let transitions = old_transition_map
                        .get(&key)
                        .ok_or(format!("key not found: {}", key))?
                        .clone();
                    for (ind, t) in transitions.iter().enumerate() {
                        for tapenum in 0..self.tape_count {
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
                                        new_tm.add_transition(
                                            state_init_tape.clone(),
                                            vec![t.symbols[tapenum].clone() + "^"],
                                            state_mid_tape.clone(),
                                            vec![t.new_symbols[tapenum].clone() + "_"],
                                            vec![Direction::Right],
                                        );
                                        new_tm.add_transition(
                                            state_mid_tape.clone(),
                                            vec![tape_sep_symbol.clone()],
                                            state_mid_mid_tape.clone(),
                                            vec![tape_sep_symbol.clone()],
                                            vec![Direction::Left],
                                        );
                                        new_tm.add_transition(
                                            state_mid_tape.clone(),
                                            vec![self.blank_symbol.clone()],
                                            state_mid_mid_tape.clone(),
                                            vec![self.blank_symbol.clone()],
                                            vec![Direction::Left],
                                        );
                                        for symb in new_compound_symbols.clone() {
                                            new_tm.add_transition(
                                                state_mid_mid_tape.clone(),
                                                vec![symb.clone()],
                                                state_mid_mid_tape.clone() + "<COPY_CYCLE_RIGHT>",
                                                vec![symb.clone() + "<COPY>"],
                                                vec![Direction::Right],
                                            );
                                            states_to_copy.push(state_mid_mid_tape.clone());
                                            if !symb.ends_with("^") {
                                                new_tm.add_transition(
                                                    state_mid_tape.clone(),
                                                    vec![
                                                        symb.clone()
                                                            .strip_suffix("_")
                                                            .unwrap_or(&symb)
                                                            .to_string()
                                                            + "_",
                                                    ],
                                                    state_end_tape.clone(),
                                                    vec![
                                                        symb.clone()
                                                            .strip_suffix("_")
                                                            .unwrap_or(&symb)
                                                            .to_string()
                                                            + "^",
                                                    ],
                                                    vec![Direction::Left],
                                                );
                                            }
                                        }
                                    } else if t.directions[tapenum] == Direction::Left {
                                        new_tm.add_transition(
                                            state_init_tape.clone(),
                                            vec![t.symbols[tapenum].clone() + "^"],
                                            state_mid_tape.clone(),
                                            vec![t.new_symbols[tapenum].clone() + "_"],
                                            vec![Direction::Left],
                                        );
                                        new_tm.add_transition(
                                            state_mid_tape.clone(),
                                            vec![tape_sep_symbol.clone()],
                                            state_mid_tape.clone() + "<COPY_CYCLE_RIGHT>",
                                            vec![tape_sep_symbol.clone() + "<COPY>"],
                                            vec![Direction::Right],
                                        );
                                        states_to_copy.push(state_mid_tape.clone());
                                        for symb in new_compound_symbols.clone() {
                                            if !symb.ends_with("^") {
                                                new_tm.add_transition(
                                                    state_mid_tape.clone(),
                                                    vec![
                                                        symb.clone()
                                                            .strip_suffix("_")
                                                            .unwrap_or(&symb)
                                                            .to_string()
                                                            + "_",
                                                    ],
                                                    state_end_tape.clone(),
                                                    vec![
                                                        symb.clone()
                                                            .strip_suffix("_")
                                                            .unwrap_or(&symb)
                                                            .to_string()
                                                            + "^",
                                                    ],
                                                    vec![Direction::Left],
                                                );
                                            }
                                        }
                                        new_tm.add_transition(
                                            state_mid_tape.clone(),
                                            vec![self.blank_symbol.clone()],
                                            state_end_tape.clone(),
                                            vec![self.blank_symbol.clone() + "^"],
                                            vec![Direction::Left],
                                        );
                                    } else {
                                        new_tm.add_transition(
                                            state_init_tape.clone(),
                                            vec![t.symbols[tapenum].clone() + "^"],
                                            state_end_tape.clone(),
                                            vec![t.new_symbols[tapenum].clone() + "^"],
                                            vec![Direction::Left],
                                        );
                                    }
                                } else {
                                    new_tm.add_transition(
                                        state_init_tape.clone(),
                                        vec![symbol.clone()],
                                        state_init_tape.clone(),
                                        vec![symbol.clone()],
                                        vec![Direction::Left],
                                    );
                                    new_tm.add_transition(
                                        state_end_tape.clone(),
                                        vec![symbol.clone()],
                                        state_end_tape.clone(),
                                        vec![symbol.clone()],
                                        vec![Direction::Left],
                                    );
                                }
                            }
                            if tapenum == 0 {
                                new_tm.add_transition(
                                    state_end_tape.clone(),
                                    vec![self.blank_symbol.clone()],
                                    t.new_state.clone(),
                                    vec![self.blank_symbol.clone()],
                                    vec![Direction::Right],
                                );
                            } else {
                                new_tm.add_transition(
                                    state_end_tape.clone(),
                                    vec![tape_sep_symbol.clone()],
                                    actual_state.clone()
                                        + "<WRITE_TR"
                                        + &ind.to_string()
                                        + "_TP_"
                                        + &(tapenum - 1).to_string()
                                        + "_START>",
                                    vec![tape_sep_symbol.clone()],
                                    vec![Direction::Left],
                                );
                            }
                            if tapenum == self.tape_count - 1 {
                                new_tm.add_transition(
                                    actual_state.clone(),
                                    vec![self.blank_symbol.clone()],
                                    state_init_tape.clone(),
                                    vec![self.blank_symbol.clone()],
                                    vec![Direction::Left],
                                );
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
                new_tm.add_transition(
                    state_copy_a.clone(),
                    vec![symbol.clone()],
                    state_copy_a.clone(),
                    vec![symbol.clone()],
                    vec![Direction::Right],
                );
                new_tm.add_transition(
                    state_copy_b.clone(),
                    vec![self.blank_symbol.clone()],
                    state_copy_c.clone(),
                    vec![self.blank_symbol.clone()],
                    vec![Direction::Left],
                );
                let state_copy_d = state.clone() + "<COPY_SYMBOL_" + &symbol.clone() + ">";
                if !states_vec.contains(&state_copy_d) {
                    states_vec.push(state_copy_d.clone());
                }
                new_tm.add_transition(
                    state_copy_c.clone(),
                    vec![symbol.clone()],
                    state_copy_d.clone(),
                    vec![self.blank_symbol.clone()],
                    vec![Direction::Right],
                );
                new_tm.add_transition(
                    state_copy_d.clone(),
                    vec![self.blank_symbol.clone()],
                    state_copy_b.clone(),
                    vec![symbol.clone()],
                    vec![Direction::Left],
                );
                let symbol_with_copy = symbol.clone() + "<COPY>";
                new_tm.add_transition(
                    state_copy_c.clone(),
                    vec![symbol_with_copy.clone()],
                    state_copy_e.clone(),
                    vec![symbol.clone()],
                    vec![Direction::Right],
                );
                if !new_tape_alphabet.contains(&symbol_with_copy) {
                    new_tape_alphabet.push(symbol_with_copy);
                }
                if state.ends_with("COPY>") {
                    new_tm.add_transition(
                        state_copy_e.clone(),
                        vec![self.blank_symbol.clone()],
                        state
                            .clone()
                            .strip_suffix("_COPY>")
                            .unwrap_or(&state)
                            .to_string()
                            + "_^FOUND>",
                        vec![self.blank_symbol.clone() + "_"],
                        vec![Direction::Stay],
                    );
                } else {
                    new_tm.add_transition(
                        state_copy_e.clone(),
                        vec![self.blank_symbol.clone()],
                        state.clone(),
                        vec![self.blank_symbol.clone() + "_"],
                        vec![Direction::Stay],
                    );
                }
            }
            new_tm.add_transition(
                state_copy_a.clone(),
                vec![self.blank_symbol.clone()],
                state_copy_b.clone(),
                vec![self.blank_symbol.clone()],
                vec![Direction::Stay],
            );
        }
        fn state_to_final(
            state: String,
            states_vec: &mut Vec<String>,
            new_tm: &mut TuringMachine,
            new_compound_symbols: Vec<String>,
            tape_sep_symbol: String,
            old_tm: &TuringMachine,
        ) -> String {
            let state_final_1 = state.clone() + "<OTHER_TP>";
            let state_final_2 = state.clone() + "<END>";
            if !states_vec.contains(&state_final_1) {
                states_vec.push(state_final_1.clone());
            }
            if !states_vec.contains(&state_final_2) {
                states_vec.push(state_final_2.clone());
            }
            for symbol in new_compound_symbols.clone() {
                new_tm.add_transition(
                    state.clone(),
                    vec![symbol.clone()],
                    state.clone(),
                    vec![symbol
                        .clone()
                        .strip_suffix("^")
                        .unwrap_or(&symbol)
                        .to_string()
                        .strip_suffix("_")
                        .unwrap_or(symbol.clone().strip_suffix("^").unwrap_or(&symbol))
                        .to_string()],
                    vec![Direction::Right],
                );
                new_tm.add_transition(
                    state_final_1.clone(),
                    vec![symbol.clone()],
                    state_final_1.clone(),
                    vec![old_tm.blank_symbol.clone()],
                    vec![Direction::Right],
                );
            }
            new_tm.add_transition(
                state.clone(),
                vec![old_tm.blank_symbol.clone()],
                state_final_1.clone(),
                vec![old_tm.blank_symbol.clone()],
                vec![Direction::Stay],
            );
            new_tm.add_transition(
                state.clone(),
                vec![tape_sep_symbol.clone()],
                state_final_1.clone(),
                vec![old_tm.blank_symbol.clone()],
                vec![Direction::Right],
            );
            new_tm.add_transition(
                state_final_1.clone(),
                vec![tape_sep_symbol.clone()],
                state_final_1.clone(),
                vec![old_tm.blank_symbol.clone()],
                vec![Direction::Right],
            );
            new_tm.add_transition(
                state_final_1.clone(),
                vec![old_tm.blank_symbol.clone()],
                state_final_2.clone(),
                vec![old_tm.blank_symbol.clone()],
                vec![Direction::Right],
            );
            state_final_2
        }
        if !states_vec.contains(&initial_state_fake) {
            states_vec.push(initial_state_fake.clone());
        }
        for state in &new_states {
            if !states_vec.contains(state) {
                states_vec.push(state.clone());
            }
        }
        new_tm.tape_alphabet = new_tape_alphabet.clone();
        if !self.accept_state.is_empty() {
            new_tm.accept_state = state_to_final(
                self.accept_state.clone(),
                &mut states_vec,
                &mut new_tm,
                new_compound_symbols.clone(),
                tape_sep_symbol.clone(),
                self,
            )
        }
        if !self.reject_state.is_empty() {
            new_tm.reject_state = state_to_final(
                self.reject_state.clone(),
                &mut states_vec,
                &mut new_tm,
                new_compound_symbols.clone(),
                tape_sep_symbol.clone(),
                self,
            )
        }
        if !self.halt_state.is_empty() {
            new_tm.halt_state = state_to_final(
                self.halt_state.clone(),
                &mut states_vec,
                &mut new_tm,
                new_compound_symbols.clone(),
                tape_sep_symbol.clone(),
                self,
            )
        }
        new_tm.states = states_vec.clone();
        Ok(new_tm)
    }

    pub fn encoding_to_tm(encoding: String) -> Result<TuringMachine, String> {
        let mut tm = TuringMachine::new();
        let mut transitions: Vec<&str> = encoding.split(")").collect();
        transitions.pop();
        if transitions.is_empty() {
            return Err(format!("invalid encoding: {}", encoding));
        }
        for transition in transitions {
            let transition = transition.trim();
            let transition = transition
                .strip_prefix("(")
                .ok_or("Invalid transition: missing opening parenthesis")?;
            let mut transition = transition.split(";");
            let state = transition
                .next()
                .ok_or("Invalid transition: missing state")?
                .to_string();
            let mut new_state = String::new();
            let mut symbols = Vec::new();
            let mut found_all = false;
            while !found_all {
                let symbol = transition
                    .next()
                    .ok_or("Invalid transition: missing symbol")?
                    .to_string();
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
                new_symbols.push(
                    transition
                        .next()
                        .ok_or("Invalid transition: missing new symbol")?
                        .to_string(),
                );
            }
            let mut directions = Vec::new();
            for _ in 0..tm.tape_count {
                let direction = transition
                    .next()
                    .ok_or("Invalid transition: missing direction")?;
                match direction {
                    "L" => directions.push(Direction::Left),
                    "R" => directions.push(Direction::Right),
                    "S" => directions.push(Direction::Stay),
                    _ => (),
                }
            }
            tm.add_transition(
                state.to_string(),
                symbols.clone(),
                new_state.to_string(),
                new_symbols.clone(),
                directions.clone(),
            );
            if !tm.states.contains(&state.to_string()) {
                tm.states.push(state.to_string());
            }
            if state.starts_with("y") {
                tm.accept_state = state.to_string();
            } else if state.starts_with("n") {
                tm.reject_state = state.to_string();
            } else if state.starts_with("h") {
                tm.halt_state = state.to_string();
            } else if state.starts_with("i") {
                tm.initial_state = state.to_string();
            }
            if !tm.states.contains(&new_state.to_string()) {
                tm.states.push(new_state.to_string());
            }
            if new_state.starts_with("y") {
                tm.accept_state = new_state.to_string();
            } else if new_state.starts_with("n") {
                tm.reject_state = new_state.to_string();
            } else if new_state.starts_with("h") {
                tm.halt_state = new_state.to_string();
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
        }
        Ok(tm)
    }

    pub fn encoding_to_orig(
        encoding: String,
        orig_alphabet_encoding: std::collections::HashMap<String, String>,
        orig_state_encoding: std::collections::HashMap<String, String>,
    ) -> Result<TuringMachine, String> {
        let tm = TuringMachine::encoding_to_tm(encoding)?;
        let mut orig_tm: TuringMachine = TuringMachine {
            initial_state: orig_state_encoding
                .get(&tm.initial_state)
                .ok_or(format!("key not found: {}", tm.initial_state))?
                .clone(),
            accept_state: "".to_string(),
            reject_state: "".to_string(),
            halt_state: if tm.halt_state.is_empty() {
                orig_state_encoding
                    .get(&tm.halt_state)
                    .ok_or(format!("key not found: {}", tm.halt_state))?
                    .clone()
            } else {
                "".to_string()
            },
            states: tm
                .states
                .iter()
                .map(|state| {
                    orig_state_encoding
                        .get(state)
                        .ok_or(format!("key not found: {}", state))
                })
                .collect::<Result<Vec<_>, String>>()?
                .into_iter()
                .cloned()
                .collect(),
            input_alphabet: tm
                .input_alphabet
                .iter()
                .map(|symbol| {
                    orig_alphabet_encoding
                        .get(symbol)
                        .ok_or(format!("key not found: {}", symbol))
                })
                .collect::<Result<Vec<_>, String>>()?
                .into_iter()
                .cloned()
                .collect(),
            transitions: tm
                .transitions
                .iter()
                .map(|transition| -> Result<Transition, String> {
                    Ok(Transition {
                        state: orig_state_encoding
                            .get(&transition.state)
                            .ok_or(format!("key not found: {}", transition.state))?
                            .clone(),
                        symbols: transition
                            .symbols
                            .iter()
                            .map(|symbol| {
                                orig_alphabet_encoding
                                    .get(symbol)
                                    .ok_or(format!("key not found: {}", symbol))
                                    .clone()
                            })
                            .collect::<Result<Vec<_>, String>>()?
                            .into_iter()
                            .cloned()
                            .collect(),
                        new_state: orig_state_encoding
                            .get(&transition.new_state)
                            .ok_or(format!("key not found: {}", transition.new_state))?
                            .clone(),
                        new_symbols: transition
                            .new_symbols
                            .iter()
                            .map(|symbol| {
                                orig_alphabet_encoding
                                    .get(symbol)
                                    .ok_or(format!("key not found: {}", symbol))
                                    .clone()
                            })
                            .collect::<Result<Vec<_>, String>>()?
                            .into_iter()
                            .cloned()
                            .collect(),
                        directions: transition.directions.clone(),
                    })
                })
                .collect::<Result<Vec<_>, String>>()?,
            blank_symbol: orig_alphabet_encoding
                .get(&tm.blank_symbol)
                .ok_or(format!("key not found: {}", tm.blank_symbol))?
                .clone(),
            tape_alphabet: tm
                .tape_alphabet
                .iter()
                .map(|symbol| {
                    orig_alphabet_encoding
                        .get(symbol)
                        .ok_or(format!("key not found: {}", symbol))
                })
                .collect::<Result<Vec<_>, String>>()?
                .into_iter()
                .cloned()
                .collect(),
            tape_count: tm.tape_count,
            next_state_id: 0,
        };
        if !tm.accept_state.is_empty() {
            orig_tm.accept_state = orig_state_encoding
                .get(&tm.accept_state)
                .ok_or(format!("key not found: {}", tm.accept_state))?
                .clone();
        }
        if !tm.reject_state.is_empty() {
            orig_tm.reject_state = orig_state_encoding
                .get(&tm.reject_state)
                .ok_or(format!("key not found: {}", tm.reject_state))?
                .clone();
        }
        Ok(orig_tm)
    }

    pub fn nth_turing_machine(nth: u128) -> Result<String, String> {
        let alphabet = vec![
            "0".to_string(),
            "1".to_string(),
            ";".to_string(),
            "(".to_string(),
            ")".to_string(),
            "a".to_string(),
            "b".to_string(),
            "t".to_string(),
            "y".to_string(),
            "n".to_string(),
            "h".to_string(),
            "i".to_string(),
            "R".to_string(),
            "L".to_string(),
            "S".to_string(),
        ];
        let mut p = 0;
        let mut i = 0;
        let mut tm_string = "".to_string();
        while p != nth {
            i += 1;
            tm_string = utils::uint2str(i, alphabet.clone())?;
            if TuringMachine::check_tm_encoding(tm_string.clone())? {
                p += 1;
            }
        }
        Ok(tm_string)
    }

    pub fn check_tm_encoding(encoding: String) -> Result<bool, String> {
        if encoding.len() < 15 {
            return Ok(false);
        }
        let mut transitions: Vec<&str> = encoding.split(")").collect();
        if transitions.last().unwrap_or(&"").trim() != "" {
            return Ok(false);
        }
        transitions.pop();
        for transition in transitions {
            let transition = transition.trim();
            let transition = transition
                .strip_prefix("(")
                .ok_or("unable to strip prefix '(' from a transition".to_string())?;
            let mut transition = transition.split(";");
            let state = transition
                .next()
                .ok_or("there is no state in one transition".to_string())?
                .to_string();
            if !(state.starts_with("y")
                || state.starts_with("n")
                || state.starts_with("h")
                || state.starts_with("i")
                || state.starts_with("q"))
            {
                return Ok(false);
            }
            for char in state.chars().skip(1) {
                if !(char == '0' || char == '1') {
                    return Ok(false);
                }
            }
            let symbol = transition
                .next()
                .ok_or("Invalid transition: missing symbol")?
                .to_string();
            if !(symbol.starts_with("a") || symbol.starts_with("b") || symbol.starts_with("t")) {
                return Ok(false);
            }
            for char in symbol.chars().skip(1) {
                if !(char == '0' || char == '1') {
                    return Ok(false);
                }
            }
            let new_state = transition
                .next()
                .ok_or("Invalid transition: missing new state")?
                .to_string();
            if !(new_state.starts_with("y")
                || new_state.starts_with("n")
                || new_state.starts_with("h")
                || new_state.starts_with("i")
                || new_state.starts_with("q"))
            {
                return Ok(false);
            }
            for char in new_state.chars().skip(1) {
                if !(char == '0' || char == '1') {
                    return Ok(false);
                }
            }
            let new_symbol = transition
                .next()
                .ok_or("Invalid transition: missing new symbol")?
                .to_string();
            if !(new_symbol.starts_with("a")
                || new_symbol.starts_with("b")
                || new_symbol.starts_with("t"))
            {
                return Ok(false);
            }
            for char in new_symbol.chars().skip(1) {
                if !(char == '0' || char == '1') {
                    return Ok(false);
                }
            }
            let direction = transition
                .next()
                .ok_or("Invalid transition: missing direction")?
                .to_string();
            if !(direction == "L" || direction == "R" || direction == "S") {
                return Ok(false);
            }
        }
        Ok(true)
    }
}
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new() {
        let tm = TuringMachine::new();
        assert_eq!(tm.initial_state, "");
        assert_eq!(tm.accept_state, "");
        assert_eq!(tm.reject_state, "");
        assert_eq!(tm.halt_state, "");
        assert_eq!(tm.blank_symbol, "");
        assert_eq!(tm.states.len(), 0);
        assert_eq!(tm.input_alphabet.len(), 0);
        assert_eq!(tm.tape_alphabet.len(), 0);
        assert_eq!(tm.transitions.len(), 0);
        assert_eq!(tm.tape_count, 1);
        assert_eq!(tm.next_state_id, 0);
    }

    #[test]
    fn test_add_state() {
        let mut tm = TuringMachine::new();
        let state = tm.add_state();
        assert_eq!(state, "state 0");
        assert_eq!(tm.states.len(), 1);
        assert_eq!(tm.states[0], "state 0");
        assert_eq!(tm.next_state_id, 1);
    }

    #[test]
    fn test_add_transition() {
        let mut tm = TuringMachine::new();
        let state = tm.add_state();
        tm.add_transition(
            state.clone(),
            vec!["0".to_string()],
            "state 1".to_string(),
            vec!["1".to_string()],
            vec![Direction::Right],
        );
        assert_eq!(tm.transitions.len(), 1);
        assert_eq!(tm.transitions[0].state, state);
        assert_eq!(tm.transitions[0].symbols, vec!["0".to_string()]);
        assert_eq!(tm.transitions[0].new_state, "state 1".to_string());
        assert_eq!(tm.transitions[0].new_symbols, vec!["1".to_string()]);
        assert_eq!(tm.transitions[0].directions, vec![Direction::Right]);
    }

    #[test]
    fn test_is_final() {
        let mut tm = TuringMachine::new();
        tm.accept_state = "accept".to_string();
        tm.reject_state = "reject".to_string();
        tm.halt_state = "halt".to_string();
        assert!(tm.is_final(&"accept".to_string()));
        assert!(tm.is_final(&"reject".to_string()));
        assert!(tm.is_final(&"halt".to_string()));
        assert!(!tm.is_final(&"other".to_string()));
    }

    #[test]
    fn test_direction_eq() {
        assert_eq!(Direction::Left, Direction::Left);
        assert_eq!(Direction::Right, Direction::Right);
        assert_eq!(Direction::Stay, Direction::Stay);
        assert_ne!(Direction::Left, Direction::Right);
        assert_ne!(Direction::Left, Direction::Stay);
        assert_ne!(Direction::Right, Direction::Stay);
    }

    #[test]
    fn test_direction_from_string() {
        assert!(matches!(Direction::from_string("L"), Direction::Left));
        assert!(matches!(Direction::from_string("R"), Direction::Right));
        assert!(matches!(Direction::from_string("S"), Direction::Stay));
        assert!(matches!(Direction::from_string("other"), Direction::Stay));
    }
    #[test]
    fn test_final_states() {
        let mut tm = TuringMachine::new();
        tm.accept_state = "accept".to_string();
        tm.reject_state = "reject".to_string();
        tm.halt_state = "halt".to_string();

        let final_states = tm.final_states();
        assert_eq!(final_states.len(), 3);
        assert!(final_states.contains(&"accept".to_string()));
        assert!(final_states.contains(&"reject".to_string()));
        assert!(final_states.contains(&"halt".to_string()));
    }

    #[test]
    fn test_is_deterministic() {
        let mut tm = TuringMachine::new();

        // Single transition for state/symbol pair is deterministic
        tm.add_transition(
            "q0".to_string(),
            vec!["0".to_string()],
            "q1".to_string(),
            vec!["1".to_string()],
            vec![Direction::Right],
        );
        assert!(tm.is_deterministic());

        // Multiple transitions for same state/symbol pair is non-deterministic
        tm.add_transition(
            "q0".to_string(),
            vec!["0".to_string()],
            "q2".to_string(),
            vec!["1".to_string()],
            vec![Direction::Left],
        );
        assert!(!tm.is_deterministic());
    }

    #[test]
    fn test_transition_equality() {
        let t1 = Transition {
            state: "q0".to_string(),
            symbols: vec!["0".to_string()],
            new_state: "q1".to_string(),
            new_symbols: vec!["1".to_string()],
            directions: vec![Direction::Right],
        };

        let t2 = Transition {
            state: "q0".to_string(),
            symbols: vec!["0".to_string()],
            new_state: "q1".to_string(),
            new_symbols: vec!["1".to_string()],
            directions: vec![Direction::Right],
        };

        let t3 = Transition {
            state: "q0".to_string(),
            symbols: vec!["1".to_string()],
            new_state: "q1".to_string(),
            new_symbols: vec!["0".to_string()],
            directions: vec![Direction::Left],
        };

        assert_eq!(t1, t2);
        assert_ne!(t1, t3);
    }

    #[test]
    fn test_tape_operations() {
        let tape = Tape {
            tape: vec!["0".to_string(), "1".to_string(), "0".to_string()],
            head: 1,
        };

        assert_eq!(tape.tape.len(), 3);
        assert_eq!(tape.head, 1);
        assert_eq!(tape.tape[tape.head], "1".to_string());
    }
    #[test]
    fn test_simulation() {
        let mut tm = TuringMachine::new();
        tm.blank_symbol = "B".to_string();
        tm.initial_state = "qstart".to_string();
        tm.accept_state = "qaccept".to_string();
        tm.reject_state = "qreject".to_string();
        tm.states = vec![
            "qstart".to_string(),
            "q0".to_string(),
            "q1".to_string(),
            "qaccept".to_string(),
            "qreject".to_string(),
        ];
        tm.input_alphabet = vec!["0".to_string(), "1".to_string()];
        tm.tape_alphabet = vec!["0".to_string(), "1".to_string(), "B".to_string()];

        // Simple machine that accepts strings ending in 1
        tm.add_transition(
            "q0".to_string(),
            vec!["1".to_string()],
            "q1".to_string(),
            vec!["1".to_string()],
            vec![Direction::Right],
        );

        tm.add_transition(
            "q0".to_string(),
            vec!["0".to_string()],
            "q0".to_string(),
            vec!["0".to_string()],
            vec![Direction::Right],
        );
        tm.add_transition(
            "q1".to_string(),
            vec!["0".to_string()],
            "q0".to_string(),
            vec!["0".to_string()],
            vec![Direction::Right],
        );
        tm.add_transition(
            "q0".to_string(),
            vec!["B".to_string()],
            "qreject".to_string(),
            vec!["B".to_string()],
            vec![Direction::Stay],
        );

        tm.add_transition(
            "qstart".to_string(),
            vec!["B".to_string()],
            "q0".to_string(),
            vec!["B".to_string()],
            vec![Direction::Right],
        );

        tm.add_transition(
            "q1".to_string(),
            vec!["B".to_string()],
            "qaccept".to_string(),
            vec!["B".to_string()],
            vec![Direction::Stay],
        );

        let computer = computer::Computer::new();
        let context = computer::Server::new();

        // Should accept "1"
        let result: (String, usize, Vec<String>, usize, Vec<String>) = tm
            .clone()
            .simulate(
                vec!["1".to_string()],
                100,
                computer.clone(),
                context.clone(),
                0,
            )
            .unwrap();
        assert_eq!(result.0, "accept");

        // Should accept "01"
        let result = tm
            .clone()
            .simulate(
                vec!["0".to_string(), "1".to_string()],
                100,
                computer.clone(),
                context.clone(),
                0,
            )
            .unwrap();
        assert_eq!(result.0, "accept");

        // Should reject "0"
        let result = tm
            .clone()
            .simulate(
                vec!["0".to_string()],
                100,
                computer.clone(),
                context.clone(),
                0,
            )
            .unwrap();
        assert_eq!(result.0, "reject");

        // Should reject empty input
        let result = tm.simulate(vec![], 100, computer, context, 0).unwrap();
        assert_eq!(result.0, "reject");
    }

    #[test]
    fn test_multi_tape_conversion() {
        let mut tm = TuringMachine::new();
        tm.blank_symbol = "B".to_string();
        tm.initial_state = "q0".to_string();
        tm.accept_state = "qaccept".to_string();
        tm.reject_state = "qreject".to_string();
        tm.tape_count = 2;
        tm.states = vec![
            "q0".to_string(),
            "qaccept".to_string(),
            "qreject".to_string(),
        ];
        tm.input_alphabet = vec!["0".to_string(), "1".to_string()];
        tm.tape_alphabet = vec!["0".to_string(), "1".to_string(), "B".to_string()];

        tm.add_transition(
            "q0".to_string(),
            vec!["1".to_string(), "B".to_string()],
            "qaccept".to_string(),
            vec!["1".to_string(), "1".to_string()],
            vec![Direction::Stay, Direction::Stay],
        );

        let single_tape = tm.convert_multi_tape_to_single_tape_tm().unwrap();

        assert_eq!(single_tape.tape_count, 1);
        assert!(single_tape.tape_alphabet.len() > tm.tape_alphabet.len());
        assert!(single_tape.states.len() > tm.states.len());
    }

    #[test]
    fn test_encoding_decoding() {
        let mut tm = TuringMachine::new();
        tm.blank_symbol = "B".to_string();
        tm.initial_state = "q0".to_string();
        tm.accept_state = "qaccept".to_string();
        tm.reject_state = "qreject".to_string();
        tm.states = vec![
            "q0".to_string(),
            "qaccept".to_string(),
            "qreject".to_string(),
        ];
        tm.input_alphabet = vec!["0".to_string()];
        tm.tape_alphabet = vec!["0".to_string(), "B".to_string()];

        tm.add_transition(
            "q0".to_string(),
            vec!["0".to_string()],
            "qaccept".to_string(),
            vec!["0".to_string()],
            vec![Direction::Stay],
        );

        let encoding = tm.to_encoding().unwrap().0;
        assert!(TuringMachine::check_tm_encoding(encoding.clone()).unwrap());

        let decoded = TuringMachine::encoding_to_tm(encoding).unwrap();
        assert_eq!(decoded.transitions.len(), tm.transitions.len());
        assert_eq!(decoded.tape_count, tm.tape_count);
    }
    #[test]
    fn test_multi_to_single_tape_equivalence() {
        let mut tm = TuringMachine::new();
        tm.blank_symbol = "B".to_string();
        tm.initial_state = "q0".to_string();
        tm.accept_state = "qa".to_string();
        tm.reject_state = "qr".to_string();
        tm.tape_count = 2;
        tm.states = vec![
            "q0".to_string(),
            "q1".to_string(),
            "qa".to_string(),
            "qr".to_string(),
        ];
        tm.input_alphabet = vec!["0".to_string(), "1".to_string()];
        tm.tape_alphabet = vec!["0".to_string(), "1".to_string(), "B".to_string()];

        // Machine that copies input from tape 1 to tape 2 and accepts if tape 2 matches tape 1
        tm.add_transition(
            "q0".to_string(),
            vec!["0".to_string(), "B".to_string()],
            "q0".to_string(),
            vec!["0".to_string(), "0".to_string()],
            vec![Direction::Right, Direction::Right],
        );

        tm.add_transition(
            "q0".to_string(),
            vec!["1".to_string(), "B".to_string()],
            "q0".to_string(),
            vec!["1".to_string(), "1".to_string()],
            vec![Direction::Right, Direction::Right],
        );

        tm.add_transition(
            "q0".to_string(),
            vec!["B".to_string(), "B".to_string()],
            "q1".to_string(),
            vec!["B".to_string(), "B".to_string()],
            vec![Direction::Left, Direction::Left],
        );

        tm.add_transition(
            "q1".to_string(),
            vec!["0".to_string(), "0".to_string()],
            "q1".to_string(),
            vec!["0".to_string(), "0".to_string()],
            vec![Direction::Left, Direction::Left],
        );

        tm.add_transition(
            "q1".to_string(),
            vec!["1".to_string(), "1".to_string()],
            "q1".to_string(),
            vec!["1".to_string(), "1".to_string()],
            vec![Direction::Left, Direction::Left],
        );

        tm.add_transition(
            "q1".to_string(),
            vec!["B".to_string(), "B".to_string()],
            "qa".to_string(),
            vec!["B".to_string(), "B".to_string()],
            vec![Direction::Stay, Direction::Stay],
        );

        let single_tape = tm.clone().convert_multi_tape_to_single_tape_tm().unwrap();

        let computer = computer::Computer::new();
        let context = computer::Server::new();

        // Test empty input
        let multi_result = tm
            .clone()
            .simulate(vec![], 1000, computer.clone(), context.clone(), 0)
            .unwrap();

        let single_result = single_tape
            .clone()
            .simulate(vec![], 1000, computer.clone(), context.clone(), 0)
            .unwrap();

        assert_eq!(multi_result.0, single_result.0);

        // Test input "0"
        let multi_result = tm
            .clone()
            .simulate(
                vec!["0".to_string()],
                1000,
                computer.clone(),
                context.clone(),
                0,
            )
            .unwrap();

        let single_result = single_tape
            .clone()
            .simulate(
                vec!["0".to_string()],
                1000,
                computer.clone(),
                context.clone(),
                0,
            )
            .unwrap();

        assert_eq!(multi_result.0, single_result.0);

        // Test input "01"
        let multi_result = tm
            .clone()
            .simulate(
                vec!["0".to_string(), "1".to_string()],
                1000,
                computer.clone(),
                context.clone(),
                0,
            )
            .unwrap();

        let single_result = single_tape
            .simulate(
                vec!["0".to_string(), "1".to_string()],
                1000,
                computer,
                context,
                0,
            )
            .unwrap();

        assert_eq!(multi_result.0, single_result.0);
    }

    #[test]
    fn test_multi_to_single_tape_edge_cases() {
        let mut tm = TuringMachine::new();
        tm.blank_symbol = "B".to_string();
        tm.initial_state = "q0".to_string();
        tm.accept_state = "qa".to_string();
        tm.reject_state = "qr".to_string();
        tm.tape_count = 3; // Test with 3 tapes
        tm.states = vec!["q0".to_string(), "qa".to_string(), "qr".to_string()];
        tm.input_alphabet = vec!["0".to_string()];
        tm.tape_alphabet = vec!["0".to_string(), "B".to_string()];

        // Machine that writes a 0 on tape 2 and 3 if there's a 0 on tape 1
        tm.add_transition(
            "q0".to_string(),
            vec!["B".to_string(), "B".to_string(), "B".to_string()],
            "q0".to_string(),
            vec!["B".to_string(), "B".to_string(), "B".to_string()],
            vec![Direction::Right, Direction::Stay, Direction::Stay],
        );
        tm.add_transition(
            "q0".to_string(),
            vec!["0".to_string(), "B".to_string(), "B".to_string()],
            "qa".to_string(),
            vec!["0".to_string(), "0".to_string(), "0".to_string()],
            vec![Direction::Stay, Direction::Stay, Direction::Stay],
        );

        let single_tape = tm.clone().convert_multi_tape_to_single_tape_tm().unwrap();

        // Test tape separator is added
        assert!(single_tape.tape_alphabet.contains(&"#".to_string()));

        // Test head markers are added
        assert!(single_tape.tape_alphabet.iter().any(|s| s.ends_with("^")));
        assert!(single_tape.tape_alphabet.iter().any(|s| s.ends_with("_")));

        // Test states for tape initialization are created
        assert!(single_tape.states.iter().any(|s| s.contains("<INIT_TP")));

        let computer = computer::Computer::new();
        let context = computer::Server::new();

        // Test input "0"
        let multi_result = tm
            .simulate(
                vec!["0".to_string()],
                100,
                computer.clone(),
                context.clone(),
                0,
            )
            .unwrap();

        let single_result = single_tape
            .simulate(vec!["0".to_string()], 100, computer, context, 0)
            .unwrap();

        assert_eq!(multi_result.0, single_result.0);
        assert_eq!(multi_result.0, "accept");
    }

    #[test]
    fn test_multi_tape_different_directions() {
        let mut tm = TuringMachine::new();
        tm.blank_symbol = "B".to_string();
        tm.initial_state = "q0".to_string();
        tm.accept_state = "qa".to_string();
        tm.reject_state = "qr".to_string();
        tm.tape_count = 2;
        tm.states = vec!["q0".to_string(), "qa".to_string(), "qr".to_string()];
        tm.input_alphabet = vec!["1".to_string()];
        tm.tape_alphabet = vec!["1".to_string(), "B".to_string()];

        // Machine that moves left on tape 1 and right on tape 2
        tm.add_transition(
            "q0".to_string(),
            vec!["1".to_string(), "B".to_string()],
            "qa".to_string(),
            vec!["1".to_string(), "1".to_string()],
            vec![Direction::Left, Direction::Right],
        );

        let single_tape = tm.clone().convert_multi_tape_to_single_tape_tm().unwrap();

        let computer = computer::Computer::new();
        let context = computer::Server::new();

        // Test behavior maintains with different movement directions
        let multi_result = tm
            .simulate(
                vec!["1".to_string()],
                100,
                computer.clone(),
                context.clone(),
                1, // Test with head not at start
            )
            .unwrap();

        let single_result = single_tape
            .simulate(vec!["1".to_string()], 100, computer, context, 1)
            .unwrap();

        assert_eq!(multi_result.0, single_result.0);
        assert_eq!(multi_result.0, "accept");
    }

    #[test]
    fn test_multi_tape_stay_direction() {
        let mut tm = TuringMachine::new();
        tm.blank_symbol = "B".to_string();
        tm.initial_state = "q0".to_string();
        tm.accept_state = "qa".to_string();
        tm.reject_state = "qr".to_string();
        tm.tape_count = 2;
        tm.states = vec!["q0".to_string(), "qa".to_string(), "qr".to_string()];
        tm.input_alphabet = vec!["1".to_string()];
        tm.tape_alphabet = vec!["1".to_string(), "B".to_string()];

        // Machine using Stay direction
        tm.add_transition(
            "q0".to_string(),
            vec!["B".to_string(), "B".to_string()],
            "q0".to_string(),
            vec!["B".to_string(), "B".to_string()],
            vec![Direction::Right, Direction::Stay],
        );
        tm.add_transition(
            "q0".to_string(),
            vec!["1".to_string(), "B".to_string()],
            "qa".to_string(),
            vec!["1".to_string(), "1".to_string()],
            vec![Direction::Stay, Direction::Stay],
        );

        let single_tape = tm.clone().convert_multi_tape_to_single_tape_tm().unwrap();

        let computer = computer::Computer::new();
        let context = computer::Server::new();

        // Test Stay direction is handled correctly
        let multi_result = tm
            .simulate(
                vec!["1".to_string()],
                100,
                computer.clone(),
                context.clone(),
                0,
            )
            .unwrap();

        let single_result = single_tape
            .simulate(vec!["1".to_string()], 100, computer, context, 0)
            .unwrap();

        assert_eq!(multi_result.0, single_result.0);
        assert_eq!(multi_result.0, "accept");
    }
}
