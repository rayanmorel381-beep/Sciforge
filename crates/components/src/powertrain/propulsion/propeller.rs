#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PropellerType {
    FixedPitch,
    VariablePitch,
    ConstantSpeed,
    Contrarotating,
    Folding,
}

#[derive(Debug, Clone)]
pub struct Propeller {
    pub propeller_type: PropellerType,
    pub diameter_m: f64,
    pub blade_count: u8,
    pub material: PropellerMaterial,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PropellerMaterial {
    Wood,
    Aluminum,
    CompositeCarbon,
    Stainless,
    Bronze,
}

impl Propeller {
    pub fn fixed(diameter_m: f64, blade_count: u8, material: PropellerMaterial) -> Self {
        Self { propeller_type: PropellerType::FixedPitch, diameter_m, blade_count, material }
    }

    pub fn variable(diameter_m: f64, blade_count: u8, material: PropellerMaterial) -> Self {
        Self { propeller_type: PropellerType::VariablePitch, diameter_m, blade_count, material }
    }

    pub fn constant_speed(diameter_m: f64, blade_count: u8, material: PropellerMaterial) -> Self {
        Self { propeller_type: PropellerType::ConstantSpeed, diameter_m, blade_count, material }
    }

    pub fn contrarotating(diameter_m: f64, blade_count: u8) -> Self {
        Self { propeller_type: PropellerType::Contrarotating, diameter_m, blade_count, material: PropellerMaterial::CompositeCarbon }
    }
}
