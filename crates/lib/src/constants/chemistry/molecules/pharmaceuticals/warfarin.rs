use super::super::Molecule;

pub fn molecule() -> Molecule {
    Molecule {
        formula: "C19H16O4",
        name: "warfarin",
        composition: &[(1, 16), (6, 19), (8, 4)],
        molar_mass: 308.328,
        category: "anticoagulant",
        state_at_stp: "solid",
        melting_point_k: Some(434.0),
        boiling_point_k: None,
        density_g_cm3: Some(1.300),
    }
}
