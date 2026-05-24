use super::super::super::Molecule;

pub fn molecule() -> Molecule {
    Molecule {
        formula: "HBr",
        name: "hydrobromic acid",
        composition: &[(1, 1), (35, 1)],
        molar_mass: 80.912,
        category: "acid",
        state_at_stp: "gas",
        melting_point_k: Some(186.3),
        boiling_point_k: Some(206.5),
        density_g_cm3: Some(3.307),
    }
}
