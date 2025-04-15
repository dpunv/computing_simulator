// file: computer.rs
// Project: Computing Simulator
// author: dp

use crate::file_handler;
use crate::lambda;
use crate::options;
use crate::ram_machine;
use crate::turing_machine;
use crate::utils;

pub type EncodingResult = (
    String,
    std::collections::HashMap<String, String>,
    std::collections::HashMap<String, String>,
);

#[derive(Clone)]
pub enum ComputingElem {
    Ram(ram_machine::RamMachine),
    Tm(Box<turing_machine::TuringMachine>),
    Lambda(lambda::Lambda),
}

#[derive(Clone)]
pub struct Computer {
    pub element: ComputingElem,
    pub mapping: std::collections::HashMap<String, String>,
}

pub type SimulationResult = (String, usize, Vec<String>, i32, Vec<String>);

#[derive(Clone)]
pub struct Server {
    pub map_computers: std::collections::HashMap<String, Computer>,
    pub computation_order: Vec<String>,
}

impl Computer {
    pub fn is_ram(&self) -> bool {
        match self.element {
            ComputingElem::Ram(_) => true,
            ComputingElem::Tm(_) => false,
            ComputingElem::Lambda(_) => false,
        }
    }

    /* pub fn is_turing(&self) -> bool {
        match self.element {
            ComputingElem::RAM(_) => {
                return false;
            },
            ComputingElem::TM(_) => {
                return true;
            },
            ComputingElem::LAMBDA(_) => {
                return false;
            }
        }
    }
    */

    /* pub fn is_lambda(&self) -> bool {
        match self.element {
            ComputingElem::Ram(_) => false,
            ComputingElem::Tm(_) => false,
            ComputingElem::Lambda(_) => true,
        }
    } */

    pub fn new() -> Computer {
        Computer {
            element: ComputingElem::Tm(Box::new(turing_machine::TuringMachine::new())),
            mapping: std::collections::HashMap::new(),
        }
    }
    pub fn to_encoding(&self) -> Result<EncodingResult, String> {
        match self.element.clone() {
            ComputingElem::Tm(m) => m.to_encoding(),
            ComputingElem::Ram(m) => m.to_encoding(),
            ComputingElem::Lambda(l) => Ok((
                l.to_string(),
                std::collections::HashMap::new(),
                std::collections::HashMap::new(),
            )),
        }
    }

    pub fn set_ram(&mut self, ram_machine: ram_machine::RamMachine) {
        self.element = ComputingElem::Ram(ram_machine);
    }

    pub fn set_turing(&mut self, turing_machine: turing_machine::TuringMachine) {
        self.element = ComputingElem::Tm(Box::new(turing_machine));
    }

    pub fn set_lambda(&mut self, lambda: lambda::Lambda) {
        self.element = ComputingElem::Lambda(lambda);
    }

