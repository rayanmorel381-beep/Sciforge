use super::super::Fuel;

pub fn fuel() -> Fuel {
    Fuel {
        name: "LNG",
        category: "gaseous",
        composition: "liquefied natural gas, CH4 at 112 K",
        density_kg_l: 0.42,
        energy_density_mj_kg: 50.0,
        energy_density_mj_l: 21.0,
        octane_ron: Some(120.0),
        cetane_number: None,
        flash_point_k: Some(85.0),
        autoignition_k: Some(810.0),
        state_at_stp: "liquid",
    }
}
