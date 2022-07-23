use super::{Value, Function, Opcode, VMError, oper::BinaryOperator};

pub struct State {
    opcode_index: usize,
    stack: Vec<Value>,
    function: Function,
}

impl State {
    pub fn new(function: Function) -> Self {
        Self {
            opcode_index: 0,
            stack: Vec::new(),
            function,
        }
    }

    pub fn next_opcode(&mut self) -> Result<Opcode, VMError> {
        self.opcode_index += 1;
        match self.function.opcodes.get(self.opcode_index - 1) {
            Some(opcode) => Ok(opcode.clone()),
            None => Err(VMError {
                message: format!("Unable to get opcode by index {}.", self.opcode_index - 1),
                line: 0,
            }),
        }
    }

    pub fn push_constant(&mut self, index: usize) -> Result<(), VMError> {
        self.stack.push(match self.function.constants.get(index) {
            Some(value) => value.clone(),
            None => return Err(VMError {
                message: format!("Unable to get constant by index {}.", index),
                line: 0,
            }),
        });
        Ok(())
    }

    pub fn binary<T: BinaryOperator>(&mut self) -> Result<(), VMError> {
        let rhs = self.stack.pop().unwrap();
        let lhs = self.stack.pop().unwrap();
        self.stack.push(match T::eval(lhs, rhs) {
            Ok(value) => value,
            Err(message) => return Err(VMError {
                message,
                line: *self.function.lines.get(self.opcode_index).unwrap(),
            }),
        });
        Ok(())
    }

    pub fn pop(&mut self) -> Option<Value> {
        self.stack.pop()
    }
}
