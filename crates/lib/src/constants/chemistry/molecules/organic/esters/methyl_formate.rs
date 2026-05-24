use super::super::super::Molecule;

pub fn molecule() -> Molecule {
    Molecule {
        formula: "HCOOCH3",
        name: "methyl formate",
        composition: &[(6, 2), (1, 4), (8, 2)],
        molar_mass: 60.052,
        category: "ester",
        state_at_stp: "liquid",
        melting_point_k: Some(174.0),
        boiling_point_k: Some(305.0),
        density_g_cm3: Some(0.974),
    }
}
