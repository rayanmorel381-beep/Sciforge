use super::super::super::Molecule;

pub fn molecule() -> Molecule {
    Molecule {
        formula: "BH3",
        name: "borane",
        composition: &[(1, 3), (5, 1)],
        molar_mass: 13.835,
        category: "hydride",
        state_at_stp: "gas",
        melting_point_k: None,
        boiling_point_k: None,
        density_g_cm3: None,
    }
}
