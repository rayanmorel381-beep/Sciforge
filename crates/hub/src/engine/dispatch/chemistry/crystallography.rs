//! Dispatch handler for crystallography functions.

use super::super::params::*;
use crate::domain::chemistry as chem;
use crate::domain::common::errors::{HubError, HubResult};
use crate::engine::experience::runner::RunOutput;

pub(super) fn dispatch(func: &str, p: &Params) -> HubResult<RunOutput> {
    match func {
        "bragg_law" => Ok(RunOutput::Scalar(
            chem::crystallography::diffraction::bragg_law(
                get_f(p, "d")?,
                get_f(p, "theta")?,
                get_i(p, "n")? as u32,
            ),
        )),
        "bragg_angle" => Ok(RunOutput::Scalar(
            chem::crystallography::diffraction::bragg_angle(
                get_f(p, "wavelength")?,
                get_f(p, "d")?,
                get_i(p, "n")? as u32,
            ),
        )),
        "interplanar_spacing_cubic" => Ok(RunOutput::Scalar(
            chem::crystallography::diffraction::interplanar_spacing_cubic(
                get_f(p, "a")?,
                get_i(p, "h")? as i32,
                get_i(p, "k")? as i32,
                get_i(p, "l")? as i32,
            ),
        )),
        "structure_factor_bcc" => Ok(RunOutput::Scalar(
            chem::crystallography::diffraction::structure_factor_bcc(
                get_i(p, "h")? as i32,
                get_i(p, "k")? as i32,
                get_i(p, "l")? as i32,
                get_f(p, "f_atom")?,
            ),
        )),
        "structure_factor_fcc" => Ok(RunOutput::Scalar(
            chem::crystallography::diffraction::structure_factor_fcc(
                get_i(p, "h")? as i32,
                get_i(p, "k")? as i32,
                get_i(p, "l")? as i32,
                get_f(p, "f_atom")?,
            ),
        )),
        "scherrer_crystal_size" => Ok(RunOutput::Scalar(
            chem::crystallography::diffraction::scherrer_crystal_size(
                get_f(p, "k")?,
                get_f(p, "wavelength")?,
                get_f(p, "fwhm")?,
                get_f(p, "theta")?,
            ),
        )),

        "cubic_volume" => Ok(RunOutput::Scalar(
            chem::crystallography::lattice::cubic_volume(get_f(p, "a")?),
        )),
        "tetragonal_volume" => Ok(RunOutput::Scalar(
            chem::crystallography::lattice::tetragonal_volume(get_f(p, "a")?, get_f(p, "c")?),
        )),
        "orthorhombic_volume" => Ok(RunOutput::Scalar(
            chem::crystallography::lattice::orthorhombic_volume(
                get_f(p, "a")?,
                get_f(p, "b")?,
                get_f(p, "c")?,
            ),
        )),
        "hexagonal_volume" => Ok(RunOutput::Scalar(
            chem::crystallography::lattice::hexagonal_volume(get_f(p, "a")?, get_f(p, "c")?),
        )),
        "packing_fraction_simple_cubic" => Ok(RunOutput::Scalar(
            chem::crystallography::lattice::packing_fraction_simple_cubic(),
        )),
        "packing_fraction_bcc" => Ok(RunOutput::Scalar(
            chem::crystallography::lattice::packing_fraction_bcc(),
        )),
        "packing_fraction_fcc" => Ok(RunOutput::Scalar(
            chem::crystallography::lattice::packing_fraction_fcc(),
        )),
        "density_from_unit_cell" => Ok(RunOutput::Scalar(
            chem::crystallography::lattice::density_from_unit_cell(
                get_i(p, "z")? as u32,
                get_f(p, "molar_mass")?,
                get_f(p, "volume")?,
                get_f(p, "avogadro")?,
            ),
        )),
        "miller_to_direction_cosines" => {
            let (a, b, c) = chem::crystallography::lattice::miller_to_direction_cosines(
                get_i(p, "h")? as i32,
                get_i(p, "k")? as i32,
                get_i(p, "l")? as i32,
            );
            Ok(RunOutput::Triple(a, b, c))
        }
        "reciprocal_lattice_vector" => Ok(RunOutput::Scalar(
            chem::crystallography::lattice::reciprocal_lattice_vector(get_f(p, "a")?),
        )),

        _ => Err(HubError::InvalidInput(format!("unknown function: {func}"))),
    }
}
