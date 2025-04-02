// file: automaton.rs
// Project: Computing Simulator
// author: dp

use crate::computer;
use crate::utils;

#[derive(Clone)]
pub struct RamMachine {
    pub instructions: Vec<Instruction>,
}

#[derive(Clone)]
pub struct Instruction {
    pub opcode: String,
    pub operand: String,
}

impl RamMachine {
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
            "CALL" => "1100",
            "MOV" => "1101",
            "LD" => "1110",
            "STD" => "1111",
            _ => "0000",
        };
        opcode.to_string()
    }

    pub fn simulate(
        self,
        input: String,
        max_steps: i32,
        this_computer_object: computer::Computer,
        context: computer::Server,
    ) -> Result<computer::SimulationResult, String> {
        let mut ir: String;
        let mut out: String = "".to_string();
        let mut pc: String = "0".to_string();
        let mut acc: String = "0".to_string();
        let mut ar: String;
        let mut mov: String = "0".to_string();
        let mut input_head = 0;
        let mut memory: std::collections::HashMap<String, String> =
            std::collections::HashMap::new();
        for (index, instr) in self.instructions.clone().into_iter().enumerate() {
            memory.insert(
                utils::int2bin(index as i32, 0),
                instr.opcode.clone() + &instr.operand.clone(),
            );
        }
        let mut computation = Vec::new();
        let mut steps = 0;
        while steps < max_steps {
            steps += 1;
            ir = memory[&pc].clone()[0..4].to_string();
            ar = memory[&pc].clone()[4..].to_string();
            pc = utils::int2bin(utils::bin2int(pc) + 1, 0);
            computation
                .push("ram;".to_string() + &ir.clone() + ";" + &ar.clone() + ";" + &acc.clone());
            match ir.as_str() {
                "0000" => {
                    // R: Read [operands] bit from input
                    let end = input_head + (utils::bin2int(ar) as usize);
                    if input.len() < end {
                        acc = format!(
                            "{:0>width$b}",
                            utils::bin2int(input[input_head..input.len()].to_string()),
                            width = end - input_head
                        )
                    } else {
                        acc = input[input_head..end].to_string();
                    }
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
                        0,
                    );
                }
                "0110" => {
                    // S: Subtract AR from ACC
                    acc = utils::int2bin(
                        utils::bin2int(acc) - (utils::bin2int(memory[&ar].clone())),
                        0,
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
                "1100" => {
                    // CALL: call a subroutine
                    let mapping_key = (utils::bin2int(ar.clone())).to_string();
                    let mapping = this_computer_object
                        .clone()
                        .get_mapping(mapping_key.clone());
                    let subroutine = context.clone().get_computer(mapping).unwrap().clone();
                    match subroutine.clone().simulate(
                        acc.clone(),
                        max_steps - steps,
                        context.clone(),
                        0,
                    ) {
                        Ok((state, _, tape, steps, sub_computation)) => {
                            computation.extend(sub_computation);
                            if state == "accept" || state == "halt" {
                                match subroutine.element {
                                    computer::ComputingElem::Tm(m) => {
                                        acc = tape
                                            .into_iter()
                                            .filter(|symb| *symb != m.blank_symbol)
                                            .collect::<Vec<String>>()
                                            .join("")
                                    }
                                    computer::ComputingElem::Ram(_) => {
                                        acc = tape.join("");
                                    }
                                    computer::ComputingElem::Lambda(_) => {
                                        acc = "0".to_string();
                                    }
                                }
                            } else {
                                return Ok((
                                    "reject".to_string(),
                                    0,
                                    vec![out],
                                    steps,
                                    computation,
                                ));
                            }
                        }
                        Err(error) => return Err(error),
                    }
                }
                "1101" => {
                    // MOV: copy the value of acc to the mov register
                    mov = acc.clone();
                }
                "1110" => {
                    // LD: load the memory at address in MOV
                    acc = memory[&mov].clone();
                }
                "1111" => {
                    // STD: store the memory at address in MOV
                    memory.insert(mov.clone(), acc.clone());
                }
                _ => {
                    // default: Halt
                    break;
                }
            }
        }
        Ok(("halt".to_string(), 0, vec![out], steps, computation))
    }

    pub fn to_encoding(&self) -> computer::EncodingResult {
        let mut encoding = "#".to_string();
        for (counter, instr) in self.instructions.clone().into_iter().enumerate() {
            let counter_number_bits = if counter > 0 {
                (counter as f32).log2().ceil() as i32
            } else {
                1
            };
            if instr.opcode == "1011" || instr.opcode == "0011" {
                // Write and Halt does not have operands
                encoding = encoding
                    + &utils::int2bin(counter as i32, (counter_number_bits) as usize)
                    + ","
                    + &instr.opcode
                    + "#";
            } else {
                encoding = encoding
                    + &utils::int2bin(counter as i32, (counter_number_bits) as usize)
                    + ","
                    + &instr.opcode
                    + &(utils::int2bin(utils::bin2int(instr.operand), 0))
                    + "#";
            }
        }
        (
            encoding,
            std::collections::HashMap::new(),
            std::collections::HashMap::new(),
        )
    }
}
