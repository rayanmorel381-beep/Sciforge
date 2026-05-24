use super::super::super::Molecule;

pub fn molecule() -> Molecule {
    Molecule {
        formula: "Al2O3",
        name: "aluminum oxide",
        composition: &[(13, 2), (8, 3)],
        molar_mass: 101.961,
        category: "mineral",
        state_at_stp: "solid",
        melting_point_k: Some(2345.0),
        boiling_point_k: Some(3250.0),
        density_g_cm3: Some(3.987),
    }
}
