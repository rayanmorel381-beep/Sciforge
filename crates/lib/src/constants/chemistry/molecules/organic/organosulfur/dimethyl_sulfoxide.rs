use super::super::super::Molecule;

pub fn molecule() -> Molecule {
    Molecule {
        formula: "C2H6OS",
        name: "dimethyl sulfoxide",
        composition: &[(6, 2), (1, 6), (8, 1), (16, 1)],
        molar_mass: 78.135,
        category: "sulfoxide",
        state_at_stp: "liquid",
        melting_point_k: Some(291.7),
        boiling_point_k: Some(462.0),
        density_g_cm3: Some(1.100),
    }
}
