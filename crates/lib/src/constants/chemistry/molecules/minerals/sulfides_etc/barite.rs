use super::super::super::Molecule;

pub fn molecule() -> Molecule {
    Molecule {
        formula: "BaSO4",
        name: "barite",
        composition: &[(8, 4), (16, 1), (56, 1)],
        molar_mass: 233.390,
        category: "sulfate",
        state_at_stp: "solid",
        melting_point_k: Some(1853.0),
        boiling_point_k: None,
        density_g_cm3: Some(4.480),
    }
}
