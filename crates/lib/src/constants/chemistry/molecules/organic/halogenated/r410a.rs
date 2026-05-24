use super::super::super::Molecule;

pub fn molecule() -> Molecule {
    Molecule {
        formula: "R410A",
        name: "R-410A (50/50 mass blend R-32 + R-125)",
        composition: &[(6, 13), (1, 17), (9, 29)],
        molar_mass: 72.585,
        category: "refrigerant_blend",
        state_at_stp: "gas",
        melting_point_k: None,
        boiling_point_k: Some(221.7),
        density_g_cm3: Some(0.002967),
    }
}
