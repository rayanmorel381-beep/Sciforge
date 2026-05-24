use super::super::super::Molecule;

pub fn molecule() -> Molecule {
    Molecule {
        formula: "CaMg(CO3)2",
        name: "dolomite",
        composition: &[(6, 2), (8, 6), (12, 1), (20, 1)],
        molar_mass: 184.401,
        category: "carbonate",
        state_at_stp: "solid",
        melting_point_k: None,
        boiling_point_k: None,
        density_g_cm3: Some(2.840),
    }
}
