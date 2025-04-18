// file: automaton.rs
// Project: Computing Simulator
// author: dp

use crate::computer;
use crate::utils;

#[derive(Clone)]
pub struct RamMachine {
    pub instructions: Vec<Instruction>,
    pub labels_map: std::collections::HashMap<String, String>,
    pub translation_map: std::collections::HashMap<String, String>
}

#[derive(Clone)]
pub struct Instruction {
    pub opcode: String,
    pub operand: String,
    pub label: String,
}

impl RamMachine {
    pub fn is_instruction(instruction: &str) -> bool {
        let instructions: Vec<&str> = vec![
            "R", "MIR", "MIL", "W", "L", "A", "S", "INIT", "ST", "JUMP", "CJUMP", "H", "CALL",
            "MOV", "LD", "STD",
        ];
        if instructions.contains(&instruction) {
            return true;
        }
        false
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
        max_steps: usize,
        this_computer_object: computer::Computer,
        context: computer::Server,
    ) -> Result<computer::SimulationResult, String> {
        let mut input = input.clone();
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
            if !instr.opcode.is_empty() {
                if instr.label.is_empty() {
                    memory.insert(
                        utils::int2bin(index as i32, 0),
                        instr.opcode.clone() + &instr.operand.clone(),
                    );
                } else {
                    memory.insert(
                        utils::int2bin(index as i32, 0),
                        instr.opcode.clone()
                            + self
                                .labels_map
                                .get(&instr.label)
                                .ok_or(format!("key not found: {}", instr.label))?,
                    );
                }
            } else {
                memory.insert(utils::int2bin(index as i32, 0), "0".to_string());
            }
        }
        let mut computation = Vec::new();
        let mut steps = 0;
        while steps < max_steps {
            /* println!("Memory: {}", memory.len());
            let mut mem_: Vec<(i32, String)> = memory
                .clone()
                .iter()
                .map(|(i, j)| (utils::bin2int(i.to_string()).unwrap(), j.to_string()))
                .collect();
            mem_.sort_by_key(|k| k.0);
            for (i, j) in mem_ {
                println!("{} -> {}", i, j);
            } */
            // print!("STEP: {} -- ", steps);
            steps += 1;
            ir = memory
                .get(&pc)
                .ok_or(format!("key not found: {}", pc))?
                .clone()[0..4]
                .to_string();
            ar = memory
                .get(&pc)
                .ok_or(format!("key not found: {}", pc))?
                .clone()[4..]
                .to_string();
            pc = utils::int2bin(utils::bin2int(pc)? + 1, 0);
            computation
                .push("ram;".to_string() + &ir.clone() + ";" + &ar.clone() + ";" + &acc.clone());
            match ir.as_str() {
                "0000" => {
                    // println!("R: acc: {} -- mov: {} -- ir: {} -- ar: {}", acc.clone(), mov.clone(), ir.clone(), ar.clone());
                    // R: Read [operands] bit from input
                    let end = input_head + (utils::bin2int(ar)? as usize);
                    if input.len() < end {
                        acc = format!(
                            "{:0>width$b}",
                            utils::bin2int(input[input_head..input.len()].to_string())?,
                            width = end - input_head
                        )
                    } else {
                        acc = input[input_head..end].to_string();
                    }
                }
                "0001" => {
                    // println!("MIR: acc: {} -- mov: {} -- ir: {} -- ar: {}", acc.clone(), mov.clone(), ir.clone(), ar.clone());
                    // MIR: move input head [operands] bits to the right
                    input_head += utils::bin2int(ar)? as usize;
                }
                "0010" => {
                    // println!("MIL: acc: {} -- mov: {} -- ir: {} -- ar: {}", acc.clone(), mov.clone(), ir.clone(), ar.clone());
                    // MIL: move input head [operands] bits to the left
                    let to_sub = utils::bin2int(ar)? as usize;
                    if input_head >= to_sub {
                        input_head -= to_sub;
                    } else {
                        let zeros = "0".repeat(to_sub - input_head);
                        input = zeros + &input;
                        input_head = 0;
                    }
                }
                "0011" => {
                    // println!("W: acc: {} -- mov: {} -- ir: {} -- ar: {}", acc.clone(), mov.clone(), ir.clone(), ar.clone());
                    // W: Write ACC to output
                    out = out + &acc.clone();
                }
                "0100" => {
                    // println!("L: acc: {} -- mov: {} -- ir: {} -- ar: {}", acc.clone(), mov.clone(), ir.clone(), ar.clone());
                    // L: Load AR to ACC
                    if !memory.contains_key(&ar) {
                        memory.insert(ar.clone(), "0".to_string());
                    }
                    acc = memory
                        .get(&ar)
                        .ok_or(format!("key not found: {}", ar))?
                        .clone();
                }
                "0101" => {
                    // println!("A: acc: {} -- mov: {} -- ir: {} -- ar: {}", acc.clone(), mov.clone(), ir.clone(), ar.clone());
                    // A: Add AR to ACC
                    acc = utils::int2bin(
                        utils::bin2int(acc)?
                            + utils::bin2int(
                                memory
                                    .get(&ar)
                                    .ok_or(format!("key not found: {}", ar))?
                                    .clone(),
                            )?,
                        0,
                    );
                }
                "0110" => {
                    // println!("S: acc: {} -- mov: {} -- ir: {} -- ar: {}", acc.clone(), mov.clone(), ir.clone(), ar.clone());
                    // S: Subtract AR from ACC
                    acc = utils::int2bin(
                        utils::bin2int(acc)?
                            - (utils::bin2int(
                                memory
                                    .get(&ar)
                                    .ok_or(format!("key not found: {}", ar))?
                                    .clone(),
                            )?),
                        0,
                    );
                }
                "0111" => {
                    // println!("INIT: acc: {} -- mov: {} -- ir: {} -- ar: {}", acc.clone(), mov.clone(), ir.clone(), ar.clone());
                    // INIT: Initialize ACC to [operands]
                    acc = ar.clone();
                }
                "1000" => {
                    // println!("ST: acc: {} -- mov: {} -- ir: {} -- ar: {}", acc.clone(), mov.clone(), ir.clone(), ar.clone());
                    // ST: Store ACC to AR
                    memory.insert(ar.clone(), acc.clone());
                }
                "1001" => {
                    // println!("JUMP: acc: {} -- mov: {} -- ir: {} -- ar: {}", acc.clone(), mov.clone(), ir.clone(), ar.clone());
                    // JUMP: Jump to AR
                    pc = ar.clone();
                }
                "1010" => {
                    // println!("CJUMP: acc: {} -- mov: {} -- ir: {} -- ar: {}", acc.clone(), mov.clone(), ir.clone(), ar.clone());
                    // CJUMP: Conditional jump to AR if ACC is 0000
                    if !acc.contains("1") {
                        pc = ar.clone();
                    }
                }
                "1011" => {
                    // println!("HALT: acc: {} -- mov: {} -- ir: {} -- ar: {}", acc.clone(), mov.clone(), ir.clone(), ar.clone());
                    // HALT: Halt
                    break;
                }
                "1100" => {
                    // println!("CALL: acc: {} -- mov: {} -- ir: {} -- ar: {}", acc.clone(), mov.clone(), ir.clone(), ar.clone());
                    // CALL: call a subroutine
                    let mapping_key = (utils::bin2int(ar.clone())?).to_string();
                    let mapping = this_computer_object
                        .clone()
                        .get_mapping(mapping_key.clone())?;
                    let subroutine = context
                        .clone()
                        .get_computer(mapping.clone())
                        .ok_or_else(|| format!("cannot find computer with name '{}'", mapping))?
                        .clone();
                    let (state, _, tape, steps, sub_computation) = subroutine.clone().simulate(
                        acc.clone(),
                        max_steps - steps,
                        context.clone(),
                        0,
                    )?;
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
                        return Ok(("reject".to_string(), 0, vec![out], steps, computation));
                    }
                }
                "1101" => {
                    // println!("MOV: acc: {} -- mov: {} -- ir: {} -- ar: {}", acc.clone(), mov.clone(), ir.clone(), ar.clone());
                    // MOV: copy the value of acc to the mov register
                    mov = acc.clone();
                }
                "1110" => {
                    // println!("LD: acc: {} -- mov: {} -- ir: {} -- ar: {}", acc.clone(), mov.clone(), ir.clone(), ar.clone());
                    // LD: load the memory at address in MOV
                    if !memory.contains_key(&mov) {
                        memory.insert(mov.clone(), "0".to_string());
                    }
                    acc = memory
                        .get(&mov)
                        .ok_or(format!("key not found: {}", mov))?
                        .clone();
                }
                "1111" => {
                    // println!("STD: acc: {} -- mov: {} -- ir: {} -- ar: {}", acc.clone(), mov.clone(), ir.clone(), ar.clone());
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

    pub fn to_encoding(&self) -> Result<computer::EncodingResult, String> {
        let mut encoding = "#".to_string();
        for (counter, instr) in self.instructions.clone().into_iter().enumerate() {
            if instr.opcode == "1011" || instr.opcode == "0011" {
                // Write and Halt does not have operands
                encoding =
                    encoding + &utils::int2bin(counter as i32, 0) + "," + &instr.opcode + "#";
            } else {
                encoding = encoding
                    + &utils::int2bin(counter as i32, 0)
                    + ","
                    + &instr.opcode
                    + &(utils::int2bin(utils::bin2int(instr.operand)?, 0))
                    + "#";
            }
        }
        Ok((
            encoding,
            std::collections::HashMap::new(),
            std::collections::HashMap::new(),
        ))
    }
}
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new_ram_machine() {
        let ram = RamMachine {
            instructions: Vec::new(),
            labels_map: std::collections::HashMap::new(),
            translation_map: std::collections::HashMap::new()
        };
        assert!(ram.instructions.is_empty());
        assert!(ram.labels_map.is_empty());
    }

    #[test]
    fn test_is_instruction() {
        assert!(RamMachine::is_instruction("R"));
        assert!(RamMachine::is_instruction("MIR"));
        assert!(RamMachine::is_instruction("JUMP"));
        assert!(!RamMachine::is_instruction("INVALID"));

        // Additional instruction tests
        assert!(RamMachine::is_instruction("MIL"));
        assert!(RamMachine::is_instruction("W"));
        assert!(RamMachine::is_instruction("CALL"));
        assert!(!RamMachine::is_instruction("TEST"));
        assert!(!RamMachine::is_instruction(""));
    }

    #[test]
    fn test_ram_instruction_lookup() {
        assert_eq!(RamMachine::ram_instruction_lookup("R".to_string()), "0000");
        assert_eq!(
            RamMachine::ram_instruction_lookup("MIR".to_string()),
            "0001"
        );
        assert_eq!(RamMachine::ram_instruction_lookup("H".to_string()), "1011");
        assert_eq!(
            RamMachine::ram_instruction_lookup("INVALID".to_string()),
            "0000"
        );

        // Additional opcode tests
        assert_eq!(
            RamMachine::ram_instruction_lookup("MIL".to_string()),
            "0010"
        );
        assert_eq!(RamMachine::ram_instruction_lookup("W".to_string()), "0011");
        assert_eq!(RamMachine::ram_instruction_lookup("L".to_string()), "0100");
        assert_eq!(
            RamMachine::ram_instruction_lookup("CALL".to_string()),
            "1100"
        );
        assert_eq!(
            RamMachine::ram_instruction_lookup("MOV".to_string()),
            "1101"
        );
    }

    #[test]
    fn test_instruction_struct() {
        let instr = Instruction {
            opcode: "R".to_string(),
            operand: "0001".to_string(),
            label: "".to_string(),
        };
        assert_eq!(instr.opcode, "R");
        assert_eq!(instr.operand, "0001");
        assert!(instr.label.is_empty());
    }

    #[test]
    fn test_ram_machine_clone() {
        let ram1 = RamMachine {
            instructions: Vec::new(),
            labels_map: std::collections::HashMap::new(),
            translation_map: std::collections::HashMap::new()
        };
        let ram2 = ram1.clone();
        assert!(ram2.instructions.is_empty());
        assert!(ram2.labels_map.is_empty());
    }

    #[test]
    fn test_basic_simulation() {
        let ram = RamMachine {
            instructions: vec![
                Instruction {
                    opcode: "0111".to_string(),  // INIT
                    operand: "1010".to_string(), // Initialize ACC with 1010
                    label: "".to_string(),
                },
                Instruction {
                    opcode: "0011".to_string(), // W
                    operand: "".to_string(),    // Write ACC to output
                    label: "".to_string(),
                },
                Instruction {
                    opcode: "1011".to_string(), // H
                    operand: "".to_string(),    // Halt
                    label: "".to_string(),
                },
            ],
            labels_map: std::collections::HashMap::new(),
            translation_map: std::collections::HashMap::new()
        };

        let computer = computer::Computer {
            element: computer::ComputingElem::Ram(ram.clone()),
            mapping: std::collections::HashMap::new(),
        };

        let context = computer::Server {
            map_computers: std::collections::HashMap::new(),
            computation_order: Vec::new(),
        };

        let result = ram.simulate("".to_string(), 100, computer, context);
        assert!(result.is_ok());
        let (state, _, output, steps, _) = result.unwrap();
        assert_eq!(state, "halt");
        assert_eq!(output[0], "1010");
        assert_eq!(steps, 3);
    }

    #[test]
    fn test_read_instruction() {
        let ram = RamMachine {
            instructions: vec![
                Instruction {
                    opcode: "0000".to_string(),  // R
                    operand: "0100".to_string(), // Read 4 bits
                    label: "".to_string(),
                },
                Instruction {
                    opcode: "0011".to_string(), // W
                    operand: "".to_string(),
                    label: "".to_string(),
                },
                Instruction {
                    opcode: "1011".to_string(), // H
                    operand: "".to_string(),
                    label: "".to_string(),
                },
            ],
            labels_map: std::collections::HashMap::new(),
            translation_map: std::collections::HashMap::new()
        };

        let computer = computer::Computer {
            element: computer::ComputingElem::Ram(ram.clone()),
            mapping: std::collections::HashMap::new(),
        };

        let context = computer::Server {
            map_computers: std::collections::HashMap::new(),
            computation_order: Vec::new(),
        };

        let result = ram.simulate("1111".to_string(), 100, computer, context);
        assert!(result.is_ok());
        let (_, _, output, _, _) = result.unwrap();
        assert_eq!(output[0], "1111");
    }

    #[test]
    fn test_arithmetic_instructions() {
        let ram = RamMachine {
            instructions: vec![
                Instruction {
                    opcode: "0111".to_string(),  // INIT
                    operand: "0101".to_string(), // Initialize ACC with 5
                    label: "".to_string(),
                },
                Instruction {
                    opcode: "1000".to_string(),  // ST
                    operand: "0000".to_string(), // Store in memory location 0
                    label: "".to_string(),
                },
                Instruction {
                    opcode: "0101".to_string(),  // A
                    operand: "0000".to_string(), // Add memory location 0
                    label: "".to_string(),
                },
                Instruction {
                    opcode: "0011".to_string(), // W
                    operand: "".to_string(),
                    label: "".to_string(),
                },
                Instruction {
                    opcode: "1011".to_string(), // H
                    operand: "".to_string(),
                    label: "".to_string(),
                },
            ],
            labels_map: std::collections::HashMap::new(),
            translation_map: std::collections::HashMap::new()
        };

        let computer = computer::Computer {
            element: computer::ComputingElem::Ram(ram.clone()),
            mapping: std::collections::HashMap::new(),
        };

        let context = computer::Server {
            map_computers: std::collections::HashMap::new(),
            computation_order: Vec::new(),
        };

        let result = ram.simulate("".to_string(), 100, computer, context);
        assert!(result.is_ok());
        let (_, _, output, _, _) = result.unwrap();
        assert_eq!(output[0], "1010"); // 5 + 5 = 10 in binary
    }
    #[test]
    fn test_input_head_movement() {
        let ram = RamMachine {
            instructions: vec![
                Instruction {
                    opcode: "0001".to_string(),  // MIR
                    operand: "0010".to_string(), // Move right 2
                    label: "".to_string(),
                },
                Instruction {
                    opcode: "0000".to_string(),  // R
                    operand: "0011".to_string(), // Read 3 bits
                    label: "".to_string(),
                },
                Instruction {
                    opcode: "0011".to_string(), // W
                    operand: "".to_string(),
                    label: "".to_string(),
                },
                Instruction {
                    opcode: "0010".to_string(),  // MIL
                    operand: "0011".to_string(), // Move left 3
                    label: "".to_string(),
                },
                Instruction {
                    opcode: "0000".to_string(),  // R
                    operand: "0010".to_string(), // Read 2 bits
                    label: "".to_string(),
                },
                Instruction {
                    opcode: "0011".to_string(), // W
                    operand: "".to_string(),
                    label: "".to_string(),
                },
                Instruction {
                    opcode: "1011".to_string(), // H
                    operand: "".to_string(),
                    label: "".to_string(),
                },
            ],
            labels_map: std::collections::HashMap::new(),
            translation_map: std::collections::HashMap::new()
        };

        let computer = computer::Computer {
            element: computer::ComputingElem::Ram(ram.clone()),
            mapping: std::collections::HashMap::new(),
        };

        let context = computer::Server {
            map_computers: std::collections::HashMap::new(),
            computation_order: Vec::new(),
        };

        let result = ram.simulate("11100111".to_string(), 100, computer, context);
        assert!(result.is_ok());
        let (_, _, output, _, _) = result.unwrap();
        assert_eq!(output[0], "10001");
    }

    #[test]
    fn test_memory_operations() {
        let ram = RamMachine {
            instructions: vec![
                Instruction {
                    opcode: "0111".to_string(),  // INIT
                    operand: "1100".to_string(), // Initialize ACC with 12
                    label: "".to_string(),
                },
                Instruction {
                    opcode: "1000".to_string(),  // ST
                    operand: "0001".to_string(), // Store in mem[1]
                    label: "".to_string(),
                },
                Instruction {
                    opcode: "0111".to_string(),  // INIT
                    operand: "0011".to_string(), // Initialize ACC with 3
                    label: "".to_string(),
                },
                Instruction {
                    opcode: "1101".to_string(),  // MOV
                    operand: "0000".to_string(), // Copy ACC to MOV
                    label: "".to_string(),
                },
                Instruction {
                    opcode: "0100".to_string(),  // L
                    operand: "0001".to_string(), // Load from mem[1]
                    label: "".to_string(),
                },
                Instruction {
                    opcode: "1111".to_string(),  // STD
                    operand: "0000".to_string(), // Store ACC at addr in MOV
                    label: "".to_string(),
                },
                Instruction {
                    opcode: "1110".to_string(),  // LD
                    operand: "0000".to_string(), // Load from addr in MOV
                    label: "".to_string(),
                },
                Instruction {
                    opcode: "0011".to_string(), // W
                    operand: "".to_string(),
                    label: "".to_string(),
                },
                Instruction {
                    opcode: "1011".to_string(), // H
                    operand: "".to_string(),
                    label: "".to_string(),
                },
            ],
            labels_map: std::collections::HashMap::new(),
            translation_map: std::collections::HashMap::new()
        };
        let computer = computer::Computer {
            element: computer::ComputingElem::Ram(ram.clone()),
            mapping: std::collections::HashMap::new(),
        };

        let context = computer::Server {
            map_computers: std::collections::HashMap::new(),
            computation_order: Vec::new(),
        };

        let result = ram.simulate("".to_string(), 100, computer, context);
        assert!(result.is_ok());
        let (_, _, output, _, _) = result.unwrap();
        assert_eq!(output[0], "1100");
    }

    #[test]
    fn test_jumps() {
        let ram = RamMachine {
            instructions: vec![
                Instruction {
                    opcode: "0111".to_string(), // INIT
                    operand: "0".to_string(),   // ACC = 0
                    label: "".to_string(),
                },
                Instruction {
                    opcode: "1010".to_string(), // CJUMP
                    operand: "100".to_string(), // Jump to 4 if ACC == 0
                    label: "".to_string(),
                },
                Instruction {
                    opcode: "0011".to_string(), // W
                    operand: "".to_string(),
                    label: "".to_string(),
                },
                Instruction {
                    opcode: "1001".to_string(), // JUMP
                    operand: "101".to_string(), // Jump to 5
                    label: "".to_string(),
                },
                Instruction {
                    opcode: "0111".to_string(), // INIT
                    operand: "1".to_string(),   // ACC = 1
                    label: "".to_string(),
                },
                Instruction {
                    opcode: "0011".to_string(), // W
                    operand: "".to_string(),
                    label: "".to_string(),
                },
                Instruction {
                    opcode: "1011".to_string(), // H
                    operand: "".to_string(),
                    label: "".to_string(),
                },
            ],
            labels_map: std::collections::HashMap::new(),
            translation_map: std::collections::HashMap::new()
        };
        let computer = computer::Computer {
            element: computer::ComputingElem::Ram(ram.clone()),
            mapping: std::collections::HashMap::new(),
        };

        let context = computer::Server {
            map_computers: std::collections::HashMap::new(),
            computation_order: Vec::new(),
        };

        let result = ram.simulate("".to_string(), 100, computer, context);
        assert!(result.is_ok());
        let (_, _, output, _, _) = result.unwrap();
        assert_eq!(output[0], "1");
    }

    #[test]
    fn test_short_input() {
        let ram = RamMachine {
            instructions: vec![
                Instruction {
                    opcode: "0000".to_string(),  // R
                    operand: "0100".to_string(), // Try to read 4 bits
                    label: "".to_string(),
                },
                Instruction {
                    opcode: "0011".to_string(), // W
                    operand: "".to_string(),
                    label: "".to_string(),
                },
                Instruction {
                    opcode: "1011".to_string(), // H
                    operand: "".to_string(),
                    label: "".to_string(),
                },
            ],
            labels_map: std::collections::HashMap::new(),
            translation_map: std::collections::HashMap::new()
        };

        let computer = computer::Computer {
            element: computer::ComputingElem::Ram(ram.clone()),
            mapping: std::collections::HashMap::new(),
        };

        let context = computer::Server {
            map_computers: std::collections::HashMap::new(),
            computation_order: Vec::new(),
        };

        let result = ram.simulate("11".to_string(), 100, computer, context);
        assert!(result.is_ok());
        let (_, _, output, _, _) = result.unwrap();
        assert_eq!(output[0], "0011");
    }

    #[test]
    fn test_subtraction() {
        let ram = RamMachine {
            instructions: vec![
                Instruction {
                    opcode: "0111".to_string(),  // INIT
                    operand: "1000".to_string(), // ACC = 8
                    label: "".to_string(),
                },
                Instruction {
                    opcode: "1000".to_string(),  // ST
                    operand: "0001".to_string(), // Store in mem[1]
                    label: "".to_string(),
                },
                Instruction {
                    opcode: "0111".to_string(),  // INIT
                    operand: "0011".to_string(), // ACC = 3
                    label: "".to_string(),
                },
                Instruction {
                    opcode: "0110".to_string(),  // S
                    operand: "0001".to_string(), // Subtract mem[1]
                    label: "".to_string(),
                },
                Instruction {
                    opcode: "0011".to_string(), // W
                    operand: "".to_string(),
                    label: "".to_string(),
                },
                Instruction {
                    opcode: "1011".to_string(), // H
                    operand: "".to_string(),
                    label: "".to_string(),
                },
            ],
            labels_map: std::collections::HashMap::new(),
            translation_map: std::collections::HashMap::new()
        };

        let computer = computer::Computer {
            element: computer::ComputingElem::Ram(ram.clone()),
            mapping: std::collections::HashMap::new(),
        };

        let context = computer::Server {
            map_computers: std::collections::HashMap::new(),
            computation_order: Vec::new(),
        };

        let result = ram.simulate("".to_string(), 100, computer, context);
        assert!(result.is_ok());
        let (_, _, output, _, _) = result.unwrap();
        assert_eq!(output[0], "11111111111111111111111111111011");
    }
    #[test]
    fn test_to_encoding() {
        let ram = RamMachine {
            instructions: vec![
                Instruction {
                    opcode: "0111".to_string(), // INIT
                    operand: "101".to_string(), // 5
                    label: "".to_string(),
                },
                Instruction {
                    opcode: "0011".to_string(), // W
                    operand: "".to_string(),
                    label: "".to_string(),
                },
                Instruction {
                    opcode: "1011".to_string(), // H
                    operand: "".to_string(),
                    label: "".to_string(),
                },
            ],
            labels_map: std::collections::HashMap::new(),
            translation_map: std::collections::HashMap::new()
        };

        let result = ram.to_encoding();
        assert!(result.is_ok());
        let (encoding, _, _) = result.unwrap();
        assert_eq!(encoding, "#0,0111101#1,0011#10,1011#");
    }

    #[test]
    fn test_label_mapping() {
        let mut labels = std::collections::HashMap::new();
        labels.insert("LOOP".to_string(), "0101".to_string());

        let ram = RamMachine {
            instructions: vec![Instruction {
                opcode: "1001".to_string(), // JUMP
                operand: "".to_string(),
                label: "LOOP".to_string(),
            }],
            labels_map: labels,
            translation_map: std::collections::HashMap::new()
        };

        let computer = computer::Computer {
            element: computer::ComputingElem::Ram(ram.clone()),
            mapping: std::collections::HashMap::new(),
        };

        let context = computer::Server {
            map_computers: std::collections::HashMap::new(),
            computation_order: Vec::new(),
        };

        let result = ram.simulate("".to_string(), 1, computer, context);
        assert!(result.is_ok());
    }

    #[test]
    fn test_max_steps_limit() {
        let ram = RamMachine {
            instructions: vec![Instruction {
                opcode: "1001".to_string(), // JUMP
                operand: "0".to_string(),   // Infinite loop
                label: "".to_string(),
            }],
            labels_map: std::collections::HashMap::new(),
            translation_map: std::collections::HashMap::new()
        };

        let computer = computer::Computer {
            element: computer::ComputingElem::Ram(ram.clone()),
            mapping: std::collections::HashMap::new(),
        };

        let context = computer::Server {
            map_computers: std::collections::HashMap::new(),
            computation_order: Vec::new(),
        };

        let result = ram.simulate("".to_string(), 10, computer, context);
        assert!(result.is_ok());
        let (_, _, _, steps, _) = result.unwrap();
        assert_eq!(steps, 10);
    }

    #[test]
    fn test_memory_boundary_conditions() {
        let ram = RamMachine {
            instructions: vec![
                Instruction {
                    opcode: "0100".to_string(),  // L
                    operand: "1111".to_string(), // Try to load from uninitialized memory
                    label: "".to_string(),
                },
                Instruction {
                    opcode: "0011".to_string(), // W
                    operand: "".to_string(),
                    label: "".to_string(),
                },
                Instruction {
                    opcode: "1011".to_string(), // H
                    operand: "".to_string(),
                    label: "".to_string(),
                },
            ],
            labels_map: std::collections::HashMap::new(),
            translation_map: std::collections::HashMap::new()
        };

        let computer = computer::Computer {
            element: computer::ComputingElem::Ram(ram.clone()),
            mapping: std::collections::HashMap::new(),
        };

        let context = computer::Server {
            map_computers: std::collections::HashMap::new(),
            computation_order: Vec::new(),
        };

        let result = ram.simulate("".to_string(), 100, computer, context);
        assert!(result.is_ok());
        let (_, _, output, _, _) = result.unwrap();
        assert_eq!(output[0], "0"); // Uninitialized memory should return 0
    }
}
