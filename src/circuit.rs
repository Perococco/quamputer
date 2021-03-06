use crate::gate::ExecutionContext;
use crate::operation::{QuantumOperation, CircuitElement};
use crate::state::QuantumState;
use serde::{Serialize,Deserialize};

#[derive(Clone, Serialize, Deserialize)]
pub struct Circuit {
    pub nb_qbits:u8,
    pub elements:Vec<CircuitElement>,
}

pub struct Executable<'a>(pub &'a CircuitElement);

impl Circuit {
    pub fn execute(&self, initial_state:&QuantumState) -> ExecutionContext {
        Execution(&self).execute(initial_state)
    }

    pub fn to_string(&self) -> serde_json::error::Result<String> {
        serde_json::to_string(self)
    }

    pub fn from_string(serialized_operation:&str) -> serde_json::error::Result<Self> {
        serde_json::from_str::<Circuit>(serialized_operation)
    }
}




struct Execution<'a>(pub &'a Circuit);

impl<'a> Execution<'a> {

    fn execute(&self, initial_state:&QuantumState) -> ExecutionContext {
        let mut context = ExecutionContext::initialize(&initial_state);
        self.0.apply(&mut context);
        return context;
    }

}