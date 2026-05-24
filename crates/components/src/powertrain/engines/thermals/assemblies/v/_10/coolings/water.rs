use crate::cooling::radiator::{
    Radiator, RadiatorMaterial,
};

pub fn standard() -> Radiator {
    Radiator::standard(RadiatorMaterial::Aluminum, 0.62)
}

pub fn performance() -> Radiator {
    Radiator::performance(RadiatorMaterial::Aluminum, 0.78)
}
