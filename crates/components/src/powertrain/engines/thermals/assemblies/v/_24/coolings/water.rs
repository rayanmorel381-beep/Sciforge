use crate::cooling::radiator::{
    Radiator, RadiatorMaterial,
};

pub fn standard() -> Radiator {
    Radiator::standard(RadiatorMaterial::Aluminum, 1.26)
}

pub fn performance() -> Radiator {
    Radiator::performance(RadiatorMaterial::Aluminum, 1.54)
}
