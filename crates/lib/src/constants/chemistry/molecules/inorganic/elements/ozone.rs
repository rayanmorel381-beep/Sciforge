use super::super::super::Molecule;

pub fn molecule() -> Molecule {
    Molecule {
        formula: "O3",
        name: "ozone",
        composition: &[(8, 3)],
        molar_mass: 47.997,
        category: "inorganic",
        state_at_stp: "gas",
        melting_point_k: Some(80.7),
        boiling_point_k: Some(161.3),
        density_g_cm3: Some(0.002_144),
    }
}
