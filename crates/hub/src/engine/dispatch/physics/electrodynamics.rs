//! Dispatch handler for electrodynamics functions.

use super::super::params::*;
use crate::domain::common::errors::{HubError, HubResult};
use crate::domain::physics as phys;
use crate::engine::experience::runner::RunOutput;

pub(super) fn dispatch(func: &str, p: &Params) -> HubResult<RunOutput> {
    match func {
        "antenna_radiation_resistance_dipole" => Ok(RunOutput::Scalar(
            phys::electrodynamics::waves::antenna_radiation_resistance_dipole(
                get_f(p, "length")?,
                get_f(p, "wavelength")?,
            ),
        )),
        "electrodynamics::brewster_angle" => Ok(RunOutput::Scalar(
            phys::electrodynamics::waves::brewster_angle(get_f(p, "n1")?, get_f(p, "n2")?),
        )),
        "capacitance_parallel_plate" => Ok(RunOutput::Scalar(
            phys::electrodynamics::fields::capacitance_parallel_plate(
                get_f(p, "area")?,
                get_f(p, "distance")?,
                get_f(p, "epsilon_r")?,
            ),
        )),
        "cyclotron_frequency" => Ok(RunOutput::Scalar(
            phys::electrodynamics::fields::cyclotron_frequency(
                get_f(p, "charge")?,
                get_f(p, "mass")?,
                get_f(p, "b")?,
            ),
        )),
        "debye_length" => Ok(RunOutput::Scalar(phys::electrodynamics::fields::debye_length(
            get_f(p, "temperature")?,
            get_f(p, "number_density")?,
            get_f(p, "charge")?,
        ))),
        "electric_potential_point" => Ok(RunOutput::Scalar(
            phys::electrodynamics::fields::electric_potential_point(get_f(p, "q")?, get_f(p, "r")?),
        )),
        "energy_capacitor" => Ok(RunOutput::Scalar(phys::electrodynamics::circuits::energy_capacitor(
            get_f(p, "c")?,
            get_f(p, "v")?,
        ))),
        "energy_inductor" => Ok(RunOutput::Scalar(phys::electrodynamics::circuits::energy_inductor(
            get_f(p, "l")?,
            get_f(p, "i")?,
        ))),
        "fresnel_rp" => Ok(RunOutput::Scalar(phys::electrodynamics::waves::fresnel_rp(
            get_f(p, "n1")?,
            get_f(p, "n2")?,
            get_f(p, "theta_i")?,
        ))),
        "fresnel_rs" => Ok(RunOutput::Scalar(phys::electrodynamics::waves::fresnel_rs(
            get_f(p, "n1")?,
            get_f(p, "n2")?,
            get_f(p, "theta_i")?,
        ))),
        "group_velocity_dispersive" => Ok(RunOutput::Scalar(
            phys::electrodynamics::waves::group_velocity_dispersive(
                get_f(p, "v_phase")?,
                get_f(p, "omega")?,
                get_f(p, "dv_domega")?,
            ),
        )),
        "inductance_solenoid" => Ok(RunOutput::Scalar(
            phys::electrodynamics::fields::inductance_solenoid(
                get_f(p, "n_turns")?,
                get_f(p, "length")?,
                get_f(p, "area")?,
            ),
        )),
        "larmor_radiation_power" => Ok(RunOutput::Scalar(
            phys::electrodynamics::waves::larmor_radiation_power(get_f(p, "charge")?, get_f(p, "accel")?),
        )),
        "larmor_radius" => Ok(RunOutput::Scalar(phys::electrodynamics::fields::larmor_radius(
            get_f(p, "mass")?,
            get_f(p, "v_perp")?,
            get_f(p, "charge")?,
            get_f(p, "b")?,
        ))),
        "magnetic_field_loop" => Ok(RunOutput::Scalar(
            phys::electrodynamics::fields::magnetic_field_loop(
                get_f(p, "current")?,
                get_f(p, "radius")?,
                get_f(p, "z")?,
            ),
        )),
        "magnetic_field_solenoid" => Ok(RunOutput::Scalar(
            phys::electrodynamics::fields::magnetic_field_solenoid(
                get_f(p, "n_per_length")?,
                get_f(p, "current")?,
            ),
        )),
        "magnetic_field_wire" => Ok(RunOutput::Scalar(
            phys::electrodynamics::fields::magnetic_field_wire(get_f(p, "current")?, get_f(p, "r")?),
        )),
        "mutual_inductance_coupling" => Ok(RunOutput::Scalar(
            phys::electrodynamics::circuits::mutual_inductance_coupling(
                get_f(p, "k")?,
                get_f(p, "l1")?,
                get_f(p, "l2")?,
            ),
        )),
        "parallel_capacitance" => Ok(RunOutput::Scalar(
            phys::electrodynamics::circuits::parallel_capacitance(get_v(p, "capacitances")?),
        )),
        "electrodynamics::parallel_resistance" => Ok(RunOutput::Scalar(
            phys::electrodynamics::circuits::parallel_resistance(get_v(p, "resistances")?),
        )),
        "phase_velocity" => Ok(RunOutput::Scalar(phys::electrodynamics::waves::phase_velocity(
            get_f(p, "epsilon_r")?,
            get_f(p, "mu_r")?,
        ))),
        "plasma_frequency" => Ok(RunOutput::Scalar(phys::electrodynamics::fields::plasma_frequency(
            get_f(p, "number_density")?,
            get_f(p, "mass")?,
            get_f(p, "charge")?,
        ))),
        "power_dissipated" => Ok(RunOutput::Scalar(phys::electrodynamics::circuits::power_dissipated(
            get_f(p, "v")?,
            get_f(p, "r")?,
        ))),
        "radiation_pressure_absorbed" => Ok(RunOutput::Scalar(
            phys::electrodynamics::waves::radiation_pressure_absorbed(get_f(p, "intensity")?),
        )),
        "radiation_pressure_reflected" => Ok(RunOutput::Scalar(
            phys::electrodynamics::waves::radiation_pressure_reflected(get_f(p, "intensity")?),
        )),
        "electrodynamics::rc_charging" => {
            Ok(RunOutput::Scalar(phys::electrodynamics::circuits::rc_charging(
                get_f(p, "v_source")?,
                get_f(p, "r")?,
                get_f(p, "c")?,
                get_f(p, "t")?,
            )))
        }
        "electrodynamics::rc_discharging" => {
            Ok(RunOutput::Scalar(phys::electrodynamics::circuits::rc_discharging(
                get_f(p, "v0")?,
                get_f(p, "r")?,
                get_f(p, "c")?,
                get_f(p, "t")?,
            )))
        }
        "rc_time_constant" => Ok(RunOutput::Scalar(phys::electrodynamics::circuits::rc_time_constant(
            get_f(p, "r")?,
            get_f(p, "c")?,
        ))),
        "rl_time_constant" => Ok(RunOutput::Scalar(phys::electrodynamics::circuits::rl_time_constant(
            get_f(p, "r")?,
            get_f(p, "l")?,
        ))),
        "series_capacitance" => Ok(RunOutput::Scalar(
            phys::electrodynamics::circuits::series_capacitance(get_v(p, "capacitances")?),
        )),
        "electrodynamics::series_resistance" => Ok(RunOutput::Scalar(
            phys::electrodynamics::circuits::series_resistance(get_v(p, "resistances")?),
        )),
        "skin_depth" => Ok(RunOutput::Scalar(phys::electrodynamics::waves::skin_depth(
            get_f(p, "frequency")?,
            get_f(p, "conductivity")?,
            get_f(p, "mu_r")?,
        ))),
        "electrodynamics::snell" => Ok(RunOutput::Scalar(phys::electrodynamics::waves::snell(
            get_f(p, "n1")?,
            get_f(p, "theta1")?,
            get_f(p, "n2")?,
        ))),
        "transformer_ratio" => Ok(RunOutput::Scalar(phys::electrodynamics::circuits::transformer_ratio(
            get_f(p, "n_primary")?,
            get_f(p, "n_secondary")?,
            get_f(p, "v_primary")?,
        ))),
        "electrodynamics::voltage_divider" => {
            Ok(RunOutput::Scalar(phys::electrodynamics::circuits::voltage_divider(
                get_f(p, "v_in")?,
                get_f(p, "r1")?,
                get_f(p, "r2")?,
            )))
        }
        "wave_impedance_free_space" => Ok(RunOutput::Scalar(
            phys::electrodynamics::waves::wave_impedance_free_space(),
        )),
        "electrodynamics::wave_number" => Ok(RunOutput::Scalar(
            phys::electrodynamics::waves::wave_number(get_f(p, "frequency")?),
        )),
        "electrodynamics::wavelength" => Ok(RunOutput::Scalar(phys::electrodynamics::waves::wavelength(
            get_f(p, "frequency")?,
        ))),
        "wheatstone_bridge_balance" => Ok(RunOutput::Scalar(
            phys::electrodynamics::circuits::wheatstone_bridge_balance(
                get_f(p, "r1")?,
                get_f(p, "r2")?,
                get_f(p, "r3")?,
            ),
        )),
        "biot_savart_segment" => {
            let dl = get_v(p, "dl")?;
            let rv = get_v(p, "r")?;
            let r = phys::electrodynamics::fields::biot_savart_segment(
                get_f(p, "current")?,
                [dl[0], dl[1], dl[2]],
                [rv[0], rv[1], rv[2]],
            );
            Ok(RunOutput::Triple(r[0], r[1], r[2]))
        }
        "electric_dipole_field" => {
            let pv = get_v(p, "p")?;
            let rv = get_v(p, "r")?;
            let r = phys::electrodynamics::fields::electric_dipole_field(
                [pv[0], pv[1], pv[2]],
                [rv[0], rv[1], rv[2]],
            );
            Ok(RunOutput::Triple(r[0], r[1], r[2]))
        }
        "electric_field_point_charge" => {
            let rv = get_v(p, "r")?;
            let r = phys::electrodynamics::fields::electric_field_point_charge(
                get_f(p, "q")?,
                [rv[0], rv[1], rv[2]],
            );
            Ok(RunOutput::Triple(r[0], r[1], r[2]))
        }
        "energy_density_em" => {
            let e = get_v(p, "e")?;
            let b = get_v(p, "b")?;
            Ok(RunOutput::Scalar(phys::electrodynamics::fields::energy_density_em(
                [e[0], e[1], e[2]],
                [b[0], b[1], b[2]],
            )))
        }
        "fdtd_1d" => {
            let mut ez = get_v(p, "ez")?.to_vec();
            let mut hy = get_v(p, "hy")?.to_vec();
            phys::electrodynamics::waves::fdtd_1d(&mut ez, &mut hy, get_u(p, "steps")?);
            Ok(RunOutput::Matrix(vec![ez, hy]))
        }
        "lorentz_force" => {
            let v = get_v(p, "v")?;
            let e = get_v(p, "e")?;
            let b = get_v(p, "b")?;
            let r = phys::electrodynamics::fields::lorentz_force(
                get_f(p, "q")?,
                [v[0], v[1], v[2]],
                [e[0], e[1], e[2]],
                [b[0], b[1], b[2]],
            );
            Ok(RunOutput::Triple(r[0], r[1], r[2]))
        }
        "magnetic_dipole_field" => {
            let mv = get_v(p, "m")?;
            let rv = get_v(p, "r")?;
            let r = phys::electrodynamics::fields::magnetic_dipole_field(
                [mv[0], mv[1], mv[2]],
                [rv[0], rv[1], rv[2]],
            );
            Ok(RunOutput::Triple(r[0], r[1], r[2]))
        }
        "poynting_vector" => {
            let e = get_v(p, "e")?;
            let b = get_v(p, "b")?;
            let r = phys::electrodynamics::fields::poynting_vector([e[0], e[1], e[2]], [b[0], b[1], b[2]]);
            Ok(RunOutput::Triple(r[0], r[1], r[2]))
        }
        "waveguide_cutoff_te" => Ok(RunOutput::Scalar(
            phys::electrodynamics::waves::waveguide_cutoff_te(
                get_i(p, "m")? as u32,
                get_i(p, "n")? as u32,
                get_f(p, "a")?,
                get_f(p, "b")?,
            ),
        )),
        _ => Err(HubError::InvalidInput(format!("unknown function: {func}"))),
    }
}
