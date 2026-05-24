use super::super::super::Molecule;

pub fn molecule() -> Molecule {
    Molecule {
        formula: "TiO2",
        name: "titanium dioxide",
        composition: &[(22, 1), (8, 2)],
        molar_mass: 79.865,
        category: "mineral",
        state_at_stp: "solid",
        melting_point_k: Some(2116.0),
        boiling_point_k: Some(3245.0),
        density_g_cm3: Some(4.23),
    }
}
