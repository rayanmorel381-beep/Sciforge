use super::super::super::Molecule;

pub fn molecule() -> Molecule {
    Molecule {
        formula: "LiH",
        name: "lithium hydride",
        composition: &[(1, 1), (3, 1)],
        molar_mass: 7.949,
        category: "hydride",
        state_at_stp: "solid",
        melting_point_k: Some(962.0),
        boiling_point_k: Some(1273.0),
        density_g_cm3: Some(0.780),
    }
}
