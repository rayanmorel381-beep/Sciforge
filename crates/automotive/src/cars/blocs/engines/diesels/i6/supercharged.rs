use crate::components::powertrain::engines::thermals::assemblies::{aspiration::Aspiration, bloc::Bloc, layout::Layout};
use crate::components::powertrain::engines::thermals::parts::culasses::Culasse;
use super::cc;

const L: Layout = Layout::Inline;
const N: u8 = 6;

fn make(cc_val: u32, bore: f64, stroke: f64, variant: &'static str, cr: f64, boost: f64) -> Bloc {
    Bloc {
        layout: L, cylinders: N,
        displacement_cc: cc_val, bore_mm: bore, stroke_mm: stroke,
        aspiration: Aspiration::Supercharged, variant,
        compression_ratio: cr, max_boost_bar: boost,
        conrod_ratio_lambda: 0.3,
        design_safety_factor: 2.5,
        head_variants: vec![Culasse::ohv_2v_1p_bowl(N), Culasse::sohc_2v_1p_pre_chamber(N), Culasse::dohc_4v_1p_bowl(N), Culasse::dohc_4v_2p_bowl(N)],
    }
}

pub fn i6_2_5_rt_d() -> Bloc { make(cc::_2_5::CC, 79.6, 83.8, "RT_d", 20.0, 0.5) }
pub fn i6_2_5_tw_d() -> Bloc { make(cc::_2_5::CC, 79.6, 83.8, "TW_d", 19.0, 0.9) }
pub fn i6_2_5_cf_d() -> Bloc { make(cc::_2_5::CC, 79.6, 83.8, "CF_d", 18.5, 1.2) }
pub fn i6_2_8_rt_d() -> Bloc { make(cc::_2_8::CC, 82.6, 86.9, "RT_d", 20.0, 0.5) }
pub fn i6_2_8_tw_d() -> Bloc { make(cc::_2_8::CC, 82.6, 86.9, "TW_d", 19.0, 0.9) }
pub fn i6_2_8_cf_d() -> Bloc { make(cc::_2_8::CC, 82.6, 86.9, "CF_d", 18.5, 1.2) }
pub fn i6_3_0_rt_d() -> Bloc { make(cc::_3_0::CC, 84.6, 89.1, "RT_d", 20.0, 0.5) }
pub fn i6_3_0_tw_d() -> Bloc { make(cc::_3_0::CC, 84.6, 89.1, "TW_d", 19.0, 0.9) }
pub fn i6_3_0_cf_d() -> Bloc { make(cc::_3_0::CC, 84.6, 89.1, "CF_d", 18.5, 1.2) }
pub fn i6_3_2_rt_d() -> Bloc { make(cc::_3_2::CC, 86.4, 90.9, "RT_d", 20.0, 0.5) }
pub fn i6_3_2_tw_d() -> Bloc { make(cc::_3_2::CC, 86.4, 90.9, "TW_d", 19.0, 0.9) }
pub fn i6_3_2_cf_d() -> Bloc { make(cc::_3_2::CC, 86.4, 90.9, "CF_d", 18.5, 1.2) }
pub fn i6_3_5_rt_d() -> Bloc { make(cc::_3_5::CC, 89.0, 93.7, "RT_d", 20.0, 0.5) }
pub fn i6_3_5_tw_d() -> Bloc { make(cc::_3_5::CC, 89.0, 93.7, "TW_d", 19.0, 0.9) }
pub fn i6_3_5_cf_d() -> Bloc { make(cc::_3_5::CC, 89.0, 93.7, "CF_d", 18.5, 1.2) }
