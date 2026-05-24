//! Dispatch handler for structural biology functions.

use super::super::params::*;
use crate::domain::biology as bio;
use crate::domain::common::errors::{HubError, HubResult};
use crate::engine::experience::runner::RunOutput;

pub(super) fn dispatch(func: &str, p: &Params) -> HubResult<RunOutput> {
    match func {
        "lennard_jones_potential" => Ok(RunOutput::Scalar(
            bio::structural::docking::lennard_jones_potential(
                get_f(p, "r")?,
                get_f(p, "epsilon")?,
                get_f(p, "sigma")?,
            ),
        )),
        "struct_coulomb_interaction" => Ok(RunOutput::Scalar(
            bio::structural::docking::coulomb_interaction(
                get_f(p, "q1")?,
                get_f(p, "q2")?,
                get_f(p, "r")?,
                get_f(p, "dielectric")?,
            ),
        )),
        "desolvation_energy" => Ok(RunOutput::Scalar(
            bio::structural::docking::desolvation_energy(
                get_f(p, "buried_area")?,
                get_f(p, "solvation_parameter")?,
            ),
        )),
        "shape_complementarity" => Ok(RunOutput::Scalar(
            bio::structural::docking::shape_complementarity(
                get_f(p, "interface_area")?,
                get_f(p, "gap_volume")?,
            ),
        )),
        "binding_free_energy" => Ok(RunOutput::Scalar(
            bio::structural::docking::binding_free_energy(
                get_f(p, "van_der_waals")?,
                get_f(p, "electrostatic")?,
                get_f(p, "desolvation")?,
                get_f(p, "entropy_penalty")?,
            ),
        )),
        "kd_from_delta_g" => Ok(RunOutput::Scalar(
            bio::structural::docking::kd_from_delta_g(
                get_f(p, "delta_g")?,
                get_f(p, "temperature")?,
            ),
        )),
        "buried_surface_area" => Ok(RunOutput::Scalar(
            bio::structural::docking::buried_surface_area(
                get_f(p, "asa_complex")?,
                get_f(p, "asa_receptor")?,
                get_f(p, "asa_ligand")?,
            ),
        )),
        "struct_hydrogen_bond_energy" => Ok(RunOutput::Scalar(
            bio::structural::docking::hydrogen_bond_energy(
                get_f(p, "distance")?,
                get_f(p, "angle_deg")?,
            ),
        )),
        "clash_score" => Ok(RunOutput::Scalar(bio::structural::docking::clash_score(
            get_v(p, "distances")?,
            get_f(p, "vdw_threshold")?,
        ))),
        "interface_residue_count" => Ok(RunOutput::Integer(
            bio::structural::docking::interface_residue_count(
                get_v(p, "distances_to_partner")?,
                get_f(p, "cutoff")?,
            ) as i64,
        )),
        "druggability_score" => Ok(RunOutput::Scalar(
            bio::structural::docking::druggability_score(
                get_f(p, "pocket_volume")?,
                get_f(p, "hydrophobicity")?,
                get_f(p, "enclosure")?,
            ),
        )),
        "euclidean_distance_3d" => {
            let a_v = get_v(p, "a")?;
            let a = [a_v[0], a_v[1], a_v[2]];
            let b_v = get_v(p, "b")?;
            let b = [b_v[0], b_v[1], b_v[2]];
            Ok(RunOutput::Scalar(
                bio::structural::geometry::euclidean_distance_3d(&a, &b),
            ))
        }
        "bond_angle" => {
            let a_v = get_v(p, "a")?;
            let a = [a_v[0], a_v[1], a_v[2]];
            let b_v = get_v(p, "b")?;
            let b = [b_v[0], b_v[1], b_v[2]];
            let c_v = get_v(p, "c")?;
            let c = [c_v[0], c_v[1], c_v[2]];
            Ok(RunOutput::Scalar(bio::structural::geometry::bond_angle(
                &a, &b, &c,
            )))
        }
        "dihedral_angle" => {
            let a_v = get_v(p, "a")?;
            let a = [a_v[0], a_v[1], a_v[2]];
            let b_v = get_v(p, "b")?;
            let b = [b_v[0], b_v[1], b_v[2]];
            let c_v = get_v(p, "c")?;
            let c = [c_v[0], c_v[1], c_v[2]];
            let d_v = get_v(p, "d")?;
            let d = [d_v[0], d_v[1], d_v[2]];
            Ok(RunOutput::Scalar(
                bio::structural::geometry::dihedral_angle(&a, &b, &c, &d),
            ))
        }
        "center_of_mass" => {
            let coords_m = get_m(p, "coords")?;
            let coords: Vec<[f64; 3]> = coords_m.iter().map(|r| [r[0], r[1], r[2]]).collect();
            {
                let r = bio::structural::geometry::center_of_mass(&coords, get_v(p, "masses")?);
                Ok(RunOutput::Triple(r[0], r[1], r[2]))
            }
        }
        "struct_radius_of_gyration" => {
            let coords_m = get_m(p, "coords")?;
            let coords: Vec<[f64; 3]> = coords_m.iter().map(|r| [r[0], r[1], r[2]]).collect();
            Ok(RunOutput::Scalar(
                bio::structural::geometry::radius_of_gyration(&coords, get_v(p, "masses")?),
            ))
        }
        "solvent_accessible_distance" => {
            let point_v = get_v(p, "point")?;
            let point = [point_v[0], point_v[1], point_v[2]];
            let surface_points_m = get_m(p, "surface_points")?;
            let surface_points: Vec<[f64; 3]> = surface_points_m
                .iter()
                .map(|r| [r[0], r[1], r[2]])
                .collect();
            Ok(RunOutput::Scalar(
                bio::structural::geometry::solvent_accessible_distance(&point, &surface_points),
            ))
        }
        "inertia_tensor" => {
            let coords_m = get_m(p, "coords")?;
            let coords: Vec<[f64; 3]> = coords_m.iter().map(|r| [r[0], r[1], r[2]]).collect();
            {
                let r = bio::structural::geometry::inertia_tensor(&coords, get_v(p, "masses")?);
                Ok(RunOutput::Matrix(
                    r.iter().map(|row| row.to_vec()).collect(),
                ))
            }
        }
        "planarity" => {
            let coords_m = get_m(p, "coords")?;
            let coords: Vec<[f64; 3]> = coords_m.iter().map(|r| [r[0], r[1], r[2]]).collect();
            Ok(RunOutput::Scalar(bio::structural::geometry::planarity(
                &coords,
            )))
        }
        "alpha_helix_propensity" => Ok(RunOutput::Scalar(
            bio::structural::secondary_structure::alpha_helix_propensity(get_v(
                p,
                "residue_propensities",
            )?),
        )),
        "beta_sheet_propensity" => Ok(RunOutput::Scalar(
            bio::structural::secondary_structure::beta_sheet_propensity(get_v(
                p,
                "residue_propensities",
            )?),
        )),
        "chou_fasman_nucleation" => Ok(RunOutput::Vector(
            bio::structural::secondary_structure::chou_fasman_nucleation(
                get_v(p, "propensities")?,
                get_u(p, "window")?,
                get_f(p, "threshold")?,
            )
            .into_iter()
            .map(|x| if x { 1.0 } else { 0.0 })
            .collect(),
        )),
        "gor_information_value" => Ok(RunOutput::Scalar(
            bio::structural::secondary_structure::gor_information_value(
                get_f(p, "residue_freq_in_structure")?,
                get_f(p, "residue_freq_overall")?,
            ),
        )),
        "coiled_coil_probability" => Ok(RunOutput::Scalar(
            bio::structural::secondary_structure::coiled_coil_probability(
                get_f(p, "heptad_score")?,
                get_f(p, "hydrophobic_moment")?,
            ),
        )),
        "disorder_prediction" => Ok(RunOutput::Scalar(
            bio::structural::secondary_structure::disorder_prediction(
                get_f(p, "hydrophobicity")?,
                get_f(p, "charge")?,
                get_f(p, "complexity")?,
            ),
        )),
        "solvent_accessibility" => Ok(RunOutput::Scalar(
            bio::structural::secondary_structure::solvent_accessibility(
                get_f(p, "residue_asa")?,
                get_f(p, "max_asa")?,
            ),
        )),
        "struct_ramachandran_energy" => Ok(RunOutput::Scalar(
            bio::structural::secondary_structure::ramachandran_energy(
                get_f(p, "phi")?,
                get_f(p, "psi")?,
            ),
        )),
        "hydrophobic_moment" => Ok(RunOutput::Scalar(
            bio::structural::secondary_structure::hydrophobic_moment(
                get_v(p, "hydrophobicities")?,
                get_f(p, "angle_deg")?,
            ),
        )),
        "rmsd" => {
            let coords_a_m = get_m(p, "coords_a")?;
            let coords_a: Vec<[f64; 3]> = coords_a_m.iter().map(|r| [r[0], r[1], r[2]]).collect();
            let coords_b_m = get_m(p, "coords_b")?;
            let coords_b: Vec<[f64; 3]> = coords_b_m.iter().map(|r| [r[0], r[1], r[2]]).collect();
            Ok(RunOutput::Scalar(bio::structural::superposition::rmsd(
                &coords_a, &coords_b,
            )))
        }
        "gdt_ts" => {
            let coords_a_m = get_m(p, "coords_a")?;
            let coords_a: Vec<[f64; 3]> = coords_a_m.iter().map(|r| [r[0], r[1], r[2]]).collect();
            let coords_b_m = get_m(p, "coords_b")?;
            let coords_b: Vec<[f64; 3]> = coords_b_m.iter().map(|r| [r[0], r[1], r[2]]).collect();
            Ok(RunOutput::Scalar(bio::structural::superposition::gdt_ts(
                &coords_a,
                &coords_b,
                get_v(p, "cutoffs")?,
            )))
        }
        "tm_score" => {
            let coords_a_m = get_m(p, "coords_a")?;
            let coords_a: Vec<[f64; 3]> = coords_a_m.iter().map(|r| [r[0], r[1], r[2]]).collect();
            let coords_b_m = get_m(p, "coords_b")?;
            let coords_b: Vec<[f64; 3]> = coords_b_m.iter().map(|r| [r[0], r[1], r[2]]).collect();
            Ok(RunOutput::Scalar(bio::structural::superposition::tm_score(
                &coords_a,
                &coords_b,
                get_u(p, "l_target")?,
            )))
        }
        "contact_map" => {
            let coords_m = get_m(p, "coords")?;
            let coords: Vec<[f64; 3]> = coords_m.iter().map(|r| [r[0], r[1], r[2]]).collect();
            Ok(RunOutput::PairVec(
                bio::structural::superposition::contact_map(&coords, get_f(p, "cutoff")?)
                    .into_iter()
                    .map(|(a, b)| (a as f64, b as f64))
                    .collect(),
            ))
        }
        "rg_from_coords" => {
            let coords_m = get_m(p, "coords")?;
            let coords: Vec<[f64; 3]> = coords_m.iter().map(|r| [r[0], r[1], r[2]]).collect();
            Ok(RunOutput::Scalar(
                bio::structural::superposition::rg_from_coords(&coords),
            ))
        }
        "solvent_accessible_surface_approx" => Ok(RunOutput::Scalar(
            bio::structural::superposition::solvent_accessible_surface_approx(
                get_v(p, "radii")?,
                get_f(p, "probe")?,
            ),
        )),
        "lrmsd" => {
            let coords_a_m = get_m(p, "coords_a")?;
            let coords_a: Vec<[f64; 3]> = coords_a_m.iter().map(|r| [r[0], r[1], r[2]]).collect();
            let coords_b_m = get_m(p, "coords_b")?;
            let coords_b: Vec<[f64; 3]> = coords_b_m.iter().map(|r| [r[0], r[1], r[2]]).collect();
            let residue_indices_v = get_v(p, "residue_indices")?;
            let residue_indices: Vec<usize> =
                residue_indices_v.iter().map(|&x| x as usize).collect();
            Ok(RunOutput::Scalar(bio::structural::superposition::lrmsd(
                &coords_a,
                &coords_b,
                &residue_indices,
            )))
        }
        "drmsd" => {
            let coords_a_m = get_m(p, "coords_a")?;
            let coords_a: Vec<[f64; 3]> = coords_a_m.iter().map(|r| [r[0], r[1], r[2]]).collect();
            let coords_b_m = get_m(p, "coords_b")?;
            let coords_b: Vec<[f64; 3]> = coords_b_m.iter().map(|r| [r[0], r[1], r[2]]).collect();
            Ok(RunOutput::Scalar(bio::structural::superposition::drmsd(
                &coords_a, &coords_b,
            )))
        }
        "b_factor_normalize" => Ok(RunOutput::Vector(
            bio::structural::superposition::b_factor_normalize(get_v(p, "b_factors")?),
        )),
        "coulomb_interaction" => Ok(RunOutput::Scalar(
            bio::structural::docking::coulomb_interaction(
                get_f(p, "q1")?,
                get_f(p, "q2")?,
                get_f(p, "r")?,
                get_f(p, "dielectric")?,
            ),
        )),
        "hydrogen_bond_energy" => Ok(RunOutput::Scalar(
            bio::structural::docking::hydrogen_bond_energy(
                get_f(p, "distance")?,
                get_f(p, "angle_deg")?,
            ),
        )),
        "ramachandran_energy" => Ok(RunOutput::Scalar(
            bio::structural::secondary_structure::ramachandran_energy(
                get_f(p, "phi")?,
                get_f(p, "psi")?,
            ),
        )),
        "absolute_contact_order" => {
            let v = get_v(p, "contacts")?;
            let contacts: Vec<(usize, usize)> = v
                .chunks(2)
                .map(|c| (c[0] as usize, c[1] as usize))
                .collect();
            Ok(RunOutput::Scalar(
                bio::structural::superposition::absolute_contact_order(
                    &contacts,
                    get_u(p, "n_residues")?,
                ),
            ))
        }
        "relative_contact_order" => {
            let v = get_v(p, "contacts")?;
            let contacts: Vec<(usize, usize)> = v
                .chunks(2)
                .map(|c| (c[0] as usize, c[1] as usize))
                .collect();
            Ok(RunOutput::Scalar(
                bio::structural::secondary_structure::relative_contact_order(
                    &contacts,
                    get_u(p, "chain_length")?,
                ),
            ))
        }
        "radius_of_gyration" => {
            let v = get_v(p, "coords")?;
            let masses = get_v(p, "masses")?;
            let coords: Vec<[f64; 3]> = v.chunks(3).map(|c| [c[0], c[1], c[2]]).collect();
            Ok(RunOutput::Scalar(
                bio::structural::geometry::radius_of_gyration(&coords, masses),
            ))
        }
        _ => Err(HubError::InvalidInput(format!("unknown function: {func}"))),
    }
}
