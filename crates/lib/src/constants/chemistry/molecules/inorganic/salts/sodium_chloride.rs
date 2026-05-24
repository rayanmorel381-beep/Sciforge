use super::super::super::Molecule;

pub fn molecule() -> Molecule {
    Molecule {
        formula: "NaCl",
        name: "sodium chloride",
        composition: &[(11, 1), (17, 1)],
        molar_mass: 58.44,
        category: "inorganic",
        state_at_stp: "solid",
        melting_point_k: Some(1074.0),
        boiling_point_k: Some(1738.0),
        density_g_cm3: Some(2.165),
    }
}
