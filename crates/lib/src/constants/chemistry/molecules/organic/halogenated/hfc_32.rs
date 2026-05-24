use super::super::super::Molecule;

pub fn molecule() -> Molecule {
    Molecule {
        formula: "CH2F2",
        name: "difluoromethane",
        composition: &[(6, 1), (1, 2), (9, 2)],
        molar_mass: 52.024,
        category: "halocarbon",
        state_at_stp: "gas",
        melting_point_k: Some(136.0),
        boiling_point_k: Some(221.5),
        density_g_cm3: Some(0.961),
    }
}
