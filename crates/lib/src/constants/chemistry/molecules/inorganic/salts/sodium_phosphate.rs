use super::super::super::Molecule;

pub fn molecule() -> Molecule {
    Molecule {
        formula: "Na3PO4",
        name: "sodium phosphate",
        composition: &[(8, 4), (11, 3), (15, 1)],
        molar_mass: 163.940,
        category: "salt",
        state_at_stp: "solid",
        melting_point_k: Some(1856.0),
        boiling_point_k: None,
        density_g_cm3: Some(2.540),
    }
}
