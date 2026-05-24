use crate::cooling::radiator::{
    Radiator, RadiatorMaterial,
};

pub fn standard() -> Radiator {
    Radiator::standard(RadiatorMaterial::Aluminum, 0.50)
}

pub fn electric_fan() -> Radiator {
    Radiator::electric_fan(RadiatorMaterial::Aluminum, 0.50)
}

pub fn copper_brass() -> Radiator {
    Radiator::standard(RadiatorMaterial::CopperBrass, 0.48)
}

pub fn performance() -> Radiator {
    Radiator::performance(RadiatorMaterial::Aluminum, 0.66)
}
