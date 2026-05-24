use super::super::super::Molecule;

pub fn molecule() -> Molecule {
    Molecule {
        formula: "Mg2SiO4",
        name: "forsterite",
        composition: &[(8, 4), (12, 2), (14, 1)],
        molar_mass: 140.693,
        category: "silicate",
        state_at_stp: "solid",
        melting_point_k: Some(2163.0),
        boiling_point_k: None,
        density_g_cm3: Some(3.270),
    }
}
