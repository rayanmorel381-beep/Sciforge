use super::super::super::Molecule;

pub fn molecule() -> Molecule {
    Molecule {
        formula: "C2H6O",
        name: "dimethyl ether",
        composition: &[(6, 2), (1, 6), (8, 1)],
        molar_mass: 46.069,
        category: "ether",
        state_at_stp: "gas",
        melting_point_k: Some(131.6),
        boiling_point_k: Some(248.3),
        density_g_cm3: Some(0.668),
    }
}