    pub fn simulate(
        self,
        input: String,
        max_steps: i32,
        context: Server,
        head: usize,
    ) -> Result<SimulationResult, String> {
        match self.element.clone() {
            ComputingElem::Ram(m) => m.simulate(input.clone(), max_steps, self, context),
            ComputingElem::Tm(m) => {
                let input_vec = utils::input_string_to_vec(m.tape_alphabet.clone(), input);
                m.simulate(input_vec, max_steps, self, context, head)
            }
            ComputingElem::Lambda(l) => {
                let mut l_new = lambda::Lambda {
                    expr: lambda::parse_lambda(&input)?,
                    references: l.references.clone(),
                    name: "".to_string(),
                    force_currying: false,
                };
                l_new.simulate()
            },
        }
    }
    pub fn add_mapping(&mut self, name: String, value: String) {
        self.mapping.insert(name, value);
    }
    pub fn get_mapping(&self, name: String) -> Result<String, String> {
        if self.mapping.contains_key(&name) {
            self.mapping.get(&name).ok_or(format!("key not found: {}", name)).cloned()
        } else {
            Ok("".to_string())
        }
    }
    pub fn to_tm(
        self: &mut Computer,
        options: &mut options::Options,
        s: &mut Server,
    ) -> Result<Computer, String> {
        match self.element.clone() {
            ComputingElem::Lambda(l) => {
                options.file = "src/standard/lambda over tm.tm".to_string();
                let mut l_new = lambda::Lambda {
                    expr: lambda::parse_lambda(&options.input)?,
                    references: l.references.clone(),
                    name: "".to_string(),
                    force_currying: false,
                };
                l_new.substitute_names();
                let input_vec = l_new.to_tokens();
                options.input = input_vec.join("");
                let input_alphabet: std::collections::HashSet<String> =
                    input_vec.into_iter().collect();
                let variables: Vec<String> = input_alphabet
                    .iter()
                    .filter(|e| *e != "(" && *e != ")" && *e != "." && *e != "/")
                    .cloned()
                    .collect();
                *self = file_handler::handle_file_reads(options.file.clone(), s)?;
                match self.element.clone() {
                    ComputingElem::Ram(_) => return Err("something went wrong".to_string()),
                    ComputingElem::Lambda(_) => return Err("something went wrong".to_string()),
                    ComputingElem::Tm(m) => {
                        let mut this = m.clone();
                        let old_transitions = m.transitions.clone();
                        let mut new_transitions = Vec::new();

                        // Helper function to create new transitions with symbol substitutions
                        fn create_substituted_transition(
                            t: &turing_machine::Transition,
                            replacements: &[(String, String)],
                        ) -> turing_machine::Transition {
                            let mut new_t = t.clone();
                            new_t.symbols = new_t
                                .symbols
                                .iter()
                                .map(|e| {
                                    for (from, to) in replacements {
                                        if e == from {
                                            return to.clone();
                                        }
                                    }
                                    e.clone()
                                })
                                .collect();
                            new_t.new_symbols = new_t
                                .new_symbols
                                .iter()
                                .map(|e| {
                                    for (from, to) in replacements {
                                        if e == from {
                                            return to.clone();
                                        }
                                    }
                                    e.clone()
                                })
                                .collect();
                            new_t
                        }

                        // Process each transition
                        for t in old_transitions.iter() {
                            let mut replacements_list = vec![vec![]];

                            // Handle 'x'
                            if t.symbols.contains(&"x".to_string()) {
                                let mut new_list = Vec::new();
                                for replacements in replacements_list {
                                    for symb in variables.iter() {
                                        let mut new_replacements = replacements.clone();
                                        new_replacements
                                            .push(("x".to_string(), symb.clone()));
                                        new_list.push(new_replacements);
                                    }
                                }
                                replacements_list = new_list;
                            }
                            if t.symbols.contains(&"x1".to_string()) {
                                let mut new_list = Vec::new();
                                for replacements in replacements_list {
                                    for symb2 in variables.iter() {
                                        let mut new_replacements = replacements.clone();
                                        new_replacements
                                            .push(("x1".to_string(), symb2.clone()));
                                        new_list.push(new_replacements);
                                    }
                                }
                                replacements_list = new_list;
                            }

                            if t.symbols.contains(&"x2".to_string()) {
                                let mut new_list = Vec::new();
                                for replacements in replacements_list {
                                    for symb2 in variables.iter() {
                                        if !replacements
                                            .contains(&("x".to_string(), symb2.to_string()))
                                        {
                                            let mut new_replacements = replacements.clone();
                                            new_replacements
                                                .push(("x2".to_string(), symb2.clone()));
                                            new_list.push(new_replacements);
                                        }
                                    }
                                }
                                replacements_list = new_list;
                            }

                            // Handle other symbol substitutions
                            fn check_d3(s: &String, vars: &[String]) -> bool {
                                !vars.contains(s)
                            }
                            type SymbolRulePredicate =
                                (String, Box<dyn Fn(&String) -> bool>);
                            //type SymbolPredicate = fn(&String) -> bool;
                            let symbol_rules: Vec<SymbolRulePredicate> = vec![
                                ("A".to_string(), Box::new(|s: &String| s != "(")),
                                ("F".to_string(), Box::new(|s: &String| s != ")")),
                                ("B".to_string(), Box::new(|s: &String| s != ".")),
                                (
                                    "C".to_string(),
                                    Box::new(|s: &String| s != "(" && s != ")"),
                                ),
                                ("D".to_string(), Box::new(|_: &String| true)),
                                ("D2".to_string(), Box::new(|_: &String| true)),
                                (
                                    "D3".to_string(),
                                    Box::new({
                                        let variables = variables.clone();
                                        move |s| check_d3(s, &variables)
                                    }),
                                ),
                                ("E".to_string(), Box::new(|s: &String| s != "/")),
                            ];

                            for (symbol, condition) in symbol_rules {
                                if t.symbols.contains(&symbol) {
                                    let mut new_list = Vec::new();
                                    for replacements in replacements_list {
                                        for symb in
                                            input_alphabet.iter().filter(|s| condition(s))
                                        {
                                            let mut new_replacements = replacements.clone();
                                            new_replacements
                                                .push((symbol.clone(), symb.clone()));
                                            new_list.push(new_replacements);
                                        }
                                    }
                                    replacements_list = new_list;
                                }
                            }

                            // Create transitions for all combinations of replacements
                            if !replacements_list.is_empty()
                                && replacements_list[0].is_empty()
                            {
                                new_transitions.push(t.clone());
                            } else {
                                for replacements in replacements_list {
                                    new_transitions.push(create_substituted_transition(
                                        t,
                                        &replacements,
                                    ));
                                }
                            }
                        }

                        this.transitions = new_transitions
                            .into_iter()
                            .map(|t| {
                                this.add_transition(
                                    t.state.clone(),
                                    t.symbols.clone(),
                                    t.new_state.clone(),
                                    t.new_symbols.clone(),
                                    t.directions.clone(),
                                );
                                t
                            })
                            .collect();

                        this.input_alphabet = input_alphabet.clone().into_iter().collect();
                        this.tape_alphabet = input_alphabet.into_iter().collect();
                        this.tape_alphabet.push("_".to_string());
                        this.tape_alphabet.push("$".to_string());
                        self.element = ComputingElem::Tm(Box::new(*this));
                    }
                }

                Ok(self.clone())
            }
            ComputingElem::Tm(_) => Err("already TM".to_string()),
            ComputingElem::Ram(m) => {
                options.file = "src/standard/ram over tm.tm".to_string();
                options.input = options.input.clone()
                    + &(m.to_encoding()?)
                    .0;
                let orig_c = self.clone();
                *self = file_handler::handle_file_reads(options.file.clone(), s)?;
                let mut layers_vec = Vec::new();
                let mut this_layer = vec![0];
                let mut internal_count = 0;
                match self.element.clone() {
                    ComputingElem::Lambda(_) => return Err("something went wrong".to_string()),
                    ComputingElem::Ram(_) => return Err("something went wrong".to_string()),
                    ComputingElem::Tm(mut m) => {
                        m.add_transition(
                            (131).to_string(),
                            vec![
                                "_".to_string(),
                                "_".to_string(),
                                "_".to_string(),
                                "_".to_string(),
                                "_".to_string(),
                                "_".to_string(),
                                "_".to_string(),
                            ],
                            (internal_count + 131).to_string(),
                            vec![
                                "_".to_string(),
                                "_".to_string(),
                                "_".to_string(),
                                "_".to_string(),
                                "_".to_string(),
                                "_".to_string(),
                                "_".to_string(),
                            ],
                            vec![
                                turing_machine::Direction::Stay,
                                turing_machine::Direction::Stay,
                                turing_machine::Direction::Stay,
                                turing_machine::Direction::Right,
                                turing_machine::Direction::Stay,
                                turing_machine::Direction::Stay,
                                turing_machine::Direction::Stay,
                            ],
                        );
                        for i in 0..(((orig_c.mapping.len() + 1) as f32).log2().ceil() as usize) {
                            internal_count += 2 ^ i;
                            let mut this_layer_new = Vec::new();
                            for state in this_layer {
                                this_layer_new.push(state * 2 + 1);
                                m.add_transition(
                                    (state + 131).to_string(),
                                    vec![
                                        "_".to_string(),
                                        "_".to_string(),
                                        "_".to_string(),
                                        "0".to_string(),
                                        "_".to_string(),
                                        "_".to_string(),
                                        "_".to_string(),
                                    ],
                                    (state * 2 + 1 + 131).to_string(),
                                    vec![
                                        "_".to_string(),
                                        "_".to_string(),
                                        "_".to_string(),
                                        "_".to_string(),
                                        "_".to_string(),
                                        "_".to_string(),
                                        "_".to_string(),
                                    ],
                                    vec![
                                        turing_machine::Direction::Stay,
                                        turing_machine::Direction::Stay,
                                        turing_machine::Direction::Stay,
                                        turing_machine::Direction::Right,
                                        turing_machine::Direction::Stay,
                                        turing_machine::Direction::Stay,
                                        turing_machine::Direction::Stay,
                                    ],
                                );
                                this_layer_new.push(state * 2 + 2);
                                m.add_transition(
                                    (state + 131).to_string(),
                                    vec![
                                        "_".to_string(),
                                        "_".to_string(),
                                        "_".to_string(),
                                        "1".to_string(),
                                        "_".to_string(),
                                        "_".to_string(),
                                        "_".to_string(),
                                    ],
                                    (state * 2 + 2 + 131).to_string(),
                                    vec![
                                        "_".to_string(),
                                        "_".to_string(),
                                        "_".to_string(),
                                        "_".to_string(),
                                        "_".to_string(),
                                        "_".to_string(),
                                        "_".to_string(),
                                    ],
                                    vec![
                                        turing_machine::Direction::Stay,
                                        turing_machine::Direction::Stay,
                                        turing_machine::Direction::Stay,
                                        turing_machine::Direction::Right,
                                        turing_machine::Direction::Stay,
                                        turing_machine::Direction::Stay,
                                        turing_machine::Direction::Stay,
                                    ],
                                );
                            }
                            layers_vec.push(this_layer_new.clone());
                            this_layer = this_layer_new;
                        }
                        for state in this_layer {
                            m.add_transition(
                                (state + internal_count + 131 - 1).to_string(),
                                vec![
                                    "_".to_string(),
                                    "_".to_string(),
                                    "_".to_string(),
                                    "_".to_string(),
                                    "_".to_string(),
                                    "_".to_string(),
                                    "_".to_string(),
                                ],
                                129.to_string(),
                                vec![
                                    "_".to_string(),
                                    "_".to_string(),
                                    "_".to_string(),
                                    "_".to_string(),
                                    "_".to_string(),
                                    "_".to_string(),
                                    "_".to_string(),
                                ],
                                vec![
                                    turing_machine::Direction::Right,
                                    turing_machine::Direction::Stay,
                                    turing_machine::Direction::Stay,
                                    turing_machine::Direction::Stay,
                                    turing_machine::Direction::Right,
                                    turing_machine::Direction::Stay,
                                    turing_machine::Direction::Stay,
                                ],
                            );
                        }
                        let new_states: Vec<String> = layers_vec
                            .concat()
                            .iter()
                            .map(|e| (e + 130).to_string())
                            .collect();
                        m.states = [m.states.clone(), new_states].concat();
                        self.set_turing(*m.clone());
                        for (ind, (_, value)) in orig_c.mapping.clone().iter().enumerate() {
                            self.add_mapping(
                                (131 + internal_count + ind).to_string(),
                                value.clone(),
                            );
                        }
                    }
                };
                Ok(self.clone())
            }
        }
    }

