#[derive(Clone)]
pub enum Opcode {
    Const(usize),
    Addict,
    Multiply,
    End,
}
