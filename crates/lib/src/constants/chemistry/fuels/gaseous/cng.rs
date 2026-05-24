use super::super::Fuel;

pub fn fuel() -> Fuel {
    Fuel {
        name: "CNG",
        category: "gaseous",
        composition: "compressed natural gas, mostly CH4",
        density_kg_l: 0.000718,
        energy_density_mj_kg: 50.0,
        energy_density_mj_l: 0.0359,
        octane_ron: Some(120.0),
        cetane_number: None,
        flash_point_k: Some(85.0),
        autoignition_k: Some(810.0),
        state_at_stp: "gas",
    }
}
