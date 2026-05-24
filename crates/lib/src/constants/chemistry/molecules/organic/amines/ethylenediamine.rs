use super::super::super::Molecule;

pub fn molecule() -> Molecule {
    Molecule {
        formula: "C2H8N2",
        name: "ethylenediamine",
        composition: &[(6, 2), (1, 8), (7, 2)],
        molar_mass: 60.099,
        category: "amine",
        state_at_stp: "liquid",
        melting_point_k: Some(282.0),
        boiling_point_k: Some(391.0),
        density_g_cm3: Some(0.899),
    }
}
