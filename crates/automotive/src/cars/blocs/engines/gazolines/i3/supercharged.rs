use crate::components::powertrain::engines::thermals::assemblies::{aspiration::Aspiration, bloc::Bloc, layout::Layout};
use crate::components::powertrain::engines::thermals::parts::culasses::Culasse;
use super::cc;

const L: Layout = Layout::Inline;
const N: u8 = 3;

fn make(cc_val: u32, bore: f64, stroke: f64, variant: &'static str, boost: f64) -> Bloc {
    Bloc {
        layout: L, cylinders: N,
        displacement_cc: cc_val, bore_mm: bore, stroke_mm: stroke,
        aspiration: Aspiration::Supercharged, variant,
        compression_ratio: 9.2, max_boost_bar: boost,
        conrod_ratio_lambda: 0.30,
        design_safety_factor: 2.5,
        head_variants: vec![Culasse::ohv_2v_1p_wedge(N), Culasse::sohc_2v_1p_wedge(N), Culasse::dohc_4v_1p_pent_roof(N), Culasse::dohc_4v_2p_hemi(N), Culasse::dohc_5v_1p(N)],
    }
}

pub fn i3_0_6_rt_g() -> Bloc { make(cc::_0_6::CC, 61.6, 67.0, "RT_g", 0.8) }
pub fn i3_0_6_tw_g() -> Bloc { make(cc::_0_6::CC, 61.6, 67.0, "TW_g", 0.9) }
pub fn i3_0_6_cf_g() -> Bloc { make(cc::_0_6::CC, 61.6, 67.0, "CF_g", 0.7) }
pub fn i3_0_7_rt_g() -> Bloc { make(cc::_0_7::CC, 64.9, 70.5, "RT_g", 0.8) }
pub fn i3_0_7_tw_g() -> Bloc { make(cc::_0_7::CC, 64.9, 70.5, "TW_g", 0.9) }
pub fn i3_0_7_cf_g() -> Bloc { make(cc::_0_7::CC, 64.9, 70.5, "CF_g", 0.7) }
pub fn i3_0_8_rt_g() -> Bloc { make(cc::_0_8::CC, 67.9, 73.8, "RT_g", 0.8) }
pub fn i3_0_8_tw_g() -> Bloc { make(cc::_0_8::CC, 67.9, 73.8, "TW_g", 0.9) }
pub fn i3_0_8_cf_g() -> Bloc { make(cc::_0_8::CC, 67.9, 73.8, "CF_g", 0.7) }
pub fn i3_0_9_rt_g() -> Bloc { make(cc::_0_9::CC, 70.6, 76.7, "RT_g", 0.8) }
pub fn i3_0_9_tw_g() -> Bloc { make(cc::_0_9::CC, 70.6, 76.7, "TW_g", 0.9) }
pub fn i3_0_9_cf_g() -> Bloc { make(cc::_0_9::CC, 70.6, 76.7, "CF_g", 0.7) }
pub fn i3_1_0_rt_g() -> Bloc { make(cc::_1_0::CC, 73.1, 79.5, "RT_g", 0.8) }
pub fn i3_1_0_tw_g() -> Bloc { make(cc::_1_0::CC, 73.1, 79.5, "TW_g", 0.9) }
pub fn i3_1_0_cf_g() -> Bloc { make(cc::_1_0::CC, 73.1, 79.5, "CF_g", 0.7) }
pub fn i3_1_1_rt_g() -> Bloc { make(cc::_1_1::CC, 75.4, 82.0, "RT_g", 0.8) }
pub fn i3_1_1_tw_g() -> Bloc { make(cc::_1_1::CC, 75.4, 82.0, "TW_g", 0.9) }
pub fn i3_1_1_cf_g() -> Bloc { make(cc::_1_1::CC, 75.4, 82.0, "CF_g", 0.7) }
pub fn i3_1_2_rt_g() -> Bloc { make(cc::_1_2::CC, 77.7, 84.5, "RT_g", 0.8) }
pub fn i3_1_2_tw_g() -> Bloc { make(cc::_1_2::CC, 77.7, 84.5, "TW_g", 0.9) }
pub fn i3_1_2_cf_g() -> Bloc { make(cc::_1_2::CC, 77.7, 84.5, "CF_g", 0.7) }
pub fn i3_1_3_rt_g() -> Bloc { make(cc::_1_3::CC, 79.8, 86.7, "RT_g", 0.8) }
pub fn i3_1_3_tw_g() -> Bloc { make(cc::_1_3::CC, 79.8, 86.7, "TW_g", 0.9) }
pub fn i3_1_3_cf_g() -> Bloc { make(cc::_1_3::CC, 79.8, 86.7, "CF_g", 0.7) }
pub fn i3_1_4_rt_g() -> Bloc { make(cc::_1_4::CC, 81.8, 88.9, "RT_g", 0.8) }
pub fn i3_1_4_tw_g() -> Bloc { make(cc::_1_4::CC, 81.8, 88.9, "TW_g", 0.9) }
pub fn i3_1_4_cf_g() -> Bloc { make(cc::_1_4::CC, 81.8, 88.9, "CF_g", 0.7) }
pub fn i3_1_5_rt_g() -> Bloc { make(cc::_1_5::CC, 83.7, 91.0, "RT_g", 0.8) }
pub fn i3_1_5_tw_g() -> Bloc { make(cc::_1_5::CC, 83.7, 91.0, "TW_g", 0.9) }
pub fn i3_1_5_cf_g() -> Bloc { make(cc::_1_5::CC, 83.7, 91.0, "CF_g", 0.7) }

pub fn all() -> Vec<Bloc> {
    vec![
        i3_0_6_rt_g(), i3_0_6_tw_g(), i3_0_6_cf_g(),
        i3_0_7_rt_g(), i3_0_7_tw_g(), i3_0_7_cf_g(),
        i3_0_8_rt_g(), i3_0_8_tw_g(), i3_0_8_cf_g(),
        i3_0_9_rt_g(), i3_0_9_tw_g(), i3_0_9_cf_g(),
        i3_1_0_rt_g(), i3_1_0_tw_g(), i3_1_0_cf_g(),
        i3_1_1_rt_g(), i3_1_1_tw_g(), i3_1_1_cf_g(),
        i3_1_2_rt_g(), i3_1_2_tw_g(), i3_1_2_cf_g(),
        i3_1_3_rt_g(), i3_1_3_tw_g(), i3_1_3_cf_g(),
        i3_1_4_rt_g(), i3_1_4_tw_g(), i3_1_4_cf_g(),
        i3_1_5_rt_g(), i3_1_5_tw_g(), i3_1_5_cf_g(),
    ]
}
