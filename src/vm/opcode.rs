#[derive(Clone)]
pub enum Opcode {
    Const(usize),
    Addict,
    Subtract,
    Multiply,
    Divide,
    And,
    Or,
    Xor,
    Shl,
    Shr,
    Mod,
    End,
}
