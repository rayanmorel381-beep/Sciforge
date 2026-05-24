use std::fmt;

#[derive(Clone, Debug)]
pub enum ProcessType {
    PPChain,
    CNOCycle,
    TripleAlpha,
    CarbonBurning,
    NeonBurning,
    OxygenBurning,
    SiliconBurning,
    SProcess,
    RProcess,
    RPProcess,
}

impl ProcessType {
    pub fn threshold_temperature_k(&self) -> f64 {
        match self {
            Self::PPChain => 4e6,
            Self::CNOCycle => 15e6,
            Self::TripleAlpha => 1e8,
            Self::CarbonBurning => 5e8,
            Self::NeonBurning => 1.2e9,
            Self::OxygenBurning => 1.5e9,
            Self::SiliconBurning => 2.7e9,
            Self::SProcess => 1e8,
            Self::RProcess => 1e9,
            Self::RPProcess => 1e9,
        }
    }

    pub fn energy_released_mev(&self) -> f64 {
        match self {
            Self::PPChain => 26.732,
            Self::CNOCycle => 25.03,
            Self::TripleAlpha => 7.275,
            Self::CarbonBurning => 13.933,
            Self::NeonBurning => 4.586,
            Self::OxygenBurning => 16.542,
            Self::SiliconBurning => 0.195,
            _ => 0.0,
        }
    }

    pub fn is_active_at(&self, temperature_k: f64) -> bool {
        temperature_k >= self.threshold_temperature_k()
    }
}

impl fmt::Display for ProcessType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::PPChain => write!(f, "pp chain"),
            Self::CNOCycle => write!(f, "CNO cycle"),
            Self::TripleAlpha => write!(f, "triple-alpha"),
            Self::CarbonBurning => write!(f, "carbon burning"),
            Self::NeonBurning => write!(f, "neon burning"),
            Self::OxygenBurning => write!(f, "oxygen burning"),
            Self::SiliconBurning => write!(f, "silicon burning"),
            Self::SProcess => write!(f, "s-process"),
            Self::RProcess => write!(f, "r-process"),
            Self::RPProcess => write!(f, "rp-process"),
        }
    }
}

pub fn active_processes(temperature_k: f64) -> Vec<ProcessType> {
    let all = [
        ProcessType::PPChain,
        ProcessType::CNOCycle,
        ProcessType::TripleAlpha,
        ProcessType::CarbonBurning,
        ProcessType::NeonBurning,
        ProcessType::OxygenBurning,
        ProcessType::SiliconBurning,
        ProcessType::SProcess,
        ProcessType::RProcess,
        ProcessType::RPProcess,
    ];
    all.into_iter()
        .filter(|p| p.is_active_at(temperature_k))
        .collect()
}

pub fn dominant_process_at(temperature_k: f64) -> Option<ProcessType> {
    if temperature_k >= 2.7e9 {
        Some(ProcessType::SiliconBurning)
    } else if temperature_k >= 1.5e9 {
        Some(ProcessType::OxygenBurning)
    } else if temperature_k >= 1.2e9 {
        Some(ProcessType::NeonBurning)
    } else if temperature_k >= 5e8 {
        Some(ProcessType::CarbonBurning)
    } else if temperature_k >= 1e8 {
        Some(ProcessType::TripleAlpha)
    } else if temperature_k >= 15e6 {
        Some(ProcessType::CNOCycle)
    } else if temperature_k >= 4e6 {
        Some(ProcessType::PPChain)
    } else {
        None
    }
}

pub fn process_timescale_years(process: &ProcessType, mass_solar: f64) -> f64 {
    let base = match process {
        ProcessType::PPChain => 1e10,
        ProcessType::CNOCycle => 1e10,
        ProcessType::TripleAlpha => 1e6,
        ProcessType::CarbonBurning => 1e3,
        ProcessType::NeonBurning => 1.0,
        ProcessType::OxygenBurning => 0.5,
        ProcessType::SiliconBurning => 1.0 / 365.0,
        ProcessType::SProcess => 1e5,
        ProcessType::RProcess => 1.0 / (365.0 * 86400.0),
        ProcessType::RPProcess => 1.0 / (365.0 * 86400.0),
    };
    base / mass_solar.powf(2.5)
}

pub fn process_fuel(process: &ProcessType) -> &'static str {
    match process {
        ProcessType::PPChain => "hydrogen",
        ProcessType::CNOCycle => "hydrogen (CNO catalysts)",
        ProcessType::TripleAlpha => "helium",
        ProcessType::CarbonBurning => "carbon",
        ProcessType::NeonBurning => "neon",
        ProcessType::OxygenBurning => "oxygen",
        ProcessType::SiliconBurning => "silicon",
        ProcessType::SProcess => "iron-peak seed nuclei + neutrons",
        ProcessType::RProcess => "seed nuclei + rapid neutrons",
        ProcessType::RPProcess => "seed nuclei + rapid protons",
    }
}

pub fn process_product(process: &ProcessType) -> &'static str {
    match process {
        ProcessType::PPChain => "He-4",
        ProcessType::CNOCycle => "He-4",
        ProcessType::TripleAlpha => "C-12",
        ProcessType::CarbonBurning => "Ne-20, Na-23, Mg-24",
        ProcessType::NeonBurning => "O-16, Mg-24",
        ProcessType::OxygenBurning => "Si-28, S-32",
        ProcessType::SiliconBurning => "Fe-56, Ni-56",
        ProcessType::SProcess => "heavy elements (Ba, Pb)",
        ProcessType::RProcess => "heavy elements (Eu, Au, Pt, U)",
        ProcessType::RPProcess => "proton-rich isotopes",
    }
}

pub fn nuclear_statistical_equilibrium_temp() -> f64 {
    5e9
}

pub fn neutron_capture_rate_estimate(
    neutron_density_per_cm3: f64,
    cross_section_barn: f64,
    velocity_cm_s: f64,
) -> f64 {
    neutron_density_per_cm3 * cross_section_barn * 1e-24 * velocity_cm_s
}

pub fn s_process_neutron_exposure(tau: f64, sigma_times_flux: f64) -> f64 {
    (-tau * sigma_times_flux).exp()
}

pub fn r_process_waiting_point_z(neutron_separation_energy_mev: f64, temperature_gk: f64) -> f64 {
    neutron_separation_energy_mev / (0.0862 * temperature_gk)
}

pub fn pp_chain_branches(temperature_k: f64) -> (f64, f64, f64) {
    let t6 = temperature_k / 1e6;
    if t6 < 14.0 {
        (0.85, 0.15, 0.00)
    } else if t6 < 23.0 {
        (0.15, 0.84, 0.01)
    } else {
        (0.02, 0.30, 0.68)
    }
}

pub fn cno_cycle_is_dominant(temperature_k: f64, metallicity: f64) -> bool {
    let crossover = 1.7e7 - 2e6 * metallicity.log10().max(-3.0);
    temperature_k > crossover
}
