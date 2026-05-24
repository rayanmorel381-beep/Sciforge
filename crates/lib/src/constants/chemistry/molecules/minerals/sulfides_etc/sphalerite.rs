use super::super::super::Molecule;

pub fn molecule() -> Molecule {
    Molecule {
        formula: "ZnS",
        name: "sphalerite",
        composition: &[(16, 1), (30, 1)],
        molar_mass: 97.474,
        category: "sulfide",
        state_at_stp: "solid",
        melting_point_k: Some(2103.0),
        boiling_point_k: None,
        density_g_cm3: Some(4.050),
    }
}
