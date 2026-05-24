use super::super::super::Molecule;

pub fn molecule() -> Molecule {
    Molecule {
        formula: "CuO",
        name: "copper(II) oxide",
        composition: &[(8, 1), (29, 1)],
        molar_mass: 79.545,
        category: "oxide",
        state_at_stp: "solid",
        melting_point_k: Some(1599.0),
        boiling_point_k: Some(2273.0),
        density_g_cm3: Some(6.310),
    }
}
