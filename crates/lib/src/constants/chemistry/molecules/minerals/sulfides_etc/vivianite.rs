use super::super::super::Molecule;

pub fn molecule() -> Molecule {
    Molecule {
        formula: "Fe3(PO4)2·8H2O",
        name: "vivianite",
        composition: &[(1, 16), (8, 16), (15, 2), (26, 3)],
        molar_mass: 501.609,
        category: "phosphate",
        state_at_stp: "solid",
        melting_point_k: None,
        boiling_point_k: None,
        density_g_cm3: Some(2.690),
    }
}
