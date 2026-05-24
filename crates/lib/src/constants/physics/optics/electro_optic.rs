#[derive(Debug, Clone, Copy)]
pub struct ElectroOptic {
    pub formula: &'static str,
    pub pockels_r_pm_per_v: f64,
    pub kerr_n2_m2_per_w: f64,
    pub n_ref: f64,
}

pub const TABLE: &[ElectroOptic] = &[
    ElectroOptic { formula: "LiNbO3", pockels_r_pm_per_v: 30.8, kerr_n2_m2_per_w: 1.8e-19, n_ref: 2.286 },
    ElectroOptic { formula: "KDP",    pockels_r_pm_per_v: 10.5, kerr_n2_m2_per_w: 2.0e-20, n_ref: 1.510 },
    ElectroOptic { formula: "BBO",    pockels_r_pm_per_v: 2.7,  kerr_n2_m2_per_w: 4.5e-20, n_ref: 1.655 },
    ElectroOptic { formula: "KTP",    pockels_r_pm_per_v: 36.3, kerr_n2_m2_per_w: 2.4e-19, n_ref: 1.840 },
    ElectroOptic { formula: "BaTiO3", pockels_r_pm_per_v: 730.0,kerr_n2_m2_per_w: 0.0,     n_ref: 2.420 },
    ElectroOptic { formula: "SiO2",   pockels_r_pm_per_v: 0.0,  kerr_n2_m2_per_w: 2.7e-20, n_ref: 1.458 },
    ElectroOptic { formula: "Si",     pockels_r_pm_per_v: 0.0,  kerr_n2_m2_per_w: 4.5e-18, n_ref: 3.480 },
    ElectroOptic { formula: "GaAs",   pockels_r_pm_per_v: 1.43, kerr_n2_m2_per_w: 1.5e-17, n_ref: 3.470 },
    ElectroOptic { formula: "RTP",    pockels_r_pm_per_v: 38.5, kerr_n2_m2_per_w: 2.4e-19, n_ref: 1.769 },
];

pub fn by_formula(formula: &str) -> Option<&'static ElectroOptic> {
    TABLE.iter().find(|e| e.formula == formula)
}
