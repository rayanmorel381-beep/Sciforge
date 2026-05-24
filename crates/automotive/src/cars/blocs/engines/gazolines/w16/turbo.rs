use crate::components::powertrain::engines::thermals::assemblies::{aspiration::Aspiration, bloc::Bloc, layout::Layout};
use crate::components::powertrain::engines::thermals::parts::culasses::Culasse;
use super::cc;

const L: Layout = Layout::W;
const N: u8 = 16;

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

pub fn w16_7_0_ss_g() -> Bloc { make(cc::_7_0::CC, 84.9, 77.2, "SS_g", 10.0, 1.3) }
pub fn w16_7_0_ts_g() -> Bloc { make(cc::_7_0::CC, 84.9, 77.2, "TS_g",  9.5, 1.6) }
pub fn w16_7_0_vg_g() -> Bloc { make(cc::_7_0::CC, 84.9, 77.2, "VG_g",  8.5, 1.8) }
pub fn w16_8_0_ss_g() -> Bloc { make(cc::_8_0::CC, 88.8, 80.7, "SS_g", 10.0, 1.3) }
pub fn w16_8_0_ts_g() -> Bloc { make(cc::_8_0::CC, 88.8, 80.7, "TS_g",  9.5, 1.6) }
pub fn w16_8_0_vg_g() -> Bloc { make(cc::_8_0::CC, 88.8, 80.7, "VG_g",  8.5, 1.8) }
pub fn w16_8_4_ss_g() -> Bloc { make(cc::_8_4::CC, 90.3, 82.1, "SS_g", 10.0, 1.3) }
pub fn w16_8_4_ts_g() -> Bloc { make(cc::_8_4::CC, 90.3, 82.1, "TS_g",  9.5, 1.6) }
pub fn w16_8_4_vg_g() -> Bloc { make(cc::_8_4::CC, 90.3, 82.1, "VG_g",  8.5, 1.8) }

pub fn all() -> Vec<Bloc> {
    vec![w16_7_0_ss_g(), w16_7_0_ts_g(), w16_7_0_vg_g(), w16_8_0_ss_g(), w16_8_0_ts_g(), w16_8_0_vg_g(), w16_8_4_ss_g(), w16_8_4_ts_g(), w16_8_4_vg_g()]
}
