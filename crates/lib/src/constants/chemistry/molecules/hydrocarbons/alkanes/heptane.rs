use super::super::super::Molecule;

pub fn molecule() -> Molecule {
    Molecule {
        formula: "C7H16",
        name: "heptane",
        composition: &[(6, 7), (1, 16)],
        molar_mass: 100.202,
        category: "alkane",
        state_at_stp: "liquid",
        melting_point_k: Some(182.6),
        boiling_point_k: Some(371.57),
        density_g_cm3: Some(0.6837),
    }
}
