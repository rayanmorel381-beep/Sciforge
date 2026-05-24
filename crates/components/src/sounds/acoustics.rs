use super::{LayoutKind, VoiceLoudness};
use sciforge_core::materials::Material;

pub const P_REF_PA: f64 = 20.0e-6;
pub const I_REF_W_M2: f64 = 1.0e-12;
pub const R_AIR: f64 = 287.0;
pub const M_AIR_KG_MOL: f64 = 0.02897;
pub const GAMMA_AIR: f64 = 1.40;
pub const GAMMA_EXHAUST: f64 = 1.33;
pub const M_EXHAUST_KG_MOL: f64 = 0.029;

pub fn fundamental_hz(rpm: u32, cylinders: u8) -> f64 {
    (rpm as f64 / 60.0) * (cylinders as f64 / 2.0)
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum BoostKind {
    Natural,
    Turbo,
    TwinTurbo,
    SuperchargerWhine,
    SuperchargerHowl,
}

pub fn estimate_egt_k(displacement_l: f64, layout: LayoutKind, boost: BoostKind) -> f64 {
    let layout_base = match layout {
        LayoutKind::Radial => 1020.0,
        LayoutKind::Flat => 1060.0,
        LayoutKind::H => 1062.0,
        LayoutKind::Inline => 1100.0,
        LayoutKind::Vr => 1110.0,
        LayoutKind::W => 1115.0,
        LayoutKind::X => 1118.0,
        LayoutKind::V => 1130.0,
    };
    let displacement_bias: f64 = if displacement_l < 1.5 {
        40.0
    } else if displacement_l < 3.0 {
        0.0
    } else {
        -25.0
    };
    let boost_bias = match boost {
        BoostKind::Natural => 0.0,
        BoostKind::Turbo => 90.0,
        BoostKind::TwinTurbo => 70.0,
        BoostKind::SuperchargerWhine => 50.0,
        BoostKind::SuperchargerHowl => 65.0,
    };
    (layout_base + displacement_bias + boost_bias).clamp(950.0, 1280.0)
}

pub fn estimate_charge_temp_k(boost: BoostKind, intercooled: bool) -> f64 {
    let raw = match boost {
        BoostKind::Natural => 318.0,
        BoostKind::Turbo => 408.0,
        BoostKind::TwinTurbo => 388.0,
        BoostKind::SuperchargerWhine => 398.0,
        BoostKind::SuperchargerHowl => 418.0,
    };
    if intercooled {
        318.0 + (raw - 318.0) * 0.35
    } else {
        raw
    }
}

pub fn idle_rpm(displacement_l: f64, layout: LayoutKind) -> u32 {
    let base = if displacement_l < 1.0 {
        1000.0
    } else if displacement_l < 1.8 {
        900.0
    } else if displacement_l < 2.2 {
        850.0
    } else if displacement_l < 4.0 {
        800.0
    } else {
        700.0
    };
    let bias: f64 = match layout {
        LayoutKind::Radial => 50.0,
        LayoutKind::Vr | LayoutKind::W | LayoutKind::X => -20.0,
        LayoutKind::Flat | LayoutKind::H => -30.0,
        _ => 0.0,
    };
    (base + bias).round() as u32
}

pub fn redline_rpm(
    displacement_l: f64,
    bore_mm: f64,
    stroke_mm: f64,
    cylinders: u8,
    layout: LayoutKind,
) -> u32 {
    let bs_ratio = bore_mm / stroke_mm;
    let cyl_bonus = (cylinders as f64 - 4.0) * 150.0;
    let base = match layout {
        LayoutKind::Radial => 2800.0 - displacement_l * 30.0,
        LayoutKind::W | LayoutKind::X => 6500.0 - displacement_l * 80.0,
        LayoutKind::Flat | LayoutKind::Vr | LayoutKind::H => 7100.0 - displacement_l * 100.0,
        LayoutKind::V => 7300.0 - displacement_l * 90.0,
        LayoutKind::Inline => 7400.0 - displacement_l * 110.0,
    };
    let ratio_bonus = ((bs_ratio - 1.0) * 700.0).clamp(-400.0, 1200.0);
    (base + ratio_bonus + cyl_bonus).max(2200.0).round() as u32
}

pub fn speed_of_sound_air(temperature_k: f64) -> f64 {
    (GAMMA_AIR * R_AIR * temperature_k).sqrt()
}

pub fn speed_of_sound_exhaust(egt_k: f64) -> f64 {
    let r_universal = 8.314;
    (GAMMA_EXHAUST * (r_universal / M_EXHAUST_KG_MOL) * egt_k).sqrt()
}

pub fn gas_density(pressure_pa: f64, temperature_k: f64, molar_mass_kg_mol: f64) -> f64 {
    pressure_pa * molar_mass_kg_mol / (8.314 * temperature_k)
}

pub fn acoustic_impedance(rho: f64, c: f64) -> f64 {
    rho * c
}

pub fn solid_impedance(material: &Material) -> f64 {
    let c_solid = (material.young_modulus_pa / material.density()).sqrt();
    material.density() * c_solid
}

pub fn port_reflection_coeff(z_gas: f64, z_solid: f64) -> f64 {
    ((z_solid - z_gas) / (z_solid + z_gas)).powi(2)
}

pub fn plane_wave_spl_db(rho: f64, c: f64, particle_velocity_m_s: f64, distance_m: f64) -> f64 {
    let p_at_source = rho * c * particle_velocity_m_s.abs();
    let p_at_distance = p_at_source / distance_m.max(0.5);
    20.0 * (p_at_distance.max(P_REF_PA) / P_REF_PA).log10()
}

pub fn exhaust_loudness_db(
    displacement_l: f64,
    voice: VoiceLoudness,
    egt_k: f64,
    rpm: u32,
    cylinders: u8,
    exhaust_port_diameter_m: f64,
) -> f64 {
    let c_exh = speed_of_sound_exhaust(egt_k);
    let rho_exh = gas_density(101325.0, egt_k, M_EXHAUST_KG_MOL);
    let firing_freq = fundamental_hz(rpm, cylinders);
    let port_area = std::f64::consts::PI * exhaust_port_diameter_m.powi(2) / 4.0;
    let blowdown_volume_m3 = displacement_l * 1.0e-3 / cylinders as f64;
    let mean_velocity = blowdown_volume_m3 * firing_freq / port_area.max(1.0e-6);
    let physical = plane_wave_spl_db(rho_exh, c_exh, mean_velocity, 1.0);
    let stylistic = voice.base_db() + voice.liter_gain_db_per_l() * displacement_l;
    0.55 * physical + 0.45 * stylistic
}

pub fn induction_loudness_db(
    displacement_l: f64,
    rpm: u32,
    cylinders: u8,
    intake_diameter_m: f64,
    charge_temp_k: f64,
) -> f64 {
    let c_air = speed_of_sound_air(charge_temp_k);
    let rho_air = gas_density(101325.0, charge_temp_k, M_AIR_KG_MOL);
    let firing_freq = fundamental_hz(rpm, cylinders);
    let port_area = std::f64::consts::PI * intake_diameter_m.powi(2) / 4.0;
    let suction_volume_m3 = displacement_l * 1.0e-3 / cylinders as f64;
    let mean_velocity = suction_volume_m3 * firing_freq / port_area.max(1.0e-6);
    let physical = plane_wave_spl_db(rho_air, c_air, mean_velocity, 1.0);
    physical - 12.0
}

pub fn harmonic_richness(
    head_material: &Material,
    egt_k: f64,
    displacement_l: f64,
) -> f64 {
    let c_exh = speed_of_sound_exhaust(egt_k);
    let rho_exh = gas_density(101325.0, egt_k, M_EXHAUST_KG_MOL);
    let z_gas = acoustic_impedance(rho_exh, c_exh);
    let z_head = solid_impedance(head_material);
    let r = port_reflection_coeff(z_gas, z_head);
    (0.40 + 0.45 * r + 0.05 * displacement_l).clamp(0.30, 1.20)
}

pub fn turbo_spool_hz(rpm: u32, compressor_inducer_mm: f64, twin: bool) -> f64 {
    let blade_count = if compressor_inducer_mm < 50.0 { 11.0 } else { 12.0 };
    let turbo_rpm_max = 280000.0 - compressor_inducer_mm * 1500.0;
    let load_fraction = (rpm as f64 / 6500.0).min(1.0);
    let turbo_rpm = turbo_rpm_max * load_fraction.powf(0.7);
    let base = turbo_rpm * blade_count / 60.0;
    if twin { base * 1.05 } else { base }
}

pub fn supercharger_whine_hz(rpm: u32, drive_ratio: f64, lobes: u8) -> f64 {
    rpm as f64 * drive_ratio * lobes as f64 / 60.0
}

pub fn intake_helmholtz_hz(
    speed_of_sound_m_s: f64,
    runner_area_m2: f64,
    plenum_volume_m3: f64,
    runner_length_m: f64,
) -> f64 {
    (speed_of_sound_m_s / (2.0 * std::f64::consts::PI))
        * (runner_area_m2 / (plenum_volume_m3.max(1.0e-6) * runner_length_m.max(0.01))).sqrt()
}

pub fn expansion_chamber_tl_db(
    freq_hz: f64,
    area_ratio: f64,
    chamber_length_m: f64,
    gas_temp_k: f64,
) -> f64 {
    let c = speed_of_sound_exhaust(gas_temp_k);
    let k = 2.0 * std::f64::consts::PI * freq_hz / c;
    let m = area_ratio.max(1.0001);
    let factor = 1.0 + 0.25 * (m - 1.0 / m).powi(2) * (k * chamber_length_m).sin().powi(2);
    10.0 * factor.log10()
}

pub fn dynamic_egt_k(static_egt_wot_k: f64, rpm: u32, redline_rpm_v: u32, throttle_0_1: f64) -> f64 {
    let idle_egt_k = 700.0;
    let rpm_frac = (rpm as f64 / redline_rpm_v.max(1) as f64).clamp(0.0, 1.0);
    let load = throttle_0_1.clamp(0.0, 1.0);
    let blend = (0.4 * load + 0.6 * load * rpm_frac.powf(0.3)).clamp(0.0, 1.0);
    idle_egt_k + (static_egt_wot_k - idle_egt_k) * blend
}

pub fn dynamic_charge_temp_k(
    static_charge_wot_k: f64,
    rpm: u32,
    redline_rpm_v: u32,
    throttle_0_1: f64,
    boost: BoostKind,
) -> f64 {
    let ambient_k = 298.0;
    let rpm_frac = (rpm as f64 / redline_rpm_v.max(1) as f64).clamp(0.0, 1.0);
    let load = throttle_0_1.clamp(0.0, 1.0);
    let boost_weight = match boost {
        BoostKind::Natural => 0.0,
        BoostKind::SuperchargerWhine | BoostKind::SuperchargerHowl => rpm_frac,
        BoostKind::Turbo | BoostKind::TwinTurbo => (rpm_frac.powf(0.6) * load).clamp(0.0, 1.0),
    };
    let intake_warming = (static_charge_wot_k - ambient_k) * boost_weight.max(load * 0.25);
    ambient_k + intake_warming
}
