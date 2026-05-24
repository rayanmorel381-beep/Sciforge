use super::super::super::Molecule;

pub fn molecule() -> Molecule {
    Molecule {
        formula: "MgCl2",
        name: "magnesium chloride",
        composition: &[(12, 1), (17, 2)],
        molar_mass: 95.211,
        category: "salt",
        state_at_stp: "solid",
        melting_point_k: Some(987.0),
        boiling_point_k: Some(1685.0),
        density_g_cm3: Some(2.320),
    }
}
