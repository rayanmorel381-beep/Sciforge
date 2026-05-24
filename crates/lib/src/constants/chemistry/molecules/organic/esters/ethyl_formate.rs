use super::super::super::Molecule;

pub fn molecule() -> Molecule {
    Molecule {
        formula: "HCOOC2H5",
        name: "ethyl formate",
        composition: &[(6, 3), (1, 6), (8, 2)],
        molar_mass: 74.079,
        category: "ester",
        state_at_stp: "liquid",
        melting_point_k: Some(193.6),
        boiling_point_k: Some(327.4),
        density_g_cm3: Some(0.917),
    }
}
