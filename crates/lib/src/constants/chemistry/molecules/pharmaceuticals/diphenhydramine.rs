use super::super::Molecule;

pub fn molecule() -> Molecule {
    Molecule {
        formula: "C17H21NO",
        name: "diphenhydramine",
        composition: &[(1, 21), (6, 17), (7, 1), (8, 1)],
        molar_mass: 255.355,
        category: "antihistamine",
        state_at_stp: "solid",
        melting_point_k: Some(441.0),
        boiling_point_k: None,
        density_g_cm3: Some(1.060),
    }
}
