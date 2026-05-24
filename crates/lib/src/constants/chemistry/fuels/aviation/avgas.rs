use super::super::Fuel;

pub fn fuel() -> Fuel {
    Fuel {
        name: "Avgas 100LL",
        category: "aviation",
        composition: "leaded aviation gasoline, RON 100",
        density_kg_l: 0.71,
        energy_density_mj_kg: 43.5,
        energy_density_mj_l: 30.9,
        octane_ron: Some(100.0),
        cetane_number: None,
        flash_point_k: Some(233.0),
        autoignition_k: Some(711.0),
        state_at_stp: "liquid",
    }
}
