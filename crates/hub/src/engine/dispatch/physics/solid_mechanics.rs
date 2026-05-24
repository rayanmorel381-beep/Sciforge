//! Dispatch handler for solid mechanics functions.

use super::super::params::*;
use crate::domain::common::errors::{HubError, HubResult};
use crate::domain::physics as phys;
use crate::engine::experience::runner::RunOutput;

pub(super) fn dispatch(func: &str, p: &Params) -> HubResult<RunOutput> {
    match func {
        "bauschinger_effect" => Ok(RunOutput::Scalar(
            phys::solid_mechanics::plasticity::bauschinger_effect(
                get_f(p, "forward_yield")?,
                get_f(p, "reverse_yield")?,
            ),
        )),
        "beam_bending_stress" => Ok(RunOutput::Scalar(
            phys::solid_mechanics::stress::beam_bending_stress(
                get_f(p, "m")?,
                get_f(p, "y")?,
                get_f(p, "i")?,
            ),
        )),
        "beam_deflection_cantilever" => Ok(RunOutput::Scalar(
            phys::solid_mechanics::stress::beam_deflection_cantilever(
                get_f(p, "p")?,
                get_f(p, "l")?,
                get_f(p, "e")?,
                get_f(p, "i")?,
            ),
        )),
        "bulk_modulus" => Ok(RunOutput::Scalar(
            phys::solid_mechanics::elasticity::bulk_modulus(get_f(p, "e")?, get_f(p, "nu")?),
        )),
        "column_euler_buckling" => Ok(RunOutput::Scalar(
            phys::solid_mechanics::stress::column_euler_buckling(
                get_f(p, "e")?,
                get_f(p, "i")?,
                get_f(p, "l")?,
            ),
        )),
        "crack_opening_displacement" => Ok(RunOutput::Scalar(
            phys::solid_mechanics::fracture::crack_opening_displacement(
                get_f(p, "k")?,
                get_f(p, "sigma_y")?,
                get_f(p, "e")?,
            ),
        )),
        "crack_tip_plastic_zone" => Ok(RunOutput::Scalar(
            phys::solid_mechanics::fracture::crack_tip_plastic_zone(
                get_f(p, "k")?,
                get_f(p, "sigma_y")?,
            ),
        )),
        "deviatoric_stress" => {
            let r = phys::solid_mechanics::stress::deviatoric_stress(
                get_f(p, "sx")?,
                get_f(p, "sy")?,
                get_f(p, "sz")?,
            );
            Ok(RunOutput::Triple(r.0, r.1, r.2))
        }
        "energy_release_rate" => Ok(RunOutput::Scalar(
            phys::solid_mechanics::fracture::energy_release_rate(get_f(p, "k")?, get_f(p, "e")?),
        )),
        "fatigue_life_basquin" => Ok(RunOutput::Scalar(
            phys::solid_mechanics::fracture::fatigue_life_basquin(
                get_f(p, "sigma_f")?,
                get_f(p, "sigma_a")?,
                get_f(p, "b")?,
            ),
        )),
        "fatigue_life_coffin_manson" => Ok(RunOutput::Scalar(
            phys::solid_mechanics::fracture::fatigue_life_coffin_manson(
                get_f(p, "ef")?,
                get_f(p, "ea")?,
                get_f(p, "c")?,
            ),
        )),
        "fracture_toughness_plane_strain" => Ok(RunOutput::Scalar(
            phys::solid_mechanics::fracture::fracture_toughness_plane_strain(
                get_f(p, "kic")?,
                get_f(p, "sigma_y")?,
            ),
        )),
        "griffith_critical_stress" => Ok(RunOutput::Scalar(
            phys::solid_mechanics::fracture::griffith_critical_stress(
                get_f(p, "e")?,
                get_f(p, "gamma")?,
                get_f(p, "a")?,
            ),
        )),
        "hardening_power_law" => Ok(RunOutput::Scalar(
            phys::solid_mechanics::plasticity::hardening_power_law(
                get_f(p, "k")?,
                get_f(p, "strain_plastic")?,
                get_f(p, "n")?,
            ),
        )),
        "hertz_contact_pressure" => Ok(RunOutput::Scalar(
            phys::solid_mechanics::stress::hertz_contact_pressure(
                get_f(p, "force")?,
                get_f(p, "r1")?,
                get_f(p, "r2")?,
                get_f(p, "e_star")?,
            ),
        )),
        "hooke_strain" => Ok(RunOutput::Scalar(
            phys::solid_mechanics::elasticity::hooke_strain(get_f(p, "stress")?, get_f(p, "e")?),
        )),
        "hooke_stress" => Ok(RunOutput::Scalar(
            phys::solid_mechanics::elasticity::hooke_stress(get_f(p, "e")?, get_f(p, "strain")?),
        )),
        "hydrostatic_stress" => Ok(RunOutput::Scalar(
            phys::solid_mechanics::elasticity::hydrostatic_stress(
                get_f(p, "sx")?,
                get_f(p, "sy")?,
                get_f(p, "sz")?,
            ),
        )),
        "isotropic_hardening" => Ok(RunOutput::Scalar(
            phys::solid_mechanics::plasticity::isotropic_hardening(
                get_f(p, "yield_0")?,
                get_f(p, "h")?,
                get_f(p, "plastic_strain")?,
            ),
        )),
        "j_integral" => Ok(RunOutput::Scalar(
            phys::solid_mechanics::fracture::j_integral(get_f(p, "energy_release")?),
        )),
        "lame_first" => Ok(RunOutput::Scalar(
            phys::solid_mechanics::elasticity::lame_first(get_f(p, "e")?, get_f(p, "nu")?),
        )),
        "max_shear_stress_2d" => Ok(RunOutput::Scalar(
            phys::solid_mechanics::stress::max_shear_stress_2d(
                get_f(p, "sx")?,
                get_f(p, "sy")?,
                get_f(p, "txy")?,
            ),
        )),
        "miners_rule" => Ok(RunOutput::Scalar(
            phys::solid_mechanics::fracture::miners_rule(
                get_v(p, "cycles")?,
                get_v(p, "max_cycles")?,
            ),
        )),
        "mohr_circle_center" => Ok(RunOutput::Scalar(
            phys::solid_mechanics::stress::mohr_circle_center(get_f(p, "sx")?, get_f(p, "sy")?),
        )),
        "mohr_circle_radius" => Ok(RunOutput::Scalar(
            phys::solid_mechanics::stress::mohr_circle_radius(
                get_f(p, "sx")?,
                get_f(p, "sy")?,
                get_f(p, "txy")?,
            ),
        )),
        "necking_criterion" => Ok(RunOutput::Scalar(
            phys::solid_mechanics::plasticity::necking_criterion(get_f(p, "n")?),
        )),
        "paris_law" => Ok(RunOutput::Scalar(
            phys::solid_mechanics::fracture::paris_law(
                get_f(p, "c")?,
                get_f(p, "delta_k")?,
                get_f(p, "m")?,
            ),
        )),
        "plane_stress_strain" => {
            let r = phys::solid_mechanics::elasticity::plane_stress_strain(
                get_f(p, "stress_x")?,
                get_f(p, "stress_y")?,
                get_f(p, "e")?,
                get_f(p, "nu")?,
            );
            Ok(RunOutput::Pair(r.0, r.1))
        }
        "plastic_work" => Ok(RunOutput::Scalar(
            phys::solid_mechanics::plasticity::plastic_work(
                get_v(p, "stress")?,
                get_v(p, "d_plastic_strain")?,
            ),
        )),
        "poisson_lateral_strain" => Ok(RunOutput::Scalar(
            phys::solid_mechanics::elasticity::poisson_lateral_strain(
                get_f(p, "axial_strain")?,
                get_f(p, "nu")?,
            ),
        )),
        "principal_stresses_2d" => {
            let r = phys::solid_mechanics::stress::principal_stresses_2d(
                get_f(p, "sx")?,
                get_f(p, "sy")?,
                get_f(p, "txy")?,
            );
            Ok(RunOutput::Pair(r.0, r.1))
        }
        "ramberg_osgood" => Ok(RunOutput::Scalar(
            phys::solid_mechanics::plasticity::ramberg_osgood(
                get_f(p, "stress")?,
                get_f(p, "e")?,
                get_f(p, "k")?,
                get_f(p, "n")?,
            ),
        )),
        "shear_modulus" => Ok(RunOutput::Scalar(
            phys::solid_mechanics::elasticity::shear_modulus(get_f(p, "e")?, get_f(p, "nu")?),
        )),
        "strain_energy_density" => Ok(RunOutput::Scalar(
            phys::solid_mechanics::elasticity::strain_energy_density(
                get_f(p, "stress")?,
                get_f(p, "strain")?,
            ),
        )),
        "strain_rate_sensitivity" => Ok(RunOutput::Scalar(
            phys::solid_mechanics::plasticity::strain_rate_sensitivity(
                get_f(p, "stress1")?,
                get_f(p, "stress2")?,
                get_f(p, "rate1")?,
                get_f(p, "rate2")?,
            ),
        )),
        "stress_corrosion_threshold" => Ok(RunOutput::Boolean(
            phys::solid_mechanics::fracture::stress_corrosion_threshold(
                get_f(p, "k_iscc")?,
                get_f(p, "sigma")?,
                get_f(p, "a")?,
            ),
        )),
        "stress_intensity_factor" => Ok(RunOutput::Scalar(
            phys::solid_mechanics::fracture::stress_intensity_factor(
                get_f(p, "sigma")?,
                get_f(p, "a")?,
                get_f(p, "geometry_factor")?,
            ),
        )),
        "stress_invariant_j2" => Ok(RunOutput::Scalar(
            phys::solid_mechanics::stress::stress_invariant_j2(
                get_f(p, "s1")?,
                get_f(p, "s2")?,
                get_f(p, "s3")?,
            ),
        )),
        "stress_rotation_2d" => {
            let r = phys::solid_mechanics::stress::stress_rotation_2d(
                get_f(p, "sx")?,
                get_f(p, "sy")?,
                get_f(p, "txy")?,
                get_f(p, "theta")?,
            );
            Ok(RunOutput::Triple(r.0, r.1, r.2))
        }
        "thermal_strain" => Ok(RunOutput::Scalar(
            phys::solid_mechanics::elasticity::thermal_strain(
                get_f(p, "alpha")?,
                get_f(p, "delta_t")?,
            ),
        )),
        "thermal_stress" => Ok(RunOutput::Scalar(
            phys::solid_mechanics::elasticity::thermal_stress(
                get_f(p, "e")?,
                get_f(p, "alpha")?,
                get_f(p, "delta_t")?,
            ),
        )),
        "torsion_shear_stress" => Ok(RunOutput::Scalar(
            phys::solid_mechanics::stress::torsion_shear_stress(
                get_f(p, "t")?,
                get_f(p, "r")?,
                get_f(p, "j")?,
            ),
        )),
        "tresca" => Ok(RunOutput::Scalar(
            phys::solid_mechanics::plasticity::tresca(
                get_f(p, "s1")?,
                get_f(p, "s2")?,
                get_f(p, "s3")?,
            ),
        )),
        "true_strain" => Ok(RunOutput::Scalar(
            phys::solid_mechanics::plasticity::true_strain(get_f(p, "engineering_strain")?),
        )),
        "true_stress" => Ok(RunOutput::Scalar(
            phys::solid_mechanics::plasticity::true_stress(
                get_f(p, "engineering_stress")?,
                get_f(p, "engineering_strain")?,
            ),
        )),
        "volumetric_strain" => Ok(RunOutput::Scalar(
            phys::solid_mechanics::elasticity::volumetric_strain(
                get_f(p, "ex")?,
                get_f(p, "ey")?,
                get_f(p, "ez")?,
            ),
        )),
        "von_mises" => Ok(RunOutput::Scalar(
            phys::solid_mechanics::plasticity::von_mises(
                get_f(p, "s1")?,
                get_f(p, "s2")?,
                get_f(p, "s3")?,
            ),
        )),
        "yield_offset_strain" => Ok(RunOutput::Scalar(
            phys::solid_mechanics::plasticity::yield_offset_strain(
                get_f(p, "total_strain")?,
                get_f(p, "offset")?,
            ),
        )),
        "vacancy_concentration" => Ok(RunOutput::Scalar(
            phys::solid_mechanics::crystallography::vacancy_concentration(
                get_f(p, "ev")?,
                get_f(p, "t")?,
            ),
        )),
        "diffusion_coefficient" => Ok(RunOutput::Scalar(
            phys::transport::diffusion::diffusion_coefficient(
                get_f(p, "d0")?,
                get_f(p, "q")?,
                get_f(p, "t")?,
            ),
        )),
        "fick_first_law" => Ok(RunOutput::Scalar(
            phys::transport::diffusion::fick_first_law(get_f(p, "d")?, get_f(p, "dc_dx")?),
        )),
        _ => Err(HubError::InvalidInput(format!("unknown function: {func}"))),
    }
}
