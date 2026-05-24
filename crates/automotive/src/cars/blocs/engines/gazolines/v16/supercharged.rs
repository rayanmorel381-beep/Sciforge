use crate::components::powertrain::engines::thermals::assemblies::{aspiration::Aspiration, bloc::Bloc, layout::Layout};
use crate::components::powertrain::engines::thermals::parts::culasses::Culasse;
use super::cc;

const L: Layout = Layout::V;
const N: u8 = 16;

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

pub fn v16_6_0_rt_g() -> Bloc { make(cc::_6_0::CC, 81.2, 72.5, "RT_g", 0.8) }
pub fn v16_6_0_tw_g() -> Bloc { make(cc::_6_0::CC, 81.2, 72.5, "TW_g", 0.9) }
pub fn v16_6_0_cf_g() -> Bloc { make(cc::_6_0::CC, 81.2, 72.5, "CF_g", 0.7) }
pub fn v16_6_75_rt_g() -> Bloc { make(cc::_6_75::CC, 84.4, 75.4, "RT_g", 0.8) }
pub fn v16_6_75_tw_g() -> Bloc { make(cc::_6_75::CC, 84.4, 75.4, "TW_g", 0.9) }
pub fn v16_6_75_cf_g() -> Bloc { make(cc::_6_75::CC, 84.4, 75.4, "CF_g", 0.7) }
pub fn v16_7_0_rt_g() -> Bloc { make(cc::_7_0::CC, 85.4, 76.2, "RT_g", 0.8) }
pub fn v16_7_0_tw_g() -> Bloc { make(cc::_7_0::CC, 85.4, 76.2, "TW_g", 0.9) }
pub fn v16_7_0_cf_g() -> Bloc { make(cc::_7_0::CC, 85.4, 76.2, "CF_g", 0.7) }
pub fn v16_7_4_rt_g() -> Bloc { make(cc::_7_4::CC, 87.0, 77.7, "RT_g", 0.8) }
pub fn v16_7_4_tw_g() -> Bloc { make(cc::_7_4::CC, 87.0, 77.7, "TW_g", 0.9) }
pub fn v16_7_4_cf_g() -> Bloc { make(cc::_7_4::CC, 87.0, 77.7, "CF_g", 0.7) }
pub fn v16_8_0_rt_g() -> Bloc { make(cc::_8_0::CC, 89.3, 79.7, "RT_g", 0.8) }
pub fn v16_8_0_tw_g() -> Bloc { make(cc::_8_0::CC, 89.3, 79.7, "TW_g", 0.9) }
pub fn v16_8_0_cf_g() -> Bloc { make(cc::_8_0::CC, 89.3, 79.7, "CF_g", 0.7) }
pub fn v16_8_4_rt_g() -> Bloc { make(cc::_8_4::CC, 90.8, 81.1, "RT_g", 0.8) }
pub fn v16_8_4_tw_g() -> Bloc { make(cc::_8_4::CC, 90.8, 81.1, "TW_g", 0.9) }
pub fn v16_8_4_cf_g() -> Bloc { make(cc::_8_4::CC, 90.8, 81.1, "CF_g", 0.7) }
pub fn v16_9_0_rt_g() -> Bloc { make(cc::_9_0::CC, 92.9, 82.9, "RT_g", 0.8) }
pub fn v16_9_0_tw_g() -> Bloc { make(cc::_9_0::CC, 92.9, 82.9, "TW_g", 0.9) }
pub fn v16_9_0_cf_g() -> Bloc { make(cc::_9_0::CC, 92.9, 82.9, "CF_g", 0.7) }
pub fn v16_13_6_rt_g() -> Bloc { make(cc::_13_6::CC, 106.6, 95.2, "RT_g", 0.8) }
pub fn v16_13_6_tw_g() -> Bloc { make(cc::_13_6::CC, 106.6, 95.2, "TW_g", 0.9) }
pub fn v16_13_6_cf_g() -> Bloc { make(cc::_13_6::CC, 106.6, 95.2, "CF_g", 0.7) }

pub fn all() -> Vec<Bloc> {
    vec![
        v16_6_0_rt_g(), v16_6_0_tw_g(), v16_6_0_cf_g(), v16_6_75_rt_g(), v16_6_75_tw_g(), v16_6_75_cf_g(),
        v16_7_0_rt_g(), v16_7_0_tw_g(), v16_7_0_cf_g(), v16_7_4_rt_g(), v16_7_4_tw_g(), v16_7_4_cf_g(),
        v16_8_0_rt_g(), v16_8_0_tw_g(), v16_8_0_cf_g(), v16_8_4_rt_g(), v16_8_4_tw_g(), v16_8_4_cf_g(),
        v16_9_0_rt_g(), v16_9_0_tw_g(), v16_9_0_cf_g(), v16_13_6_rt_g(), v16_13_6_tw_g(), v16_13_6_cf_g(),
    ]
}
