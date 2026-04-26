//! Dispatch handler for optics functions.

use super::super::params::*;
use crate::hub::domain::common::errors::{HubError, HubResult};
use crate::hub::domain::physics as phys;
use crate::hub::engine::experience::runner::RunOutput;

pub(super) fn dispatch(func: &str, p: &Params) -> HubResult<RunOutput> {
    match func {
        "abbe_number" => Ok(RunOutput::Scalar(phys::optics::refraction::abbe_number(
            get_f(p, "nd")?,
            get_f(p, "nf")?,
            get_f(p, "nc")?,
        ))),
        "airy_disk_radius" => Ok(RunOutput::Scalar(
            phys::optics::diffraction::airy_disk_radius(
                get_f(p, "wavelength")?,
                get_f(p, "f_number")?,
            ),
        )),
        "birefringence" => Ok(RunOutput::Scalar(
            phys::optics::polarization::birefringence(
                get_f(p, "n_extraordinary")?,
                get_f(p, "n_ordinary")?,
            ),
        )),
        "bragg_condition" => Ok(RunOutput::Scalar(
            phys::optics::diffraction::bragg_condition(
                get_f(p, "d_spacing")?,
                get_f(p, "theta")?,
                get_f(p, "wavelength")?,
            ),
        )),
        "optics::refraction::brewster_angle" => Ok(RunOutput::Scalar(
            phys::optics::refraction::brewster_angle(get_f(p, "n1")?, get_f(p, "n2")?),
        )),
        "brewster_reflectance" => Ok(RunOutput::Scalar(
            phys::optics::polarization::brewster_reflectance(get_f(p, "n1")?, get_f(p, "n2")?),
        )),
        "cauchy_dispersion" => Ok(RunOutput::Scalar(
            phys::optics::refraction::cauchy_dispersion(
                get_f(p, "a")?,
                get_f(p, "b")?,
                get_f(p, "wavelength")?,
            ),
        )),
        "circular_aperture_first_zero" => Ok(RunOutput::Scalar(
            phys::optics::diffraction::circular_aperture_first_zero(
                get_f(p, "wavelength")?,
                get_f(p, "diameter")?,
            ),
        )),
        "circular_dichroism" => Ok(RunOutput::Scalar(
            phys::optics::polarization::circular_dichroism(
                get_f(p, "a_left")?,
                get_f(p, "a_right")?,
            ),
        )),
        "coherence_length" => Ok(RunOutput::Scalar(
            phys::optics::interference::coherence_length(
                get_f(p, "wavelength")?,
                get_f(p, "delta_wavelength")?,
            ),
        )),
        "coherence_time" => Ok(RunOutput::Scalar(
            phys::optics::interference::coherence_time(get_f(p, "delta_nu")?),
        )),
        "constructive_condition" => Ok(RunOutput::Boolean(
            phys::optics::interference::constructive_condition(
                get_f(p, "optical_path_diff")?,
                get_f(p, "wavelength")?,
            ),
        )),
        "optics::refraction::critical_angle" => Ok(RunOutput::Scalar(
            phys::optics::refraction::critical_angle(get_f(p, "n1")?, get_f(p, "n2")?),
        )),
        "diffraction_grating_maxima" => Ok(RunOutput::Scalar(
            phys::optics::diffraction::diffraction_grating_maxima(
                get_f(p, "d")?,
                get_f(p, "wavelength")?,
                get_i(p, "order")? as i32,
            ),
        )),
        "double_slit_intensity" => Ok(RunOutput::Scalar(
            phys::optics::diffraction::double_slit_intensity(
                get_f(p, "theta")?,
                get_f(p, "d")?,
                get_f(p, "wavelength")?,
            ),
        )),
        "ellipticity" => Ok(RunOutput::Scalar(phys::optics::polarization::ellipticity(
            get_f(p, "major")?,
            get_f(p, "minor")?,
        ))),
        "fabry_perot_finesse" => Ok(RunOutput::Scalar(
            phys::optics::interference::fabry_perot_finesse(get_f(p, "r")?),
        )),
        "fabry_perot_transmittance" => Ok(RunOutput::Scalar(
            phys::optics::interference::fabry_perot_transmittance(
                get_f(p, "r")?,
                get_f(p, "delta")?,
            ),
        )),
        "fraunhofer_distance" => Ok(RunOutput::Scalar(
            phys::optics::diffraction::fraunhofer_distance(
                get_f(p, "aperture")?,
                get_f(p, "wavelength")?,
            ),
        )),
        "free_spectral_range" => Ok(RunOutput::Scalar(
            phys::optics::interference::free_spectral_range(get_f(p, "d")?, get_f(p, "n")?),
        )),
        "fresnel_reflectance_p" => Ok(RunOutput::Scalar(
            phys::optics::refraction::fresnel_reflectance_p(
                get_f(p, "n1")?,
                get_f(p, "theta_i")?,
                get_f(p, "n2")?,
                get_f(p, "theta_t")?,
            ),
        )),
        "fresnel_reflectance_s" => Ok(RunOutput::Scalar(
            phys::optics::refraction::fresnel_reflectance_s(
                get_f(p, "n1")?,
                get_f(p, "theta_i")?,
                get_f(p, "n2")?,
                get_f(p, "theta_t")?,
            ),
        )),
        "fringe_spacing" => Ok(RunOutput::Scalar(
            phys::optics::interference::fringe_spacing(
                get_f(p, "wavelength")?,
                get_f(p, "d")?,
                get_f(p, "l")?,
            ),
        )),
        "grating_dispersion" => Ok(RunOutput::Scalar(
            phys::optics::diffraction::grating_dispersion(
                get_i(p, "order")? as i32,
                get_f(p, "d")?,
                get_f(p, "theta")?,
            ),
        )),
        "lensmaker_equation" => Ok(RunOutput::Scalar(
            phys::optics::refraction::lensmaker_equation(
                get_f(p, "n")?,
                get_f(p, "r1")?,
                get_f(p, "r2")?,
            ),
        )),
        "magnification" => Ok(RunOutput::Scalar(phys::optics::refraction::magnification(
            get_f(p, "image_dist")?,
            get_f(p, "object_dist")?,
        ))),
        "malus_law" => Ok(RunOutput::Scalar(phys::optics::polarization::malus_law(
            get_f(p, "i0")?,
            get_f(p, "theta")?,
        ))),
        "michelson_path_difference" => Ok(RunOutput::Scalar(
            phys::optics::interference::michelson_path_difference(get_f(p, "mirror_displacement")?),
        )),
        "newton_ring_radius" => Ok(RunOutput::Scalar(
            phys::optics::interference::newton_ring_radius(
                get_i(p, "m")? as u32,
                get_f(p, "wavelength")?,
                get_f(p, "r")?,
            ),
        )),
        "numerical_aperture" => Ok(RunOutput::Scalar(
            phys::optics::refraction::numerical_aperture(get_f(p, "n")?, get_f(p, "half_angle")?),
        )),
        "optical_path_length" => Ok(RunOutput::Scalar(
            phys::optics::refraction::optical_path_length(get_f(p, "n")?, get_f(p, "d")?),
        )),
        "quarter_wave_plate_phase" => Ok(RunOutput::Scalar(
            phys::optics::polarization::quarter_wave_plate_phase(
                get_f(p, "wavelength")?,
                get_f(p, "n_fast")?,
                get_f(p, "n_slow")?,
            ),
        )),
        "rayleigh_criterion" => Ok(RunOutput::Scalar(
            phys::optics::diffraction::rayleigh_criterion(
                get_f(p, "wavelength")?,
                get_f(p, "aperture")?,
            ),
        )),
        "resolving_power_grating" => Ok(RunOutput::Scalar(
            phys::optics::diffraction::resolving_power_grating(
                get_i(p, "order")? as i32,
                get_i(p, "n_slits")? as u32,
            ),
        )),
        "retardance" => Ok(RunOutput::Scalar(phys::optics::polarization::retardance(
            get_f(p, "birefringence")?,
            get_f(p, "thickness")?,
            get_f(p, "wavelength")?,
        ))),
        "single_slit_intensity" => Ok(RunOutput::Scalar(
            phys::optics::diffraction::single_slit_intensity(
                get_f(p, "theta")?,
                get_f(p, "a")?,
                get_f(p, "wavelength")?,
            ),
        )),
        "optics::refraction::snell" => Ok(RunOutput::Scalar(phys::optics::refraction::snell(
            get_f(p, "n1")?,
            get_f(p, "theta1")?,
            get_f(p, "n2")?,
        ))),
        "specific_rotation" => Ok(RunOutput::Scalar(
            phys::optics::polarization::specific_rotation(
                get_f(p, "observed")?,
                get_f(p, "l")?,
                get_f(p, "c")?,
            ),
        )),
        "thin_film_phase_shift" => Ok(RunOutput::Scalar(
            phys::optics::interference::thin_film_phase_shift(
                get_f(p, "n")?,
                get_f(p, "thickness")?,
                get_f(p, "wavelength")?,
                get_f(p, "theta")?,
            ),
        )),
        "thin_lens_equation" => Ok(RunOutput::Scalar(
            phys::optics::refraction::thin_lens_equation(
                get_f(p, "focal")?,
                get_f(p, "object_dist")?,
            ),
        )),
        "two_beam_intensity" => Ok(RunOutput::Scalar(
            phys::optics::interference::two_beam_intensity(
                get_f(p, "i1")?,
                get_f(p, "i2")?,
                get_f(p, "delta_phase")?,
            ),
        )),
        "visibility" => Ok(RunOutput::Scalar(phys::optics::interference::visibility(
            get_f(p, "i_max")?,
            get_f(p, "i_min")?,
        ))),
        "degree_of_polarization" => {
            let v = get_v(p, "s")?;
            let s = [v[0], v[1], v[2], v[3]];
            Ok(RunOutput::Scalar(
                phys::optics::polarization::degree_of_polarization(&s),
            ))
        }
        "jones_rotation" => {
            let r = phys::optics::polarization::jones_rotation(get_f(p, "theta")?);
            Ok(RunOutput::Matrix(
                r.iter().map(|row| row.to_vec()).collect(),
            ))
        }
        "stokes_parameters" => {
            let r = phys::optics::polarization::stokes_parameters(
                get_f(p, "ex")?,
                get_f(p, "ey")?,
                get_f(p, "delta")?,
            );
            Ok(RunOutput::Vector(r.to_vec()))
        }
        "wavelength_to_energy_ev" => Ok(RunOutput::Scalar(
            phys::optics::scattering::wavelength_to_energy_ev(get_f(p, "wavelength_nm")?),
        )),
        "energy_ev_to_wavelength_nm" => Ok(RunOutput::Scalar(
            phys::optics::scattering::energy_ev_to_wavelength_nm(get_f(p, "energy_ev")?),
        )),
        "wavelength_angstrom_to_m" => Ok(RunOutput::Scalar(
            phys::optics::scattering::wavelength_angstrom_to_m(get_f(p, "wavelength_angstrom")?),
        )),
        "wavelength_m_to_angstrom" => Ok(RunOutput::Scalar(
            phys::optics::scattering::wavelength_m_to_angstrom(get_f(p, "wavelength_m")?),
        )),
        "photon_energy_joule_to_ev" => Ok(RunOutput::Scalar(
            phys::optics::scattering::photon_energy_joule_to_ev(get_f(p, "energy_j")?),
        )),
        "size_parameter" => Ok(RunOutput::Scalar(phys::optics::scattering::size_parameter(
            get_f(p, "radius")?,
            get_f(p, "wavelength")?,
        ))),
        _ => Err(HubError::InvalidInput(format!("unknown function: {func}"))),
    }
}
