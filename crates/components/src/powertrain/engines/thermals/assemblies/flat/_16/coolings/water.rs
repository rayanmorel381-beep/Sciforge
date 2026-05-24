use crate::cooling::radiator::{
    Radiator, RadiatorMaterial,
};

pub fn standard() -> Radiator {
    Radiator::standard(RadiatorMaterial::Aluminum, 0.82)
}

pub fn electric_fan() -> Radiator {
    Radiator::electric_fan(RadiatorMaterial::Aluminum, 0.82)
}

pub fn copper_brass() -> Radiator {
    Radiator::standard(RadiatorMaterial::CopperBrass, 0.8)
}

pub fn performance() -> Radiator {
    Radiator::performance(RadiatorMaterial::Aluminum, 1.06)
}
