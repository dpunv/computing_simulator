//! # Computing Simulator
//! 
//! This module provides core functionality for simulating various types of computing machines
//! and managing their interactions. It implements a unified interface for different computational
//! models including RAM machines, Turing machines, and Lambda calculus.
//!
//! ## Core Components
//!
//! * `Computer` - A wrapper structure that can contain different types of computing elements
//! * `Server` - A management structure that handles multiple computers and their execution order
//! * `ComputingElem` - An enum representing different types of computing machines
//!
//! ## Features
//!
//! * Supports multiple computation models:
//!   - RAM machines
//!   - Turing machines (single and multi-tape)
//!   - Lambda calculus
//! * Conversion between different computation models
//! * Simulation of computations with step limits
//! * Management of multiple computing machines
//! * Input/output encoding and mapping
//!
//! ## Type Definitions
//!
//! * `EncodingResult` - Represents the result of encoding a computer
//! * `SimulationResult` - Contains the results of a simulation including state, output, and steps
//!
//! ## Key Structures
//!
//! ### Computer
//! Represents a single computing machine with its associated mappings and state.
//! 
//! ### Server
//! Manages multiple computers and their execution order, providing a framework
//! for complex computations involving multiple machines.
//!
//! ## Conversions
//!
//! The module supports conversions between different computation models:
//! * Lambda calculus to Turing machine
//! * RAM Machines to Turing machine
//! * Turing machine to RAM machine
//! * Multi-tape to single-tape Turing machine
//!
//! ## Error Handling
//!
//! Most methods return `Result` types to handle various error conditions that may arise
//! during computation or conversion processes.
//!
//! ## Author
//!
//! - dp
//! 
//! # License
//! 
//! This project is licensed under the MIT License. See the LICENSE file for details.

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

/// Represents different types of computing machines that can be simulated.
///
/// This enum encapsulates various computational models supported by the simulator:
/// * RAM machines - Basic register-based computational model
/// * Turing machines - Standard and multi-tape variants
/// * Lambda calculus - Functional computation model
///
/// Each variant contains the corresponding machine implementation:
/// * `Ram` - Contains a boxed `RamMachine` instance
/// * `Tm` - Contains a boxed `TuringMachine` instance
/// * `Lambda` - Contains a boxed `Lambda` instance for lambda calculus computations
///
/// The enum implements `Clone` to allow duplication of computing elements when needed.
#[derive(Clone)]
pub enum ComputingElem {
    Ram(Box<ram_machine::RamMachine>),
    Tm(Box<turing_machine::TuringMachine>),
    Lambda(Box<lambda::Lambda>),
}

/// A structure representing a computing machine with its associated mappings and configuration.
///
/// The `Computer` struct serves as a container for different types of computing elements
/// (RAM machines, Turing machines, or Lambda calculus) and maintains their associated mappings.
/// It provides a unified interface for working with different computational models and
/// managing their state and behavior.
///
/// # Fields
///
/// * `element` - The core computing element (RAM, Turing machine, or Lambda calculus)
/// * `mapping` - A HashMap containing name-value pairs for subroutines calling, mapping internal names to computer names in the context
///
/// # Features
///
/// * Supports multiple computation models through the `ComputingElem` enum
/// * Provides conversion between different computation models
/// * Maintains configuration mappings
/// * Supports simulation with step limits
/// * Handles input/output encoding
///
/// # Conversions
///
/// The structure supports conversions between different computation models:
/// * Lambda calculus to Turing machine
/// * RAM machine to Turing machine
/// * Turing machine to RAM machine
/// * Multi-tape to single-tape Turing machine
///
/// # Note
///
/// The computer's behavior and capabilities depend on the type of computing element
/// it contains. Different methods may behave differently based on the underlying
/// computation model.
#[derive(Clone)]
pub struct Computer {
    pub element: ComputingElem,
    pub mapping: std::collections::HashMap<String, String>,
}

pub type SimulationResult = (String, usize, Vec<String>, usize, Vec<String>);

/// A management structure that coordinates multiple computing machines and their execution sequence.
///
/// The `Server` acts as an orchestrator for complex computations involving multiple computing
/// elements. It maintains a registry of named computers and controls their execution order,
/// enabling sequential processing of computations across different machines.
///
/// # Fields
///
/// * `map_computers` - A HashMap storing computing machines indexed by their names
/// * `computation_order` - A vector defining the sequence of computer executions
///
/// # Features
///
/// * Manages multiple computing machines under unique identifiers
/// * Controls execution order through a configurable sequence
/// * Supports dynamic addition and retrieval of computers
/// * Enables chained computation across multiple machines
/// * Handles execution with customizable step limits
/// * Provides detailed computation results including state, output, and steps
///
/// # Returns
///
/// The `execute` method returns a `Result` containing:
/// * Final state of computation
/// * Final head position
/// * Output string
/// * Total steps executed
/// * Vector of computation log entries
///
/// # Error Handling
///
/// Operations may fail with descriptive error messages in cases such as:
/// * Attempting to execute with an empty server
/// * Referencing non-existent computers
/// * Invalid computation order
/// * Execution step limits exceeded
///
/// # Note
///
/// The server ensures proper isolation between different computing elements while
/// maintaining their execution sequence. It's particularly useful for complex
/// computations that require multiple stages or different computation models.
#[derive(Clone)]
pub struct Server {
    pub map_computers: std::collections::HashMap<String, Computer>,
    pub computation_order: Vec<String>,
}

impl Computer {
    /// Checks if the computer's element is a RAM machine.
    ///
    /// # Returns
    ///
    /// * `true` - if the computer's element is a RAM machine
    /// * `false` - if the computer's element is a Turing machine or Lambda calculus
    ///
    pub fn is_ram(&self) -> bool {
        match self.element {
            ComputingElem::Ram(_) => true,
            ComputingElem::Tm(_) => false,
            ComputingElem::Lambda(_) => false,
        }
    }

    /*
    /// Checks if the computer's element is a Turing machine.
    ///
    /// # Returns
    ///
    /// * `true` - if the computer's element is a Turing machine
    /// * `false` - if the computer's element is a RAM machine or Lambda calculus
    ///
    pub fn is_turing(&self) -> bool {
        match self.element {
            ComputingElem::RAM(_) => false,
            ComputingElem::TM(_) => true,
            ComputingElem::LAMBDA(_) => false,
        }
    }
    */

    /* 
    /// Checks if the computer's element is a Lambda calculus.
    ///
    /// # Returns
    ///
    /// * `true` - if the computer's element is a Lambda calculus
    /// * `false` - if the computer's element is a Turing machine or RAM machine
    ///
    pub fn is_lambda(&self) -> bool {
        match self.element {
            ComputingElem::Ram(_) => false,
            ComputingElem::Tm(_) => false,
            ComputingElem::Lambda(_) => true,
        }
    } */

    /// Converts a multi-tape Turing machine to a single-tape Turing machine.
    ///
    /// This method only applies to Turing machine elements. It performs the conversion 
    /// by implementing the standard construction that simulates a multi-tape Turing machine
    /// using a single tape. The conversion preserves the computational behavior of the
    /// original machine.
    ///
    /// # Returns
    ///
    /// * `Ok(Computer)` - A new Computer instance containing the equivalent single-tape Turing machine
    /// * `Err(String)` - An error message if the computer's element is not a Turing machine
    ///
    /// # Errors
    ///
    /// Returns an error if:
    /// * The computer's element is not a Turing machine
    /// * The conversion process fails
    ///
    /// The method preserves all mappings and properties of the original computer while
    /// transforming the internal Turing machine representation to use a single tape.
    pub fn convert_to_singletape(&mut self) -> Result<Computer, String> {
        match self.element {
            ComputingElem::Tm(ref m) => {
                self.set_turing(m.convert_multitape_to_singletape_tm()?);
                return Ok(self.clone());
            }
            _ => return Err("not a turing machine".to_string()),
        }
    }

