use super::super::super::Molecule;

pub fn molecule() -> Molecule {
    Molecule {
        formula: "C16H32O2",
        name: "palmitic acid",
        composition: &[(6, 16), (1, 32), (8, 2)],
        molar_mass: 256.424,
        category: "fatty acid",
        state_at_stp: "solid",
        melting_point_k: Some(335.7),
        boiling_point_k: Some(624.0),
        density_g_cm3: Some(0.852),
    }
}
