use super::super::super::Molecule;

pub fn molecule() -> Molecule {
    Molecule {
        formula: "C2H6S2",
        name: "dimethyl disulfide",
        composition: &[(6, 2), (1, 6), (16, 2)],
        molar_mass: 94.199,
        category: "disulfide",
        state_at_stp: "liquid",
        melting_point_k: Some(188.4),
        boiling_point_k: Some(382.9),
        density_g_cm3: Some(1.062),
    }
}
