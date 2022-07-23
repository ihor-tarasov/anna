pub mod oper;

mod opcode;
mod value;
mod function;

pub use value::Value;
pub use value::Real;
pub use opcode::Opcode;
pub use function::Function;
pub use function::FunctionBuilder;

pub struct VMError {
    pub message: String,
    pub line: usize,
}

pub type VMResult = Result<Value, VMError>;

mod state;
mod run;

pub use state::State;
pub use run::run;
