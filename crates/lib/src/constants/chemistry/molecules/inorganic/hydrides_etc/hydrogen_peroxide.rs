use super::super::super::Molecule;

pub fn molecule() -> Molecule {
    Molecule {
        formula: "H2O2",
        name: "hydrogen peroxide",
        composition: &[(1, 2), (8, 2)],
        molar_mass: 34.014,
        category: "peroxide",
        state_at_stp: "liquid",
        melting_point_k: Some(272.74),
        boiling_point_k: Some(423.35),
        density_g_cm3: Some(1.450),
    }
}
