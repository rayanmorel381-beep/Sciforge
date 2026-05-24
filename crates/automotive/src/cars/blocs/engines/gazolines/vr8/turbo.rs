use crate::components::powertrain::engines::thermals::assemblies::{aspiration::Aspiration, bloc::Bloc, layout::Layout};
use crate::components::powertrain::engines::thermals::parts::culasses::Culasse;
use super::cc;

const L: Layout = Layout::Vr;
const N: u8 = 8;

fn make(cc_val: u32, bore: f64, stroke: f64, variant: &'static str, cr: f64, boost: f64) -> Bloc {
    Bloc {
        layout: L, cylinders: N,
        displacement_cc: cc_val, bore_mm: bore, stroke_mm: stroke,
        aspiration: Aspiration::Turbo(1), variant,
        compression_ratio: cr, max_boost_bar: boost,
        conrod_ratio_lambda: 0.30,
        design_safety_factor: 2.5,
        head_variants: vec![Culasse::ohv_2v_1p_wedge(N), Culasse::sohc_2v_1p_wedge(N), Culasse::dohc_4v_1p_pent_roof(N), Culasse::dohc_4v_2p_hemi(N), Culasse::dohc_5v_1p(N)],
    }
}

pub fn vr8_4_0_ss_g() -> Bloc { make(cc::_4_0::CC, 87.4, 83.2, "SS_g", 10.0, 1.3) }
pub fn vr8_4_0_ts_g() -> Bloc { make(cc::_4_0::CC, 87.4, 83.2, "TS_g",  9.5, 1.6) }
pub fn vr8_4_0_vg_g() -> Bloc { make(cc::_4_0::CC, 87.4, 83.2, "VG_g",  8.5, 1.8) }
pub fn vr8_4_4_ss_g() -> Bloc { make(cc::_4_4::CC, 90.3, 86.0, "SS_g", 10.0, 1.3) }
pub fn vr8_4_4_ts_g() -> Bloc { make(cc::_4_4::CC, 90.3, 86.0, "TS_g",  9.5, 1.6) }
pub fn vr8_4_4_vg_g() -> Bloc { make(cc::_4_4::CC, 90.3, 86.0, "VG_g",  8.5, 1.8) }

pub fn all() -> Vec<Bloc> {
    vec![vr8_4_0_ss_g(), vr8_4_0_ts_g(), vr8_4_0_vg_g(), vr8_4_4_ss_g(), vr8_4_4_ts_g(), vr8_4_4_vg_g()]
}
