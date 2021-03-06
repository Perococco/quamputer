use crate::builder::{QuantumCircuitBuilder};
use crate::state::QuantumState;
use crate::gate::StandardGate::{Hadamard, CNot};

pub struct QuantumComputer {
    nb_qbits: u8,
}

impl QuantumComputer {
    /// Create a new computer
    pub fn new(nb_qbits: u8) -> Self {
        Self { nb_qbits }
    }


    /// Create a circuit builder initial with operation
    /// to set all the qbits in Bell state
    pub fn bell_state(&self) -> QuantumCircuitBuilder {
        let mut builder = self.new_circuit_builder();
        builder.add_operation(Hadamard(0));
        for i in 1..self.nb_qbits {
            builder.add_operation(CNot(i, [i - 1]));
        }
        builder
    }


    /// Create a new circuit builder to create
    /// circuit this computer can run
    pub fn new_circuit_builder(&self) -> QuantumCircuitBuilder {
        QuantumCircuitBuilder::new(self.nb_qbits)
    }

    /// Create a initial quantum state with all amplitudes to zero
    pub fn zero_state(&self) -> QuantumState {
        QuantumState::zero(self.nb_qbits)
    }

    /// Create a initial quantum state with the same amplitudes for all qbits
    pub fn same_amplitude(&self, qbit_idx: &[usize]) -> QuantumState {
        QuantumState::same_amplitude(self.nb_qbits, qbit_idx)
    }
    pub fn nb_qbits(&self) -> u8 {
        self.nb_qbits
    }
}
