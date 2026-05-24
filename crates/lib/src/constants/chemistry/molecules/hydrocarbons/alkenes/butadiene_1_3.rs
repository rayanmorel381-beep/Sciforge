use super::super::super::Molecule;

pub fn molecule() -> Molecule {
    Molecule {
        formula: "C4H6",
        name: "1,3-butadiene",
        composition: &[(6, 4), (1, 6)],
        molar_mass: 54.092,
        category: "diene",
        state_at_stp: "gas",
        melting_point_k: Some(164.25),
        boiling_point_k: Some(268.74),
        density_g_cm3: Some(0.615),
    }
}
