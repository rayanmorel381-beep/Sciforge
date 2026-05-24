use super::super::Molecule;

pub fn molecule() -> Molecule {
    Molecule {
        formula: "C15H22O5",
        name: "artemisinin",
        composition: &[(1, 22), (6, 15), (8, 5)],
        molar_mass: 282.332,
        category: "antimalarial",
        state_at_stp: "solid",
        melting_point_k: Some(429.0),
        boiling_point_k: None,
        density_g_cm3: Some(1.240),
    }
}
