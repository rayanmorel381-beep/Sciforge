//! Dispatch handler for wavefunction functions.

use crate::hub::domain::common::errors::{HubError, HubResult};
use crate::hub::domain::physics as phys;
use crate::hub::engine::dispatch::params::*;
use crate::hub::engine::experience::runner::RunOutput;

pub(crate) fn dispatch(func: &str, p: &Params) -> HubResult<RunOutput> {
    match func {
        "associated_legendre" => Ok(RunOutput::Scalar(phys::quantum::associated_legendre(
            get_i(p, "l")? as u32,
            get_i(p, "m")? as i32,
            get_f(p, "x")?,
        ))),
        "bohr_radius" => Ok(RunOutput::Scalar(phys::quantum::bohr_radius())),
        "clebsch_gordan" => Ok(RunOutput::Scalar(phys::quantum::clebsch_gordan(
            get_f(p, "j1")?,
            get_f(p, "m1")?,
            get_f(p, "j2")?,
            get_f(p, "m2")?,
            get_f(p, "j")?,
            get_f(p, "m")?,
        ))),
        "harmonic_oscillator_energy" => Ok(RunOutput::Scalar(
            phys::quantum::harmonic_oscillator_energy(get_i(p, "n")? as u32, get_f(p, "omega")?),
        )),
        "harmonic_oscillator_wf" => Ok(RunOutput::Scalar(phys::quantum::harmonic_oscillator_wf(
            get_i(p, "n")? as u32,
            get_f(p, "x")?,
            get_f(p, "mass")?,
            get_f(p, "omega")?,
        ))),
        "hydrogen_energy" => Ok(RunOutput::Scalar(phys::quantum::hydrogen_energy(
            get_i(p, "n")? as u32,
        ))),
        "hydrogen_radial_r10" => Ok(RunOutput::Scalar(phys::quantum::hydrogen_radial_r10(
            get_f(p, "r")?,
        ))),
        "hydrogen_radial_r20" => Ok(RunOutput::Scalar(phys::quantum::hydrogen_radial_r20(
            get_f(p, "r")?,
        ))),
        "hydrogen_radial_r21" => Ok(RunOutput::Scalar(phys::quantum::hydrogen_radial_r21(
            get_f(p, "r")?,
        ))),
        "infinite_well_energy" => Ok(RunOutput::Scalar(phys::quantum::infinite_well_energy(
            get_i(p, "n")? as u32,
            get_f(p, "length")?,
            get_f(p, "mass")?,
        ))),
        "infinite_well_wf" => Ok(RunOutput::Scalar(phys::quantum::infinite_well_wf(
            get_i(p, "n")? as u32,
            get_f(p, "x")?,
            get_f(p, "length")?,
        ))),
        "landau_levels" => Ok(RunOutput::Scalar(phys::quantum::landau_levels(
            get_i(p, "n")? as u32,
            get_f(p, "b_field")?,
            get_f(p, "mass")?,
            get_f(p, "charge")?,
        ))),
        "spherical_harmonic_real" => Ok(RunOutput::Scalar(phys::quantum::spherical_harmonic_real(
            get_i(p, "l")? as u32,
            get_i(p, "m")? as i32,
            get_f(p, "theta")?,
            get_f(p, "phi")?,
        ))),
        "tunneling_coefficient" => Ok(RunOutput::Scalar(phys::quantum::tunneling_coefficient(
            get_f(p, "energy")?,
            get_f(p, "v0")?,
            get_f(p, "width")?,
            get_f(p, "mass")?,
        ))),
        "wigner_3j" => Ok(RunOutput::Scalar(phys::quantum::wigner_3j(
            get_f(p, "j1")?,
            get_f(p, "j2")?,
            get_f(p, "j3")?,
            get_f(p, "m1")?,
            get_f(p, "m2")?,
            get_f(p, "m3")?,
        ))),
        "zeeman_splitting" => Ok(RunOutput::Scalar(phys::quantum::zeeman_splitting(
            get_i(p, "m_l")? as i32,
            get_f(p, "b_field")?,
        ))),
        "angular_momentum_coupling" => {
            let r = phys::quantum::angular_momentum_coupling(get_f(p, "j1")?, get_f(p, "j2")?);
            Ok(RunOutput::Matrix(
                r.iter().map(|&(a, b, c)| vec![a, b, c]).collect(),
            ))
        }
        "expectation_momentum" => {
            let psi = get_cv(p, "psi")?;
            Ok(RunOutput::Scalar(phys::quantum::expectation_momentum(
                &psi,
                get_f(p, "dx")?,
            )))
        }
        "expectation_position" => {
            let psi = get_cv(p, "psi")?;
            Ok(RunOutput::Scalar(phys::quantum::expectation_position(
                &psi,
                get_v(p, "x")?,
                get_f(p, "dx")?,
            )))
        }
        "gaussian_packet" => {
            let r = phys::quantum::gaussian_packet(
                get_f(p, "x")?,
                get_f(p, "x0")?,
                get_f(p, "sigma")?,
                get_f(p, "k0")?,
            );
            Ok(RunOutput::Complex(r.re, r.im))
        }
        "inner_product" => {
            let psi = get_cv(p, "psi")?;
            let phi = get_cv(p, "phi")?;
            let r = phys::quantum::inner_product(&psi, &phi, get_f(p, "dx")?);
            Ok(RunOutput::Complex(r.re, r.im))
        }
        "normalize_wavefunction" => {
            let mut psi = get_cv(p, "psi")?;
            phys::quantum::normalize(&mut psi, get_f(p, "dx")?);
            Ok(RunOutput::ComplexVector(
                psi.iter().map(|c| (c.re, c.im)).collect(),
            ))
        }
        "plane_wave" => {
            let r = phys::quantum::plane_wave(
                get_f(p, "x")?,
                get_f(p, "k")?,
                get_f(p, "omega")?,
                get_f(p, "t")?,
            );
            Ok(RunOutput::Complex(r.re, r.im))
        }
        "probability_density" => {
            let psi = get_cv(p, "psi")?;
            Ok(RunOutput::Vector(phys::quantum::probability_density(&psi)))
        }
        "spherical_harmonic" => {
            let r = phys::quantum::spherical_harmonic(
                get_i(p, "l")? as u32,
                get_i(p, "m")? as i32,
                get_f(p, "theta")?,
                get_f(p, "phi")?,
            );
            Ok(RunOutput::Complex(r.re, r.im))
        }
        "time_evolve_split_step" => {
            let mut psi = get_cv(p, "psi")?;
            phys::quantum::time_evolve_split_step(
                &mut psi,
                get_v(p, "v")?,
                get_f(p, "dx")?,
                get_f(p, "dt")?,
                get_f(p, "mass")?,
                get_u(p, "steps")?,
            );
            Ok(RunOutput::ComplexVector(
                psi.iter().map(|c| (c.re, c.im)).collect(),
            ))
        }
        "transition_probability" => {
            let psi_i = get_cv(p, "psi_initial")?;
            let psi_f = get_cv(p, "psi_final")?;
            Ok(RunOutput::Scalar(phys::quantum::transition_probability(
                &psi_i,
                &psi_f,
                get_f(p, "dx")?,
            )))
        }
        _ => Err(HubError::InvalidInput(format!("unknown function: {func}"))),
    }
}
