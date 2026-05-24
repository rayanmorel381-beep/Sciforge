#[derive(Debug, Clone, Copy)]
pub struct Bandgap {
    pub formula: &'static str,
    pub eg_ev: f64,
    pub kind: &'static str,
    pub temperature_k: f64,
    pub varshni_alpha_ev_per_k: f64,
    pub varshni_beta_k: f64,
}

pub const TABLE: &[Bandgap] = &[
    Bandgap { formula: "Si",    eg_ev: 1.12,  kind: "indirect", temperature_k: 300.0, varshni_alpha_ev_per_k: 4.73e-4, varshni_beta_k: 636.0  },
    Bandgap { formula: "Ge",    eg_ev: 0.66,  kind: "indirect", temperature_k: 300.0, varshni_alpha_ev_per_k: 4.77e-4, varshni_beta_k: 235.0  },
    Bandgap { formula: "C",     eg_ev: 5.47,  kind: "indirect", temperature_k: 300.0, varshni_alpha_ev_per_k: 0.0,     varshni_beta_k: 0.0    },
    Bandgap { formula: "GaAs",  eg_ev: 1.42,  kind: "direct",   temperature_k: 300.0, varshni_alpha_ev_per_k: 5.405e-4,varshni_beta_k: 204.0  },
    Bandgap { formula: "GaP",   eg_ev: 2.26,  kind: "indirect", temperature_k: 300.0, varshni_alpha_ev_per_k: 5.771e-4,varshni_beta_k: 372.0  },
    Bandgap { formula: "GaN",   eg_ev: 3.40,  kind: "direct",   temperature_k: 300.0, varshni_alpha_ev_per_k: 9.09e-4, varshni_beta_k: 830.0  },
    Bandgap { formula: "InAs",  eg_ev: 0.36,  kind: "direct",   temperature_k: 300.0, varshni_alpha_ev_per_k: 2.76e-4, varshni_beta_k: 93.0   },
    Bandgap { formula: "InP",   eg_ev: 1.35,  kind: "direct",   temperature_k: 300.0, varshni_alpha_ev_per_k: 4.906e-4,varshni_beta_k: 327.0  },
    Bandgap { formula: "InSb",  eg_ev: 0.17,  kind: "direct",   temperature_k: 300.0, varshni_alpha_ev_per_k: 3.20e-4, varshni_beta_k: 170.0  },
    Bandgap { formula: "AlAs",  eg_ev: 2.17,  kind: "indirect", temperature_k: 300.0, varshni_alpha_ev_per_k: 6.0e-4,  varshni_beta_k: 408.0  },
    Bandgap { formula: "AlN",   eg_ev: 6.20,  kind: "direct",   temperature_k: 300.0, varshni_alpha_ev_per_k: 1.799e-3,varshni_beta_k: 1462.0 },
    Bandgap { formula: "AlP",   eg_ev: 2.45,  kind: "indirect", temperature_k: 300.0, varshni_alpha_ev_per_k: 0.0,     varshni_beta_k: 0.0    },
    Bandgap { formula: "SiC",   eg_ev: 3.26,  kind: "indirect", temperature_k: 300.0, varshni_alpha_ev_per_k: 6.5e-4,  varshni_beta_k: 1300.0 },
    Bandgap { formula: "ZnO",   eg_ev: 3.37,  kind: "direct",   temperature_k: 300.0, varshni_alpha_ev_per_k: 0.0,     varshni_beta_k: 0.0    },
    Bandgap { formula: "ZnS",   eg_ev: 3.68,  kind: "direct",   temperature_k: 300.0, varshni_alpha_ev_per_k: 0.0,     varshni_beta_k: 0.0    },
    Bandgap { formula: "ZnSe",  eg_ev: 2.70,  kind: "direct",   temperature_k: 300.0, varshni_alpha_ev_per_k: 0.0,     varshni_beta_k: 0.0    },
    Bandgap { formula: "CdS",   eg_ev: 2.42,  kind: "direct",   temperature_k: 300.0, varshni_alpha_ev_per_k: 0.0,     varshni_beta_k: 0.0    },
    Bandgap { formula: "CdSe",  eg_ev: 1.74,  kind: "direct",   temperature_k: 300.0, varshni_alpha_ev_per_k: 0.0,     varshni_beta_k: 0.0    },
    Bandgap { formula: "CdTe",  eg_ev: 1.50,  kind: "direct",   temperature_k: 300.0, varshni_alpha_ev_per_k: 0.0,     varshni_beta_k: 0.0    },
    Bandgap { formula: "PbS",   eg_ev: 0.41,  kind: "direct",   temperature_k: 300.0, varshni_alpha_ev_per_k: 0.0,     varshni_beta_k: 0.0    },
    Bandgap { formula: "PbSe",  eg_ev: 0.27,  kind: "direct",   temperature_k: 300.0, varshni_alpha_ev_per_k: 0.0,     varshni_beta_k: 0.0    },
    Bandgap { formula: "PbTe",  eg_ev: 0.32,  kind: "direct",   temperature_k: 300.0, varshni_alpha_ev_per_k: 0.0,     varshni_beta_k: 0.0    },
    Bandgap { formula: "TiO2",  eg_ev: 3.20,  kind: "indirect", temperature_k: 300.0, varshni_alpha_ev_per_k: 0.0,     varshni_beta_k: 0.0    },
    Bandgap { formula: "Cu2O",  eg_ev: 2.17,  kind: "direct",   temperature_k: 300.0, varshni_alpha_ev_per_k: 0.0,     varshni_beta_k: 0.0    },
    Bandgap { formula: "MoS2",  eg_ev: 1.29,  kind: "indirect", temperature_k: 300.0, varshni_alpha_ev_per_k: 0.0,     varshni_beta_k: 0.0    },
    Bandgap { formula: "Sn",    eg_ev: 0.08,  kind: "direct",   temperature_k: 300.0, varshni_alpha_ev_per_k: 0.0,     varshni_beta_k: 0.0    },
    Bandgap { formula: "Se",    eg_ev: 1.74,  kind: "indirect", temperature_k: 300.0, varshni_alpha_ev_per_k: 0.0,     varshni_beta_k: 0.0    },
    Bandgap { formula: "Te",    eg_ev: 0.33,  kind: "direct",   temperature_k: 300.0, varshni_alpha_ev_per_k: 0.0,     varshni_beta_k: 0.0    },
];

pub fn by_formula(formula: &str) -> Option<&'static Bandgap> {
    TABLE.iter().find(|b| b.formula == formula)
}

pub fn varshni_eg_at_k(b: &Bandgap, t_k: f64) -> f64 {
    if b.varshni_alpha_ev_per_k == 0.0 {
        return b.eg_ev;
    }
    let eg0 = b.eg_ev + b.varshni_alpha_ev_per_k * b.temperature_k.powi(2)
        / (b.temperature_k + b.varshni_beta_k);
    eg0 - b.varshni_alpha_ev_per_k * t_k.powi(2) / (t_k + b.varshni_beta_k)
}
