//! Dispatch handler for fluid mechanics functions.

use super::super::params::*;
use crate::domain::common::errors::{HubError, HubResult};
use crate::domain::physics as phys;
use crate::engine::experience::runner::RunOutput;

pub(super) fn dispatch(func: &str, p: &Params) -> HubResult<RunOutput> {
    match func {
        "bernoulli_height" => Ok(RunOutput::Scalar(
            phys::fluid_mechanics::flow::bernoulli_height(
                get_f(p, "rho")?,
                get_f(p, "v1")?,
                get_f(p, "p1")?,
                get_f(p, "h1")?,
                get_f(p, "v2")?,
                get_f(p, "p2")?,
                get_f(p, "g")?,
            ),
        )),
        "bernoulli_pressure" => Ok(RunOutput::Scalar(
            phys::fluid_mechanics::flow::bernoulli_pressure(
                get_f(p, "rho")?,
                get_f(p, "v1")?,
                get_f(p, "p1")?,
                get_f(p, "v2")?,
            ),
        )),
        "blasius_thickness" => Ok(RunOutput::Scalar(
            phys::fluid_mechanics::boundary_layer::blasius_thickness(
                get_f(p, "x")?,
                get_f(p, "re_x")?,
            ),
        )),
        "capillary_number" => Ok(RunOutput::Scalar(
            phys::fluid_mechanics::waves::capillary_number(
                get_f(p, "mu")?,
                get_f(p, "v")?,
                get_f(p, "sigma")?,
            ),
        )),
        "continuity_velocity" => Ok(RunOutput::Scalar(
            phys::fluid_mechanics::flow::continuity_velocity(
                get_f(p, "a1")?,
                get_f(p, "v1")?,
                get_f(p, "a2")?,
            ),
        )),
        "darcy_weisbach" => Ok(RunOutput::Scalar(
            phys::fluid_mechanics::flow::darcy_weisbach(
                get_f(p, "f")?,
                get_f(p, "l")?,
                get_f(p, "d")?,
                get_f(p, "rho")?,
                get_f(p, "v")?,
            ),
        )),
        "deep_water_speed" => Ok(RunOutput::Scalar(
            phys::fluid_mechanics::waves::deep_water_speed(get_f(p, "g")?, get_f(p, "wavelength")?),
        )),
        "displacement_thickness" => Ok(RunOutput::Scalar(
            phys::fluid_mechanics::boundary_layer::displacement_thickness(
                get_f(p, "x")?,
                get_f(p, "re_x")?,
            ),
        )),
        "drag_force" => Ok(RunOutput::Scalar(phys::fluid_mechanics::flow::drag_force(
            get_f(p, "cd")?,
            get_f(p, "rho")?,
            get_f(p, "v")?,
            get_f(p, "a")?,
        ))),
        "eddy_viscosity" => Ok(RunOutput::Scalar(
            phys::fluid_mechanics::turbulence::eddy_viscosity(
                get_f(p, "mixing_length")?,
                get_f(p, "du_dy")?,
            ),
        )),
        "energy_spectrum_kolmogorov" => Ok(RunOutput::Scalar(
            phys::fluid_mechanics::turbulence::energy_spectrum_kolmogorov(
                get_f(p, "c_k")?,
                get_f(p, "epsilon")?,
                get_f(p, "k")?,
            ),
        )),
        "falkner_skan_beta" => Ok(RunOutput::Scalar(
            phys::fluid_mechanics::boundary_layer::falkner_skan_beta(get_f(p, "m")?),
        )),
        "friction_velocity" => Ok(RunOutput::Scalar(
            phys::fluid_mechanics::turbulence::friction_velocity(
                get_f(p, "tau_wall")?,
                get_f(p, "rho")?,
            ),
        )),
        "froude_number" => Ok(RunOutput::Scalar(
            phys::fluid_mechanics::waves::froude_number(
                get_f(p, "v")?,
                get_f(p, "g")?,
                get_f(p, "depth")?,
            ),
        )),
        "hagen_poiseuille" => Ok(RunOutput::Scalar(
            phys::fluid_mechanics::flow::hagen_poiseuille(
                get_f(p, "delta_p")?,
                get_f(p, "r")?,
                get_f(p, "l")?,
                get_f(p, "mu")?,
            ),
        )),
        "hydraulic_diameter" => Ok(RunOutput::Scalar(
            phys::fluid_mechanics::flow::hydraulic_diameter(
                get_f(p, "area")?,
                get_f(p, "perimeter")?,
            ),
        )),
        "integral_length_scale" => Ok(RunOutput::Scalar(
            phys::fluid_mechanics::turbulence::integral_length_scale(
                get_f(p, "tke")?,
                get_f(p, "epsilon")?,
            ),
        )),
        "kolmogorov_length_scale" => Ok(RunOutput::Scalar(
            phys::fluid_mechanics::turbulence::kolmogorov_length_scale(
                get_f(p, "nu")?,
                get_f(p, "epsilon")?,
            ),
        )),
        "kolmogorov_time_scale" => Ok(RunOutput::Scalar(
            phys::fluid_mechanics::turbulence::kolmogorov_time_scale(
                get_f(p, "nu")?,
                get_f(p, "epsilon")?,
            ),
        )),
        "kolmogorov_velocity_scale" => Ok(RunOutput::Scalar(
            phys::fluid_mechanics::turbulence::kolmogorov_velocity_scale(
                get_f(p, "nu")?,
                get_f(p, "epsilon")?,
            ),
        )),
        "law_of_wall" => Ok(RunOutput::Scalar(
            phys::fluid_mechanics::turbulence::law_of_wall(
                get_f(p, "u_tau")?,
                get_f(p, "y")?,
                get_f(p, "nu")?,
            ),
        )),
        "lift_force" => Ok(RunOutput::Scalar(phys::fluid_mechanics::flow::lift_force(
            get_f(p, "cl")?,
            get_f(p, "rho")?,
            get_f(p, "v")?,
            get_f(p, "a")?,
        ))),
        "mach_number" => Ok(RunOutput::Scalar(
            phys::fluid_mechanics::waves::mach_number(get_f(p, "v")?, get_f(p, "c")?),
        )),
        "mixing_length" => Ok(RunOutput::Scalar(
            phys::fluid_mechanics::turbulence::mixing_length(get_f(p, "kappa")?, get_f(p, "y")?),
        )),
        "momentum_thickness" => Ok(RunOutput::Scalar(
            phys::fluid_mechanics::boundary_layer::momentum_thickness(
                get_f(p, "x")?,
                get_f(p, "re_x")?,
            ),
        )),
        "nusselt_flat_plate_laminar" => Ok(RunOutput::Scalar(
            phys::fluid_mechanics::boundary_layer::nusselt_flat_plate_laminar(
                get_f(p, "re")?,
                get_f(p, "pr")?,
            ),
        )),
        "reynolds_number" => Ok(RunOutput::Scalar(
            phys::fluid_mechanics::flow::reynolds_number(
                get_f(p, "rho")?,
                get_f(p, "v")?,
                get_f(p, "l")?,
                get_f(p, "mu")?,
            ),
        )),
        "separation_criterion" => Ok(RunOutput::Boolean(
            phys::fluid_mechanics::boundary_layer::separation_criterion(get_f(p, "dp_dx")?),
        )),
        "shallow_water_speed" => Ok(RunOutput::Scalar(
            phys::fluid_mechanics::waves::shallow_water_speed(get_f(p, "g")?, get_f(p, "depth")?),
        )),
        "shape_factor" => Ok(RunOutput::Scalar(
            phys::fluid_mechanics::boundary_layer::shape_factor(
                get_f(p, "displacement")?,
                get_f(p, "momentum")?,
            ),
        )),
        "skin_friction_laminar" => Ok(RunOutput::Scalar(
            phys::fluid_mechanics::boundary_layer::skin_friction_laminar(get_f(p, "re_x")?),
        )),
        "skin_friction_turbulent" => Ok(RunOutput::Scalar(
            phys::fluid_mechanics::boundary_layer::skin_friction_turbulent(get_f(p, "re_x")?),
        )),
        "sound_speed_ideal_gas" => Ok(RunOutput::Scalar(
            phys::fluid_mechanics::waves::sound_speed_ideal_gas(
                get_f(p, "gamma")?,
                get_f(p, "r")?,
                get_f(p, "t")?,
                get_f(p, "m")?,
            ),
        )),
        "stanton_number" => Ok(RunOutput::Scalar(
            phys::fluid_mechanics::boundary_layer::stanton_number(
                get_f(p, "nu")?,
                get_f(p, "re")?,
                get_f(p, "pr")?,
            ),
        )),
        "stokes_drag" => Ok(RunOutput::Scalar(phys::fluid_mechanics::flow::stokes_drag(
            get_f(p, "mu")?,
            get_f(p, "r")?,
            get_f(p, "v")?,
        ))),
        "taylor_microscale" => Ok(RunOutput::Scalar(
            phys::fluid_mechanics::turbulence::taylor_microscale(
                get_f(p, "u_rms")?,
                get_f(p, "epsilon")?,
                get_f(p, "nu")?,
            ),
        )),
        "terminal_velocity_sphere" => Ok(RunOutput::Scalar(
            phys::fluid_mechanics::flow::terminal_velocity_sphere(
                get_f(p, "rho_p")?,
                get_f(p, "rho_f")?,
                get_f(p, "r")?,
                get_f(p, "mu")?,
                get_f(p, "g")?,
            ),
        )),
        "thermal_bl_thickness" => Ok(RunOutput::Scalar(
            phys::fluid_mechanics::boundary_layer::thermal_bl_thickness(
                get_f(p, "delta")?,
                get_f(p, "pr")?,
            ),
        )),
        "torricelli" => Ok(RunOutput::Scalar(phys::fluid_mechanics::flow::torricelli(
            get_f(p, "g")?,
            get_f(p, "h")?,
        ))),
        "turbulence_intensity" => Ok(RunOutput::Scalar(
            phys::fluid_mechanics::turbulence::turbulence_intensity(
                get_f(p, "u_rms")?,
                get_f(p, "u_mean")?,
            ),
        )),
        "turbulent_bl_thickness" => Ok(RunOutput::Scalar(
            phys::fluid_mechanics::boundary_layer::turbulent_bl_thickness(
                get_f(p, "x")?,
                get_f(p, "re_x")?,
            ),
        )),
        "turbulent_kinetic_energy" => Ok(RunOutput::Scalar(
            phys::fluid_mechanics::turbulence::turbulent_kinetic_energy(
                get_f(p, "u_prime")?,
                get_f(p, "v_prime")?,
                get_f(p, "w_prime")?,
            ),
        )),
        "water_hammer_pressure" => Ok(RunOutput::Scalar(
            phys::fluid_mechanics::waves::water_hammer_pressure(
                get_f(p, "rho")?,
                get_f(p, "c")?,
                get_f(p, "delta_v")?,
            ),
        )),
        "wave_energy_density" => Ok(RunOutput::Scalar(
            phys::fluid_mechanics::waves::wave_energy_density(
                get_f(p, "rho")?,
                get_f(p, "g")?,
                get_f(p, "amplitude")?,
            ),
        )),
        "wave_frequency" => Ok(RunOutput::Scalar(
            phys::fluid_mechanics::waves::wave_frequency(get_f(p, "period")?),
        )),
        "fluid_mechanics::waves::wave_number" => Ok(RunOutput::Scalar(
            phys::fluid_mechanics::waves::wave_number(get_f(p, "wavelength")?),
        )),
        "weber_number" => Ok(RunOutput::Scalar(
            phys::fluid_mechanics::waves::weber_number(
                get_f(p, "rho")?,
                get_f(p, "v")?,
                get_f(p, "l")?,
                get_f(p, "sigma")?,
            ),
        )),
        _ => Err(HubError::InvalidInput(format!("unknown function: {func}"))),
    }
}
