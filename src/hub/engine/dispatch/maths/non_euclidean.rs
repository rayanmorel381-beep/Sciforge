//! Dispatch handler for non-Euclidean geometry functions.

use super::super::params::*;
use crate::hub::domain::common::errors::{HubError, HubResult};
use crate::hub::domain::maths;
use crate::hub::engine::experience::runner::RunOutput;

pub(super) fn dispatch(func: &str, p: &Params) -> HubResult<RunOutput> {
    match func {
        "gaussian_curvature_sphere" => Ok(RunOutput::Scalar(
            maths::non_euclidean::gaussian_curvature_sphere(get_f(p, "radius")?),
        )),
        "gaussian_curvature_hyperbolic" => Ok(RunOutput::Scalar(
            maths::non_euclidean::gaussian_curvature_hyperbolic(get_f(p, "curvature")?),
        )),
        "poincare_disk_distance" => {
            let p1 = get_v(p, "p1")?;
            let p2 = get_v(p, "p2")?;
            Ok(RunOutput::Scalar(
                maths::non_euclidean::poincare_disk_distance(&[p1[0], p1[1]], &[p2[0], p2[1]]),
            ))
        }
        "spherical_distance" => Ok(RunOutput::Scalar(maths::non_euclidean::spherical_distance(
            get_f(p, "r")?,
            get_f(p, "theta1")?,
            get_f(p, "phi1")?,
            get_f(p, "theta2")?,
            get_f(p, "phi2")?,
        ))),
        "parallel_transport_sphere" => {
            let v = get_v(p, "vector")?;
            let r = maths::non_euclidean::parallel_transport_sphere(
                &[v[0], v[1]],
                get_f(p, "start_theta")?,
                get_f(p, "start_phi")?,
                get_f(p, "end_theta")?,
                get_f(p, "end_phi")?,
                get_u(p, "steps")?,
            );
            Ok(RunOutput::Pair(r[0], r[1]))
        }
        "geodesic_deviation_magnitude" => Ok(RunOutput::Scalar(
            maths::non_euclidean::geodesic_deviation_magnitude(
                get_f(p, "separation")?,
                get_f(p, "velocity")?,
                get_f(p, "curvature")?,
            ),
        )),
        "upper_half_plane_distance" => {
            let p1 = get_v(p, "p1")?;
            let p2 = get_v(p, "p2")?;
            Ok(RunOutput::Scalar(
                maths::non_euclidean::upper_half_plane_distance(&[p1[0], p1[1]], &[p2[0], p2[1]]),
            ))
        }
        "hyperboloid_distance" => {
            let p1 = get_v(p, "p1")?;
            let p2 = get_v(p, "p2")?;
            Ok(RunOutput::Scalar(
                maths::non_euclidean::hyperboloid_distance(
                    &[p1[0], p1[1], p1[2]],
                    &[p2[0], p2[1], p2[2]],
                ),
            ))
        }
        "spherical_area" => Ok(RunOutput::Scalar(maths::non_euclidean::spherical_area(
            get_f(p, "r")?,
            get_f(p, "solid_angle_sr")?,
        ))),
        "spherical_excess" => {
            let a = get_v(p, "angles")?;
            Ok(RunOutput::Scalar(maths::non_euclidean::spherical_excess(
                &[a[0], a[1], a[2]],
            )))
        }
        "hyperbolic_area_triangle" => {
            let a = get_v(p, "angles")?;
            Ok(RunOutput::Scalar(
                maths::non_euclidean::hyperbolic_area_triangle(&[a[0], a[1], a[2]]),
            ))
        }
        "weyl_tensor_vanishes_2d" => Ok(RunOutput::Boolean(
            maths::non_euclidean::weyl_tensor_vanishes_2d(),
        )),
        "kretschmer_scalar_schwarzschild" => Ok(RunOutput::Scalar(
            maths::non_euclidean::kretschmer_scalar_schwarzschild(get_f(p, "rs")?, get_f(p, "r")?),
        )),
        "schwarzschild_radius" => Ok(RunOutput::Scalar(
            maths::non_euclidean::schwarzschild_radius(get_f(p, "mass_kg")?),
        )),
        "proper_time_schwarzschild" => Ok(RunOutput::Scalar(
            maths::non_euclidean::proper_time_schwarzschild(
                get_f(p, "coord_time")?,
                get_f(p, "r")?,
                get_f(p, "rs")?,
            ),
        )),
        "gravitational_redshift" => Ok(RunOutput::Scalar(
            maths::non_euclidean::gravitational_redshift(
                get_f(p, "freq_emit")?,
                get_f(p, "r_emit")?,
                get_f(p, "r_obs")?,
                get_f(p, "rs")?,
            ),
        )),
        "kerr_ergosphere_radius" => Ok(RunOutput::Scalar(
            maths::non_euclidean::kerr_ergosphere_radius(
                get_f(p, "m")?,
                get_f(p, "a")?,
                get_f(p, "theta")?,
            ),
        )),
        "isco_schwarzschild" => Ok(RunOutput::Scalar(maths::non_euclidean::isco_schwarzschild(
            get_f(p, "rs")?,
        ))),
        "photon_sphere_radius" => Ok(RunOutput::Scalar(
            maths::non_euclidean::photon_sphere_radius(get_f(p, "rs")?),
        )),
        "hawking_temperature" => Ok(RunOutput::Scalar(
            maths::non_euclidean::hawking_temperature(get_f(p, "mass_kg")?),
        )),
        "black_hole_entropy" => Ok(RunOutput::Scalar(maths::non_euclidean::black_hole_entropy(
            get_f(p, "mass_kg")?,
        ))),
        "kerr_event_horizon" => Ok(RunOutput::Scalar(maths::non_euclidean::kerr_event_horizon(
            get_f(p, "m")?,
            get_f(p, "a")?,
        ))),
        "kerr_cauchy_horizon" => Ok(RunOutput::Scalar(
            maths::non_euclidean::kerr_cauchy_horizon(get_f(p, "m")?, get_f(p, "a")?),
        )),
        "schwarzschild_time_dilation" => Ok(RunOutput::Scalar(
            maths::non_euclidean::schwarzschild_time_dilation(get_f(p, "r")?, get_f(p, "rs")?),
        )),
        "gravitational_lensing_angle" => Ok(RunOutput::Scalar(
            maths::non_euclidean::gravitational_lensing_angle(
                get_f(p, "mass_kg")?,
                get_f(p, "impact_param")?,
            ),
        )),
        "orbital_velocity_schwarzschild" => Ok(RunOutput::Scalar(
            maths::non_euclidean::orbital_velocity_schwarzschild(get_f(p, "rs")?, get_f(p, "r")?),
        )),
        "tidal_force_schwarzschild" => Ok(RunOutput::Scalar(
            maths::non_euclidean::tidal_force_schwarzschild(
                get_f(p, "mass_kg")?,
                get_f(p, "r")?,
                get_f(p, "delta_r")?,
            ),
        )),
        "hawking_luminosity" => Ok(RunOutput::Scalar(maths::non_euclidean::hawking_luminosity(
            get_f(p, "mass_kg")?,
        ))),
        "black_hole_evaporation_time" => Ok(RunOutput::Scalar(
            maths::non_euclidean::black_hole_evaporation_time(get_f(p, "mass_kg")?),
        )),
        "bekenstein_bound" => Ok(RunOutput::Scalar(maths::non_euclidean::bekenstein_bound(
            get_f(p, "energy_j")?,
            get_f(p, "radius_m")?,
        ))),
        "penrose_energy_extraction" => Ok(RunOutput::Scalar(
            maths::non_euclidean::penrose_energy_extraction(get_f(p, "m")?, get_f(p, "a")?),
        )),
        "frame_dragging_rate" => Ok(RunOutput::Scalar(
            maths::non_euclidean::frame_dragging_rate(
                get_f(p, "mass_kg")?,
                get_f(p, "angular_momentum")?,
                get_f(p, "r")?,
            ),
        )),
        "reissner_nordstrom_outer_horizon" => Ok(RunOutput::Scalar(
            maths::non_euclidean::reissner_nordstrom_outer_horizon(
                get_f(p, "mass_kg")?,
                get_f(p, "charge_c")?,
            ),
        )),
        "flrw_scale_factor" => Ok(RunOutput::Scalar(maths::non_euclidean::flrw_scale_factor(
            get_f(p, "h0")?,
            get_f(p, "omega_m")?,
            get_f(p, "omega_lambda")?,
            get_f(p, "t")?,
        ))),
        "ne_hubble_parameter" => Ok(RunOutput::Scalar(maths::non_euclidean::hubble_parameter(
            get_f(p, "h0")?,
            get_f(p, "omega_m")?,
            get_f(p, "omega_lambda")?,
            get_f(p, "z")?,
        ))),
        "ne_comoving_distance" => Ok(RunOutput::Scalar(maths::non_euclidean::comoving_distance(
            get_f(p, "h0")?,
            get_f(p, "omega_m")?,
            get_f(p, "omega_lambda")?,
            get_f(p, "z")?,
        ))),
        "ne_luminosity_distance" => Ok(RunOutput::Scalar(
            maths::non_euclidean::luminosity_distance(
                get_f(p, "h0")?,
                get_f(p, "omega_m")?,
                get_f(p, "omega_lambda")?,
                get_f(p, "z")?,
            ),
        )),
        "ne_angular_diameter_distance" => Ok(RunOutput::Scalar(
            maths::non_euclidean::angular_diameter_distance(
                get_f(p, "h0")?,
                get_f(p, "omega_m")?,
                get_f(p, "omega_lambda")?,
                get_f(p, "z")?,
            ),
        )),
        "ne_lookback_time" => Ok(RunOutput::Scalar(maths::non_euclidean::lookback_time(
            get_f(p, "h0")?,
            get_f(p, "omega_m")?,
            get_f(p, "omega_lambda")?,
            get_f(p, "z")?,
        ))),
        "ne_distance_modulus" => Ok(RunOutput::Scalar(maths::non_euclidean::distance_modulus(
            get_f(p, "h0")?,
            get_f(p, "omega_m")?,
            get_f(p, "omega_lambda")?,
            get_f(p, "z")?,
        ))),
        "ne_age_of_universe" => Ok(RunOutput::Scalar(maths::non_euclidean::age_of_universe(
            get_f(p, "h0")?,
            get_f(p, "omega_m")?,
            get_f(p, "omega_lambda")?,
        ))),
        "ne_critical_density" => Ok(RunOutput::Scalar(maths::non_euclidean::critical_density(
            get_f(p, "h0_km_s_mpc")?,
        ))),
        "deceleration_parameter" => Ok(RunOutput::Scalar(
            maths::non_euclidean::deceleration_parameter(
                get_f(p, "omega_m")?,
                get_f(p, "omega_lambda")?,
            ),
        )),
        "cosmic_time_matter_dominated" => Ok(RunOutput::Scalar(
            maths::non_euclidean::cosmic_time_matter_dominated(get_f(p, "h0")?, get_f(p, "z")?),
        )),
        "horizon_distance" => Ok(RunOutput::Scalar(maths::non_euclidean::horizon_distance(
            get_f(p, "h0")?,
            get_f(p, "omega_m")?,
            get_f(p, "z")?,
        ))),
        "dark_energy_equation_of_state" => Ok(RunOutput::Scalar(
            maths::non_euclidean::dark_energy_equation_of_state(
                get_f(p, "omega_lambda")?,
                get_f(p, "omega_m")?,
                get_f(p, "z")?,
            ),
        )),
        "ne_proper_distance" => Ok(RunOutput::Scalar(maths::non_euclidean::proper_distance(
            get_f(p, "h0")?,
            get_f(p, "omega_m")?,
            get_f(p, "omega_lambda")?,
            get_f(p, "z")?,
        ))),
        "volume_comoving" => Ok(RunOutput::Scalar(maths::non_euclidean::volume_comoving(
            get_f(p, "h0")?,
            get_f(p, "omega_m")?,
            get_f(p, "omega_lambda")?,
            get_f(p, "z")?,
        ))),
        "cmb_temperature_at_redshift" => Ok(RunOutput::Scalar(
            maths::non_euclidean::cmb_temperature_at_redshift(get_f(p, "t0")?, get_f(p, "z")?),
        )),
        "recombination_redshift" => Ok(RunOutput::Scalar(
            maths::non_euclidean::recombination_redshift(),
        )),
        "matter_radiation_equality_redshift" => Ok(RunOutput::Scalar(
            maths::non_euclidean::matter_radiation_equality_redshift(
                get_f(p, "omega_m")?,
                get_f(p, "omega_r")?,
            ),
        )),

        _ => Err(HubError::InvalidInput(format!("unknown function: {func}"))),
    }
}
