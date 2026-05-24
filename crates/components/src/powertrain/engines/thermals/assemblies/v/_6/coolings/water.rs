use crate::cooling::radiator::{
    Radiator, RadiatorMaterial,
};

pub fn standard() -> Radiator {
    Radiator::standard(RadiatorMaterial::Aluminum, 0.42)
}

pub fn electric_fan() -> Radiator {
    Radiator::electric_fan(RadiatorMaterial::Aluminum, 0.42)
}

pub fn copper_brass() -> Radiator {
    Radiator::standard(RadiatorMaterial::CopperBrass, 0.40)
}

pub fn performance() -> Radiator {
    Radiator::performance(RadiatorMaterial::Aluminum, 0.56)
}
