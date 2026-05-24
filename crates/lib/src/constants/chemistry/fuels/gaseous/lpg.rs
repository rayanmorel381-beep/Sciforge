use super::super::Fuel;

pub fn fuel() -> Fuel {
    Fuel {
        name: "LPG",
        category: "gaseous",
        composition: "propane and butane mixture",
        density_kg_l: 0.55,
        energy_density_mj_kg: 46.1,
        energy_density_mj_l: 25.4,
        octane_ron: Some(105.0),
        cetane_number: None,
        flash_point_k: Some(169.0),
        autoignition_k: Some(723.0),
        state_at_stp: "liquid",
    }
}
