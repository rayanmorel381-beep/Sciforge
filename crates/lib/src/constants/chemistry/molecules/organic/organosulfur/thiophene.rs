use super::super::super::Molecule;

pub fn molecule() -> Molecule {
    Molecule {
        formula: "C4H4S",
        name: "thiophene",
        composition: &[(6, 4), (1, 4), (16, 1)],
        molar_mass: 84.140,
        category: "heterocyclic sulfur",
        state_at_stp: "liquid",
        melting_point_k: Some(234.9),
        boiling_point_k: Some(357.3),
        density_g_cm3: Some(1.051),
    }
}
