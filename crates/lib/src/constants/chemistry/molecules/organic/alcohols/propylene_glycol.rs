use super::super::super::Molecule;

pub fn molecule() -> Molecule {
    Molecule {
        formula: "C3H8O2",
        name: "propylene glycol",
        composition: &[(6, 3), (1, 8), (8, 2)],
        molar_mass: 76.094,
        category: "alcohol",
        state_at_stp: "liquid",
        melting_point_k: Some(213.0),
        boiling_point_k: Some(461.0),
        density_g_cm3: Some(1.036),
    }
}
