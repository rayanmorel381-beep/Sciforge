use super::super::super::Molecule;

pub fn molecule() -> Molecule {
    Molecule {
        formula: "SF6",
        name: "sulfur hexafluoride",
        composition: &[(16, 1), (9, 6)],
        molar_mass: 146.055,
        category: "inorganic",
        state_at_stp: "gas",
        melting_point_k: Some(222.45),
        boiling_point_k: Some(209.25),
        density_g_cm3: Some(0.006_17),
    }
}
