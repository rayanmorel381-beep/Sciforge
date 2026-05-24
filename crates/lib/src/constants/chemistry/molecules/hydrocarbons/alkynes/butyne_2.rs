use super::super::super::Molecule;

pub fn molecule() -> Molecule {
    Molecule {
        formula: "C4H6",
        name: "2-butyne",
        composition: &[(6, 4), (1, 6)],
        molar_mass: 54.092,
        category: "alkyne",
        state_at_stp: "liquid",
        melting_point_k: Some(240.9),
        boiling_point_k: Some(300.13),
        density_g_cm3: Some(0.691),
    }
}
