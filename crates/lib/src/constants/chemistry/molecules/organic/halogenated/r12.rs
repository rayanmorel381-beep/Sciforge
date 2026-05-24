use super::super::super::Molecule;

pub fn molecule() -> Molecule {
    Molecule {
        formula: "CCl2F2",
        name: "dichlorodifluoromethane",
        composition: &[(6, 1), (9, 2), (17, 2)],
        molar_mass: 120.913,
        category: "halocarbon",
        state_at_stp: "gas",
        melting_point_k: Some(115.0),
        boiling_point_k: Some(243.4),
        density_g_cm3: Some(1.486),
    }
}
