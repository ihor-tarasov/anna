#[derive(Clone)]
pub enum Opcode {
    Const(usize),
    Addict,
    Subtract,
    Multiply,
    Divide,
    End,
}
