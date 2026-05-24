use super::super::super::Molecule;

pub fn molecule() -> Molecule {
    Molecule {
        formula: "C21H28O5",
        name: "aldosterone",
        composition: &[(6, 21), (1, 28), (8, 5)],
        molar_mass: 360.444,
        category: "steroid hormone",
        state_at_stp: "solid",
        melting_point_k: Some(381.0),
        boiling_point_k: None,
        density_g_cm3: Some(1.270),
    }
}
