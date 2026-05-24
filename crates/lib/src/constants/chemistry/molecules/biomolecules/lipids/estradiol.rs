use super::super::super::Molecule;

pub fn molecule() -> Molecule {
    Molecule {
        formula: "C18H24O2",
        name: "estradiol",
        composition: &[(6, 18), (1, 24), (8, 2)],
        molar_mass: 272.382,
        category: "steroid hormone",
        state_at_stp: "solid",
        melting_point_k: Some(449.0),
        boiling_point_k: None,
        density_g_cm3: Some(1.270),
    }
}
