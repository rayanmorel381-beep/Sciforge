use sciforge_core::materials::Material;
use sciforge_core::materials::alus::cast::{A356, AC4B};
use sciforge_core::materials::ceramics::coatings::ALUMINA_AL2O3;
use sciforge_core::materials::irons::cast_iron::CGI_400;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum BankArrangement {
    SingleBank,
    DualBank,
    QuadBank,
}

#[derive(Debug, Clone)]
pub struct CylinderBank {
    pub bank_arrangement: BankArrangement,
    pub cylinder_count: u8,
    pub cylinders_per_bank: u8,
    pub bore_mm: f64,
    pub compression_ratio: f64,
    pub valves_per_cylinder: u8,
    pub nikasil_lined: bool,
}

impl CylinderBank {
    pub fn block_material(&self) -> &'static Material {
        if matches!(self.bank_arrangement, BankArrangement::QuadBank) || self.cylinders_per_bank >= 6 {
            &CGI_400
        } else {
            &AC4B
        }
    }

    pub fn liner_material(&self) -> &'static Material {
        if self.nikasil_lined {
            &ALUMINA_AL2O3
        } else {
            &CGI_400
        }
    }

    pub fn head_material(&self) -> &'static Material {
        if matches!(self.bank_arrangement, BankArrangement::QuadBank) && self.cylinder_count >= 12 {
            &CGI_400
        } else if self.cylinder_count <= 4 {
            &AC4B
        } else {
            &A356
        }
    }

    pub fn inline(count: u8, bore_mm: f64, compression_ratio: f64) -> Self {
        Self {
            bank_arrangement: BankArrangement::SingleBank,
            cylinder_count: count,
            cylinders_per_bank: count,
            bore_mm,
            compression_ratio,
            valves_per_cylinder: 4,
            nikasil_lined: false,
        }
    }

    pub fn v(count: u8, bore_mm: f64, compression_ratio: f64) -> Self {
        Self {
            bank_arrangement: BankArrangement::DualBank,
            cylinder_count: count,
            cylinders_per_bank: count / 2,
            bore_mm,
            compression_ratio,
            valves_per_cylinder: 4,
            nikasil_lined: false,
        }
    }

    pub fn w(count: u8, bore_mm: f64, compression_ratio: f64) -> Self {
        Self {
            bank_arrangement: BankArrangement::QuadBank,
            cylinder_count: count,
            cylinders_per_bank: count / 4,
            bore_mm,
            compression_ratio,
            valves_per_cylinder: 4,
            nikasil_lined: true,
        }
    }

    pub fn h(count: u8, bore_mm: f64, compression_ratio: f64) -> Self {
        Self {
            bank_arrangement: BankArrangement::QuadBank,
            cylinder_count: count,
            cylinders_per_bank: count / 4,
            bore_mm,
            compression_ratio,
            valves_per_cylinder: 4,
            nikasil_lined: false,
        }
    }

    pub fn x(count: u8, bore_mm: f64, compression_ratio: f64) -> Self {
        Self {
            bank_arrangement: BankArrangement::QuadBank,
            cylinder_count: count,
            cylinders_per_bank: count / 4,
            bore_mm,
            compression_ratio,
            valves_per_cylinder: 4,
            nikasil_lined: true,
        }
    }

    pub fn flat(count: u8, bore_mm: f64, compression_ratio: f64) -> Self {
        Self {
            bank_arrangement: BankArrangement::DualBank,
            cylinder_count: count,
            cylinders_per_bank: count / 2,
            bore_mm,
            compression_ratio,
            valves_per_cylinder: 4,
            nikasil_lined: false,
        }
    }

    pub fn radial(count: u8, bore_mm: f64, compression_ratio: f64) -> Self {
        Self {
            bank_arrangement: BankArrangement::SingleBank,
            cylinder_count: count,
            cylinders_per_bank: count,
            bore_mm,
            compression_ratio,
            valves_per_cylinder: 2,
            nikasil_lined: false,
        }
    }
}
