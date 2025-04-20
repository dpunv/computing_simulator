//! RAM (Random Access Machine) Implementation Module
//!
//! This module provides a simulation of a Random Access Machine, which is a computational model
//! consisting of an accumulator (ACC), program counter (PC), input/output mechanisms, and random
//! access memory.
//!
//! # Architecture
//!
//! The RAM machine implements the following components:
//! * Program Counter (PC): Points to the next instruction to execute
//! * Accumulator (ACC): Main register for arithmetic and logical operations
//! * Memory: Random access storage indexed by binary strings
//! * Input/Output: Handles binary string input/output operations
//! * Move Register (MOV): Additional register for memory operations
//!
//! # Instruction Set
//!
//! The machine supports 16 instructions (4-bit opcodes):
//! * `R (0000)`: Read from input
//! * `MIR (0001)`: Move input head right
//! * `MIL (0010)`: Move input head left
//! * `W (0011)`: Write ACC to output
//! * `L (0100)`: Load from memory to ACC
//! * `A (0101)`: Add memory content to ACC
//! * `S (0110)`: Subtract memory content from ACC
//! * `INIT (0111)`: Initialize ACC with constant
//! * `ST (1000)`: Store ACC to memory
//! * `JUMP (1001)`: Unconditional jump
//! * `CJUMP (1010)`: Conditional jump if ACC is zero
//! * `H (1011)`: Halt execution
//! * `CALL (1100)`: Call subroutine
//! * `MOV (1101)`: Copy ACC to MOV register
//! * `LD (1110)`: Load from memory address in MOV
//! * `STD (1111)`: Store to memory address in MOV
//!
//! # Structures
//!
//! ## RamMachine
//! Main structure representing the RAM machine, containing:
//! * instructions: Vector of machine instructions
//! * labels_map: Mapping of symbolic labels to numeric values
//! * translation_map: Mapping for optional output symbol translations
//!
//! ## Instruction
//! Structure representing a single RAM machine instruction:
//! * opcode: The operation code (4-bit binary string)
//! * operand: The instruction's operand
//! * label: Optional symbolic label
//!
//! # Notes
//! - All memory addresses and values are represented as binary strings
//! - The machine operates on discrete steps with a maximum step limit
//! - Uninitialized memory locations return "0" by default
//! - The simulation can be integrated with other computational models through the CALL instruction
//!
//! ## Author
//!
//! - dp
//!
//! # License
//!
//! This project is licensed under the MIT License. See the LICENSE file for details.

use crate::computer;
use crate::utils;

/// A Random Access Machine (RAM) implementation representing a computational model.
///
/// The RAM machine consists of a set of instructions, label mappings, and translation mappings
/// that allow for basic computational operations and control flow.
///
/// # Components
///
/// * Program Counter (PC): Points to the next instruction to execute
/// * Accumulator (ACC): Main register for arithmetic and logical operations
/// * Memory: Random access storage indexed by binary strings
/// * Input/Output: Handles binary string input/output operations
/// * Move Register (MOV): Additional register for memory operations
///
/// # Fields
///
/// * `instructions` - A vector containing all machine instructions to be executed
/// * `labels_map` - A hashmap mapping symbolic labels to their corresponding values
/// * `translation_map` - A hashmap storing mappings for optional output symbol translations
///
/// # Notes
///
/// - All memory values and addresses are represented as binary strings
/// - Uninitialized memory locations return "0" by default
/// - The machine operates on discrete steps with a configurable maximum step limit
/// - Supports integration with other computational models through the CALL instruction
#[derive(Clone)]
pub struct RamMachine {
    pub instructions: Vec<Instruction>,
    pub labels_map: std::collections::HashMap<String, String>,
    pub translation_map: std::collections::HashMap<String, String>,
}

/// A structure representing a single instruction in the RAM machine.
///
/// # Fields
///
/// * `opcode` - String representing a 4-bit binary representing the operation code
/// * `operand` - String representing the instruction's operand (if any) in binary format
/// * `label` - String representing an optional symbolic label to be substituted to an operand value at runtime
///
/// # Notes
///
/// - The opcode should be one of the 16 valid RAM machine instructions
/// - Labels are used for operand substitution and fixed memory addressing
/// - Some instructions (like HALT and WRITE) don't require operands
#[derive(Clone)]
pub struct Instruction {
    pub opcode: String,
    pub operand: String,
    pub label: String,
}

