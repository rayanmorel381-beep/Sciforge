use super::super::super::Molecule;

pub fn molecule() -> Molecule {
    Molecule {
        formula: "As4S4",
        name: "realgar",
        composition: &[(16, 4), (33, 4)],
        molar_mass: 427.949,
        category: "sulfide",
        state_at_stp: "solid",
        melting_point_k: Some(594.0),
        boiling_point_k: Some(838.0),
        density_g_cm3: Some(3.560),
    }
}
