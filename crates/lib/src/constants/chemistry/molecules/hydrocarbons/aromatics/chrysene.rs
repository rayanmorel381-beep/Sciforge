use super::super::super::Molecule;

pub fn molecule() -> Molecule {
    Molecule {
        formula: "C18H12",
        name: "chrysene",
        composition: &[(6, 18), (1, 12)],
        molar_mass: 228.288,
        category: "polycyclic_aromatic",
        state_at_stp: "solid",
        melting_point_k: Some(528.0),
        boiling_point_k: Some(721.0),
        density_g_cm3: Some(1.274),
    }
}