    /// Creates a new Computer instance initialized with a default Turing machine.
    ///
    /// This constructor creates a new Computer with the following default settings:
    /// - The computing element is set to a new, empty Turing machine
    /// - The mapping hashmap is initialized as empty
    ///
    /// # Returns
    ///
    /// Returns a new `Computer` instance with default initialization.
    ///
    pub fn new() -> Computer {
        Computer {
            element: ComputingElem::Tm(Box::new(turing_machine::TuringMachine::new())),
            mapping: std::collections::HashMap::new(),
        }
    }

    /// Converts the current computing element to its string encoding representation.
    ///
    /// This method converts the computer's computing element (RAM machine, Turing machine, or Lambda calculus)
    /// into a standardized string encoding format along with any associated mappings.
    ///
    /// # Returns
    ///
    /// * `Ok(EncodingResult)` - A tuple containing:
    ///   - The string encoding of the computing element
    ///   - Two HashMaps containing any necessary mapping information
    /// * `Err(String)` - An error message if the encoding process fails
    ///
    /// # Note
    ///
    /// The exact format of the encoding depends on the type of computing element:
    /// - For Turing machines: Uses the TM-specific encoding format
    /// - For RAM machines: Uses the RAM-specific encoding format
    /// - For Lambda calculus: Returns the string representation with empty mappings
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

    /// Sets the computer's computing element to a RAM machine.
    ///
    /// This method updates the computer's internal element to use the provided RAM machine,
    /// replacing any existing computing element (Turing machine or Lambda calculus).
    ///
    /// # Arguments
    ///
    /// * `ram_machine` - A RAM machine instance to be set as the computer's computing element
    pub fn set_ram(&mut self, ram_machine: ram_machine::RamMachine) {
        self.element = ComputingElem::Ram(Box::new(ram_machine));
    }

    /// Sets the computer's computing element to a Turing machine.
    ///
    /// This method updates the computer's internal element to use the provided Turing machine,
    /// replacing any existing computing element (RAM machine or Lambda calculus).
    ///
    /// # Arguments
    ///
    /// * `turing_machine` - A Turing machine instance to be set as the computer's computing element
    ///
    pub fn set_turing(&mut self, turing_machine: turing_machine::TuringMachine) {
        self.element = ComputingElem::Tm(Box::new(turing_machine));
    }

    /// Sets the computer's computing element to a Lambda calculus.
    ///
    /// This method updates the computer's internal element to use the provided Lambda calculus,
    /// replacing any existing computing element (RAM machine or Turing machine).
    ///
    /// # Arguments
    ///
    /// * `lambda` - A Lambda calculus instance to be set as the computer's computing element
    ///
    pub fn set_lambda(&mut self, lambda: lambda::Lambda) {
        self.element = ComputingElem::Lambda(Box::new(lambda));
    }

