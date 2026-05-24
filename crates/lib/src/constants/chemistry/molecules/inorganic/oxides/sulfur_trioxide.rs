use super::super::super::Molecule;

pub fn molecule() -> Molecule {
    Molecule {
        formula: "SO3",
        name: "sulfur trioxide",
        composition: &[(8, 3), (16, 1)],
        molar_mass: 80.066,
        category: "oxide",
        state_at_stp: "liquid",
        melting_point_k: Some(289.95),
        boiling_point_k: Some(318.0),
        density_g_cm3: Some(1.920),
    }
}
