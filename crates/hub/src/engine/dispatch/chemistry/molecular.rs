//! Dispatch handler for molecular chemistry functions.

use super::super::params::*;
use crate::domain::chemistry as chem;
use crate::domain::common::errors::{HubError, HubResult};
use crate::engine::experience::runner::RunOutput;

pub(super) fn dispatch(func: &str, p: &Params) -> HubResult<RunOutput> {
    match func {
        "bond_order" => Ok(RunOutput::Scalar(chem::molecular::bonding::bond_order(
            get_i(p, "bonding_electrons")? as u32,
            get_i(p, "antibonding_electrons")? as u32,
        ))),
        "dipole_moment" => {
            let m = get_m(p, "charges")?;
            let charges: Vec<(f64, [f64; 3])> =
                m.iter().map(|r| (r[0], [r[1], r[2], r[3]])).collect();
            let res = chem::molecular::bonding::dipole_moment(&charges);
            Ok(RunOutput::Triple(res[0], res[1], res[2]))
        }
        "dipole_magnitude" => {
            let v = get_v(p, "mu")?;
            let mu = [v[0], v[1], v[2]];
            Ok(RunOutput::Scalar(
                chem::molecular::bonding::dipole_magnitude(&mu),
            ))
        }
        "coulomb_integral" => Ok(RunOutput::Scalar(
            chem::molecular::bonding::coulomb_integral(get_f(p, "z_eff")?, get_f(p, "n")?),
        )),
        "slater_shielding" => {
            let eb = get_v(p, "electrons_below")?;
            let eb_u32: Vec<u32> = eb.iter().map(|&x| x as u32).collect();
            Ok(RunOutput::Scalar(
                chem::molecular::bonding::slater_shielding(
                    &eb_u32,
                    get_v(p, "shielding_constants")?,
                ),
            ))
        }
        "electronegativity_mulliken" => Ok(RunOutput::Scalar(
            chem::molecular::bonding::electronegativity_mulliken(get_f(p, "ie")?, get_f(p, "ea")?),
        )),
        "formal_charge" => Ok(RunOutput::Integer(chem::molecular::bonding::formal_charge(
            get_i(p, "valence")? as i32,
            get_i(p, "lone_pair")? as i32,
            get_i(p, "bonding")? as i32,
        ) as i64)),
        "molar_mass" => {
            let cv = get_v(p, "counts")?;
            let counts: Vec<u32> = cv.iter().map(|&x| x as u32).collect();
            Ok(RunOutput::Scalar(chem::molecular::bonding::molar_mass(
                get_v(p, "atomic_masses")?,
                &counts,
            )))
        }
        "percent_composition" => Ok(RunOutput::Scalar(
            chem::molecular::bonding::percent_composition(
                get_f(p, "element_mass")?,
                get_i(p, "element_count")? as u32,
                get_f(p, "molar_mass")?,
            ),
        )),
        "electronegativity_allred_rochow" => Ok(RunOutput::Scalar(
            chem::molecular::bonding::electronegativity_allred_rochow(
                get_f(p, "z_eff")?,
                get_f(p, "r_cov")?,
            ),
        )),
        "polarizability_estimate" => Ok(RunOutput::Scalar(
            chem::molecular::bonding::polarizability_estimate(get_f(p, "volume_angstrom3")?),
        )),
        "bond_dissociation_energy_pauling" => Ok(RunOutput::Scalar(
            chem::molecular::bonding::bond_dissociation_energy_pauling(
                get_f(p, "d_aa")?,
                get_f(p, "d_bb")?,
                get_f(p, "en_diff")?,
            ),
        )),

        "vsepr_angle" => Ok(RunOutput::Scalar(chem::molecular::geometry::vsepr_angle(
            get_i(p, "bonding_pairs")? as u32,
            get_i(p, "lone_pairs")? as u32,
        ))),
        "hybridization_sp" => Ok(RunOutput::Text(
            chem::molecular::geometry::hybridization_sp(get_i(p, "bonding_regions")? as u32)
                .to_string(),
        )),
        "ideal_gas_moles" => Ok(RunOutput::Scalar(
            chem::molecular::geometry::ideal_gas_moles(
                get_f(p, "p")?,
                get_f(p, "v")?,
                get_f(p, "t")?,
            ),
        )),
        "molecular_geometry_name" => Ok(RunOutput::Text(
            chem::molecular::geometry::molecular_geometry_name(
                get_i(p, "bonding_pairs")? as u32,
                get_i(p, "lone_pairs")? as u32,
            )
            .to_string(),
        )),
        "bond_length_estimate" => Ok(RunOutput::Scalar(
            chem::molecular::geometry::bond_length_estimate(get_f(p, "r1")?, get_f(p, "r2")?),
        )),
        "bond_energy_badger" => Ok(RunOutput::Scalar(
            chem::molecular::geometry::bond_energy_badger(
                get_f(p, "r_e")?,
                get_f(p, "a")?,
                get_f(p, "b")?,
            ),
        )),
        "coordination_geometry_angles" => Ok(RunOutput::Scalar(
            chem::molecular::geometry::coordination_geometry_angles(
                get_i(p, "coordination")? as u32
            ),
        )),
        "effective_nuclear_charge" => Ok(RunOutput::Scalar(
            chem::molecular::geometry::effective_nuclear_charge(
                get_i(p, "z")? as u32,
                get_f(p, "s")?,
            ),
        )),

        _ => Err(HubError::InvalidInput(format!("unknown function: {func}"))),
    }
}