    /// Simulates the execution of the current computing element with the given input.
    ///
    /// This method runs the simulation of the computer's computing element (RAM machine, 
    /// Turing machine, or Lambda calculus) with the specified parameters.
    ///
    /// # Arguments
    ///
    /// * `input` - The input string to be processed by the computing element
    /// * `max_steps` - The maximum number of steps the simulation should run before stopping
    /// * `context` - The server context used for simulation tracking and control
    /// * `head` - The initial head position (primarily used for Turing machines)
    ///
    /// # Returns
    ///
    /// * `Ok(SimulationResult)` - The result of the simulation containing:
    ///   - The final configuration
    ///   - The execution history
    ///   - Any additional simulation-specific data
    /// * `Err(String)` - An error message if the simulation fails
    ///
    /// # Note
    ///
    /// The behavior varies depending on the type of computing element:
    /// - For RAM machines: Processes the input as RAM instructions
    /// - For Turing machines: Converts input to tape symbols and simulates TM execution
    /// - For Lambda calculus: Evaluates the lambda expression
    pub fn simulate(
        self,
        input: String,
        max_steps: usize,
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
                l_new.simulate(max_steps)
            }
        }
    }

    /// Adds a new mapping entry to the computer's mapping collection.
    ///
    /// This method associates a name with a value in the computer's internal mapping hashmap.
    /// If the name already exists, its value will be updated with the new value.
    ///
    /// # Arguments
    ///
    /// * `name` - The key/name for the mapping entry
    /// * `value` - The value to be associated with the name
    ///
    pub fn add_mapping(&mut self, name: String, value: String) {
        self.mapping.insert(name, value);
    }

    /// Retrieves the value associated with a given name from the computer's mapping collection.
    ///
    /// # Arguments
    ///
    /// * `name` - The key/name whose value should be retrieved
    ///
    /// # Returns
    ///
    /// * `Ok(String)` - The value associated with the name, or an empty string if the name doesn't exist
    /// * `Err(String)` - An error message if the mapping retrieval fails
    ///
    pub fn get_mapping(&self, name: String) -> Result<String, String> {
        if self.mapping.contains_key(&name) {
            self.mapping
                .get(&name)
                .ok_or(format!("key not found: {}", name))
                .cloned()
        } else {
            Ok("".to_string())
        }
    }

    /// Converts the current computing element to a Turing machine representation.
    ///
    /// This method transforms either a Lambda calculus expression or a RAM machine into an equivalent
    /// Turing machine representation. The conversion process preserves the computational behavior
    /// of the original element.
    ///
    /// # Arguments
    ///
    /// * `self` - A mutable reference to the Computer instance
    /// * `options` - A mutable reference to Options containing configuration settings
    /// * `s` - A mutable reference to the Server context
    ///
    /// # Returns
    ///
    /// * `Ok(Computer)` - A new Computer instance containing the equivalent Turing machine
    /// * `Err(String)` - An error message if the conversion fails
    ///
    /// # Behavior
    ///
    /// For Lambda calculus:
    /// - Converts the lambda expression to tokens
    /// - Uses a standard construction from "lambda over tm.tm"
    /// - Preserves variable bindings and substitutions
    ///
    /// For RAM machines:
    /// - Uses a standard construction from "ram over tm.tm"
    /// - Preserves the original RAM program's behavior
    /// - Maintains instruction mappings
    ///
    /// For Turing machines:
    /// - Returns an error as conversion is unnecessary
    ///
    /// # Notes
    ///
    /// - The conversion maintains all mappings from the original computer
    /// - The resulting Turing machine uses a modified alphabet to accommodate the source computation
    /// - For Lambda calculus, variables and syntax tokens are encoded appropriately
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
                let mapping = self.mapping.clone();
                *self = file_handler::handle_file_reads(options.file.clone(), s)?;
                self.mapping = mapping;
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
                                        new_replacements.push(("x".to_string(), symb.clone()));
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
                                        new_replacements.push(("x1".to_string(), symb2.clone()));
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
                            type SymbolRulePredicate = (String, Box<dyn Fn(&String) -> bool>);
                            //type SymbolPredicate = fn(&String) -> bool;
                            let symbol_rules: Vec<SymbolRulePredicate> = vec![
                                ("A".to_string(), Box::new(|s: &String| s != "(")),
                                ("F".to_string(), Box::new(|s: &String| s != ")")),
                                ("B".to_string(), Box::new(|s: &String| s != ".")),
                                ("C".to_string(), Box::new(|s: &String| s != "(" && s != ")")),
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
                                        for symb in input_alphabet.iter().filter(|s| condition(s)) {
                                            let mut new_replacements = replacements.clone();
                                            new_replacements.push((symbol.clone(), symb.clone()));
                                            new_list.push(new_replacements);
                                        }
                                    }
                                    replacements_list = new_list;
                                }
                            }

                            // Create transitions for all combinations of replacements
                            if !replacements_list.is_empty() && replacements_list[0].is_empty() {
                                new_transitions.push(t.clone());
                            } else {
                                for replacements in replacements_list {
                                    new_transitions
                                        .push(create_substituted_transition(t, &replacements));
                                }
                            }
                        }

                        this.transitions = new_transitions
                            .into_iter()
                            .inspect(|t| {
                                this.add_transition(
                                    t.state.clone(),
                                    t.symbols.clone(),
                                    t.new_state.clone(),
                                    t.new_symbols.clone(),
                                    t.directions.clone(),
                                );
                            })
                            .collect();

                        this.input_alphabet = input_alphabet.clone().into_iter().collect();
                        this.tape_alphabet =
                            [input_alphabet.into_iter().collect(), m.tape_alphabet].concat();
                        self.element = ComputingElem::Tm(Box::new(*this));
                    }
                }
                Ok(self.clone())
            }
            ComputingElem::Tm(_) => Err("already TM".to_string()),
            ComputingElem::Ram(m) => {
                options.file = "src/standard/ram over tm.tm".to_string();
                options.input = options.input.clone() + &(m.to_encoding()?).0;
                let orig_c = self.clone();
                let mapping = self.mapping.clone();
                *self = file_handler::handle_file_reads(options.file.clone(), s)?;
                self.mapping = mapping;
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

    /// Converts the current computing element to a RAM machine representation.
    ///
    /// This method transforms either a Lambda calculus expression or a Turing machine into an equivalent
    /// RAM machine representation. The conversion process preserves the computational behavior
    /// of the original element.
    ///
    /// # Arguments
    ///
    /// * `self` - A mutable reference to the Computer instance
    /// * `options` - A mutable reference to Options containing configuration settings
    /// * `s` - A mutable reference to the Server context
    ///
    /// # Returns
    ///
    /// * `Ok(Computer)` - A new Computer instance containing the equivalent RAM machine
    /// * `Err(String)` - An error message if the conversion fails
    ///
    /// # Behavior
    ///
    /// For Turing machines:
    /// - Converts states and symbols to binary representations
    /// - Uses a standard construction from "tm over ram.ram"
    /// - Preserves the transition function and tape behavior
    /// - Note: The subroutine calling doesn't work yet
    ///
    /// For Lambda calculus:
    /// - First converts to Turing machine
    /// - Then converts the resulting TM to RAM
    /// - Note: the feature is higly experimental and is inefficent. Use with caution.
    ///
    /// For RAM machines:
    /// - Returns an error as conversion is unnecessary
    ///
    /// # Notes
    ///
    /// - The conversion maintains all mappings from the original computer
    /// - States and symbols are encoded using binary strings
    /// - Special states (initial, accept, reject, halt) are properly mapped
    /// - Includes configuration for state size, symbol size, and blank character
    /// - The resulting RAM machine simulates the original computation step by step
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
                let mut translation_map: std::collections::HashMap<String, String> = states_map
                    .iter()
                    .map(|(k, v)| (format!("state {}", k), v.to_string()))
                    .collect();
                translation_map.extend(symbols_map
                    .iter()
                    .map(|(k, v)| (format!("symbol {}", k), v.to_string())));
                options.input = "1".to_string()
                    + &symbols_map
                        .get(&m.blank_symbol)
                        .ok_or_else(|| "Blank symbol not found in mapping".to_string())?
                        .to_owned()
                    + "1"
                    + &utils::input_string_to_vec(m.tape_alphabet.clone(), options.input.clone())
                        .iter()
                        .map(|s| {
                            symbols_map
                                .get(s)
                                .cloned()
                                .ok_or_else(|| format!("Symbol '{}' not found in mapping", s))
                        })
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
                let mapping = self.mapping.clone();
                *self = file_handler::handle_file_reads(options.file.clone(), s)?;
                self.mapping = mapping;
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
                        ram.translation_map = translation_map;
                        self.element = ComputingElem::Ram(ram.clone());
                        Ok(self.clone())
                    }
                    ComputingElem::Tm(_) => Err("something went wrong".to_string()),
                    ComputingElem::Lambda(_) => Err("something went wrong".to_string()),
                }
            }
            ComputingElem::Lambda(_) => {
                *self = self.to_tm(options, s)?;
                self.convert_to_singletape()?;
                self.to_ram(options, s)
            }
        }
    }
}

/// Implementation of the Server struct which manages multiple computing elements
impl Server {
    /// Creates a new empty Server instance
    ///
    /// # Returns
    /// * `Server` - A new Server with empty HashMaps and Vectors
    pub fn new() -> Server {
        Server {
            map_computers: std::collections::HashMap::new(),
            computation_order: Vec::new(),
        }
    }

    /// Adds a computer to the server's map of computers
    ///
    /// # Arguments
    /// * `name` - String identifier for the computer
    /// * `computer` - Computer instance to be added
    pub fn add_computer(&mut self, name: String, computer: Computer) {
        self.map_computers.insert(name, computer);
    }

    /// Gets a mutable reference to a computer by name
    ///
    /// # Arguments
    /// * `name` - String identifier of the computer to retrieve
    ///
    /// # Returns
    /// * `Option<&mut Computer>` - Some(computer) if found, None if not present
    pub fn get_computer(&mut self, name: String) -> Option<&mut Computer> {
        self.map_computers.get_mut(&name)
    }

    /// Checks if a computer with the given name exists in the server
    ///
    /// # Arguments
    /// * `name` - String identifier to check
    ///
    /// # Returns
    /// * `bool` - true if computer exists, false otherwise
    pub fn contains(&self, name: String) -> bool {
        self.map_computers.contains_key(&name)
    }

    /// Gets the name of the computer at position n in the computation order
    ///
    /// # Arguments
    /// * `n` - Index in the computation order
    ///
    /// # Returns
    /// * `String` - Name of the computer at that position
    pub fn computes_at(&self, n: usize) -> String {
        self.computation_order[n].clone()
    }

    /// Sets or adds a computer name at a specific position in the computation order
    ///
    /// # Arguments
    /// * `n` - Position in the computation order
    /// * `name` - Name of the computer to place at that position
    pub fn set_computation_order_at(&mut self, n: usize, name: String) {
        if n < self.computation_order.len() {
            self.computation_order[n] = name;
        } else {
            self.computation_order.push(name);
        }
    }

