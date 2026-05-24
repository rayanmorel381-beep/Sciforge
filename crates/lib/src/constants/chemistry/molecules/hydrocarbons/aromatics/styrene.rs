use super::super::super::Molecule;

pub fn molecule() -> Molecule {
    Molecule {
        formula: "C8H8",
        name: "styrene",
        composition: &[(6, 8), (1, 8)],
        molar_mass: 104.152,
        category: "aromatic",
        state_at_stp: "liquid",
        melting_point_k: Some(242.5),
        boiling_point_k: Some(418.31),
        density_g_cm3: Some(0.909),
    }
}
