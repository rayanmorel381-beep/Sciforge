use crate::cooling::radiator::{
    Radiator, RadiatorMaterial,
};

pub fn standard() -> Radiator {
    Radiator::performance(RadiatorMaterial::Aluminum, 1.25)
}

pub fn race() -> Radiator {
    Radiator::performance(RadiatorMaterial::Aluminum, 1.55)
}
