// file: computer.rs
// Project: Computing Simulator
// author: dp

use crate::ram_machine;
use crate::turing_machine;
use crate::utils;

pub type EncodingResult = (
    String,
    std::collections::HashMap<String, String>,
    std::collections::HashMap<String, String>,
);

#[derive(Clone)]
pub struct Computer {
    pub ram_machine: Option<ram_machine::RamMachine>,
    pub turing_machine: Option<turing_machine::TuringMachine>,
    pub mapping: std::collections::HashMap<String, String>,
}

#[derive(Clone)]
pub struct Server {
    pub map_computers: std::collections::HashMap<String, Computer>,
    pub computation_order: Vec<String>,
}

impl Computer {
    pub fn is_ram(&self) -> bool {
        self.ram_machine.is_some()
    }

    pub fn is_turing(&self) -> bool {
        self.turing_machine.is_some()
    }

    pub fn new() -> Computer {
        Computer {
            ram_machine: None,
            turing_machine: None,
            mapping: std::collections::HashMap::new(),
        }
    }
    pub fn to_encoding(&self) -> Result<EncodingResult, String> {
        if self.is_ram() {
            return Ok(self.ram_machine.as_ref().unwrap().to_encoding());
        } else if self.is_turing() {
            return Ok(self.turing_machine.as_ref().unwrap().to_encoding());
        } else {
            return Err("empty computer".to_string());
        }
    }

    pub fn set_ram(&mut self, ram_machine: ram_machine::RamMachine) {
        self.ram_machine = Some(ram_machine);
        self.turing_machine = None;
    }

    pub fn set_turing(&mut self, turing_machine: turing_machine::TuringMachine) {
        self.turing_machine = Some(turing_machine);
        self.ram_machine = None;
    }

    pub fn simulate(
        self,
        input: String,
        max_steps: i32,
        context: Server,
        head: usize,
    ) -> Result<(String, usize, Vec<String>, i32), String> {
        if self.is_ram() {
            self.ram_machine
                .clone()
                .unwrap()
                .simulate(input.clone(), max_steps, self, context)
        } else {
            let input_vec = utils::input_string_to_vec(
                self.turing_machine.clone().unwrap().tape_alphabet.clone(),
                input,
            );
            self.turing_machine
                .clone()
                .unwrap()
                .simulate(input_vec, max_steps, self, context, head)
        }
    }
    pub fn add_mapping(&mut self, name: String, value: String) {
        self.mapping.insert(name, value);
    }
    pub fn get_mapping(&self, name: String) -> String {
        if self.mapping.contains_key(&name) {
            self.mapping[&name].clone()
        } else {
            "".to_string()
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
    ) -> Result<(String, usize, String, i32), String> {
        let mut steps: i32 = 0;
        let mut output: String = input;
        let mut final_state = "".to_string();
        let mut current_head = 0;
        for name in self.computation_order.clone() {
            let computer = self.get_computer(name.clone()).unwrap();
            let result = computer
                .clone()
                .simulate(output, max_steps - steps, self.clone(), 0);
            match result {
                Ok((state, head, tape, s)) => {
                    final_state = state;
                    current_head = head;
                    output = tape.join("");
                    steps += s;
                }
                Err(e) => {
                    return Err(e);
                }
            }
        }
        Ok((final_state, current_head, output, steps))
    }
}
