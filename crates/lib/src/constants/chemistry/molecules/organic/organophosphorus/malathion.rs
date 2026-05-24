use super::super::super::Molecule;

pub fn molecule() -> Molecule {
    Molecule {
        formula: "C10H19O6PS2",
        name: "malathion",
        composition: &[(6, 10), (1, 19), (8, 6), (15, 1), (16, 2)],
        molar_mass: 330.358,
        category: "organothiophosphate",
        state_at_stp: "liquid",
        melting_point_k: Some(276.0),
        boiling_point_k: Some(429.0),
        density_g_cm3: Some(1.230),
    }
}
