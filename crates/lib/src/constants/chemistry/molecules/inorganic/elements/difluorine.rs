use super::super::super::Molecule;

pub fn molecule() -> Molecule {
    Molecule {
        formula: "F2",
        name: "difluorine",
        composition: &[(9, 2)],
        molar_mass: 37.997,
        category: "inorganic",
        state_at_stp: "gas",
        melting_point_k: Some(53.48),
        boiling_point_k: Some(85.03),
        density_g_cm3: Some(0.001_696),
    }
}
