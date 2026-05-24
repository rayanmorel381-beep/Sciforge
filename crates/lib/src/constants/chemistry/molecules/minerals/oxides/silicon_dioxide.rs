use super::super::super::Molecule;

pub fn molecule() -> Molecule {
    Molecule {
        formula: "SiO2",
        name: "silicon dioxide",
        composition: &[(14, 1), (8, 2)],
        molar_mass: 60.083,
        category: "mineral",
        state_at_stp: "solid",
        melting_point_k: Some(1986.0),
        boiling_point_k: Some(3220.0),
        density_g_cm3: Some(2.648),
    }
}
