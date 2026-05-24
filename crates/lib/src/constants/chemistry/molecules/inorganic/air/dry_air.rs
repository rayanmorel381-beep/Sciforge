use crate::constants::chemistry::molecules::Molecule;

pub fn molecule() -> Molecule {
    Molecule {
        formula: "AIR",
        name: "dry air",
        composition: &[(7, 156), (8, 42), (18, 1)],
        molar_mass: 28.9647,
        category: "mixture",
        state_at_stp: "gas",
        melting_point_k: None,
        boiling_point_k: Some(78.8),
        density_g_cm3: Some(0.001225),
    }
}
