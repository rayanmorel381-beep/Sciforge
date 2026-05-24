use super::super::Molecule;

pub fn molecule() -> Molecule {
    Molecule {
        formula: "C8H10N4O2",
        name: "caffeine",
        composition: &[(6, 8), (1, 10), (7, 4), (8, 2)],
        molar_mass: 194.194,
        category: "pharmaceutical",
        state_at_stp: "solid",
        melting_point_k: Some(508.0),
        boiling_point_k: Some(451.0),
        density_g_cm3: Some(1.23),
    }
}
