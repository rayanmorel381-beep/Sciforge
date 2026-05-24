use super::super::super::Molecule;

pub fn molecule() -> Molecule {
    Molecule {
        formula: "C2H3Br",
        name: "vinyl bromide",
        composition: &[(6, 2), (1, 3), (35, 1)],
        molar_mass: 106.949,
        category: "haloalkene",
        state_at_stp: "gas",
        melting_point_k: Some(135.0),
        boiling_point_k: Some(289.0),
        density_g_cm3: Some(1.493),
    }
}
