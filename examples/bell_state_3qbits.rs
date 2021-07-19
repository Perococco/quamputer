use quamputer::computer::QuantumComputer;
use quamputer::gate::StandardGate::{Hadamard, CNot};
use quamputer::condition::StopCondition::{MaxZeroSampling, Or};
use quamputer::operation::CircuitElement::Gate;


fn main() -> Result<(),String> {
    let computer = QuantumComputer::new(3);

    let circuit = {
        computer.new_circuit_builder()
            .add_loop(computer.bell_state().add_measure("q0", 0), MaxZeroSampling{id:"q0".to_string(), nb:10})
            .build()?
    };

    let initial_state = computer.zero_state();

    let result = circuit.execute(&initial_state);

    println!("{:?}",result.get_count("q0"));
    Ok(())
}

