use super::super::super::Molecule;

pub fn molecule() -> Molecule {
    Molecule {
        formula: "P2O5",
        name: "phosphorus pentoxide",
        composition: &[(8, 5), (15, 2)],
        molar_mass: 141.944,
        category: "oxide",
        state_at_stp: "solid",
        melting_point_k: Some(613.0),
        boiling_point_k: Some(633.0),
        density_g_cm3: Some(2.390),
    }
}