    pub fn to_ram(
        self: &mut Computer,
        options: &mut options::Options,
        s: &mut Server,
    ) -> Result<Computer, String> {
        match self.element.clone() {
            ComputingElem::Ram(_) => Err("already a ram".to_string()),
            ComputingElem::Tm(m) => {
                options.file = "src/standard/tm over ram.ram".to_string();
                let state_size = (m.states.len() as f32).log2().ceil() as usize;
                let symbol_size = (m.tape_alphabet.len() as f32).log2().ceil() as usize;
                let states_map: std::collections::HashMap<String, String> = m
                    .states
                    .iter()
                    .enumerate()
                    .map(|(index, state)| (state.clone(), utils::int2bin(index as i32, state_size)))
                    .collect();
                let symbols_map: std::collections::HashMap<String, String> = m
                    .tape_alphabet
                    .iter()
                    .enumerate()
                    .map(|(index, state)| {
                        (state.clone(), utils::int2bin(index as i32, symbol_size))
                    })
                    .collect();
                states_map.iter().for_each(|(k, v)| println!("STATE {} -> {}", k, v));
                symbols_map.iter().for_each(|(k, v)| println!("SYMBOL {} -> {}", k, v));
                options.input = "1".to_string()
                    + &symbols_map.get(&m.blank_symbol).ok_or_else(|| "Blank symbol not found in mapping".to_string())?.to_owned()
                    + "1"
                    + &utils::input_string_to_vec(m.tape_alphabet.clone(), options.input.clone())
                        .iter()
                        .map(|s| symbols_map.get(s).cloned().ok_or_else(|| format!("Symbol '{}' not found in mapping", s)))
                        .collect::<Result<Vec<String>, String>>()?
                        .join("1")
                    + "0"
                    + &m.transitions
                        .iter()
                        .map(|t| -> Result<String, String> {
                            let state = states_map.get(&t.state).ok_or_else(|| {
                                format!("State '{}' not found in mapping", t.state)
                            })?;
                            let symbol = symbols_map.get(&t.symbols[0]).ok_or_else(|| {
                                format!("Symbol '{}' not found in mapping", t.symbols[0])
                            })?;
                            let new_state = states_map.get(&t.new_state).ok_or_else(|| {
                                format!("State '{}' not found in mapping", t.new_state)
                            })?;
                            let new_symbol =
                                symbols_map.get(&t.new_symbols[0]).ok_or_else(|| {
                                    format!("Symbol '{}' not found in mapping", t.new_symbols[0])
                                })?;
                            let direction = match t.directions[0] {
                                turing_machine::Direction::Left => "01",
                                turing_machine::Direction::Right => "10",
                                turing_machine::Direction::Stay => "00",
                            };

                            Ok(state.to_owned() + symbol + new_state + new_symbol + direction)
                        })
                        .collect::<Result<Vec<String>, String>>()?
                        .join("1")
                    + "0";
                *self = file_handler::handle_file_reads(options.file.clone(), s)?;
                match self.element.clone() {
                    ComputingElem::Ram(mut ram) => {
                        ram.labels_map.insert(
                            "STATE_SIZE".to_string(),
                            utils::int2bin(state_size as i32, 0),
                        );
                        ram.labels_map.insert(
                            "SYMBOL_SIZE".to_string(),
                            utils::int2bin(symbol_size as i32, 0),
                        );
                        ram.labels_map.insert(
                            "INIT_STATE".to_string(),
                            states_map
                                .get(&m.initial_state)
                                .ok_or_else(|| {
                                    format!(
                                        "Initial state '{}' not found in state mapping",
                                        m.initial_state
                                    )
                                })?
                                .to_owned(),
                        );
                        ram.labels_map.insert(
                            "ACCEPT_STATE".to_string(),
                            states_map
                                .get(&m.accept_state)
                                .unwrap_or(&utils::int2bin(2_i32.pow(state_size as u32), 0))
                                .to_owned(),
                        );
                        ram.labels_map.insert(
                            "REJECT_STATE".to_string(),
                            states_map
                                .get(&m.reject_state)
                                .unwrap_or(&utils::int2bin(2_i32.pow(state_size as u32), 0))
                                .to_owned(),
                        );
                        ram.labels_map.insert(
                            "HALT_STATE".to_string(),
                            states_map
                                .get(&m.halt_state)
                                .unwrap_or(&utils::int2bin(2_i32.pow(state_size as u32), 0))
                                .to_owned(),
                        );
                        ram.labels_map.insert(
                            "BLANK_CHAR".to_string(),
                            symbols_map
                                .get(&m.blank_symbol)
                                .ok_or_else(|| {
                                    format!(
                                        "Blank symbol '{}' not found in symbol mapping",
                                        m.blank_symbol
                                    )
                                })?
                                .to_owned(),
                        );
                        self.element = ComputingElem::Ram(ram.clone());
                        Ok(self.clone())
                    }
                    ComputingElem::Tm(_) => Err("something went wrong".to_string()),
                    ComputingElem::Lambda(_) => Err("something went wrong".to_string()),
                }
            }
            ComputingElem::Lambda(_) => {
                *self = self.to_tm(options, s)?;
                self.to_ram(options, s)
            },
        }
    }
}

