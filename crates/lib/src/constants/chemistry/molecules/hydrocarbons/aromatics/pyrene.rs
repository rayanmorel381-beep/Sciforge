use super::super::super::Molecule;

pub fn molecule() -> Molecule {
    Molecule {
        formula: "C16H10",
        name: "pyrene",
        composition: &[(6, 16), (1, 10)],
        molar_mass: 202.255,
        category: "polycyclic_aromatic",
        state_at_stp: "solid",
        melting_point_k: Some(423.8),
        boiling_point_k: Some(677.0),
        density_g_cm3: Some(1.271),
    }
}
