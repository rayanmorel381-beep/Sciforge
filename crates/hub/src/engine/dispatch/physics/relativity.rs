//! Dispatch handler for relativity functions.

use super::super::params::*;
use crate::domain::common::errors::{HubError, HubResult};
use crate::domain::physics as phys;
use crate::engine::experience::runner::RunOutput;

pub(super) fn dispatch(func: &str, p: &Params) -> HubResult<RunOutput> {
    match func {
        "aberration" => Ok(RunOutput::Scalar(phys::relativity::aberration(
            get_f(p, "theta")?,
            get_f(p, "v")?,
        ))),
        "beta" => Ok(RunOutput::Scalar(phys::relativity::beta(get_f(p, "v")?))),
        "bremsstrahlung_power_classical" => Ok(RunOutput::Scalar(
            phys::relativity::bremsstrahlung_power_classical(
                get_f(p, "charge")?,
                get_f(p, "accel")?,
            ),
        )),
        "compton_wavelength_shift" => Ok(RunOutput::Scalar(
            phys::relativity::compton_wavelength_shift(get_f(p, "angle")?),
        )),
        "energy_momentum_relation" => Ok(RunOutput::Scalar(
            phys::relativity::energy_momentum_relation(get_f(p, "mass")?, get_f(p, "momentum")?),
        )),
        "gamma_factor" => Ok(RunOutput::Scalar(phys::relativity::gamma_factor(get_f(
            p, "v",
        )?))),
        "headlight_effect" => Ok(RunOutput::Scalar(phys::relativity::headlight_effect(
            get_f(p, "theta_rest")?,
            get_f(p, "v")?,
        ))),
        "inverse_lorentz_transform" => {
            let r = phys::relativity::inverse_lorentz_transform(
                get_f(p, "t_prime")?,
                get_f(p, "x_prime")?,
                get_f(p, "v")?,
            );
            Ok(RunOutput::Pair(r.0, r.1))
        }
        "kinetic_energy_relativistic" => Ok(RunOutput::Scalar(
            phys::relativity::kinetic_energy_relativistic(get_f(p, "mass")?, get_f(p, "v")?),
        )),
        "length_contraction" => Ok(RunOutput::Scalar(phys::relativity::length_contraction(
            get_f(p, "proper_length")?,
            get_f(p, "v")?,
        ))),
        "lorentz_transform" => {
            let r =
                phys::relativity::lorentz_transform(get_f(p, "t")?, get_f(p, "x")?, get_f(p, "v")?);
            Ok(RunOutput::Pair(r.0, r.1))
        }
        "proper_acceleration_to_coordinate" => {
            let r = phys::relativity::proper_acceleration_to_coordinate(
                get_f(p, "proper_accel")?,
                get_f(p, "proper_time")?,
            );
            Ok(RunOutput::Pair(r.0, r.1))
        }
        "rapidity" => Ok(RunOutput::Scalar(phys::relativity::rapidity(get_f(
            p, "v",
        )?))),
        "relativity::relativistic_doppler" => {
            Ok(RunOutput::Scalar(phys::relativity::relativistic_doppler(
                get_f(p, "freq")?,
                get_f(p, "v")?,
                get_f(p, "angle")?,
            )))
        }
        "relativistic_energy" => Ok(RunOutput::Scalar(phys::relativity::relativistic_energy(
            get_f(p, "mass")?,
            get_f(p, "v")?,
        ))),
        "relativistic_kinetic_energy_from_gamma" => Ok(RunOutput::Scalar(
            phys::relativity::relativistic_kinetic_energy_from_gamma(
                get_f(p, "mass")?,
                get_f(p, "gamma")?,
            ),
        )),
        "relativistic_momentum" => Ok(RunOutput::Scalar(phys::relativity::relativistic_momentum(
            get_f(p, "mass")?,
            get_f(p, "v")?,
        ))),
        "rest_energy" => Ok(RunOutput::Scalar(phys::relativity::rest_energy(get_f(
            p, "mass",
        )?))),
        "synchrotron_power" => Ok(RunOutput::Scalar(phys::relativity::synchrotron_power(
            get_f(p, "charge")?,
            get_f(p, "mass")?,
            get_f(p, "gamma")?,
            get_f(p, "radius")?,
        ))),
        "threshold_energy" => Ok(RunOutput::Scalar(phys::relativity::threshold_energy(
            get_f(p, "m_target")?,
            get_f(p, "m_products_sum")?,
        ))),
        "time_dilation" => Ok(RunOutput::Scalar(phys::relativity::time_dilation(
            get_f(p, "proper_time")?,
            get_f(p, "v")?,
        ))),
        "transverse_doppler" => Ok(RunOutput::Scalar(phys::relativity::transverse_doppler(
            get_f(p, "freq")?,
            get_f(p, "v")?,
        ))),
        "twin_paradox_age" => Ok(RunOutput::Scalar(phys::relativity::twin_paradox_age(
            get_f(p, "v")?,
            get_f(p, "coord_time")?,
        ))),
        "velocity_addition" => Ok(RunOutput::Scalar(phys::relativity::velocity_addition(
            get_f(p, "u")?,
            get_f(p, "v")?,
        ))),
        "velocity_from_rapidity" => Ok(RunOutput::Scalar(
            phys::relativity::velocity_from_rapidity(get_f(p, "phi")?),
        )),
        "boost_matrix" => {
            let v = get_v(p, "v")?;
            let r = phys::relativity::boost_matrix([v[0], v[1], v[2]]);
            Ok(RunOutput::Matrix(
                r.iter().map(|row| row.to_vec()).collect(),
            ))
        }
        "four_momentum" => {
            let v = get_v(p, "v")?;
            let r = phys::relativity::four_momentum(get_f(p, "mass")?, [v[0], v[1], v[2]]);
            Ok(RunOutput::Vector(r.to_vec()))
        }
        "invariant_mass_two_body" => {
            let p1 = get_v(p, "p1")?;
            let p2 = get_v(p, "p2")?;
            Ok(RunOutput::Scalar(
                phys::relativity::invariant_mass_two_body(
                    [p1[0], p1[1], p1[2], p1[3]],
                    [p2[0], p2[1], p2[2], p2[3]],
                ),
            ))
        }
        "lorentz_transform_4" => {
            let ev = get_v(p, "event")?;
            let v = get_v(p, "v")?;
            let r = phys::relativity::lorentz_transform_4(
                [ev[0], ev[1], ev[2], ev[3]],
                [v[0], v[1], v[2]],
            );
            Ok(RunOutput::Vector(r.to_vec()))
        }
        "mandelstam_s" => {
            let p1 = get_v(p, "p1")?;
            let p2 = get_v(p, "p2")?;
            Ok(RunOutput::Scalar(phys::relativity::mandelstam_s(
                [p1[0], p1[1], p1[2], p1[3]],
                [p2[0], p2[1], p2[2], p2[3]],
            )))
        }
        "velocity_addition_3d" => {
            let u = get_v(p, "u")?;
            let dir = get_v(p, "dir")?;
            let r = phys::relativity::velocity_addition_3d(
                [u[0], u[1], u[2]],
                get_f(p, "v")?,
                [dir[0], dir[1], dir[2]],
            );
            Ok(RunOutput::Triple(r[0], r[1], r[2]))
        }
        _ => Err(HubError::InvalidInput(format!("unknown function: {func}"))),
    }
}
