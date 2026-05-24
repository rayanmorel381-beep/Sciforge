use super::super::super::Molecule;

pub fn molecule() -> Molecule {
    Molecule {
        formula: "N2O5",
        name: "dinitrogen pentoxide",
        composition: &[(7, 2), (8, 5)],
        molar_mass: 108.010,
        category: "oxide",
        state_at_stp: "solid",
        melting_point_k: Some(314.0),
        boiling_point_k: Some(320.0),
        density_g_cm3: Some(1.642),
    }
}
