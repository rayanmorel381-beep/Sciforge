#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CompressorType {
    BeltDriven,
    Electric,
    Variable,
}

#[derive(Debug, Clone)]
pub struct AcCompressor {
    pub compressor_type: CompressorType,
    pub displacement_cc: f64,
    pub refrigerant: Refrigerant,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Refrigerant {
    R134a,
    R1234yf,
    Co2R744,
}

impl AcCompressor {
    pub fn belt_driven(displacement_cc: f64) -> Self {
        Self { compressor_type: CompressorType::BeltDriven, displacement_cc, refrigerant: Refrigerant::R1234yf }
    }

    pub fn electric(displacement_cc: f64) -> Self {
        Self { compressor_type: CompressorType::Electric, displacement_cc, refrigerant: Refrigerant::R1234yf }
    }

    pub fn variable(displacement_cc: f64) -> Self {
        Self { compressor_type: CompressorType::Variable, displacement_cc, refrigerant: Refrigerant::R1234yf }
    }
}
