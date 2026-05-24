#[derive(Debug, Clone, Copy)]
pub struct WilsonParameters {
    pub component_a: &'static str,
    pub component_b: &'static str,
    pub lambda_ab_minus_aa_j_per_mol: f64,
    pub lambda_ba_minus_bb_j_per_mol: f64,
    pub molar_volume_a_cm3_per_mol: f64,
    pub molar_volume_b_cm3_per_mol: f64,
}

pub const WILSON_TABLE: &[WilsonParameters] = &[
    WilsonParameters { component_a: "C2H5OH", component_b: "H2O",   lambda_ab_minus_aa_j_per_mol:  276.76, lambda_ba_minus_bb_j_per_mol: 6014.07, molar_volume_a_cm3_per_mol: 58.68,  molar_volume_b_cm3_per_mol: 18.07 },
    WilsonParameters { component_a: "CH3OH",  component_b: "H2O",   lambda_ab_minus_aa_j_per_mol:  -88.48, lambda_ba_minus_bb_j_per_mol: 4034.86, molar_volume_a_cm3_per_mol: 40.73,  molar_volume_b_cm3_per_mol: 18.07 },
    WilsonParameters { component_a: "C3H6O",  component_b: "H2O",   lambda_ab_minus_aa_j_per_mol:  291.27, lambda_ba_minus_bb_j_per_mol: 6242.41, molar_volume_a_cm3_per_mol: 73.93,  molar_volume_b_cm3_per_mol: 18.07 },
    WilsonParameters { component_a: "CH3OH",  component_b: "C6H6",  lambda_ab_minus_aa_j_per_mol: 6052.75, lambda_ba_minus_bb_j_per_mol: 1734.05, molar_volume_a_cm3_per_mol: 40.73,  molar_volume_b_cm3_per_mol: 89.41 },
    WilsonParameters { component_a: "C2H5OH", component_b: "C6H6",  lambda_ab_minus_aa_j_per_mol: 6072.70, lambda_ba_minus_bb_j_per_mol: 1340.04, molar_volume_a_cm3_per_mol: 58.68,  molar_volume_b_cm3_per_mol: 89.41 },
    WilsonParameters { component_a: "C6H6",   component_b: "C7H8",  lambda_ab_minus_aa_j_per_mol:  150.10, lambda_ba_minus_bb_j_per_mol:   42.30, molar_volume_a_cm3_per_mol: 89.41,  molar_volume_b_cm3_per_mol: 106.85 },
    WilsonParameters { component_a: "C6H14",  component_b: "C7H16", lambda_ab_minus_aa_j_per_mol:   53.11, lambda_ba_minus_bb_j_per_mol:   17.40, molar_volume_a_cm3_per_mol: 131.51, molar_volume_b_cm3_per_mol: 147.47 },
    WilsonParameters { component_a: "C3H6O",  component_b: "CHCl3", lambda_ab_minus_aa_j_per_mol:-1631.05, lambda_ba_minus_bb_j_per_mol:-1370.10, molar_volume_a_cm3_per_mol: 73.93,  molar_volume_b_cm3_per_mol: 80.67 },
];

impl WilsonParameters {
    pub fn lambda_ab(&self, temperature_k: f64) -> f64 {
        let r = 8.314_462_618;
        (self.molar_volume_b_cm3_per_mol / self.molar_volume_a_cm3_per_mol)
            * (-self.lambda_ab_minus_aa_j_per_mol / (r * temperature_k)).exp()
    }

    pub fn lambda_ba(&self, temperature_k: f64) -> f64 {
        let r = 8.314_462_618;
        (self.molar_volume_a_cm3_per_mol / self.molar_volume_b_cm3_per_mol)
            * (-self.lambda_ba_minus_bb_j_per_mol / (r * temperature_k)).exp()
    }

    pub fn activity_coeff_a(&self, x_a: f64, temperature_k: f64) -> f64 {
        let x_b = 1.0 - x_a;
        let lab = self.lambda_ab(temperature_k);
        let lba = self.lambda_ba(temperature_k);
        let term1 = -(x_a + x_b * lab).ln();
        let term2 = x_b * (lab / (x_a + x_b * lab) - lba / (x_b + x_a * lba));
        (term1 + term2).exp()
    }

    pub fn activity_coeff_b(&self, x_a: f64, temperature_k: f64) -> f64 {
        let x_b = 1.0 - x_a;
        let lab = self.lambda_ab(temperature_k);
        let lba = self.lambda_ba(temperature_k);
        let term1 = -(x_b + x_a * lba).ln();
        let term2 = x_a * (lba / (x_b + x_a * lba) - lab / (x_a + x_b * lab));
        (term1 + term2).exp()
    }
}

pub fn wilson_by_pair(a: &str, b: &str) -> Option<&'static WilsonParameters> {
    WILSON_TABLE.iter().find(|w| (w.component_a == a && w.component_b == b)
        || (w.component_a == b && w.component_b == a))
}
