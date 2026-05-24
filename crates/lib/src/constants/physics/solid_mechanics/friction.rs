#[derive(Debug, Clone, Copy)]
pub struct FrictionPair {
    pub material_a: &'static str,
    pub material_b: &'static str,
    pub mu_static: f64,
    pub mu_kinetic: f64,
    pub condition: &'static str,
}

pub const TABLE: &[FrictionPair] = &[
    FrictionPair { material_a: "steel",    material_b: "steel",    mu_static: 0.74, mu_kinetic: 0.57, condition: "dry" },
    FrictionPair { material_a: "steel",    material_b: "steel",    mu_static: 0.16, mu_kinetic: 0.06, condition: "lubricated" },
    FrictionPair { material_a: "steel",    material_b: "Al",       mu_static: 0.61, mu_kinetic: 0.47, condition: "dry" },
    FrictionPair { material_a: "steel",    material_b: "Cu",       mu_static: 0.53, mu_kinetic: 0.36, condition: "dry" },
    FrictionPair { material_a: "steel",    material_b: "brass",    mu_static: 0.51, mu_kinetic: 0.44, condition: "dry" },
    FrictionPair { material_a: "steel",    material_b: "PTFE",     mu_static: 0.04, mu_kinetic: 0.04, condition: "dry" },
    FrictionPair { material_a: "steel",    material_b: "ice",      mu_static: 0.03, mu_kinetic: 0.02, condition: "dry" },
    FrictionPair { material_a: "steel",    material_b: "wood",     mu_static: 0.50, mu_kinetic: 0.30, condition: "dry" },
    FrictionPair { material_a: "Al",       material_b: "Al",       mu_static: 1.05, mu_kinetic: 1.40, condition: "dry" },
    FrictionPair { material_a: "Al",       material_b: "Al",       mu_static: 0.30, mu_kinetic: 0.20, condition: "lubricated" },
    FrictionPair { material_a: "Cu",       material_b: "Cu",       mu_static: 1.21, mu_kinetic: 1.00, condition: "dry" },
    FrictionPair { material_a: "wood",     material_b: "wood",     mu_static: 0.50, mu_kinetic: 0.30, condition: "dry" },
    FrictionPair { material_a: "wood",     material_b: "wood",     mu_static: 0.20, mu_kinetic: 0.20, condition: "wet" },
    FrictionPair { material_a: "rubber",   material_b: "asphalt",  mu_static: 0.90, mu_kinetic: 0.70, condition: "dry" },
    FrictionPair { material_a: "rubber",   material_b: "asphalt",  mu_static: 0.50, mu_kinetic: 0.25, condition: "wet" },
    FrictionPair { material_a: "rubber",   material_b: "concrete", mu_static: 1.00, mu_kinetic: 0.80, condition: "dry" },
    FrictionPair { material_a: "rubber",   material_b: "ice",      mu_static: 0.15, mu_kinetic: 0.10, condition: "dry" },
    FrictionPair { material_a: "elastomer",material_b: "glass",    mu_static: 0.80, mu_kinetic: 0.60, condition: "dry" },
    FrictionPair { material_a: "PTFE",     material_b: "PTFE",     mu_static: 0.04, mu_kinetic: 0.04, condition: "dry" },
    FrictionPair { material_a: "leather",  material_b: "wood",     mu_static: 0.50, mu_kinetic: 0.40, condition: "dry" },
    FrictionPair { material_a: "leather",  material_b: "metal",    mu_static: 0.60, mu_kinetic: 0.50, condition: "dry" },
    FrictionPair { material_a: "ice",      material_b: "ice",      mu_static: 0.10, mu_kinetic: 0.03, condition: "dry" },
    FrictionPair { material_a: "Cu_brass", material_b: "steel",    mu_static: 0.51, mu_kinetic: 0.44, condition: "dry" },
    FrictionPair { material_a: "graphite", material_b: "steel",    mu_static: 0.10, mu_kinetic: 0.10, condition: "dry" },
];

pub fn by_pair(material_a: &str, material_b: &str, condition: &str) -> Option<&'static FrictionPair> {
    TABLE.iter().find(|p| {
        ((p.material_a == material_a && p.material_b == material_b)
            || (p.material_a == material_b && p.material_b == material_a))
            && p.condition == condition
    })
}