impl RamMachine {
    /// Checks if a given instruction string is a valid RAM machine instruction.
    ///
    /// # Arguments
    ///
    /// * `instruction` - A string slice containing the instruction to validate
    ///
    /// # Returns
    ///
    /// Returns `true` if the instruction is valid, `false` otherwise.
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

    /// Converts a RAM machine instruction string to its corresponding 4-bit opcode.
    ///
    /// # Arguments
    ///
    /// * `instruction` - A String containing the instruction to convert
    ///
    /// # Returns
    ///
    /// Returns a String containing the 4-bit binary opcode.
    ///
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

    /// Simulates the execution of the RAM machine.
    ///
    /// # Arguments
    ///
    /// * `self` - The RAM machine instance
    /// * `input` - Input string to process
    /// * `max_steps` - Maximum number of simulation steps
    /// * `this_computer_object` - Reference to the current computer object
    /// * `context` - Server context for handling subroutine calls
    ///
    /// # Returns
    ///
    /// Returns a Result containing a tuple with:
    /// * Final state ("halt" or "reject")
    /// * Final position (always 0 for RAM machines)
    /// * Output vector
    /// * Number of steps executed
    /// * Computation history vector
    ///
    /// # Errors
    ///
    /// Returns an error string if:
    /// * Invalid memory access occurs
    /// * Binary conversion fails
    /// * Subroutine calls fail
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
                    // MIR: move input head [operands] bits to the right
                    input_head += utils::bin2int(ar)? as usize;
                }
                "0010" => {
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
                    // W: Write ACC to output
                    out = out + &acc.clone();
                }
                "0100" => {
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
                    // MOV: copy the value of acc to the mov register
                    mov = acc.clone();
                }
                "1110" => {
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

    /// Converts the RAM machine to its encoding representation.
    ///
    /// # Returns
    ///
    /// Returns a Result containing:
    /// * A String representing the encoded RAM machine
    /// * Two empty HashMaps for labels and translations
    ///
    /// The encoding format is: `#address,opcode[operand]#`
    ///
    /// # Errors
    ///
    /// Returns an error string if binary conversion fails.
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
            translation_map: std::collections::HashMap::new(),
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
            translation_map: std::collections::HashMap::new(),
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
            translation_map: std::collections::HashMap::new(),
        };

        let computer = computer::Computer {
            element: computer::ComputingElem::Ram(Box::new(ram.clone())),
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
            translation_map: std::collections::HashMap::new(),
        };

        let computer = computer::Computer {
            element: computer::ComputingElem::Ram(Box::new(ram.clone())),
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
            translation_map: std::collections::HashMap::new(),
        };

        let computer = computer::Computer {
            element: computer::ComputingElem::Ram(Box::new(ram.clone())),
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
            translation_map: std::collections::HashMap::new(),
        };

        let computer = computer::Computer {
            element: computer::ComputingElem::Ram(Box::new(ram.clone())),
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
            translation_map: std::collections::HashMap::new(),
        };
        let computer = computer::Computer {
            element: computer::ComputingElem::Ram(Box::new(ram.clone())),
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
            translation_map: std::collections::HashMap::new(),
        };
        let computer = computer::Computer {
            element: computer::ComputingElem::Ram(Box::new(ram.clone())),
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
            translation_map: std::collections::HashMap::new(),
        };

        let computer = computer::Computer {
            element: computer::ComputingElem::Ram(Box::new(ram.clone())),
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
            translation_map: std::collections::HashMap::new(),
        };

        let computer = computer::Computer {
            element: computer::ComputingElem::Ram(Box::new(ram.clone())),
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
            translation_map: std::collections::HashMap::new(),
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
            translation_map: std::collections::HashMap::new(),
        };

        let computer = computer::Computer {
            element: computer::ComputingElem::Ram(Box::new(ram.clone())),
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
            translation_map: std::collections::HashMap::new(),
        };

        let computer = computer::Computer {
            element: computer::ComputingElem::Ram(Box::new(ram.clone())),
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
            translation_map: std::collections::HashMap::new(),
        };

        let computer = computer::Computer {
            element: computer::ComputingElem::Ram(Box::new(ram.clone())),
            mapping: std::collections::HashMap::new(),
        };

        let context = computer::Server {
            map_computers: std::collections::HashMap::new(),
            computation_order: Vec::new(),
        };

        let result = ram.simulate("".to_string(), 100, computer, context);
        assert!(result.is_ok());
        let (_, _, output, _, _) = result.unwrap();
        assert_eq!(output[0], "0");
    }
}
