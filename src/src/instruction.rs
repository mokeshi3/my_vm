pub enum Opcode {
    HLT,
    IGL,
}

pub struct Instruction {
    opcode: Opcode,
}

impl Instruction {
    pub fn new(opcode: Opcode) -> Self {
        Self {
            opcode: Opcode,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_create_hlt() {
        let opcode = Opcode::HLT;
        assert_eq!(opcode, Opcode::HLT);
    }

    #[test]
    fn test_create_hlt() {
        let instruction = Instruction::new(Opcode::HLT);
        assert_eq!(instruction, Opcode::HLT);
    }
}