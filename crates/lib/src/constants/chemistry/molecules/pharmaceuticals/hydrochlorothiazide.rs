use super::super::Molecule;

pub fn molecule() -> Molecule {
    Molecule {
        formula: "C7H8ClN3O4S2",
        name: "hydrochlorothiazide",
        composition: &[(1, 8), (6, 7), (7, 3), (8, 4), (16, 2), (17, 1)],
        molar_mass: 297.740,
        category: "diuretic",
        state_at_stp: "solid",
        melting_point_k: Some(536.0),
        boiling_point_k: None,
        density_g_cm3: Some(1.700),
    }
}
