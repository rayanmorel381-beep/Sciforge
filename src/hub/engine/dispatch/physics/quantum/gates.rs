//! Dispatch handler for quantum gate functions.

use crate::hub::domain::common::errors::{HubError, HubResult};
use crate::hub::domain::physics as phys;
use crate::hub::engine::dispatch::params::*;
use crate::hub::engine::experience::runner::RunOutput;
use crate::maths::complex::Complex;

pub(crate) fn dispatch(func: &str, p: &Params) -> HubResult<RunOutput> {
    match func {
        "apply_gate" => {
            let gate: Vec<Vec<Complex>> = get_m(p, "gate")?
                .iter()
                .map(|row| row.chunks(2).map(|c| Complex::new(c[0], c[1])).collect())
                .collect();
            let state = get_cv(p, "state")?;
            let r = phys::quantum::apply_gate(&gate, &state);
            Ok(RunOutput::ComplexVector(
                r.iter().map(|c| (c.re, c.im)).collect(),
            ))
        }
        "apply_single_qubit_gate" => {
            let gate: Vec<Vec<Complex>> = get_m(p, "gate")?
                .iter()
                .map(|row| row.chunks(2).map(|c| Complex::new(c[0], c[1])).collect())
                .collect();
            let mut state = get_cv(p, "state")?;
            phys::quantum::apply_single_qubit_gate(
                &gate,
                &mut state,
                get_u(p, "target")?,
                get_u(p, "n_qubits")?,
            );
            Ok(RunOutput::ComplexVector(
                state.iter().map(|c| (c.re, c.im)).collect(),
            ))
        }
        "bell_phi_minus" => {
            let r = phys::quantum::bell_phi_minus();
            Ok(RunOutput::ComplexVector(
                r.iter().map(|c| (c.re, c.im)).collect(),
            ))
        }
        "bell_phi_plus" => {
            let r = phys::quantum::bell_phi_plus();
            Ok(RunOutput::ComplexVector(
                r.iter().map(|c| (c.re, c.im)).collect(),
            ))
        }
        "bell_psi_minus" => {
            let r = phys::quantum::bell_psi_minus();
            Ok(RunOutput::ComplexVector(
                r.iter().map(|c| (c.re, c.im)).collect(),
            ))
        }
        "bell_psi_plus" => {
            let r = phys::quantum::bell_psi_plus();
            Ok(RunOutput::ComplexVector(
                r.iter().map(|c| (c.re, c.im)).collect(),
            ))
        }
        "bloch_vector" => {
            let cv = get_cv(p, "state")?;
            let r = phys::quantum::bloch_vector(&[cv[0], cv[1]]);
            Ok(RunOutput::Triple(r.0, r.1, r.2))
        }
        "cnot_gate" => {
            let r = phys::quantum::cnot_gate();
            Ok(RunOutput::ComplexMatrix(
                r.iter()
                    .map(|row| row.iter().map(|c| (c.re, c.im)).collect())
                    .collect(),
            ))
        }
        "cz_gate" => {
            let r = phys::quantum::cz_gate();
            Ok(RunOutput::ComplexMatrix(
                r.iter()
                    .map(|row| row.iter().map(|c| (c.re, c.im)).collect())
                    .collect(),
            ))
        }
        "hadamard_gate" => {
            let r = phys::quantum::hadamard_gate();
            Ok(RunOutput::ComplexMatrix(
                r.iter()
                    .map(|row| row.iter().map(|c| (c.re, c.im)).collect())
                    .collect(),
            ))
        }
        "identity_2" => {
            let r = phys::quantum::identity_2();
            Ok(RunOutput::ComplexMatrix(
                r.iter()
                    .map(|row| row.iter().map(|c| (c.re, c.im)).collect())
                    .collect(),
            ))
        }
        "measure_probabilities" => {
            let state = get_cv(p, "state")?;
            Ok(RunOutput::Vector(phys::quantum::measure_probabilities(
                &state,
            )))
        }
        "pauli_x" => {
            let r = phys::quantum::pauli_x();
            Ok(RunOutput::ComplexMatrix(
                r.iter()
                    .map(|row| row.iter().map(|c| (c.re, c.im)).collect())
                    .collect(),
            ))
        }
        "pauli_y" => {
            let r = phys::quantum::pauli_y();
            Ok(RunOutput::ComplexMatrix(
                r.iter()
                    .map(|row| row.iter().map(|c| (c.re, c.im)).collect())
                    .collect(),
            ))
        }
        "pauli_z" => {
            let r = phys::quantum::pauli_z();
            Ok(RunOutput::ComplexMatrix(
                r.iter()
                    .map(|row| row.iter().map(|c| (c.re, c.im)).collect())
                    .collect(),
            ))
        }
        "phase_gate" => {
            let r = phys::quantum::phase_gate(get_f(p, "phi")?);
            Ok(RunOutput::ComplexMatrix(
                r.iter()
                    .map(|row| row.iter().map(|c| (c.re, c.im)).collect())
                    .collect(),
            ))
        }
        "rotation_operator" => {
            let pauli = match get_str(p, "pauli")? {
                "x" => phys::quantum::pauli_x(),
                "y" => phys::quantum::pauli_y(),
                "z" => phys::quantum::pauli_z(),
                other => return Err(HubError::InvalidInput(format!("unknown pauli: {other}"))),
            };
            let r = phys::quantum::rotation_operator(get_f(p, "angle")?, &pauli);
            Ok(RunOutput::ComplexMatrix(
                r.iter()
                    .map(|row| row.iter().map(|c| (c.re, c.im)).collect())
                    .collect(),
            ))
        }
        "rx_gate" => {
            let r = phys::quantum::rx_gate(get_f(p, "theta")?);
            Ok(RunOutput::ComplexMatrix(
                r.iter()
                    .map(|row| row.iter().map(|c| (c.re, c.im)).collect())
                    .collect(),
            ))
        }
        "ry_gate" => {
            let r = phys::quantum::ry_gate(get_f(p, "theta")?);
            Ok(RunOutput::ComplexMatrix(
                r.iter()
                    .map(|row| row.iter().map(|c| (c.re, c.im)).collect())
                    .collect(),
            ))
        }
        "rz_gate" => {
            let r = phys::quantum::rz_gate(get_f(p, "theta")?);
            Ok(RunOutput::ComplexMatrix(
                r.iter()
                    .map(|row| row.iter().map(|c| (c.re, c.im)).collect())
                    .collect(),
            ))
        }
        "s_gate" => {
            let r = phys::quantum::s_gate();
            Ok(RunOutput::ComplexMatrix(
                r.iter()
                    .map(|row| row.iter().map(|c| (c.re, c.im)).collect())
                    .collect(),
            ))
        }
        "spin_down" => {
            let r = phys::quantum::spin_down();
            Ok(RunOutput::ComplexVector(
                r.iter().map(|c| (c.re, c.im)).collect(),
            ))
        }
        "spin_minus_x" => {
            let r = phys::quantum::spin_minus_x();
            Ok(RunOutput::ComplexVector(
                r.iter().map(|c| (c.re, c.im)).collect(),
            ))
        }
        "spin_minus_y" => {
            let r = phys::quantum::spin_minus_y();
            Ok(RunOutput::ComplexVector(
                r.iter().map(|c| (c.re, c.im)).collect(),
            ))
        }
        "spin_plus_x" => {
            let r = phys::quantum::spin_plus_x();
            Ok(RunOutput::ComplexVector(
                r.iter().map(|c| (c.re, c.im)).collect(),
            ))
        }
        "spin_plus_y" => {
            let r = phys::quantum::spin_plus_y();
            Ok(RunOutput::ComplexVector(
                r.iter().map(|c| (c.re, c.im)).collect(),
            ))
        }
        "spin_up" => {
            let r = phys::quantum::spin_up();
            Ok(RunOutput::ComplexVector(
                r.iter().map(|c| (c.re, c.im)).collect(),
            ))
        }
        "swap_gate" => {
            let r = phys::quantum::swap_gate();
            Ok(RunOutput::ComplexMatrix(
                r.iter()
                    .map(|row| row.iter().map(|c| (c.re, c.im)).collect())
                    .collect(),
            ))
        }
        "t_gate" => {
            let r = phys::quantum::t_gate();
            Ok(RunOutput::ComplexMatrix(
                r.iter()
                    .map(|row| row.iter().map(|c| (c.re, c.im)).collect())
                    .collect(),
            ))
        }
        "toffoli_gate" => {
            let r = phys::quantum::toffoli_gate();
            Ok(RunOutput::ComplexMatrix(
                r.iter()
                    .map(|row| row.iter().map(|c| (c.re, c.im)).collect())
                    .collect(),
            ))
        }
        _ => Err(HubError::InvalidInput(format!("unknown function: {func}"))),
    }
}
