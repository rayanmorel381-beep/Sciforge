#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum BleedPort {
    LowPressureCompressor,
    IntermediateCompressor,
    HighPressureCompressor,
}

#[derive(Debug, Clone)]
pub struct BleedSystem {
    pub port: BleedPort,
    pub bleed_pressure_bar: f64,
    pub bleed_temp_k: f64,
    pub max_flow_kg_s: f64,
    pub anti_ice: bool,
    pub cabin_pressurization: bool,
    pub engine_start_assist: bool,
}

impl BleedSystem {
    pub fn hpc_bleed(max_flow_kg_s: f64) -> Self {
        Self {
            port: BleedPort::HighPressureCompressor,
            bleed_pressure_bar: 30.0,
            bleed_temp_k: 750.0,
            max_flow_kg_s,
            anti_ice: true,
            cabin_pressurization: true,
            engine_start_assist: true,
        }
    }

    pub fn ipc_bleed(max_flow_kg_s: f64) -> Self {
        Self {
            port: BleedPort::IntermediateCompressor,
            bleed_pressure_bar: 12.0,
            bleed_temp_k: 550.0,
            max_flow_kg_s,
            anti_ice: true,
            cabin_pressurization: false,
            engine_start_assist: false,
        }
    }

    pub fn lpc_bleed(max_flow_kg_s: f64) -> Self {
        Self {
            port: BleedPort::LowPressureCompressor,
            bleed_pressure_bar: 3.5,
            bleed_temp_k: 380.0,
            max_flow_kg_s,
            anti_ice: false,
            cabin_pressurization: false,
            engine_start_assist: false,
        }
    }

    pub fn power_penalty_kw(&self, core_power_kw: f64) -> f64 {
        core_power_kw * (self.max_flow_kg_s * self.bleed_pressure_bar / 1_000.0)
    }
}
