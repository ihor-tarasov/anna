use super::{State, VMResult, Opcode};
use super::oper::arithmetic::*;
use super::oper::bitwise::*;
use super::oper::comparison::*;

pub fn run(mut state: State) -> VMResult {
    loop {
        match state.next_opcode()? {
            Opcode::Const(index) => state.push_constant(index)?,
            Opcode::Addict => state.binary::<ArithmeticOperator<AddictArithmetic>>()?,
            Opcode::Subtract => state.binary::<ArithmeticOperator<SubtractArithmetic>>()?,
            Opcode::Multiply => state.binary::<ArithmeticOperator<MultiplyArithmetic>>()?,
            Opcode::Divide => state.binary::<ArithmeticOperator<DivideArithmetic>>()?,
            Opcode::And => state.binary::<BitwiseOperator<AndBitwise>>()?,
            Opcode::Or => state.binary::<BitwiseOperator<OrBitwise>>()?,
            Opcode::Xor => state.binary::<BitwiseOperator<XorBitwise>>()?,
            Opcode::Shl => state.binary::<BitwiseOperator<ShlBitwise>>()?,
            Opcode::Shr => state.binary::<BitwiseOperator<ShrBitwise>>()?,
            Opcode::Mod => state.binary::<BitwiseOperator<ModBitwise>>()?,
            Opcode::Equals => state.binary::<ComparisonOperator<EqualComparison>>()?,
            Opcode::NotEquals => state.binary::<ComparisonOperator<NotEqualComparison>>()?,
            Opcode::Less => state.binary::<ComparisonOperator<LessComparison>>()?,
            Opcode::Greater => state.binary::<ComparisonOperator<GreaterComparison>>()?,
            Opcode::LessEqual => state.binary::<ComparisonOperator<LessEqualComparison>>()?,
            Opcode::GreaterEqual => state.binary::<ComparisonOperator<GreaterEqualComparison>>()?,
            Opcode::End => return Ok(state.pop().unwrap()),
        }
    }
}
