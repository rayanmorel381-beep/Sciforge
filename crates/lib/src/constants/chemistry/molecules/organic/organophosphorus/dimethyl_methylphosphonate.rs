use super::super::super::Molecule;

pub fn molecule() -> Molecule {
    Molecule {
        formula: "C3H9O3P",
        name: "dimethyl methylphosphonate",
        composition: &[(6, 3), (1, 9), (8, 3), (15, 1)],
        molar_mass: 124.074,
        category: "organophosphonate",
        state_at_stp: "liquid",
        melting_point_k: Some(223.0),
        boiling_point_k: Some(454.0),
        density_g_cm3: Some(1.145),
    }
}
