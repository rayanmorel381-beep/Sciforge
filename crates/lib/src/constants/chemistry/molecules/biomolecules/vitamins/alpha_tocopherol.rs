use super::super::super::Molecule;

pub fn molecule() -> Molecule {
    Molecule {
        formula: "C29H50O2",
        name: "alpha-tocopherol",
        composition: &[(6, 29), (1, 50), (8, 2)],
        molar_mass: 430.717,
        category: "vitamin",
        state_at_stp: "liquid",
        melting_point_k: Some(276.0),
        boiling_point_k: Some(473.0),
        density_g_cm3: Some(0.950),
    }
}
