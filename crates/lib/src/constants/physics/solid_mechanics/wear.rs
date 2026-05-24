#[derive(Debug, Clone, Copy)]
pub struct WearPair {
    pub material_a: &'static str,
    pub material_b: &'static str,
    pub archard_k: f64,
    pub condition: &'static str,
}

pub const TABLE: &[WearPair] = &[
    WearPair { material_a: "steel",    material_b: "steel",    archard_k: 7.0e-4,  condition: "dry" },
    WearPair { material_a: "steel",    material_b: "steel",    archard_k: 1.7e-7,  condition: "lubricated" },
    WearPair { material_a: "steel",    material_b: "Cu",       archard_k: 1.5e-4,  condition: "dry" },
    WearPair { material_a: "steel",    material_b: "brass",    archard_k: 6.0e-4,  condition: "dry" },
    WearPair { material_a: "steel",    material_b: "Al",       archard_k: 5.0e-4,  condition: "dry" },
    WearPair { material_a: "steel",    material_b: "PTFE",     archard_k: 2.5e-5,  condition: "dry" },
    WearPair { material_a: "steel",    material_b: "PE",       archard_k: 1.3e-7,  condition: "dry" },
    WearPair { material_a: "Cu",       material_b: "Cu",       archard_k: 1.1e-2,  condition: "dry" },
    WearPair { material_a: "Al",       material_b: "Al",       archard_k: 5.0e-3,  condition: "dry" },
    WearPair { material_a: "Ti_6Al_4V",material_b: "steel",    archard_k: 1.4e-3,  condition: "dry" },
    WearPair { material_a: "WC",       material_b: "steel",    archard_k: 1.0e-6,  condition: "dry" },
    WearPair { material_a: "Al2O3",    material_b: "steel",    archard_k: 1.3e-7,  condition: "dry" },
    WearPair { material_a: "rubber",   material_b: "steel",    archard_k: 1.0e-3,  condition: "dry" },
    WearPair { material_a: "PTFE",     material_b: "PTFE",     archard_k: 2.0e-5,  condition: "dry" },
];

pub fn by_pair(material_a: &str, material_b: &str, condition: &str) -> Option<&'static WearPair> {
    TABLE.iter().find(|p| {
        ((p.material_a == material_a && p.material_b == material_b)
            || (p.material_a == material_b && p.material_b == material_a))
            && p.condition == condition
    })
}