    /// Executes the computation chain on the given input
    ///
    /// # Arguments
    /// * `input` - Input string to process
    /// * `max_steps` - Maximum number of computation steps allowed
    ///
    /// # Returns
    /// * `Result<(String, usize, String, usize, Vec<String>), String>` - On success returns
    ///   (final_state, head_position, output, steps_taken, computation_trace)
    ///   On error returns error message as string
    ///
    /// # Errors
    /// * Returns error if server has no computers
    /// * Returns error if computation order is empty
    /// * Returns error if a computer in the computation chain cannot be found
    pub fn execute(
        &mut self,
        input: String,
        max_steps: usize,
    ) -> Result<(String, usize, String, usize, Vec<String>), String> {
        let mut steps: usize = 0;
        let mut output: String = input;
        let mut final_state = "".to_string();
        let mut current_head = 0;
        let mut tot_comp = Vec::new();
        if self.map_computers.is_empty() {
            return Err("empty server".to_string());
        }
        if self.computation_order.is_empty() {
            return Err("empry computation order".to_string());
        }
        for name in self.computation_order.clone() {
            let computer = self.get_computer(name.clone()).ok_or_else(|| {
                format!("cannot find computer with name '{}'", name.clone()).to_string()
            })?;
            let (state, head, tape, s, computation) =
                computer
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_computer_new() {
        let computer = Computer::new();
        assert!(matches!(computer.element, ComputingElem::Tm(_)));
        assert!(computer.mapping.is_empty());
    }

    #[test]
    fn test_computer_is_ram() {
        let mut computer = Computer::new();
        assert!(!computer.is_ram());
        computer.set_ram(ram_machine::RamMachine {
            instructions: Vec::new(),
            labels_map: std::collections::HashMap::new(),
            translation_map: std::collections::HashMap::new()
        });
        assert!(computer.is_ram());
    }

    #[test]
    fn test_computer_mapping() {
        let mut computer = Computer::new();
        computer.add_mapping("test".to_string(), "value".to_string());
        assert_eq!(
            computer.get_mapping("test".to_string()),
            Ok("value".to_string())
        );
        assert_eq!(
            computer.get_mapping("missing".to_string()),
            Ok("".to_string())
        );
    }

    #[test]
    fn test_server_new() {
        let server = Server::new();
        assert!(server.map_computers.is_empty());
        assert!(server.computation_order.is_empty());
    }

    #[test]
    fn test_server_add_get_computer() {
        let mut server = Server::new();
        let computer = Computer::new();
        server.add_computer("test".to_string(), computer.clone());
        assert!(server.contains("test".to_string()));
        assert!(server.get_computer("test".to_string()).is_some());
    }

    #[test]
    fn test_server_computation_order() {
        let mut server = Server::new();
        server.set_computation_order_at(0, "first".to_string());
        server.set_computation_order_at(1, "second".to_string());
        assert_eq!(server.computes_at(0), "first");
        assert_eq!(server.computes_at(1), "second");
    }
    #[test]
    fn test_set_machine_types() {
        let mut computer = Computer::new();

        let ram = ram_machine::RamMachine {
            instructions: Vec::new(),
            labels_map: std::collections::HashMap::new(),
            translation_map: std::collections::HashMap::new()
        };
        computer.set_ram(ram);
        assert!(computer.is_ram());

        let tm = turing_machine::TuringMachine::new();
        computer.set_turing(tm);
        assert!(!computer.is_ram());

        let lambda = lambda::Lambda {
            expr: lambda::LambdaExpr::Var("".to_string()),
            references: Vec::new(),
            name: "".to_string(),
            force_currying: false,
        };
        computer.set_lambda(lambda);
        assert!(!computer.is_ram());
    }

    #[test]
    fn test_server_computation_order_operations() {
        let mut server = Server::new();

        server.set_computation_order_at(0, "a".to_string());
        server.set_computation_order_at(1, "b".to_string());
        server.set_computation_order_at(2, "c".to_string());

        assert_eq!(server.computation_order.len(), 3);
        assert_eq!(server.computes_at(0), "a");
        assert_eq!(server.computes_at(1), "b");
        assert_eq!(server.computes_at(2), "c");
    }

    #[test]
    fn test_multiple_computer_mappings() {
        let mut computer = Computer::new();

        computer.add_mapping("key1".to_string(), "val1".to_string());
        computer.add_mapping("key2".to_string(), "val2".to_string());
        computer.add_mapping("key3".to_string(), "val3".to_string());

        assert_eq!(
            computer.get_mapping("key1".to_string()),
            Ok("val1".to_string())
        );
        assert_eq!(
            computer.get_mapping("key2".to_string()),
            Ok("val2".to_string())
        );
        assert_eq!(
            computer.get_mapping("key3".to_string()),
            Ok("val3".to_string())
        );
        assert_eq!(
            computer.get_mapping("missing".to_string()),
            Ok("".to_string())
        );
    }
    #[test]
    fn test_server_add_multiple_computers() {
        let mut server = Server::new();
        let computer1 = Computer::new();
        let computer2 = Computer::new();
        let computer3 = Computer::new();

        server.add_computer("comp1".to_string(), computer1);
        server.add_computer("comp2".to_string(), computer2);
        server.add_computer("comp3".to_string(), computer3);

        assert!(server.contains("comp1".to_string()));
        assert!(server.contains("comp2".to_string()));
        assert!(server.contains("comp3".to_string()));
        assert!(!server.contains("comp4".to_string()));
    }

    #[test]
    fn test_computer_encoding() {
        let mut _computer = Computer::new();
        let _tm = turing_machine::TuringMachine {
            states: vec!["q0".to_string(), "q1".to_string()],
            input_alphabet: vec!["0".to_string(), "1".to_string()],
            tape_alphabet: vec!["0".to_string(), "1".to_string(), "_".to_string()],
            initial_state: "q0".to_string(),
            accept_state: "q1".to_string(),
            reject_state: "q1".to_string(),
            halt_state: "q1".to_string(),
            blank_symbol: "_".to_string(),
            transitions: vec![turing_machine::Transition {
                state: "q0".to_string(),
                symbols: vec!["_".to_string()],
                new_state: "q1".to_string(),
                new_symbols: vec!["_".to_string()],
                directions: vec![turing_machine::Direction::Stay],
            }],
            tape_count: 1,
            next_state_id: 10,
        };
        _computer.set_turing(_tm);
        let result = _computer.to_encoding();
        assert!(result.is_ok());
        if let Ok((encoding, map1, map2)) = result {
            assert!(!encoding.is_empty());
            assert!(!map1.is_empty());
            assert!(!map2.is_empty());
        }
    }

    #[test]
    fn test_server_empty_execution() {
        let mut server = Server::new();
        let result = server.execute("test input".to_string(), 100);
        assert!(result.is_err());
    }

    #[test]
    fn test_computer_clone() {
        let computer1 = Computer::new();
        let computer2 = computer1.clone();

        assert!(matches!(computer2.element, ComputingElem::Tm(_)));
        assert_eq!(computer1.mapping.len(), computer2.mapping.len());
    }

    #[test]
    fn test_server_clone() {
        let mut server1 = Server::new();
        server1.set_computation_order_at(0, "test".to_string());

        let server2 = server1.clone();

        assert_eq!(server1.computation_order, server2.computation_order);
        assert_eq!(server1.map_computers.len(), server2.map_computers.len());
    }
    #[test]
    fn test_computer_to_encoding_lambda() {
        let mut computer = Computer::new();
        let lambda = lambda::Lambda {
            expr: lambda::LambdaExpr::Var("x".to_string()),
            references: Vec::new(),
            name: "test".to_string(),
            force_currying: false,
        };
        computer.set_lambda(lambda);

        let result = computer.to_encoding();
        assert!(result.is_ok());
        if let Ok((encoding, map1, map2)) = result {
            assert!(!encoding.is_empty());
            assert!(map1.is_empty());
            assert!(map2.is_empty());
        }
    }

    #[test]
    fn test_computer_to_encoding_ram() {
        let mut computer = Computer::new();
        let ram = ram_machine::RamMachine {
            instructions: vec![],
            labels_map: std::collections::HashMap::new(),
            translation_map: std::collections::HashMap::new()
        };
        computer.set_ram(ram);

        let result = computer.to_encoding();
        assert!(result.is_ok());
    }

    #[test]
    fn test_server_execute_single_computer() {
        let mut server = Server::new();
        let computer = Computer::new();
        server.add_computer("test".to_string(), computer);
        server.set_computation_order_at(0, "test".to_string());

        let result = server.execute("test".to_string(), 100);
        assert!(result.is_ok());
    }

    #[test]
    fn test_computer_simulate_lambda() {
        let mut computer = Computer::new();
        let lambda = lambda::Lambda {
            expr: lambda::LambdaExpr::Var("x".to_string()),
            references: Vec::new(),
            name: "".to_string(),
            force_currying: false,
        };
        computer.set_lambda(lambda);

        let context = Server::new();
        let result = computer.simulate("(x)".to_string(), 100, context, 0);
        assert!(result.is_ok());
    }

    #[test]
    fn test_computer_simulate_ram() {
        let mut computer = Computer::new();
        let ram = ram_machine::RamMachine {
            instructions: vec![
                ram_machine::Instruction {
                    opcode: "0111".to_string(),
                    operand: "100".to_string(),
                    label: "".to_string(),
                },
                ram_machine::Instruction {
                    opcode: "0100".to_string(),
                    operand: "1111".to_string(),
                    label: "".to_string(),
                },
                ram_machine::Instruction {
                    opcode: "1000".to_string(),
                    operand: "1111".to_string(),
                    label: "".to_string(),
                },
                ram_machine::Instruction {
                    opcode: "0011".to_string(),
                    operand: "".to_string(),
                    label: "".to_string(),
                },
                ram_machine::Instruction {
                    opcode: "1011".to_string(),
                    operand: "".to_string(),
                    label: "".to_string(),
                },
            ],
            labels_map: std::collections::HashMap::new(),
            translation_map: std::collections::HashMap::new()
        };
        computer.set_ram(ram);

        let context = Server::new();
        let result = computer.simulate("".to_string(), 100, context, 0);
        assert!(result.is_ok());
    }

    #[test]
    fn test_server_multiple_executions() {
        let mut server = Server::new();
        let computer1 = Computer::new();
        let computer2 = Computer::new();

        server.add_computer("c1".to_string(), computer1);
        server.add_computer("c2".to_string(), computer2);

        server.set_computation_order_at(0, "c1".to_string());
        server.set_computation_order_at(1, "c2".to_string());

        let result1 = server.execute("test1".to_string(), 100);
        assert!(result1.is_ok());

        let result2 = server.execute("test2".to_string(), 200);
        assert!(result2.is_ok());
    }

    #[test]
    fn test_computer_mapping_overwrite() {
        let mut computer = Computer::new();
        computer.add_mapping("key".to_string(), "value1".to_string());
        computer.add_mapping("key".to_string(), "value2".to_string());

        assert_eq!(
            computer.get_mapping("key".to_string()),
            Ok("value2".to_string())
        );
    }

    #[test]
    fn test_server_invalid_computer_name() {
        let mut server = Server::new();
        server.set_computation_order_at(0, "nonexistent".to_string());

        let result = server.execute("test".to_string(), 100);
        assert!(result.is_err());
    }

    #[test]
    fn test_computer_max_steps_limit() {
        let mut computer = Computer::new();
        let ram = ram_machine::RamMachine {
            instructions: vec![
                ram_machine::Instruction {
                    opcode: "0111".to_string(),
                    operand: "100".to_string(),
                    label: "".to_string(),
                },
                ram_machine::Instruction {
                    opcode: "1001".to_string(),
                    operand: "0".to_string(),
                    label: "".to_string(),
                },
                ram_machine::Instruction {
                    opcode: "1011".to_string(),
                    operand: "".to_string(),
                    label: "".to_string(),
                },
            ],
            labels_map: std::collections::HashMap::new(),
            translation_map: std::collections::HashMap::new()
        };
        computer.set_ram(ram);

        let context = Server::new();

        let result = computer.simulate("test".to_string(), 0, context, 0);
        assert!(result.is_ok());
    }
    #[test]
    fn test_computer_to_tm_from_lambda() {
        let mut computer = Computer::new();
        let lambda = lambda::Lambda {
            expr: lambda::LambdaExpr::Var("x".to_string()),
            references: Vec::new(),
            name: "".to_string(),
            force_currying: false,
        };
        computer.set_lambda(lambda);

        let mut options = options::Options {
            file: "".to_string(),
            input: "(x)".to_string(),
            convert_to_tm: false,
            convert_to_ram: false,
            convert_to_singletape: false,
            print_computer: false,
            print_number: false,
            print_nth_tm: -1,
            help: false,
            version: false,
            max_steps: 1000,
            status: false,
            print_encoding: false,
            verbose: 1,
        };
        let mut server = Server::new();

        let result = computer.to_tm(&mut options, &mut server);
        assert!(result.is_ok());

        if let Ok(converted) = result {
            assert!(matches!(converted.element, ComputingElem::Tm(_)));
        }
    }

    #[test]
    fn test_computer_to_tm_from_ram() {
        let mut computer = Computer::new();
        let ram = ram_machine::RamMachine {
            instructions: vec![ram_machine::Instruction {
                opcode: "0111".to_string(),
                operand: "100".to_string(),
                label: "".to_string(),
            }],
            labels_map: std::collections::HashMap::new(),
            translation_map: std::collections::HashMap::new()
        };
        computer.set_ram(ram);

        let mut options = options::Options {
            file: "".to_string(),
            input: "1010".to_string(),
            convert_to_tm: false,
            convert_to_ram: false,
            convert_to_singletape: false,
            print_computer: false,
            print_number: false,
            print_nth_tm: -1,
            help: false,
            version: false,
            max_steps: 1000,
            status: false,
            print_encoding: false,
            verbose: 1,
        };
        let mut server = Server::new();

        let result = computer.to_tm(&mut options, &mut server);
        assert!(result.is_ok());

        if let Ok(converted) = result {
            assert!(matches!(converted.element, ComputingElem::Tm(_)));
        }
    }

    #[test]
    fn test_computer_to_tm_already_tm() {
        let mut computer = Computer::new();
        let tm = turing_machine::TuringMachine::new();
        computer.set_turing(tm);

        let mut options = options::Options {
            file: "".to_string(),
            input: "".to_string(),
            convert_to_tm: false,
            convert_to_ram: false,
            convert_to_singletape: false,
            print_computer: false,
            print_number: false,
            print_nth_tm: -1,
            help: false,
            version: false,
            max_steps: 1000,
            status: false,
            print_encoding: false,
            verbose: 1,
        };
        let mut server = Server::new();

        let result = computer.to_tm(&mut options, &mut server);
        assert!(result.is_err());
    }

    #[test]
    fn test_computer_to_tm_preservation() {
        // Test that important properties are preserved during conversion
        let mut computer = Computer::new();
        let lambda = lambda::Lambda {
            expr: lambda::LambdaExpr::Var("x".to_string()),
            references: Vec::new(),
            name: "test".to_string(),
            force_currying: false,
        };
        computer.set_lambda(lambda);

        let mut options = options::Options {
            file: "".to_string(),
            input: "(x)".to_string(),
            convert_to_tm: false,
            convert_to_ram: false,
            convert_to_singletape: false,
            print_computer: false,
            print_number: false,
            print_nth_tm: -1,
            help: false,
            version: false,
            max_steps: 1000,
            status: false,
            print_encoding: false,
            verbose: 1,
        };
        let mut server = Server::new();

        let result = computer.to_tm(&mut options, &mut server);
        assert!(result.is_ok());

        if let Ok(converted) = result {
            if let ComputingElem::Tm(tm) = converted.element {
                assert!(!tm.states.is_empty());
                assert!(!tm.input_alphabet.is_empty());
                assert!(!tm.tape_alphabet.is_empty());
                assert!(!tm.transitions.is_empty());
            }
        }
    }

    #[test]
    fn test_computer_to_tm_mapping_preservation() {
        let mut computer = Computer::new();
        let lambda = lambda::Lambda {
            expr: lambda::LambdaExpr::Var("x".to_string()),
            references: Vec::new(),
            name: "test".to_string(),
            force_currying: false,
        };
        computer.set_lambda(lambda);
        computer.add_mapping("key1".to_string(), "val1".to_string());
        computer.add_mapping("key2".to_string(), "val2".to_string());

        let mut options = options::Options {
            file: "".to_string(),
            input: "(x)".to_string(),
            convert_to_tm: false,
            convert_to_ram: false,
            convert_to_singletape: false,
            print_computer: false,
            print_number: false,
            print_nth_tm: -1,
            help: false,
            version: false,
            max_steps: 1000,
            status: false,
            print_encoding: false,
            verbose: 1,
        };
        let mut server = Server::new();

        let result = computer.to_tm(&mut options, &mut server);
        assert!(result.is_ok());

        if let Ok(converted) = result {
            assert_eq!(converted.mapping.len(), computer.mapping.len());
            assert_eq!(
                converted.get_mapping("key1".to_string()),
                Ok("val1".to_string())
            );
            assert_eq!(
                converted.get_mapping("key2".to_string()),
                Ok("val2".to_string())
            );
        }
    }
    #[test]
    fn test_computer_to_ram_from_tm() {
        let mut computer = Computer::new();
        let tm = turing_machine::TuringMachine {
            states: vec!["q0".to_string(), "q1".to_string()],
            input_alphabet: vec!["0".to_string(), "1".to_string()],
            tape_alphabet: vec!["0".to_string(), "1".to_string(), "_".to_string()],
            initial_state: "q0".to_string(),
            accept_state: "q1".to_string(),
            reject_state: "q1".to_string(),
            halt_state: "q1".to_string(),
            blank_symbol: "_".to_string(),
            transitions: vec![turing_machine::Transition {
                state: "q0".to_string(),
                symbols: vec!["0".to_string()],
                new_state: "q1".to_string(),
                new_symbols: vec!["1".to_string()],
                directions: vec![turing_machine::Direction::Right],
            }],
            tape_count: 1,
            next_state_id: 2,
        };
        computer.set_turing(tm);

        let mut options = options::Options {
            file: "".to_string(),
            input: "0".to_string(),
            convert_to_tm: false,
            convert_to_ram: false,
            convert_to_singletape: false,
            print_computer: false,
            print_number: false,
            print_nth_tm: -1,
            help: false,
            version: false,
            max_steps: 1000,
            status: false,
            print_encoding: false,
            verbose: 1,
        };
        let mut server = Server::new();

        let result = computer.to_ram(&mut options, &mut server);
        assert!(result.is_ok());

        if let Ok(converted) = result {
            assert!(matches!(converted.element, ComputingElem::Ram(_)));
        }
    }

    #[test]
    fn test_computer_to_ram_already_ram() {
        let mut computer = Computer::new();
        let ram = ram_machine::RamMachine {
            instructions: vec![],
            labels_map: std::collections::HashMap::new(),
            translation_map: std::collections::HashMap::new()
        };
        computer.set_ram(ram);

        let mut options = options::Options {
            file: "".to_string(),
            input: "".to_string(),
            convert_to_tm: false,
            convert_to_ram: false,
            convert_to_singletape: false,
            print_computer: false,
            print_number: false,
            print_nth_tm: -1,
            help: false,
            version: false,
            max_steps: 1000,
            status: false,
            print_encoding: false,
            verbose: 1,
        };
        let mut server = Server::new();

        let result = computer.to_ram(&mut options, &mut server);
        assert!(result.is_err());
    }

    #[test]
    fn test_computer_to_ram_preservation() {
        let mut computer = Computer::new();
        let tm = turing_machine::TuringMachine {
            states: vec!["q0".to_string(), "q1".to_string()],
            input_alphabet: vec!["0".to_string(), "1".to_string()],
            tape_alphabet: vec!["0".to_string(), "1".to_string(), "_".to_string()],
            initial_state: "q0".to_string(),
            accept_state: "q1".to_string(),
            reject_state: "q1".to_string(),
            halt_state: "q1".to_string(),
            blank_symbol: "_".to_string(),
            transitions: vec![turing_machine::Transition {
                state: "q0".to_string(),
                symbols: vec!["0".to_string()],
                new_state: "q1".to_string(),
                new_symbols: vec!["1".to_string()],
                directions: vec![turing_machine::Direction::Right],
            }],
            tape_count: 1,
            next_state_id: 2,
        };
        computer.set_turing(tm);

        let mut options = options::Options {
            file: "".to_string(),
            input: "0".to_string(),
            convert_to_tm: false,
            convert_to_ram: false,
            convert_to_singletape: false,
            print_computer: false,
            print_number: false,
            print_nth_tm: -1,
            help: false,
            version: false,
            max_steps: 1000,
            status: false,
            print_encoding: false,
            verbose: 1,
        };
        let mut server = Server::new();

        let result = computer.to_ram(&mut options, &mut server);
        assert!(result.is_ok());

        if let Ok(converted) = result {
            if let ComputingElem::Ram(ram) = converted.element {
                assert!(!ram.instructions.is_empty());
                assert!(!ram.labels_map.is_empty());
            }
        }
    }

    #[test]
    fn test_computer_to_ram_mapping_preservation() {
        let mut computer = Computer::new();
        let mut tm = turing_machine::TuringMachine::new();
        tm.states = vec!["q0".to_string(), "q1".to_string()];
        tm.input_alphabet = vec!["0".to_string(), "1".to_string()];
        tm.tape_alphabet = vec!["0".to_string(), "1".to_string(), "_".to_string()];
        tm.initial_state = "q0".to_string();
        tm.accept_state = "q1".to_string();
        tm.reject_state = "q1".to_string();
        tm.halt_state = "q1".to_string();
        tm.blank_symbol = "_".to_string();
        tm.transitions = vec![turing_machine::Transition {
            state: "q0".to_string(),
            symbols: vec!["0".to_string()],
            new_state: "q1".to_string(),
            new_symbols: vec!["1".to_string()],
            directions: vec![turing_machine::Direction::Right],
        }];
        tm.tape_count = 1;
        tm.next_state_id = 2;
        computer.set_turing(tm);
        computer.add_mapping("key1".to_string(), "val1".to_string());
        computer.add_mapping("key2".to_string(), "val2".to_string());

        let mut options = options::Options {
            file: "".to_string(),
            input: "test".to_string(),
            convert_to_tm: false,
            convert_to_ram: false,
            convert_to_singletape: false,
            print_computer: false,
            print_number: false,
            print_nth_tm: -1,
            help: false,
            version: false,
            max_steps: 1000,
            status: false,
            print_encoding: false,
            verbose: 1,
        };
        let mut server = Server::new();

        let result = computer.to_ram(&mut options, &mut server);
        assert!(result.is_ok());

        if let Ok(converted) = result {
            assert_eq!(converted.mapping.len(), computer.mapping.len());
            assert_eq!(
                converted.get_mapping("key1".to_string()),
                Ok("val1".to_string())
            );
            assert_eq!(
                converted.get_mapping("key2".to_string()),
                Ok("val2".to_string())
            );
        }
    }
    #[test]
    fn test_server_execute_complex() {
        let mut server = Server::new();

        // Add multiple computers
        let computer1 = Computer::new();
        let computer2 = Computer::new();
        let computer3 = Computer::new();

        server.add_computer("c1".to_string(), computer1);
        server.add_computer("c2".to_string(), computer2);
        server.add_computer("c3".to_string(), computer3);

        // Set computation order
        server.set_computation_order_at(0, "c1".to_string());
        server.set_computation_order_at(1, "c2".to_string());
        server.set_computation_order_at(2, "c3".to_string());

        // Test execution with chained computers
        let result = server.execute("test input".to_string(), 1000);
        assert!(result.is_ok());

        if let Ok((state, _, output, _, comp)) = result {
            assert!(!state.is_empty());
            assert!(output.is_empty());
            assert!(!comp.is_empty());
        }
    }

    #[test]
    fn test_computer_simulate_tm() {
        let mut computer = Computer::new();
        let tm = turing_machine::TuringMachine {
            states: vec!["q0".to_string(), "q1".to_string()],
            input_alphabet: vec!["0".to_string(), "1".to_string()],
            tape_alphabet: vec!["0".to_string(), "1".to_string(), "_".to_string()],
            initial_state: "q0".to_string(),
            accept_state: "q1".to_string(),
            reject_state: "q1".to_string(),
            halt_state: "q1".to_string(),
            blank_symbol: "_".to_string(),
            transitions: vec![turing_machine::Transition {
                state: "q0".to_string(),
                symbols: vec!["0".to_string()],
                new_state: "q1".to_string(),
                new_symbols: vec!["1".to_string()],
                directions: vec![turing_machine::Direction::Right],
            }],
            tape_count: 1,
            next_state_id: 2,
        };
        computer.set_turing(tm);

        let context = Server::new();
        let result = computer.simulate("0".to_string(), 100, context, 0);
        assert!(result.is_ok());
        if let Ok((state, _, tape, _, comp)) = result {
            assert!(!state.is_empty());
            assert!(!tape.is_empty());
            assert!(!comp.is_empty());
        }
    }

    #[test]
    fn test_computer_simulate_empty_input() {
        let computer = Computer::new();
        let context = Server::new();

        let result = computer.simulate("".to_string(), 100, context, 0);
        assert!(result.is_ok());
    }

    #[test]
    fn test_server_empty_computer_list() {
        let mut server = Server::new();
        assert!(server.map_computers.is_empty());
        assert!(server.computation_order.is_empty());

        let result = server.execute("test".to_string(), 100);
        assert!(result.is_err());
    }

    #[test]
    fn test_server_invalid_computation_order() {
        let mut server = Server::new();
        let computer = Computer::new();

        server.add_computer("test".to_string(), computer);
        server.set_computation_order_at(0, "invalid".to_string());

        let result = server.execute("test".to_string(), 100);
        assert!(result.is_err());
    }

    #[test]
    fn test_computer_simulate_zero_steps() {
        let computer = Computer::new();
        let context = Server::new();

        let result = computer.simulate("test".to_string(), 0, context, 0);
        assert!(result.is_err());
    }

    #[test]
    fn test_computer_simulate_large_input() {
        let computer = Computer::new();
        let context = Server::new();

        let large_input = "0".repeat(1000);
        let result = computer.simulate(large_input, 100, context, 0);
        assert!(result.is_ok());
    }

    #[test]
    fn test_server_execute_empty_computation_order() {
        let mut server = Server::new();
        let computer = Computer::new();

        server.add_computer("test".to_string(), computer);

        let result = server.execute("test".to_string(), 100);
        assert!(result.is_err());
    }

    #[test]
    fn test_computer_simulate_invalid_tm_transition() {
        let mut computer = Computer::new();
        let tm = turing_machine::TuringMachine {
            states: vec!["q0".to_string()],
            input_alphabet: vec!["0".to_string()],
            tape_alphabet: vec!["0".to_string(), "_".to_string()],
            initial_state: "q0".to_string(),
            accept_state: "q1".to_string(), // Invalid state
            reject_state: "q1".to_string(), // Invalid state
            halt_state: "q1".to_string(),   // Invalid state
            blank_symbol: "_".to_string(),
            transitions: vec![],
            tape_count: 1,
            next_state_id: 1,
        };
        computer.set_turing(tm);

        let context = Server::new();
        let result = computer.simulate("0".to_string(), 100, context, 0);
        assert!(result.is_ok());
    }

    #[test]
    fn test_computer_simulate_with_large_context() {
        let mut server = Server::new();
        for i in 0..100 {
            server.add_computer(format!("comp{}", i), Computer::new());
        }

        let computer = Computer::new();
        let result = computer.simulate("test".to_string(), 100, server, 0);
        assert!(result.is_ok());
    }

    #[test]
    fn test_computer_simulate_with_nested_context() {
        let mut inner_server = Server::new();
        inner_server.add_computer("inner".to_string(), Computer::new());

        let mut outer_server = Server::new();
        outer_server.add_computer("outer".to_string(), Computer::new());

        let computer = Computer::new();
        let result = computer.simulate("test".to_string(), 100, outer_server, 0);
        assert!(result.is_ok());
    }

    #[test]
    fn test_matches_b_tm_integration() {
        let mut opt = options::Options::default();
        opt.file = "examples/matches b.tm".to_string(); 
        opt.input = "aa".to_string();
        
        let mut server = Server::new();
        let computer = file_handler::handle_file_reads(opt.file.clone(), &mut server).unwrap();
        server.add_computer(opt.file.clone(), computer);
        server.set_computation_order_at(0, opt.file.clone());
        
        let result: Result<(String, usize, String, usize, Vec<String>), String> = server.execute(opt.input, 1000);
        assert!(result.is_ok());
        if let Ok((state, _, output, _, _)) = result {
            assert_eq!(state, "halt");
            assert_eq!(output, "aabb");
        }
    }

    #[test]
    fn test_matches_b_multitape_tm_integration() {
        let mut opt = options::Options::default();
        opt.file = "examples/matches b multitape.tm".to_string(); 
        opt.input = "aa".to_string();
        
        let mut server = Server::new();
        let computer = file_handler::handle_file_reads(opt.file.clone(), &mut server).unwrap();
        server.add_computer(opt.file.clone(), computer);
        server.set_computation_order_at(0, opt.file.clone());
        
        let result: Result<(String, usize, String, usize, Vec<String>), String> = server.execute(opt.input, 1000);
        assert!(result.is_ok());
        if let Ok((state, _, output, _, _)) = result {
            assert_eq!(state, "halt");
            assert_eq!(output, "aabb");
        }
    }

    #[test]
    fn test_matches_b_multitape_conversion_tm_integration() {
        let mut opt = options::Options::default();
        opt.file = "examples/matches b multitape.tm".to_string(); 
        opt.input = "aa".to_string();
        
        let mut server = Server::new();
        let mut computer = file_handler::handle_file_reads(opt.file.clone(), &mut server).unwrap();
        computer.convert_to_singletape().unwrap();
        server.add_computer(opt.file.clone(), computer);
        server.set_computation_order_at(0, opt.file.clone());
        
        let result: Result<(String, usize, String, usize, Vec<String>), String> = server.execute(opt.input, 1000);
        assert!(result.is_ok());
        if let Ok((state, _, output, _, _)) = result {
            assert_eq!(state, "halt");
            assert_eq!(output, "aabb");
        }
    }

    #[test]
    fn test_lambda_fact_integration() {
        let mut opt = options::Options::default();
        opt.file = "src/standard/library.lambda".to_string(); 
        opt.input = "(FACT 2)".to_string();
        
        let mut server = Server::new();
        let computer = file_handler::handle_file_reads(opt.file.clone(), &mut server).unwrap();
        server.add_computer(opt.file.clone(), computer);
        server.set_computation_order_at(0, opt.file.clone());
        
        let result: Result<(String, usize, String, usize, Vec<String>), String> = server.execute(opt.input, 1000);
        assert!(result.is_ok());
        if let Ok((state, _, _, _, _)) = result {
            assert_eq!(state, "2");
        }
    }
    #[test]
    fn test_lambda_succ_conversion_integration() {
        let mut opt = options::Options::default();
        opt.file = "src/standard/library.lambda".to_string(); 
        opt.input = "(SUCC 3)".to_string();
        opt.max_steps = 10000;
        
        let mut server = Server::new();
        let computer = file_handler::handle_file_reads(opt.file.clone(), &mut server).unwrap().to_tm(&mut opt, &mut server).unwrap();
        server.add_computer(opt.file.clone(), computer);
        server.set_computation_order_at(0, opt.file.clone());
        
        let result: Result<(String, usize, String, usize, Vec<String>), String> = server.execute(opt.input, opt.max_steps);
        assert!(result.is_ok());
        if let Ok((state, _, output, _, _)) = result {
            assert_eq!(state, "halt");
            assert_eq!(output, "(/fx.(f(f(f(fx)))))");
        }
    }

    /* #[test]
    fn test_lambda_succ_double_conversion_integration() {
        let mut opt = options::Options::default();
        opt.file = "src/standard/library.lambda".to_string(); 
        opt.input = "(SUCC 3)".to_string();
        opt.max_steps = 100000;
        
        let mut server = Server::new();
        let computer = file_handler::handle_file_reads(opt.file.clone(), &mut server).unwrap().to_ram(&mut opt, &mut server).unwrap();
        server.add_computer(opt.file.clone(), computer);
        server.set_computation_order_at(0, opt.file.clone());
        
        let result: Result<(String, usize, String, usize, Vec<String>), String> = server.execute(opt.input, opt.max_steps);
        assert!(result.is_ok());
        if let Ok((state, _, output, _, _)) = result {
            assert_eq!(state, "halt");
            assert_eq!(output, "(/fx.(f(f(f(fx)))))");
        }
    } */

    #[test]
    fn test_plusfive_multitape_tm_integration() {
        let mut opt = options::Options::default();
        opt.file = "examples/plusfive.tm".to_string(); 
        opt.input = "010".to_string();
        
        let mut server = Server::new();
        let computer = file_handler::handle_file_reads(opt.file.clone(), &mut server).unwrap();
        server.add_computer(opt.file.clone(), computer);
        server.set_computation_order_at(0, opt.file.clone());
        
        let result: Result<(String, usize, String, usize, Vec<String>), String> = server.execute(opt.input, 1000);
        assert!(result.is_ok());
        if let Ok((state, _, output, _, _)) = result {
            assert_eq!(state, "halt");
            assert_eq!(output, "111");
        }
    }

    #[test]
    fn test_plusfive_ram_integration() {
        let mut opt = options::Options::default();
        opt.file = "examples/plusfive.ram".to_string(); 
        opt.input = "010".to_string();
        
        let mut server = Server::new();
        let computer = file_handler::handle_file_reads(opt.file.clone(), &mut server).unwrap();
        server.add_computer(opt.file.clone(), computer);
        server.set_computation_order_at(0, opt.file.clone());
        
        let result: Result<(String, usize, String, usize, Vec<String>), String> = server.execute(opt.input, 1000);
        assert!(result.is_ok());
        if let Ok((state, _, output, _, _)) = result {
            assert_eq!(state, "halt");
            assert_eq!(output, "111");
        }
    }

    #[test]
    fn test_dyn_ram_integration() {
        let mut opt = options::Options::default();
        opt.file = "examples/dyn.ram".to_string(); 
        opt.input = "010".to_string();
        
        let mut server = Server::new();
        let computer = file_handler::handle_file_reads(opt.file.clone(), &mut server).unwrap();
        server.add_computer(opt.file.clone(), computer);
        server.set_computation_order_at(0, opt.file.clone());
        
        let result: Result<(String, usize, String, usize, Vec<String>), String> = server.execute(opt.input, 1000);
        assert!(result.is_ok());
        if let Ok((state, _, output, _, _)) = result {
            assert_eq!(state, "halt");
            assert_eq!(output, "1");
        }
    }

    #[test]
    fn test_dyn_conversion_ram_integration() {
        let mut opt = options::Options::default();
        opt.file = "examples/dyn.ram".to_string(); 
        opt.input = "010".to_string();
        
        let mut server = Server::new();
        let computer = file_handler::handle_file_reads(opt.file.clone(), &mut server).unwrap().to_tm(&mut opt, &mut server).unwrap();
        server.add_computer(opt.file.clone(), computer);
        server.set_computation_order_at(0, opt.file.clone());
        
        let result: Result<(String, usize, String, usize, Vec<String>), String> = server.execute(opt.input, 100000);
        assert!(result.is_ok());
        if let Ok((state, _, output, _, _)) = result {
            assert_eq!(state, "halt");
            assert_eq!(output, "0000001");
        }
    }

    #[test]
    fn test_pda_integration() {
        let mut opt: options::Options = options::Options::default();
        opt.file = "examples/0n1m2m3n.pda".to_string(); 
        opt.input = "0011122233".to_string();
        
        let mut server = Server::new();
        let computer = file_handler::handle_file_reads(opt.file.clone(), &mut server).unwrap();
        server.add_computer(opt.file.clone(), computer);
        server.set_computation_order_at(0, opt.file.clone());
        
        let result: Result<(String, usize, String, usize, Vec<String>), String> = server.execute(opt.input, 1000);
        assert!(result.is_ok());
        if let Ok((state, _, output, _, _)) = result {
            assert_eq!(state, "halt");
            assert_eq!(output, "");
        }
    }

    #[test]
    fn test_regex_integration() {
        let mut opt: options::Options = options::Options::default();
        opt.file = "examples/regex.reg".to_string(); 
        opt.input = "abbbcddce".to_string();
        
        let mut server = Server::new();
        let computer = file_handler::handle_file_reads(opt.file.clone(), &mut server).unwrap();
        server.add_computer(opt.file.clone(), computer);
        server.set_computation_order_at(0, opt.file.clone());
        
        let result: Result<(String, usize, String, usize, Vec<String>), String> = server.execute(opt.input, 1000);
        assert!(result.is_ok());
        if let Ok((state, _, output, _, _)) = result {
            assert_eq!(state, "accept");
            assert_eq!(output, "");
        }
    }

}
