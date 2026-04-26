pub const HH_RESTING_POTENTIAL: f64 = -65.0;
pub const HH_INIT_M: f64 = 0.05;
pub const HH_INIT_H: f64 = 0.6;
pub const HH_INIT_N: f64 = 0.32;
pub const HH_MEMBRANE_CAPACITANCE: f64 = 1.0;
pub const HH_G_NA: f64 = 120.0;
pub const HH_G_K: f64 = 36.0;
pub const HH_G_L: f64 = 0.3;
pub const HH_E_NA: f64 = 50.0;
pub const HH_E_K: f64 = -77.0;
pub const HH_E_L: f64 = -54.4;

pub const HH_ALPHA_M_V_SHIFT: f64 = 40.0;
pub const HH_ALPHA_M_COEFF: f64 = 0.1;
pub const HH_BETA_M_MULTIPLIER: f64 = 4.0;
pub const HH_BETA_M_EXP_COEFF: f64 = 0.0556;
pub const HH_GATING_V_SHIFT: f64 = 65.0;
pub const HH_ALPHA_H_COEFF: f64 = 0.07;
pub const HH_ALPHA_H_EXP_COEFF: f64 = 0.05;
pub const HH_BETA_H_EXP_COEFF: f64 = 0.1;
pub const HH_BETA_H_V_SHIFT: f64 = 35.0;
pub const HH_ALPHA_N_V_SHIFT: f64 = 55.0;
pub const HH_ALPHA_N_COEFF: f64 = 0.01;
pub const HH_BETA_N_COEFF: f64 = 0.125;
pub const HH_BETA_N_EXP_COEFF: f64 = 0.0125;

pub const IZHI_RESTING_POTENTIAL: f64 = -65.0;
pub const IZHI_INIT_U: f64 = -14.0;
pub const IZHI_QUAD_COEFF: f64 = 0.04;
pub const IZHI_LINEAR_COEFF: f64 = 5.0;
pub const IZHI_CONSTANT_COEFF: f64 = 140.0;
pub const IZHI_SPIKE_THRESHOLD: f64 = 30.0;

pub const IZHI_RS_A: f64 = 0.02;
pub const IZHI_RS_B: f64 = 0.2;
pub const IZHI_RS_C: f64 = -65.0;
pub const IZHI_RS_D: f64 = 8.0;

pub const IZHI_FS_A: f64 = 0.1;
pub const IZHI_FS_B: f64 = 0.2;
pub const IZHI_FS_C: f64 = -65.0;
pub const IZHI_FS_D: f64 = 2.0;

pub const IZHI_BURST_A: f64 = 0.02;
pub const IZHI_BURST_B: f64 = 0.2;
pub const IZHI_BURST_C: f64 = -50.0;
pub const IZHI_BURST_D: f64 = 2.0;

pub const ML_INIT_V: f64 = -60.0;
pub const ML_CM: f64 = 20.0;
pub const ML_G_CA: f64 = 4.4;
pub const ML_G_K: f64 = 8.0;
pub const ML_G_L: f64 = 2.0;
pub const ML_E_CA: f64 = 120.0;
pub const ML_E_K: f64 = -84.0;
pub const ML_E_L: f64 = -60.0;
pub const ML_V1: f64 = -1.2;
pub const ML_V2: f64 = 18.0;
pub const ML_V3: f64 = 2.0;
pub const ML_V4: f64 = 30.0;
pub const ML_PHI: f64 = 0.04;

pub const LIF_RESTING_POTENTIAL: f64 = -65.0;
pub const LIF_THRESHOLD: f64 = -50.0;
pub const LIF_TAU: f64 = 10.0;
pub const LIF_RESISTANCE: f64 = 10.0;

pub const NMDA_MG_KD: f64 = 3.57;
pub const NMDA_VOLTAGE_COEFF: f64 = 0.062;
pub const SYNAPTIC_CA_HALF_SAT: f64 = 0.5;
