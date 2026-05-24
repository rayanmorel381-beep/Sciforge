use super::super::super::Molecule;

pub fn molecule() -> Molecule {
    Molecule {
        formula: "C8H8O3",
        name: "vanillin",
        composition: &[(6, 8), (1, 8), (8, 3)],
        molar_mass: 152.149,
        category: "aldehyde",
        state_at_stp: "solid",
        melting_point_k: Some(354.0),
        boiling_point_k: Some(558.0),
        density_g_cm3: Some(1.056),
    }
}
