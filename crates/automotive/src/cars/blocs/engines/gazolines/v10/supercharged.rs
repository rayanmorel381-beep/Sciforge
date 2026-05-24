use crate::components::powertrain::engines::thermals::assemblies::{aspiration::Aspiration, bloc::Bloc, layout::Layout};
use crate::components::powertrain::engines::thermals::parts::culasses::Culasse;
use super::cc;

const L: Layout = Layout::V;
const N: u8 = 10;

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

pub fn v10_4_0_rt_g() -> Bloc { make(cc::_4_0::CC, 82.4, 74.9, "RT_g", 0.8) }
pub fn v10_4_0_tw_g() -> Bloc { make(cc::_4_0::CC, 82.4, 74.9, "TW_g", 0.9) }
pub fn v10_4_0_cf_g() -> Bloc { make(cc::_4_0::CC, 82.4, 74.9, "CF_g", 0.7) }
pub fn v10_4_2_rt_g() -> Bloc { make(cc::_4_2::CC, 83.8, 76.2, "RT_g", 0.8) }
pub fn v10_4_2_tw_g() -> Bloc { make(cc::_4_2::CC, 83.8, 76.2, "TW_g", 0.9) }
pub fn v10_4_2_cf_g() -> Bloc { make(cc::_4_2::CC, 83.8, 76.2, "CF_g", 0.7) }
pub fn v10_5_0_rt_g() -> Bloc { make(cc::_5_0::CC, 88.8, 80.7, "RT_g", 0.8) }
pub fn v10_5_0_tw_g() -> Bloc { make(cc::_5_0::CC, 88.8, 80.7, "TW_g", 0.9) }
pub fn v10_5_0_cf_g() -> Bloc { make(cc::_5_0::CC, 88.8, 80.7, "CF_g", 0.7) }
pub fn v10_5_2_rt_g() -> Bloc { make(cc::_5_2::CC, 90.0, 81.8, "RT_g", 0.8) }
pub fn v10_5_2_tw_g() -> Bloc { make(cc::_5_2::CC, 90.0, 81.8, "TW_g", 0.9) }
pub fn v10_5_2_cf_g() -> Bloc { make(cc::_5_2::CC, 90.0, 81.8, "CF_g", 0.7) }
pub fn v10_5_7_rt_g() -> Bloc { make(cc::_5_7::CC, 92.8, 84.4, "RT_g", 0.8) }
pub fn v10_5_7_tw_g() -> Bloc { make(cc::_5_7::CC, 92.8, 84.4, "TW_g", 0.9) }
pub fn v10_5_7_cf_g() -> Bloc { make(cc::_5_7::CC, 92.8, 84.4, "CF_g", 0.7) }
pub fn v10_5_5_rt_g() -> Bloc { make(cc::_5_5::CC, 91.7, 83.4, "RT_g", 0.8) }
pub fn v10_5_5_tw_g() -> Bloc { make(cc::_5_5::CC, 91.7, 83.4, "TW_g", 0.9) }
pub fn v10_5_5_cf_g() -> Bloc { make(cc::_5_5::CC, 91.7, 83.4, "CF_g", 0.7) }
pub fn v10_6_0_rt_g() -> Bloc { make(cc::_6_0::CC, 94.4, 85.8, "RT_g", 0.8) }
pub fn v10_6_0_tw_g() -> Bloc { make(cc::_6_0::CC, 94.4, 85.8, "TW_g", 0.9) }
pub fn v10_6_0_cf_g() -> Bloc { make(cc::_6_0::CC, 94.4, 85.8, "CF_g", 0.7) }
pub fn v10_8_3_rt_g() -> Bloc { make(cc::_8_3::CC, 105.1, 95.5, "RT_g", 0.8) }
pub fn v10_8_3_tw_g() -> Bloc { make(cc::_8_3::CC, 105.1, 95.5, "TW_g", 0.9) }
pub fn v10_8_3_cf_g() -> Bloc { make(cc::_8_3::CC, 105.1, 95.5, "CF_g", 0.7) }
pub fn v10_8_4_rt_g() -> Bloc { make(cc::_8_4::CC, 105.6, 96.0, "RT_g", 0.8) }
pub fn v10_8_4_tw_g() -> Bloc { make(cc::_8_4::CC, 105.6, 96.0, "TW_g", 0.9) }
pub fn v10_8_4_cf_g() -> Bloc { make(cc::_8_4::CC, 105.6, 96.0, "CF_g", 0.7) }

pub fn all() -> Vec<Bloc> {
    vec![
        v10_4_0_rt_g(), v10_4_0_tw_g(), v10_4_0_cf_g(),
        v10_4_2_rt_g(), v10_4_2_tw_g(), v10_4_2_cf_g(),
        v10_5_0_rt_g(), v10_5_0_tw_g(), v10_5_0_cf_g(),
        v10_5_2_rt_g(), v10_5_2_tw_g(), v10_5_2_cf_g(),
        v10_5_7_rt_g(), v10_5_7_tw_g(), v10_5_7_cf_g(),
        v10_8_3_rt_g(), v10_8_3_tw_g(), v10_8_3_cf_g(),
        v10_8_4_rt_g(), v10_8_4_tw_g(), v10_8_4_cf_g(),
    ]
}
