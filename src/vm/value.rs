use std::hash::Hash;


#[derive(Debug, PartialEq, Clone)]
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

#[derive(Debug, PartialEq, Eq, Hash, Clone)]
pub enum Value {
    Int(i64),
    Real(Real),
}
