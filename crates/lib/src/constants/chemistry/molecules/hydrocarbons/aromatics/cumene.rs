use super::super::super::Molecule;

pub fn molecule() -> Molecule {
    Molecule {
        formula: "C9H12",
        name: "cumene",
        composition: &[(6, 9), (1, 12)],
        molar_mass: 120.194,
        category: "aromatic",
        state_at_stp: "liquid",
        melting_point_k: Some(177.14),
        boiling_point_k: Some(425.56),
        density_g_cm3: Some(0.862),
    }
}
