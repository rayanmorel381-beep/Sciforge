use super::super::super::Molecule;

pub fn molecule() -> Molecule {
    Molecule {
        formula: "C2H6",
        name: "ethane",
        composition: &[(6, 2), (1, 6)],
        molar_mass: 30.069,
        category: "alkane",
        state_at_stp: "gas",
        melting_point_k: Some(90.4),
        boiling_point_k: Some(184.55),
        density_g_cm3: Some(0.001282),
    }
}
