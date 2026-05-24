use super::super::Fuel;

pub fn fuel() -> Fuel {
    Fuel {
        name: "Jet A-1",
        category: "aviation",
        composition: "kerosene-type aviation fuel",
        density_kg_l: 0.804,
        energy_density_mj_kg: 43.15,
        energy_density_mj_l: 34.7,
        octane_ron: None,
        cetane_number: Some(45.0),
        flash_point_k: Some(311.0),
        autoignition_k: Some(483.0),
        state_at_stp: "liquid",
    }
}
