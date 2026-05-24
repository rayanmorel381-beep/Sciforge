use super::super::super::Molecule;

pub fn molecule() -> Molecule {
    Molecule {
        formula: "C12H8",
        name: "acenaphthylene",
        composition: &[(6, 12), (1, 8)],
        molar_mass: 152.196,
        category: "polycyclic_aromatic",
        state_at_stp: "solid",
        melting_point_k: Some(365.2),
        boiling_point_k: Some(543.0),
        density_g_cm3: Some(0.899),
    }
}
