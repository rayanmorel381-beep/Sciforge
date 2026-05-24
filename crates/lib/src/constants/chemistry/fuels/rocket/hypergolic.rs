use super::super::Fuel;

pub fn fuel() -> Fuel {
    Fuel {
        name: "hypergolic UDMH/N2O4",
        category: "rocket",
        composition: "unsymmetrical dimethylhydrazine with dinitrogen tetroxide",
        density_kg_l: 0.793,
        energy_density_mj_kg: 9.4,
        energy_density_mj_l: 7.45,
        octane_ron: None,
        cetane_number: None,
        flash_point_k: None,
        autoignition_k: None,
        state_at_stp: "liquid",
    }
}
