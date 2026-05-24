use super::super::super::Molecule;

pub fn molecule() -> Molecule {
    Molecule {
        formula: "CF4",
        name: "tetrafluoromethane",
        composition: &[(6, 1), (9, 4)],
        molar_mass: 88.004,
        category: "halocarbon",
        state_at_stp: "gas",
        melting_point_k: Some(89.5),
        boiling_point_k: Some(145.2),
        density_g_cm3: Some(1.603),
    }
}
