use super::super::super::Molecule;

pub fn molecule() -> Molecule {
    Molecule {
        formula: "Cr2O3",
        name: "chromium(III) oxide",
        composition: &[(8, 3), (24, 2)],
        molar_mass: 151.989,
        category: "oxide",
        state_at_stp: "solid",
        melting_point_k: Some(2708.0),
        boiling_point_k: Some(4273.0),
        density_g_cm3: Some(5.220),
    }
}
