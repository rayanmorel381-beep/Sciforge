pub mod _2_8;
pub mod _2_9;
pub mod _3_0;
pub mod _3_1;
pub mod _3_2;
pub mod _3_3;
pub mod _3_4;
pub mod _3_5;
pub mod _3_6;

use super::{CYLINDERS, Displacement};
use crate::sounds::{LayoutKind, VoiceLoudness};
use crate::sounds::acoustics::{self as ac, BoostKind};
use sciforge_core::materials::alus::cast::AC4B;

const LAYOUT: LayoutKind = LayoutKind::W;
const HEAD_MATERIAL: &std::sync::LazyLock<sciforge_core::materials::Material> = &AC4B;
const INTERCOOLED: bool = true;
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SoundCharacter { W6Smooth, W6VR, W6Sport, Aggressive, Race }

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum InductionSound {
    NaturalIntake, TurboWhistle, TwinTurboWhistle, SuperchargerWhine, SuperchargerHowl,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ExhaustVoice {
    Muffled, Sporty, Aggressive, Race, W6Burble, W6Sport, LuxuryHum,
}

#[derive(Debug, Clone, Copy)]
pub struct EngineSound {
    pub idle_rpm: u32,
    pub redline_rpm: u32,
    pub fundamental_idle_hz: f64,
    pub fundamental_redline_hz: f64,
    pub character: SoundCharacter,
    pub induction: InductionSound,
    pub exhaust: ExhaustVoice,
    pub exhaust_loudness_db: f64,
    pub induction_loudness_db: f64,
    pub harmonic_richness: f64,
    pub crackle_pop: bool,
    pub turbo_spool_hz: Option<f64>,
    pub blow_off_valve: bool,
    pub supercharger_whine_hz: Option<f64>,
}

pub fn character_for(d: &Displacement) -> SoundCharacter {
    if d.liters() >= 3.5 { SoundCharacter::W6Sport }
    else if d.liters() >= 3.1 { SoundCharacter::W6VR }
    else { SoundCharacter::W6Smooth }
}

pub fn fundamental_hz(rpm: u32) -> f64 {
    ac::fundamental_hz(rpm, CYLINDERS)
}

pub fn idle_rpm_for(d: &Displacement) -> u32 {
    ac::idle_rpm(d.liters(), LAYOUT)
}

pub fn redline_rpm_for(d: &Displacement) -> u32 {
    ac::redline_rpm(d.liters(), d.bore_mm, d.stroke_mm, CYLINDERS, LAYOUT)
}

fn voice_loudness(exhaust: ExhaustVoice) -> VoiceLoudness {
    match exhaust {
        ExhaustVoice::Muffled => VoiceLoudness::Muffled,
        ExhaustVoice::Sporty => VoiceLoudness::Sporty,
        ExhaustVoice::Aggressive => VoiceLoudness::Aggressive,
        ExhaustVoice::Race => VoiceLoudness::Race,
        ExhaustVoice::W6Burble => VoiceLoudness::VrBurble,
        ExhaustVoice::W6Sport => VoiceLoudness::W8Smooth,
        ExhaustVoice::LuxuryHum => VoiceLoudness::LuxuryHum,
    }
}

pub fn natural(d: &Displacement, exhaust: ExhaustVoice) -> EngineSound {
    let idle = idle_rpm_for(d);
    let redline = redline_rpm_for(d);
    let liters = d.liters();
    let exhaust_port_d_m = (d.bore_mm * 0.35) / 1000.0;
    let intake_port_d_m = (d.bore_mm * 0.40) / 1000.0;
    let egt_k = ac::estimate_egt_k(liters, LAYOUT, BoostKind::Natural);
    let charge_k = ac::estimate_charge_temp_k(BoostKind::Natural, INTERCOOLED);
    let exhaust_loudness = ac::exhaust_loudness_db(liters, voice_loudness(exhaust), egt_k, redline, CYLINDERS, exhaust_port_d_m);
    let induction_loudness = ac::induction_loudness_db(liters, redline, CYLINDERS, intake_port_d_m, charge_k);
    let richness = ac::harmonic_richness(HEAD_MATERIAL, egt_k, liters);
    EngineSound {
        idle_rpm: idle,
        redline_rpm: redline,
        fundamental_idle_hz: fundamental_hz(idle),
        fundamental_redline_hz: fundamental_hz(redline),
        character: character_for(d),
        induction: InductionSound::NaturalIntake,
        exhaust,
        exhaust_loudness_db: exhaust_loudness,
        induction_loudness_db: induction_loudness,
        harmonic_richness: richness,
        crackle_pop: matches!(exhaust, ExhaustVoice::Aggressive | ExhaustVoice::Race | ExhaustVoice::W6Burble),
        turbo_spool_hz: None,
        blow_off_valve: false,
        supercharger_whine_hz: None,
    }
}

pub fn turbo(d: &Displacement, exhaust: ExhaustVoice, twin: bool) -> EngineSound {
    let boost = if twin { BoostKind::TwinTurbo } else { BoostKind::Turbo };
    let liters = d.liters();
    let redline = redline_rpm_for(d);
    let exhaust_port_d_m = (d.bore_mm * 0.35) / 1000.0;
    let intake_port_d_m = (d.bore_mm * 0.40) / 1000.0;
    let egt_k = ac::estimate_egt_k(liters, LAYOUT, boost);
    let charge_k = ac::estimate_charge_temp_k(boost, INTERCOOLED);
    let mut base = natural(d, exhaust);
    base.induction = if twin { InductionSound::TwinTurboWhistle } else { InductionSound::TurboWhistle };
    base.exhaust_loudness_db = ac::exhaust_loudness_db(liters, voice_loudness(exhaust), egt_k, redline, CYLINDERS, exhaust_port_d_m) - 3.0;
    base.induction_loudness_db = ac::induction_loudness_db(liters, redline, CYLINDERS, intake_port_d_m, charge_k) + 8.0;
    base.harmonic_richness = ac::harmonic_richness(HEAD_MATERIAL, egt_k, liters) + 0.10;
    let inducer_mm = 30.0 + liters * 12.0;
    base.turbo_spool_hz = Some(ac::turbo_spool_hz(base.redline_rpm, inducer_mm, twin));
    base.blow_off_valve = true;
    base.crackle_pop = true;
    base
}

pub fn supercharged(d: &Displacement, exhaust: ExhaustVoice, howl: bool) -> EngineSound {
    let boost = if howl { BoostKind::SuperchargerHowl } else { BoostKind::SuperchargerWhine };
    let liters = d.liters();
    let redline = redline_rpm_for(d);
    let exhaust_port_d_m = (d.bore_mm * 0.35) / 1000.0;
    let intake_port_d_m = (d.bore_mm * 0.40) / 1000.0;
    let egt_k = ac::estimate_egt_k(liters, LAYOUT, boost);
    let charge_k = ac::estimate_charge_temp_k(boost, INTERCOOLED);
    let mut base = natural(d, exhaust);
    base.induction = if howl { InductionSound::SuperchargerHowl } else { InductionSound::SuperchargerWhine };
    base.exhaust_loudness_db = ac::exhaust_loudness_db(liters, voice_loudness(exhaust), egt_k, redline, CYLINDERS, exhaust_port_d_m);
    base.induction_loudness_db = ac::induction_loudness_db(liters, redline, CYLINDERS, intake_port_d_m, charge_k) + 12.0;
    base.harmonic_richness = ac::harmonic_richness(HEAD_MATERIAL, egt_k, liters) + 0.15;
    let drive_ratio = if howl { 2.4 } else { 2.1 };
    let lobes = if howl { 3 } else { 4 };
    base.supercharger_whine_hz = Some(ac::supercharger_whine_hz(base.redline_rpm, drive_ratio, lobes));
    base
}

