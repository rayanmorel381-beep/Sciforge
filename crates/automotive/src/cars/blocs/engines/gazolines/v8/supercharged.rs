use crate::components::powertrain::engines::thermals::assemblies::{aspiration::Aspiration, bloc::Bloc, layout::Layout};
use crate::components::powertrain::engines::thermals::parts::culasses::Culasse;
use super::cc;

const L: Layout = Layout::V;
const N: u8 = 8;

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

pub fn v8_3_5_rt_g() -> Bloc { make(cc::_3_5::CC, 84.4, 78.1, "RT_g", 0.8) }
pub fn v8_3_5_tw_g() -> Bloc { make(cc::_3_5::CC, 84.4, 78.1, "TW_g", 0.9) }
pub fn v8_3_5_cf_g() -> Bloc { make(cc::_3_5::CC, 84.4, 78.1, "CF_g", 0.7) }
pub fn v8_3_9_rt_g() -> Bloc { make(cc::_3_9::CC, 87.5, 81.0, "RT_g", 0.8) }
pub fn v8_3_9_tw_g() -> Bloc { make(cc::_3_9::CC, 87.5, 81.0, "TW_g", 0.9) }
pub fn v8_3_9_cf_g() -> Bloc { make(cc::_3_9::CC, 87.5, 81.0, "CF_g", 0.7) }
pub fn v8_4_0_rt_g() -> Bloc { make(cc::_4_0::CC, 88.3, 81.8, "RT_g", 0.8) }
pub fn v8_4_0_tw_g() -> Bloc { make(cc::_4_0::CC, 88.3, 81.8, "TW_g", 0.9) }
pub fn v8_4_0_cf_g() -> Bloc { make(cc::_4_0::CC, 88.3, 81.8, "CF_g", 0.7) }
pub fn v8_4_4_rt_g() -> Bloc { make(cc::_4_4::CC, 91.1, 84.4, "RT_g", 0.8) }
pub fn v8_4_4_tw_g() -> Bloc { make(cc::_4_4::CC, 91.1, 84.4, "TW_g", 0.9) }
pub fn v8_4_4_cf_g() -> Bloc { make(cc::_4_4::CC, 91.1, 84.4, "CF_g", 0.7) }
pub fn v8_4_5_rt_g() -> Bloc { make(cc::_4_5::CC, 91.8, 85.0, "RT_g", 0.8) }
pub fn v8_4_5_tw_g() -> Bloc { make(cc::_4_5::CC, 91.8, 85.0, "TW_g", 0.9) }
pub fn v8_4_5_cf_g() -> Bloc { make(cc::_4_5::CC, 91.8, 85.0, "CF_g", 0.7) }
pub fn v8_4_6_rt_g() -> Bloc { make(cc::_4_6::CC, 92.5, 85.6, "RT_g", 0.8) }
pub fn v8_4_6_tw_g() -> Bloc { make(cc::_4_6::CC, 92.5, 85.6, "TW_g", 0.9) }
pub fn v8_4_6_cf_g() -> Bloc { make(cc::_4_6::CC, 92.5, 85.6, "CF_g", 0.7) }
pub fn v8_4_8_rt_g() -> Bloc { make(cc::_4_8::CC, 93.8, 86.9, "RT_g", 0.8) }
pub fn v8_4_8_tw_g() -> Bloc { make(cc::_4_8::CC, 93.8, 86.9, "TW_g", 0.9) }
pub fn v8_4_8_cf_g() -> Bloc { make(cc::_4_8::CC, 93.8, 86.9, "CF_g", 0.7) }
pub fn v8_5_0_rt_g() -> Bloc { make(cc::_5_0::CC, 95.1, 88.1, "RT_g", 0.8) }
pub fn v8_5_0_tw_g() -> Bloc { make(cc::_5_0::CC, 95.1, 88.1, "TW_g", 0.9) }
pub fn v8_5_0_cf_g() -> Bloc { make(cc::_5_0::CC, 95.1, 88.1, "CF_g", 0.7) }
pub fn v8_5_3_rt_g() -> Bloc { make(cc::_5_3::CC, 96.9, 89.7, "RT_g", 0.8) }
pub fn v8_5_3_tw_g() -> Bloc { make(cc::_5_3::CC, 96.9, 89.7, "TW_g", 0.9) }
pub fn v8_5_3_cf_g() -> Bloc { make(cc::_5_3::CC, 96.9, 89.7, "CF_g", 0.7) }
pub fn v8_5_5_rt_g() -> Bloc { make(cc::_5_5::CC, 98.1, 90.8, "RT_g", 0.8) }
pub fn v8_5_5_tw_g() -> Bloc { make(cc::_5_5::CC, 98.1, 90.8, "TW_g", 0.9) }
pub fn v8_5_5_cf_g() -> Bloc { make(cc::_5_5::CC, 98.1, 90.8, "CF_g", 0.7) }
pub fn v8_5_7_rt_g() -> Bloc { make(cc::_5_7::CC, 99.3, 91.9, "RT_g", 0.8) }
pub fn v8_5_7_tw_g() -> Bloc { make(cc::_5_7::CC, 99.3, 91.9, "TW_g", 0.9) }
pub fn v8_5_7_cf_g() -> Bloc { make(cc::_5_7::CC, 99.3, 91.9, "CF_g", 0.7) }
pub fn v8_6_0_rt_g() -> Bloc { make(cc::_6_0::CC, 101.0, 93.5, "RT_g", 0.8) }
pub fn v8_6_0_tw_g() -> Bloc { make(cc::_6_0::CC, 101.0, 93.5, "TW_g", 0.9) }
pub fn v8_6_0_cf_g() -> Bloc { make(cc::_6_0::CC, 101.0, 93.5, "CF_g", 0.7) }
pub fn v8_6_2_rt_g() -> Bloc { make(cc::_6_2::CC, 102.1, 94.5, "RT_g", 0.8) }
pub fn v8_6_2_tw_g() -> Bloc { make(cc::_6_2::CC, 102.1, 94.5, "TW_g", 0.9) }
pub fn v8_6_2_cf_g() -> Bloc { make(cc::_6_2::CC, 102.1, 94.5, "CF_g", 0.7) }
pub fn v8_6_4_rt_g() -> Bloc { make(cc::_6_4::CC, 103.2, 95.6, "RT_g", 0.8) }
pub fn v8_6_4_tw_g() -> Bloc { make(cc::_6_4::CC, 103.2, 95.6, "TW_g", 0.9) }
pub fn v8_6_4_cf_g() -> Bloc { make(cc::_6_4::CC, 103.2, 95.6, "CF_g", 0.7) }
pub fn v8_7_0_rt_g() -> Bloc { make(cc::_7_0::CC, 106.4, 98.5, "RT_g", 0.8) }
pub fn v8_7_0_tw_g() -> Bloc { make(cc::_7_0::CC, 106.4, 98.5, "TW_g", 0.9) }
pub fn v8_7_0_cf_g() -> Bloc { make(cc::_7_0::CC, 106.4, 98.5, "CF_g", 0.7) }

