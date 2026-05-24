#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CranksetMaterial {
    Steel,
    Aluminum,
    CarbonFibre,
    Titanium,
}

#[derive(Debug, Clone)]
pub struct Crankset {
    pub material: CranksetMaterial,
    pub arm_length_mm: f64,
    pub chainring_count: u8,
    pub chainring_teeth: Vec<u8>,
    pub hollow_axle: bool,
}

impl Crankset {
    pub fn single(material: CranksetMaterial, arm_length_mm: f64, teeth: u8) -> Self {
        Self { material, arm_length_mm, chainring_count: 1, chainring_teeth: vec![teeth], hollow_axle: false }
    }

    pub fn double(material: CranksetMaterial, arm_length_mm: f64, teeth: [u8; 2]) -> Self {
        Self { material, arm_length_mm, chainring_count: 2, chainring_teeth: teeth.to_vec(), hollow_axle: true }
    }

    pub fn triple(material: CranksetMaterial, arm_length_mm: f64, teeth: [u8; 3]) -> Self {
        Self { material, arm_length_mm, chainring_count: 3, chainring_teeth: teeth.to_vec(), hollow_axle: false }
    }
}
