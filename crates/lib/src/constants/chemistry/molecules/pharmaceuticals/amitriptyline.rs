use super::super::Molecule;

pub fn molecule() -> Molecule {
    Molecule {
        formula: "C20H23N",
        name: "amitriptyline",
        composition: &[(1, 23), (6, 20), (7, 1)],
        molar_mass: 277.408,
        category: "antidepressant",
        state_at_stp: "solid",
        melting_point_k: Some(469.0),
        boiling_point_k: None,
        density_g_cm3: Some(1.060),
    }
}