impl Server {
    pub fn new() -> Server {
        Server {
            map_computers: std::collections::HashMap::new(),
            computation_order: Vec::new(),
        }
    }

    pub fn add_computer(&mut self, name: String, computer: Computer) {
        self.map_computers.insert(name, computer);
    }

    pub fn get_computer(&mut self, name: String) -> Option<&mut Computer> {
        self.map_computers.get_mut(&name)
    }

    pub fn contains(&self, name: String) -> bool {
        self.map_computers.contains_key(&name)
    }

    /* pub fn computation_order(&self) -> Vec<String> {
        self.computation_order.clone()
    } */

    pub fn computes_at(&self, n: usize) -> String {
        self.computation_order[n].clone()
    }

    /* pub fn set_computation_order(&mut self, order: Vec<String>) {
        self.computation_order = order;
    } */

    pub fn set_computation_order_at(&mut self, n: usize, name: String) {
        if n < self.computation_order.len() {
            self.computation_order[n] = name;
        } else {
            self.computation_order.push(name);
        }
    }

    pub fn execute(
        &mut self,
        input: String,
        max_steps: i32,
    ) -> Result<(String, usize, String, i32, Vec<String>), String> {
        let mut steps: i32 = 0;
        let mut output: String = input;
        let mut final_state = "".to_string();
        let mut current_head = 0;
        let mut tot_comp = Vec::new();
        for name in self.computation_order.clone() {
            let computer = self.get_computer(name.clone()).ok_or_else(|| format!("cannot find computer with name '{}'", name.clone()).to_string())?;
            let (state, head, tape, s, computation) = computer
                .clone()
                .simulate(output, max_steps - steps, self.clone(), 0)?;
                    final_state = state;
                    current_head = head;
                    output = tape.join("");
                    steps += s;
                    tot_comp.extend(computation);
        }
        let last_computer = self
            .get_computer(self.computation_order[self.computation_order.len() - 1].clone())
            .ok_or_else(|| "cannot find computer".to_string())?;
        match last_computer.element.clone() {
            ComputingElem::Lambda(_) => {}
            ComputingElem::Ram(_) => {}
            ComputingElem::Tm(m) => {
                output = utils::input_string_to_vec(m.tape_alphabet.clone(), output)
                    .into_iter()
                    .filter(|e| *e != m.blank_symbol)
                    .collect::<Vec<String>>()
                    .join("");
            }
        }
        Ok((final_state, current_head, output, steps, tot_comp))
    }
}
