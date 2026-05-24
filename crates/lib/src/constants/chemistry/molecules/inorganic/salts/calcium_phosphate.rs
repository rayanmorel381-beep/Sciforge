use super::super::super::Molecule;

pub fn molecule() -> Molecule {
    Molecule {
        formula: "Ca3(PO4)2",
        name: "calcium phosphate",
        composition: &[(8, 8), (15, 2), (20, 3)],
        molar_mass: 310.180,
        category: "salt",
        state_at_stp: "solid",
        melting_point_k: Some(1943.0),
        boiling_point_k: None,
        density_g_cm3: Some(3.07),
    }
}
