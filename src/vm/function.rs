use super::{Opcode, Value};

pub struct Function {
    pub opcodes: Box<[Opcode]>,
    pub constants: Box<[Value]>,
    pub lines: Box<[usize]>,
}

pub struct FunctionBuilder {
    opcodes: Vec<Opcode>,
    constants: Vec<Value>,
    lines: Vec<usize>,
    constants_map: std::collections::HashMap<Value, usize>,
}

impl FunctionBuilder {
    pub fn new() -> Self {
        Self {
            opcodes: Vec::new(),
            constants: Vec::new(),
            lines: Vec::new(),
            constants_map: std::collections::HashMap::new(),
        }
    }

    pub fn push_opcode(&mut self, opcode: Opcode, line: usize) -> usize {
        self.opcodes.push(opcode);
        self.lines.push(line);
        self.lines.len() - 1
    }

    pub fn push_constant(&mut self, value: Value) -> usize {
        if let Some(index) = self.constants_map.get(&value) {
            *index
        } else {
            self.constants.push(value.clone());
            self.constants_map.insert(value, self.constants.len() - 1);
            self.constants.len() - 1
        }
    }

    pub fn build(self) -> Function {
        Function {
            opcodes: self.opcodes.into_boxed_slice(),
            constants: self.constants.into_boxed_slice(),
            lines: self.lines.into_boxed_slice(),
        }
    }
}
