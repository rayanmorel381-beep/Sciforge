#[derive(Debug, Clone, Copy)]
pub struct Restitution {
    pub material_a: &'static str,
    pub material_b: &'static str,
    pub coefficient: f64,
    pub condition: &'static str,
}

pub const TABLE: &[Restitution] = &[
    Restitution { material_a: "steel",    material_b: "steel",    coefficient: 0.78, condition: "ball_drop" },
    Restitution { material_a: "steel",    material_b: "Al",       coefficient: 0.60, condition: "ball_drop" },
    Restitution { material_a: "Al",       material_b: "Al",       coefficient: 0.55, condition: "ball_drop" },
    Restitution { material_a: "glass",    material_b: "glass",    coefficient: 0.95, condition: "ball_drop" },
    Restitution { material_a: "ivory",    material_b: "ivory",    coefficient: 0.89, condition: "billiard" },
    Restitution { material_a: "rubber",   material_b: "concrete", coefficient: 0.65, condition: "ball_drop" },
    Restitution { material_a: "rubber",   material_b: "steel",    coefficient: 0.85, condition: "ball_drop" },
    Restitution { material_a: "wood",     material_b: "wood",     coefficient: 0.50, condition: "ball_drop" },
    Restitution { material_a: "tennis",   material_b: "concrete", coefficient: 0.73, condition: "official" },
    Restitution { material_a: "basket",   material_b: "wood",     coefficient: 0.76, condition: "official" },
    Restitution { material_a: "golf",     material_b: "steel",    coefficient: 0.78, condition: "club_face" },
    Restitution { material_a: "baseball", material_b: "wood_bat", coefficient: 0.55, condition: "official" },
    Restitution { material_a: "lead",     material_b: "lead",     coefficient: 0.20, condition: "ball_drop" },
    Restitution { material_a: "clay",     material_b: "steel",    coefficient: 0.10, condition: "ball_drop" },
    Restitution { material_a: "ice",      material_b: "ice",      coefficient: 0.10, condition: "puck" },
];

pub fn by_pair(material_a: &str, material_b: &str) -> Option<&'static Restitution> {
    TABLE.iter().find(|r| {
        (r.material_a == material_a && r.material_b == material_b)
            || (r.material_a == material_b && r.material_b == material_a)
    })
}
