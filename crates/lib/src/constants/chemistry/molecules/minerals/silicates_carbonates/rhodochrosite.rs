use super::super::super::Molecule;

pub fn molecule() -> Molecule {
    Molecule {
        formula: "MnCO3",
        name: "rhodochrosite",
        composition: &[(6, 1), (8, 3), (25, 1)],
        molar_mass: 114.947,
        category: "carbonate",
        state_at_stp: "solid",
        melting_point_k: None,
        boiling_point_k: None,
        density_g_cm3: Some(3.690),
    }
}
