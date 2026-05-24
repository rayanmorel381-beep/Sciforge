use super::super::super::Molecule;

pub fn molecule() -> Molecule {
    Molecule {
        formula: "FeS2",
        name: "pyrite",
        composition: &[(16, 2), (26, 1)],
        molar_mass: 119.975,
        category: "sulfide",
        state_at_stp: "solid",
        melting_point_k: Some(1467.0),
        boiling_point_k: None,
        density_g_cm3: Some(5.010),
    }
}
