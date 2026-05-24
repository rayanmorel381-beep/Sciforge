use super::super::super::Molecule;

pub fn molecule() -> Molecule {
    Molecule {
        formula: "C8H10",
        name: "p-xylene",
        composition: &[(6, 8), (1, 10)],
        molar_mass: 106.167,
        category: "aromatic",
        state_at_stp: "liquid",
        melting_point_k: Some(286.4),
        boiling_point_k: Some(411.5),
        density_g_cm3: Some(0.861),
    }
}
