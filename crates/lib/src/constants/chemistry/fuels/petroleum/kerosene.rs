use super::super::Fuel;

pub fn fuel() -> Fuel {
    Fuel {
        name: "kerosene",
        category: "petroleum",
        composition: "C9-C16 hydrocarbons",
        density_kg_l: 0.810,
        energy_density_mj_kg: 43.0,
        energy_density_mj_l: 34.8,
        octane_ron: None,
        cetane_number: Some(45.0),
        flash_point_k: Some(311.0),
        autoignition_k: Some(493.0),
        state_at_stp: "liquid",
    }
}
