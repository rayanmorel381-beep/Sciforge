use crate::constants::chemistry::molecules::Molecule;

pub fn molecule() -> Molecule {
    Molecule {
        formula: "AIR_HUMID",
        name: "humid air (50% RH, 293.15 K)",
        composition: &[(7, 154), (8, 41), (18, 1), (1, 2), (8, 1)],
        molar_mass: 28.838,
        category: "mixture",
        state_at_stp: "gas",
        melting_point_k: None,
        boiling_point_k: None,
        density_g_cm3: Some(0.001194),
    }
}
