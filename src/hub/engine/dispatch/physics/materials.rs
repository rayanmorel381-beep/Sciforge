//! Dispatch handler for materials science functions.

use super::super::params::*;
use crate::hub::domain::common::errors::{HubError, HubResult};
use crate::hub::domain::physics as phys;
use crate::hub::engine::experience::runner::RunOutput;

pub(super) fn dispatch(func: &str, p: &Params) -> HubResult<RunOutput> {
    match func {
        "band_gap_temperature" => Ok(RunOutput::Scalar(
            phys::materials::semiconductors::band_gap_temperature(
                get_f(p, "eg0")?,
                get_f(p, "alpha")?,
                get_f(p, "beta")?,
                get_f(p, "t")?,
            ),
        )),
        "bragg_angle" => Ok(RunOutput::Scalar(
            phys::materials::crystallography::bragg_angle(
                get_f(p, "d")?,
                get_f(p, "wavelength")?,
                get_i(p, "n")? as i32,
            ),
        )),
        "built_in_potential" => Ok(RunOutput::Scalar(
            phys::materials::semiconductors::built_in_potential(
                get_f(p, "na")?,
                get_f(p, "nd")?,
                get_f(p, "ni")?,
                get_f(p, "t")?,
            ),
        )),
        "carburization_depth" => Ok(RunOutput::Scalar(
            phys::materials::diffusion::carburization_depth(get_f(p, "d")?, get_f(p, "t")?),
        )),
        "clausius_clapeyron_slope" => Ok(RunOutput::Scalar(
            phys::materials::phases::clausius_clapeyron_slope(
                get_f(p, "delta_h")?,
                get_f(p, "t")?,
                get_f(p, "delta_v")?,
            ),
        )),
        "coarsening_rate" => Ok(RunOutput::Scalar(phys::materials::phases::coarsening_rate(
            get_f(p, "k")?,
            get_f(p, "t")?,
            get_f(p, "t0")?,
        ))),
        "conductivity_semiconductor" => Ok(RunOutput::Scalar(
            phys::materials::semiconductors::conductivity_semiconductor(
                get_f(p, "n")?,
                get_f(p, "mu_e")?,
                get_f(p, "p")?,
                get_f(p, "mu_h")?,
            ),
        )),
        "critical_nucleus_radius" => Ok(RunOutput::Scalar(
            phys::materials::phases::critical_nucleus_radius(
                get_f(p, "gamma")?,
                get_f(p, "delta_gv")?,
            ),
        )),
        "d_spacing_cubic" => Ok(RunOutput::Scalar(
            phys::materials::crystallography::d_spacing_cubic(
                get_f(p, "a")?,
                get_i(p, "h")? as i32,
                get_i(p, "k")? as i32,
                get_i(p, "l")? as i32,
            ),
        )),
        "debye_temperature" => Ok(RunOutput::Scalar(
            phys::materials::crystallography::debye_temperature(
                get_f(p, "theta_d")?,
                get_f(p, "t")?,
            ),
        )),
        "depletion_width" => Ok(RunOutput::Scalar(
            phys::materials::semiconductors::depletion_width(
                get_f(p, "epsilon")?,
                get_f(p, "v_bi")?,
                get_f(p, "na")?,
                get_f(p, "nd")?,
            ),
        )),
        "diffusion_coefficient" => Ok(RunOutput::Scalar(
            phys::materials::diffusion::diffusion_coefficient(
                get_f(p, "d0")?,
                get_f(p, "q")?,
                get_f(p, "t")?,
            ),
        )),
        "diffusion_length" => Ok(RunOutput::Scalar(
            phys::materials::diffusion::diffusion_length(get_f(p, "d")?, get_f(p, "t")?),
        )),
        "diode_current" => Ok(RunOutput::Scalar(
            phys::materials::semiconductors::diode_current(
                get_f(p, "is")?,
                get_f(p, "v")?,
                get_f(p, "t")?,
                get_f(p, "n")?,
            ),
        )),
        "doping_ionization" => Ok(RunOutput::Scalar(
            phys::materials::semiconductors::doping_ionization(
                get_f(p, "nd")?,
                get_f(p, "ed")?,
                get_f(p, "t")?,
            ),
        )),
        "drift_velocity" => Ok(RunOutput::Scalar(
            phys::materials::semiconductors::drift_velocity(get_f(p, "mu")?, get_f(p, "e_field")?),
        )),
        "materials::semiconductors::fermi_dirac" => Ok(RunOutput::Scalar(
            phys::materials::semiconductors::fermi_dirac(
                get_f(p, "e")?,
                get_f(p, "ef")?,
                get_f(p, "t")?,
            ),
        )),
        "materials::semiconductors::fermi_energy" => Ok(RunOutput::Scalar(
            phys::materials::semiconductors::fermi_energy(get_f(p, "n")?, get_f(p, "m_eff")?),
        )),
        "fick_first_law" => Ok(RunOutput::Scalar(
            phys::materials::diffusion::fick_first_law(get_f(p, "d")?, get_f(p, "dc_dx")?),
        )),
        "fick_second_law_solution" => Ok(RunOutput::Scalar(
            phys::materials::diffusion::fick_second_law_solution(
                get_f(p, "c0")?,
                get_f(p, "cs")?,
                get_f(p, "x")?,
                get_f(p, "d")?,
                get_f(p, "t")?,
            ),
        )),
        "gibbs_phase_rule" => Ok(RunOutput::Scalar(
            phys::materials::phases::gibbs_phase_rule(get_i(p, "c")? as u32, get_i(p, "p")? as u32)
                as f64,
        )),
        "grain_boundary_diffusion" => Ok(RunOutput::Scalar(
            phys::materials::diffusion::grain_boundary_diffusion(
                get_f(p, "d_gb")?,
                get_f(p, "delta")?,
                get_f(p, "d_l")?,
                get_f(p, "grain_size")?,
            ),
        )),
        "hall_coefficient" => Ok(RunOutput::Scalar(
            phys::materials::semiconductors::hall_coefficient(
                get_f(p, "n")?,
                get_f(p, "p")?,
                get_f(p, "mu_e")?,
                get_f(p, "mu_h")?,
            ),
        )),
        "interdiffusion_coefficient" => Ok(RunOutput::Scalar(
            phys::materials::diffusion::interdiffusion_coefficient(
                get_f(p, "d_a")?,
                get_f(p, "d_b")?,
                get_f(p, "x_a")?,
            ),
        )),
        "intrinsic_carrier_concentration" => Ok(RunOutput::Scalar(
            phys::materials::semiconductors::intrinsic_carrier_concentration(
                get_f(p, "eg")?,
                get_f(p, "t")?,
                get_f(p, "nc")?,
                get_f(p, "nv")?,
            ),
        )),
        "jmak" => Ok(RunOutput::Scalar(phys::materials::phases::jmak(
            get_f(p, "k")?,
            get_f(p, "t")?,
            get_f(p, "n")?,
        ))),
        "kirkendall_velocity" => Ok(RunOutput::Scalar(
            phys::materials::diffusion::kirkendall_velocity(
                get_f(p, "d_a")?,
                get_f(p, "d_b")?,
                get_f(p, "dc_dx")?,
                get_f(p, "c_total")?,
            ),
        )),
        "lattice_parameter_from_density" => Ok(RunOutput::Scalar(
            phys::materials::crystallography::lattice_parameter_from_density(
                get_f(p, "m")?,
                get_f(p, "z")?,
                get_f(p, "rho")?,
            ),
        )),
        "lever_rule" => {
            let r = phys::materials::phases::lever_rule(
                get_f(p, "c0")?,
                get_f(p, "c_alpha")?,
                get_f(p, "c_beta")?,
            );
            Ok(RunOutput::Pair(r.0, r.1))
        }
        "materials::diffusion::mean_free_path" => Ok(RunOutput::Scalar(
            phys::materials::diffusion::mean_free_path(get_f(p, "d")?, get_f(p, "n_density")?),
        )),
        "miller_planar_density_cubic" => Ok(RunOutput::Scalar(
            phys::materials::crystallography::miller_planar_density_cubic(
                get_f(p, "a")?,
                get_f(p, "atoms_per_plane")?,
                get_i(p, "h")? as i32,
                get_i(p, "k")? as i32,
                get_i(p, "l")? as i32,
            ),
        )),
        "nucleation_barrier" => Ok(RunOutput::Scalar(
            phys::materials::phases::nucleation_barrier(get_f(p, "gamma")?, get_f(p, "delta_gv")?),
        )),
        "nucleation_rate" => Ok(RunOutput::Scalar(phys::materials::phases::nucleation_rate(
            get_f(p, "n0")?,
            get_f(p, "delta_g_star")?,
            get_f(p, "t")?,
        ))),
        "packing_fraction_bcc" => Ok(RunOutput::Scalar(
            phys::materials::crystallography::packing_fraction_bcc(),
        )),
        "packing_fraction_fcc" => Ok(RunOutput::Scalar(
            phys::materials::crystallography::packing_fraction_fcc(),
        )),
        "partition_coefficient" => Ok(RunOutput::Scalar(
            phys::materials::phases::partition_coefficient(
                get_f(p, "c_solid")?,
                get_f(p, "c_liquid")?,
            ),
        )),
        "permeability" => Ok(RunOutput::Scalar(phys::materials::diffusion::permeability(
            get_f(p, "d")?,
            get_f(p, "s")?,
        ))),
        "regular_solution_gibbs" => Ok(RunOutput::Scalar(
            phys::materials::phases::regular_solution_gibbs(
                get_f(p, "xa")?,
                get_f(p, "omega")?,
                get_f(p, "t")?,
            ),
        )),
        "scheil_equation" => Ok(RunOutput::Scalar(phys::materials::phases::scheil_equation(
            get_f(p, "c0")?,
            get_f(p, "k")?,
            get_f(p, "fs")?,
        ))),
        "specific_heat_debye" => Ok(RunOutput::Scalar(
            phys::materials::crystallography::specific_heat_debye(
                get_f(p, "t")?,
                get_f(p, "theta_d")?,
            ),
        )),
        "spinodal_temperature" => Ok(RunOutput::Scalar(
            phys::materials::phases::spinodal_temperature(get_f(p, "omega")?),
        )),
        "structure_factor_bcc" => Ok(RunOutput::Scalar(
            phys::materials::crystallography::structure_factor_bcc(
                get_i(p, "h")? as i32,
                get_i(p, "k")? as i32,
                get_i(p, "l")? as i32,
            ),
        )),
        "structure_factor_fcc" => Ok(RunOutput::Scalar(
            phys::materials::crystallography::structure_factor_fcc(
                get_i(p, "h")? as i32,
                get_i(p, "k")? as i32,
                get_i(p, "l")? as i32,
            ),
        )),
        "vacancy_concentration" => Ok(RunOutput::Scalar(
            phys::materials::crystallography::vacancy_concentration(
                get_f(p, "ev")?,
                get_f(p, "t")?,
            ),
        )),
        _ => Err(HubError::InvalidInput(format!("unknown function: {func}"))),
    }
}
