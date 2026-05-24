use super::super::Fuel;

pub fn fuel() -> Fuel {
    Fuel {
        name: "HFO",
        category: "marine",
        composition: "heavy fuel oil, residual C20-C50",
        density_kg_l: 0.99,
        energy_density_mj_kg: 41.5,
        energy_density_mj_l: 41.1,
        octane_ron: None,
        cetane_number: None,
        flash_point_k: Some(335.0),
        autoignition_k: Some(523.0),
        state_at_stp: "liquid",
    }
}
