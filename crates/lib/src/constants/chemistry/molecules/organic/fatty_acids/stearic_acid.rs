use super::super::super::Molecule;

pub fn molecule() -> Molecule {
    Molecule {
        formula: "C18H36O2",
        name: "stearic acid",
        composition: &[(6, 18), (1, 36), (8, 2)],
        molar_mass: 284.477,
        category: "fatty acid",
        state_at_stp: "solid",
        melting_point_k: Some(342.6),
        boiling_point_k: Some(656.0),
        density_g_cm3: Some(0.847),
    }
}
