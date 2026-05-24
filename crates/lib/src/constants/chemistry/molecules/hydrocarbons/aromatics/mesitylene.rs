use super::super::super::Molecule;

pub fn molecule() -> Molecule {
    Molecule {
        formula: "C9H12",
        name: "mesitylene",
        composition: &[(6, 9), (1, 12)],
        molar_mass: 120.194,
        category: "aromatic",
        state_at_stp: "liquid",
        melting_point_k: Some(228.4),
        boiling_point_k: Some(437.9),
        density_g_cm3: Some(0.8637),
    }
}
