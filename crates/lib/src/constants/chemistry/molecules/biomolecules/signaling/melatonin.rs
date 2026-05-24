use super::super::super::Molecule;

pub fn molecule() -> Molecule {
    Molecule {
        formula: "C13H16N2O2",
        name: "melatonin",
        composition: &[(6, 13), (1, 16), (7, 2), (8, 2)],
        molar_mass: 232.281,
        category: "hormone",
        state_at_stp: "solid",
        melting_point_k: Some(390.0),
        boiling_point_k: Some(785.0),
        density_g_cm3: Some(1.180),
    }
}
