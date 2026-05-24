//! Dispatch handler for quantum operator functions.

use crate::domain::common::errors::{HubError, HubResult};
use crate::domain::physics as phys;
use crate::engine::dispatch::params::*;
use crate::engine::experience::runner::RunOutput;
use sciforge_lib::maths::complex::Complex;

pub(crate) fn dispatch(func: &str, p: &Params) -> HubResult<RunOutput> {
    match func {
        "anticommutator" => {
            let a: Vec<Vec<Complex>> = get_m(p, "a")?
                .iter()
                .map(|row| row.chunks(2).map(|c| Complex::new(c[0], c[1])).collect())
                .collect();
            let b: Vec<Vec<Complex>> = get_m(p, "b")?
                .iter()
                .map(|row| row.chunks(2).map(|c| Complex::new(c[0], c[1])).collect())
                .collect();
            let r = phys::quantum::anticommutator(&a, &b);
            Ok(RunOutput::ComplexMatrix(
                r.iter()
                    .map(|row| row.iter().map(|c| (c.re, c.im)).collect())
                    .collect(),
            ))
        }
        "commutator" => {
            let a: Vec<Vec<Complex>> = get_m(p, "a")?
                .iter()
                .map(|row| row.chunks(2).map(|c| Complex::new(c[0], c[1])).collect())
                .collect();
            let b: Vec<Vec<Complex>> = get_m(p, "b")?
                .iter()
                .map(|row| row.chunks(2).map(|c| Complex::new(c[0], c[1])).collect())
                .collect();
            let r = phys::quantum::commutator(&a, &b);
            Ok(RunOutput::ComplexMatrix(
                r.iter()
                    .map(|row| row.iter().map(|c| (c.re, c.im)).collect())
                    .collect(),
            ))
        }
        "concurrence_2qubit" => {
            let rho: Vec<Vec<Complex>> = get_m(p, "rho")?
                .iter()
                .map(|row| row.chunks(2).map(|c| Complex::new(c[0], c[1])).collect())
                .collect();
            Ok(RunOutput::Scalar(phys::quantum::concurrence_2qubit(&rho)))
        }
        "density_matrix" => {
            let state = get_cv(p, "state")?;
            let r = phys::quantum::density_matrix(&state);
            Ok(RunOutput::ComplexMatrix(
                r.iter()
                    .map(|row| row.iter().map(|c| (c.re, c.im)).collect())
                    .collect(),
            ))
        }
        "dirac_gamma0" => {
            let r = phys::quantum::dirac_gamma0();
            Ok(RunOutput::ComplexMatrix(
                r.iter()
                    .map(|row| row.iter().map(|c| (c.re, c.im)).collect())
                    .collect(),
            ))
        }
        "dirac_gamma1" => {
            let r = phys::quantum::dirac_gamma1();
            Ok(RunOutput::ComplexMatrix(
                r.iter()
                    .map(|row| row.iter().map(|c| (c.re, c.im)).collect())
                    .collect(),
            ))
        }
        "dirac_gamma2" => {
            let r = phys::quantum::dirac_gamma2();
            Ok(RunOutput::ComplexMatrix(
                r.iter()
                    .map(|row| row.iter().map(|c| (c.re, c.im)).collect())
                    .collect(),
            ))
        }
        "dirac_gamma3" => {
            let r = phys::quantum::dirac_gamma3();
            Ok(RunOutput::ComplexMatrix(
                r.iter()
                    .map(|row| row.iter().map(|c| (c.re, c.im)).collect())
                    .collect(),
            ))
        }
        "entanglement_entropy" => {
            let state = get_cv(p, "state")?;
            Ok(RunOutput::Scalar(phys::quantum::entanglement_entropy(
                &state,
                get_u(p, "dim_a")?,
            )))
        }
        "expectation_value" => {
            let op: Vec<Vec<Complex>> = get_m(p, "operator")?
                .iter()
                .map(|row| row.chunks(2).map(|c| Complex::new(c[0], c[1])).collect())
                .collect();
            let state = get_cv(p, "state")?;
            let r = phys::quantum::expectation_value(&op, &state);
            Ok(RunOutput::Complex(r.re, r.im))
        }
        "fermi_golden_rule" => {
            let vm: Vec<Vec<Complex>> = get_m(p, "v_matrix")?
                .iter()
                .map(|row| row.chunks(2).map(|c| Complex::new(c[0], c[1])).collect())
                .collect();
            let initial = get_cv(p, "initial")?;
            let finals: Vec<Vec<Complex>> = get_m(p, "finals")?
                .iter()
                .map(|row| row.chunks(2).map(|c| Complex::new(c[0], c[1])).collect())
                .collect();
            Ok(RunOutput::Scalar(phys::quantum::fermi_golden_rule(
                &vm,
                &initial,
                &finals,
                get_f(p, "density_of_states")?,
            )))
        }
        "fidelity_mixed" => {
            let rho: Vec<Vec<Complex>> = get_m(p, "rho")?
                .iter()
                .map(|row| row.chunks(2).map(|c| Complex::new(c[0], c[1])).collect())
                .collect();
            let sigma: Vec<Vec<Complex>> = get_m(p, "sigma")?
                .iter()
                .map(|row| row.chunks(2).map(|c| Complex::new(c[0], c[1])).collect())
                .collect();
            Ok(RunOutput::Scalar(phys::quantum::fidelity_mixed(
                &rho, &sigma,
            )))
        }
        "fidelity_pure" => {
            let psi = get_cv(p, "psi")?;
            let phi = get_cv(p, "phi")?;
            Ok(RunOutput::Scalar(phys::quantum::fidelity_pure(&psi, &phi)))
        }
        "first_order_energy" => {
            let vm: Vec<Vec<Complex>> = get_m(p, "v_matrix")?
                .iter()
                .map(|row| row.chunks(2).map(|c| Complex::new(c[0], c[1])).collect())
                .collect();
            let state = get_cv(p, "state")?;
            Ok(RunOutput::Scalar(phys::quantum::first_order_energy(
                &vm, &state,
            )))
        }
        "first_order_state" => {
            let vm: Vec<Vec<Complex>> = get_m(p, "v_matrix")?
                .iter()
                .map(|row| row.chunks(2).map(|c| Complex::new(c[0], c[1])).collect())
                .collect();
            let states: Vec<Vec<Complex>> = get_m(p, "states")?
                .iter()
                .map(|row| row.chunks(2).map(|c| Complex::new(c[0], c[1])).collect())
                .collect();
            let r = phys::quantum::first_order_state(
                get_v(p, "h0_energies")?,
                &vm,
                &states,
                get_u(p, "n_index")?,
            );
            Ok(RunOutput::ComplexVector(
                r.iter().map(|c| (c.re, c.im)).collect(),
            ))
        }
        "gamma5" => {
            let r = phys::quantum::gamma5();
            Ok(RunOutput::ComplexMatrix(
                r.iter()
                    .map(|row| row.iter().map(|c| (c.re, c.im)).collect())
                    .collect(),
            ))
        }
        "is_hermitian" => {
            let m: Vec<Vec<Complex>> = get_m(p, "m")?
                .iter()
                .map(|row| row.chunks(2).map(|c| Complex::new(c[0], c[1])).collect())
                .collect();
            Ok(RunOutput::Boolean(phys::quantum::is_hermitian(&m)))
        }
        "is_unitary" => {
            let m: Vec<Vec<Complex>> = get_m(p, "m")?
                .iter()
                .map(|row| row.chunks(2).map(|c| Complex::new(c[0], c[1])).collect())
                .collect();
            Ok(RunOutput::Boolean(phys::quantum::is_unitary(&m)))
        }
        "jz_operator" => {
            let r = phys::quantum::jz_operator(get_f(p, "j")?);
            Ok(RunOutput::ComplexMatrix(
                r.iter()
                    .map(|row| row.iter().map(|c| (c.re, c.im)).collect())
                    .collect(),
            ))
        }
        "ladder_operator_minus" => {
            let r = phys::quantum::ladder_operator_minus(get_f(p, "j")?);
            Ok(RunOutput::ComplexMatrix(
                r.iter()
                    .map(|row| row.iter().map(|c| (c.re, c.im)).collect())
                    .collect(),
            ))
        }
        "ladder_operator_plus" => {
            let r = phys::quantum::ladder_operator_plus(get_f(p, "j")?);
            Ok(RunOutput::ComplexMatrix(
                r.iter()
                    .map(|row| row.iter().map(|c| (c.re, c.im)).collect())
                    .collect(),
            ))
        }
        "partial_trace_b" => {
            let rho: Vec<Vec<Complex>> = get_m(p, "rho")?
                .iter()
                .map(|row| row.chunks(2).map(|c| Complex::new(c[0], c[1])).collect())
                .collect();
            let r = phys::quantum::partial_trace_b(&rho, get_u(p, "dim_a")?, get_u(p, "dim_b")?);
            Ok(RunOutput::ComplexMatrix(
                r.iter()
                    .map(|row| row.iter().map(|c| (c.re, c.im)).collect())
                    .collect(),
            ))
        }
        "purity" => {
            let rho: Vec<Vec<Complex>> = get_m(p, "rho")?
                .iter()
                .map(|row| row.chunks(2).map(|c| Complex::new(c[0], c[1])).collect())
                .collect();
            Ok(RunOutput::Scalar(phys::quantum::purity(&rho)))
        }
        "second_order_energy" => {
            let vm: Vec<Vec<Complex>> = get_m(p, "v_matrix")?
                .iter()
                .map(|row| row.chunks(2).map(|c| Complex::new(c[0], c[1])).collect())
                .collect();
            let states: Vec<Vec<Complex>> = get_m(p, "states")?
                .iter()
                .map(|row| row.chunks(2).map(|c| Complex::new(c[0], c[1])).collect())
                .collect();
            Ok(RunOutput::Scalar(phys::quantum::second_order_energy(
                get_v(p, "h0_energies")?,
                &vm,
                &states,
                get_u(p, "n_index")?,
            )))
        }
        "quantum_tensor_product" => {
            let a: Vec<Vec<Complex>> = get_m(p, "a")?
                .iter()
                .map(|row| row.chunks(2).map(|c| Complex::new(c[0], c[1])).collect())
                .collect();
            let b: Vec<Vec<Complex>> = get_m(p, "b")?
                .iter()
                .map(|row| row.chunks(2).map(|c| Complex::new(c[0], c[1])).collect())
                .collect();
            let r = phys::quantum::tensor_product(&a, &b);
            Ok(RunOutput::ComplexMatrix(
                r.iter()
                    .map(|row| row.iter().map(|c| (c.re, c.im)).collect())
                    .collect(),
            ))
        }
        "trace_complex" => {
            let m: Vec<Vec<Complex>> = get_m(p, "m")?
                .iter()
                .map(|row| row.chunks(2).map(|c| Complex::new(c[0], c[1])).collect())
                .collect();
            let r = phys::quantum::trace_complex(&m);
            Ok(RunOutput::Complex(r.re, r.im))
        }
        "uncertainty_product" => {
            let a: Vec<Vec<Complex>> = get_m(p, "op_a")?
                .iter()
                .map(|row| row.chunks(2).map(|c| Complex::new(c[0], c[1])).collect())
                .collect();
            let b: Vec<Vec<Complex>> = get_m(p, "op_b")?
                .iter()
                .map(|row| row.chunks(2).map(|c| Complex::new(c[0], c[1])).collect())
                .collect();
            let state = get_cv(p, "state")?;
            Ok(RunOutput::Scalar(phys::quantum::uncertainty_product(
                &a, &b, &state,
            )))
        }
        "quantum_variance" => {
            let op: Vec<Vec<Complex>> = get_m(p, "operator")?
                .iter()
                .map(|row| row.chunks(2).map(|c| Complex::new(c[0], c[1])).collect())
                .collect();
            let state = get_cv(p, "state")?;
            Ok(RunOutput::Scalar(phys::quantum::variance(&op, &state)))
        }
        "variational_energy" => {
            let h: Vec<Vec<Complex>> = get_m(p, "hamiltonian")?
                .iter()
                .map(|row| row.chunks(2).map(|c| Complex::new(c[0], c[1])).collect())
                .collect();
            let trial = get_cv(p, "trial")?;
            Ok(RunOutput::Scalar(phys::quantum::variational_energy(
                &h, &trial,
            )))
        }
        "von_neumann_entropy" => {
            let rho: Vec<Vec<Complex>> = get_m(p, "rho")?
                .iter()
                .map(|row| row.chunks(2).map(|c| Complex::new(c[0], c[1])).collect())
                .collect();
            Ok(RunOutput::Scalar(phys::quantum::von_neumann_entropy(&rho)))
        }
        _ => Err(HubError::InvalidInput(format!("unknown function: {func}"))),
    }
}
