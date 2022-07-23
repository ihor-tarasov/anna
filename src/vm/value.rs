use std::{hash::Hash, fmt::Debug};

#[derive(PartialEq, Clone)]
pub struct Real {
    pub data: f64,
}

impl Real {
    pub fn new(data: f64) -> Self {
        Self { data }
    }
}

impl Eq for Real { }

impl Hash for Real {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        state.write_u64(self.data.to_bits());
    }
}

#[derive(PartialEq, Eq, Hash, Clone)]
pub enum Value {
    Int(i64),
    Real(Real),
}

impl Debug for Value {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Value::Int(value) => write!(f, "{}", value),
            Value::Real(value) => write!(f, "{}", value.data),
        }
    }
}
