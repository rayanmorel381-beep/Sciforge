use super::super::super::Molecule;

pub fn molecule() -> Molecule {
    Molecule {
        formula: "K3PO4",
        name: "potassium phosphate",
        composition: &[(8, 4), (15, 1), (19, 3)],
        molar_mass: 212.270,
        category: "salt",
        state_at_stp: "solid",
        melting_point_k: Some(1653.0),
        boiling_point_k: None,
        density_g_cm3: Some(2.564),
    }
}
