use super::super::super::Molecule;

pub fn molecule() -> Molecule {
    Molecule {
        formula: "C2H7N",
        name: "ethylamine",
        composition: &[(6, 2), (1, 7), (7, 1)],
        molar_mass: 45.085,
        category: "amine",
        state_at_stp: "gas",
        melting_point_k: Some(192.2),
        boiling_point_k: Some(289.7),
        density_g_cm3: Some(0.689),
    }
}
