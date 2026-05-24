use super::super::Fuel;

pub fn fuel() -> Fuel {
    Fuel {
        name: "essence 98",
        category: "petroleum",
        composition: "C5-C12 hydrocarbons, RON 98",
        density_kg_l: 0.755,
        energy_density_mj_kg: 46.4,
        energy_density_mj_l: 35.0,
        octane_ron: Some(98.0),
        cetane_number: None,
        flash_point_k: Some(230.0),
        autoignition_k: Some(553.0),
        state_at_stp: "liquid",
    }
}
