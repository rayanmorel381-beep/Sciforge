use super::super::super::Molecule;

pub fn molecule() -> Molecule {
    Molecule {
        formula: "Fe2SiO4",
        name: "fayalite",
        composition: &[(8, 4), (14, 1), (26, 2)],
        molar_mass: 203.774,
        category: "silicate",
        state_at_stp: "solid",
        melting_point_k: Some(1478.0),
        boiling_point_k: None,
        density_g_cm3: Some(4.390),
    }
}
