use super::super::super::Molecule;

pub fn molecule() -> Molecule {
    Molecule {
        formula: "C7H7NO",
        name: "benzamide",
        composition: &[(6, 7), (1, 7), (7, 1), (8, 1)],
        molar_mass: 121.139,
        category: "amide",
        state_at_stp: "solid",
        melting_point_k: Some(403.0),
        boiling_point_k: Some(563.0),
        density_g_cm3: Some(1.341),
    }
}