pub fn all() -> Vec<Bloc> {
    vec![
        v8_3_5_rt_g(), v8_3_5_tw_g(), v8_3_5_cf_g(), v8_3_9_rt_g(), v8_3_9_tw_g(), v8_3_9_cf_g(),
        v8_4_0_rt_g(), v8_4_0_tw_g(), v8_4_0_cf_g(), v8_4_4_rt_g(), v8_4_4_tw_g(), v8_4_4_cf_g(),
        v8_4_5_rt_g(), v8_4_5_tw_g(), v8_4_5_cf_g(), v8_4_6_rt_g(), v8_4_6_tw_g(), v8_4_6_cf_g(),
        v8_4_8_rt_g(), v8_4_8_tw_g(), v8_4_8_cf_g(), v8_5_0_rt_g(), v8_5_0_tw_g(), v8_5_0_cf_g(),
        v8_5_3_rt_g(), v8_5_3_tw_g(), v8_5_3_cf_g(), v8_5_5_rt_g(), v8_5_5_tw_g(), v8_5_5_cf_g(),
        v8_5_7_rt_g(), v8_5_7_tw_g(), v8_5_7_cf_g(), v8_6_0_rt_g(), v8_6_0_tw_g(), v8_6_0_cf_g(),
        v8_6_2_rt_g(), v8_6_2_tw_g(), v8_6_2_cf_g(), v8_6_4_rt_g(), v8_6_4_tw_g(), v8_6_4_cf_g(),
        v8_7_0_rt_g(), v8_7_0_tw_g(), v8_7_0_cf_g(),
    ]
}
