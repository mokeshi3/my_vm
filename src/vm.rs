#![allow(dead_code)]
use crate::instruction::Opcode;

pub struct VM {
    registers: [i32; 32],
    pc: usize,
    program: Vec<u8>,
    remainder: u32,
}

impl VM {
    pub fn new() -> Self {
        Self {
            registers: [0; 32],
            program: vec![],
            pc: 0,
            remainder: 0,
        }
    }

    pub fn run(&mut self) {
        let mut is_done = false;
        while !is_done {
            is_done = self.execute_instruction();
        }
    }

    pub fn run_once(&mut self) {
        self.execute_instruction();
    }

    fn execute_instruction(&mut self) -> bool {
        if self.pc >= self.program.len() {
            return true;
        }

        match self.decode_opecode() {
            Opcode::HLT => {
                println!("HLT encountered");
                return true;
            },
            Opcode::LOAD => {
                let register = self.next_8_bits() as usize;
                let number = self.next_16_bits() as u16;
                self.registers[register] = number as i32;
            },
            Opcode::ADD => {
                let register1 = self.registers[self.next_8_bits() as usize];
                let register2 = self.registers[self.next_8_bits() as usize];
                self.registers[self.next_8_bits() as usize] = register1 + register2;
            },
            Opcode::SUB => {
                let register1 = self.registers[self.next_8_bits() as usize];
                let register2 = self.registers[self.next_8_bits() as usize];
                self.registers[self.next_8_bits() as usize] = register1 - register2;
            },
            Opcode::MUL => {
                 let register1 = self.registers[self.next_8_bits() as usize];
                let register2 = self.registers[self.next_8_bits() as usize];
                self.registers[self.next_8_bits() as usize] = register1 * register2;
           },
            Opcode::DIV => {
                let register1 = self.registers[self.next_8_bits() as usize];
                let register2 = self.registers[self.next_8_bits() as usize];
                self.registers[self.next_8_bits() as usize] = register1 / register2;
                self.remainder = (register1 % register2) as u32;
            },
            Opcode::JMP => {
                let target = self.registers[self.next_8_bits() as usize];
                self.pc = target as usize;
            },
            _ => {
                println!("Unrecoginzed opcode found! Terminating!");
                return true;
            }
        }
        false
    }

    fn decode_opecode(&mut self) -> Opcode {
        let result = Opcode::from(self.program[self.pc]);
        self.pc += 1;
        result
    }

    fn next_8_bits(&mut self) -> u8 {
        let result = self.program[self.pc];
        self.pc += 1;
        result
    }

    fn next_16_bits(&mut self) -> u16 {
        let result = ((self.program[self.pc] as u16) << 8) | self.program[self.pc + 1] as u16;
        self.pc += 2;
        result
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_create_vm() {
        let test_vm = VM::new();

        for i in 0..32 {
            assert_eq!(test_vm.registers[i], 0);
        }
    }

    #[test]
    fn test_opcode_hlt() {
        let mut test_vm = VM::new();
        let test_bytes = vec![5, 5, 5, 5];
        test_vm.program = test_bytes;
        test_vm.run();
        assert_eq!(test_vm.pc, 1);
    }

    #[test]
    fn test_opcode_igl() {
        let mut test_vm = VM::new();
        let test_bytes = vec![254, 0, 0, 0];
        test_vm.program = test_bytes;
        test_vm.run();
        assert_eq!(test_vm.pc, 1);
    }

    #[test]
    fn test_load_opcode() {
        let mut test_vm = VM::new();
        test_vm.program = vec![0, 0, 1, 244];
        test_vm.run();
        assert_eq!(test_vm.registers[0], 500);
    }

    #[test]
    fn test_add_opcode() {
        let mut test_vm = VM::new();
        let v1: u8 = 10;
        let v2: u8 = 5;
        test_vm.program = vec![0, 0, 0, v1, 0, 1, 0, 5, 1, 0, 1, 2];
        test_vm.run();
        assert_eq!(test_vm.registers[2], (v1+v2) as i32);
    }

    #[test]
    fn test_sub_opcode() {
        let mut test_vm = VM::new();
        let v1: u8 = 10;
        let v2: u8 = 5;
        test_vm.program = vec![0, 0, 0, v1, 0, 1, 0, 5, 2, 0, 1, 2];
        test_vm.run();
        assert_eq!(test_vm.registers[2], (v1-v2) as i32);
    }

    #[test]
    fn test_mul_opcode() {
        let mut test_vm = VM::new();
        let v1: u8 = 10;
        let v2: u8 = 5;
        test_vm.program = vec![0, 0, 0, v1, 0, 1, 0, 5, 3, 0, 1, 2];
        test_vm.run();
        assert_eq!(test_vm.registers[2], (v1*v2) as i32);
    }

    #[test]
    fn test_div_opcode() {
        let mut test_vm = VM::new();
        let v1: u8 = 10;
        let v2: u8 = 5;
        test_vm.program = vec![0, 0, 0, v1, 0, 1, 0, 5, 4, 0, 1, 2];
        test_vm.run();
        assert_eq!(test_vm.registers[2], (v1/v2) as u32 as i32);
        assert_eq!(test_vm.remainder, (v1%v2) as u32);
    }

    #[test]
    fn test_jmp_opcode() {
        let mut test_vm = VM::new();
        test_vm.registers[0] = 3;
        test_vm.registers[1] = 2;
        test_vm.registers[2] = 3;
        test_vm.program = vec![6, 0, 5, 1, 1, 2, 3];
        test_vm.run();
        assert_eq!(test_vm.registers[3], 5);
    }
}
