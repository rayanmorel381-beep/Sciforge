use super::super::super::Molecule;

pub fn molecule() -> Molecule {
    Molecule {
        formula: "CePO4",
        name: "monazite",
        composition: &[(8, 4), (15, 1), (58, 1)],
        molar_mass: 235.094,
        category: "phosphate",
        state_at_stp: "solid",
        melting_point_k: None,
        boiling_point_k: None,
        density_g_cm3: Some(5.150),
    }
}
