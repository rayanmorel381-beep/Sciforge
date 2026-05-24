use crate::components::powertrain::engines::thermals::assemblies::{aspiration::Aspiration, bloc::Bloc, layout::Layout};
use crate::components::powertrain::engines::thermals::parts::culasses::Culasse;
use super::cc;

const L: Layout = Layout::Inline;
const N: u8 = 4;

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

pub fn i4_1_0_rt_g() -> Bloc { make(cc::_1_0::CC, 66.4, 72.2, "RT_g", 0.8) }
pub fn i4_1_0_tw_g() -> Bloc { make(cc::_1_0::CC, 66.4, 72.2, "TW_g", 0.9) }
pub fn i4_1_0_cf_g() -> Bloc { make(cc::_1_0::CC, 66.4, 72.2, "CF_g", 0.7) }
pub fn i4_1_1_rt_g() -> Bloc { make(cc::_1_1::CC, 68.6, 74.6, "RT_g", 0.8) }
pub fn i4_1_1_tw_g() -> Bloc { make(cc::_1_1::CC, 68.6, 74.6, "TW_g", 0.9) }
pub fn i4_1_1_cf_g() -> Bloc { make(cc::_1_1::CC, 68.6, 74.6, "CF_g", 0.7) }
pub fn i4_1_2_rt_g() -> Bloc { make(cc::_1_2::CC, 70.6, 76.7, "RT_g", 0.8) }
pub fn i4_1_2_tw_g() -> Bloc { make(cc::_1_2::CC, 70.6, 76.7, "TW_g", 0.9) }
pub fn i4_1_2_cf_g() -> Bloc { make(cc::_1_2::CC, 70.6, 76.7, "CF_g", 0.7) }
pub fn i4_1_3_rt_g() -> Bloc { make(cc::_1_3::CC, 72.5, 78.8, "RT_g", 0.8) }
pub fn i4_1_3_tw_g() -> Bloc { make(cc::_1_3::CC, 72.5, 78.8, "TW_g", 0.9) }
pub fn i4_1_3_cf_g() -> Bloc { make(cc::_1_3::CC, 72.5, 78.8, "CF_g", 0.7) }
pub fn i4_1_4_rt_g() -> Bloc { make(cc::_1_4::CC, 74.3, 80.8, "RT_g", 0.8) }
pub fn i4_1_4_tw_g() -> Bloc { make(cc::_1_4::CC, 74.3, 80.8, "TW_g", 0.9) }
pub fn i4_1_4_cf_g() -> Bloc { make(cc::_1_4::CC, 74.3, 80.8, "CF_g", 0.7) }
pub fn i4_1_5_rt_g() -> Bloc { make(cc::_1_5::CC, 76.0, 82.6, "RT_g", 0.8) }
pub fn i4_1_5_tw_g() -> Bloc { make(cc::_1_5::CC, 76.0, 82.6, "TW_g", 0.9) }
pub fn i4_1_5_cf_g() -> Bloc { make(cc::_1_5::CC, 76.0, 82.6, "CF_g", 0.7) }
pub fn i4_1_6_rt_g() -> Bloc { make(cc::_1_6::CC, 77.7, 84.5, "RT_g", 0.8) }
pub fn i4_1_6_tw_g() -> Bloc { make(cc::_1_6::CC, 77.7, 84.5, "TW_g", 0.9) }
pub fn i4_1_6_cf_g() -> Bloc { make(cc::_1_6::CC, 77.7, 84.5, "CF_g", 0.7) }
pub fn i4_1_7_rt_g() -> Bloc { make(cc::_1_7::CC, 79.3, 86.2, "RT_g", 0.8) }
pub fn i4_1_7_tw_g() -> Bloc { make(cc::_1_7::CC, 79.3, 86.2, "TW_g", 0.9) }
pub fn i4_1_7_cf_g() -> Bloc { make(cc::_1_7::CC, 79.3, 86.2, "CF_g", 0.7) }
pub fn i4_1_8_rt_g() -> Bloc { make(cc::_1_8::CC, 80.8, 87.8, "RT_g", 0.8) }
pub fn i4_1_8_tw_g() -> Bloc { make(cc::_1_8::CC, 80.8, 87.8, "TW_g", 0.9) }
pub fn i4_1_8_cf_g() -> Bloc { make(cc::_1_8::CC, 80.8, 87.8, "CF_g", 0.7) }
pub fn i4_1_9_rt_g() -> Bloc { make(cc::_1_9::CC, 82.2, 89.3, "RT_g", 0.8) }
pub fn i4_1_9_tw_g() -> Bloc { make(cc::_1_9::CC, 82.2, 89.3, "TW_g", 0.9) }
pub fn i4_1_9_cf_g() -> Bloc { make(cc::_1_9::CC, 82.2, 89.3, "CF_g", 0.7) }
pub fn i4_2_0_rt_g() -> Bloc { make(cc::_2_0::CC, 83.7, 91.0, "RT_g", 0.8) }
pub fn i4_2_0_tw_g() -> Bloc { make(cc::_2_0::CC, 83.7, 91.0, "TW_g", 0.9) }
pub fn i4_2_0_cf_g() -> Bloc { make(cc::_2_0::CC, 83.7, 91.0, "CF_g", 0.7) }
pub fn i4_2_1_rt_g() -> Bloc { make(cc::_2_1::CC, 85.0, 92.4, "RT_g", 0.8) }
pub fn i4_2_1_tw_g() -> Bloc { make(cc::_2_1::CC, 85.0, 92.4, "TW_g", 0.9) }
pub fn i4_2_1_cf_g() -> Bloc { make(cc::_2_1::CC, 85.0, 92.4, "CF_g", 0.7) }
pub fn i4_2_2_rt_g() -> Bloc { make(cc::_2_2::CC, 86.4, 93.9, "RT_g", 0.8) }
pub fn i4_2_2_tw_g() -> Bloc { make(cc::_2_2::CC, 86.4, 93.9, "TW_g", 0.9) }
pub fn i4_2_2_cf_g() -> Bloc { make(cc::_2_2::CC, 86.4, 93.9, "CF_g", 0.7) }
pub fn i4_2_3_rt_g() -> Bloc { make(cc::_2_3::CC, 87.7, 95.3, "RT_g", 0.8) }
pub fn i4_2_3_tw_g() -> Bloc { make(cc::_2_3::CC, 87.7, 95.3, "TW_g", 0.9) }
pub fn i4_2_3_cf_g() -> Bloc { make(cc::_2_3::CC, 87.7, 95.3, "CF_g", 0.7) }
pub fn i4_2_4_rt_g() -> Bloc { make(cc::_2_4::CC, 88.9, 96.6, "RT_g", 0.8) }
pub fn i4_2_4_tw_g() -> Bloc { make(cc::_2_4::CC, 88.9, 96.6, "TW_g", 0.9) }
pub fn i4_2_4_cf_g() -> Bloc { make(cc::_2_4::CC, 88.9, 96.6, "CF_g", 0.7) }

pub fn all() -> Vec<Bloc> {
    vec![
        i4_1_0_rt_g(), i4_1_0_tw_g(), i4_1_0_cf_g(),
        i4_1_1_rt_g(), i4_1_1_tw_g(), i4_1_1_cf_g(),
        i4_1_2_rt_g(), i4_1_2_tw_g(), i4_1_2_cf_g(),
        i4_1_3_rt_g(), i4_1_3_tw_g(), i4_1_3_cf_g(),
        i4_1_4_rt_g(), i4_1_4_tw_g(), i4_1_4_cf_g(),
        i4_1_5_rt_g(), i4_1_5_tw_g(), i4_1_5_cf_g(),
        i4_1_6_rt_g(), i4_1_6_tw_g(), i4_1_6_cf_g(),
        i4_1_7_rt_g(), i4_1_7_tw_g(), i4_1_7_cf_g(),
        i4_1_8_rt_g(), i4_1_8_tw_g(), i4_1_8_cf_g(),
        i4_1_9_rt_g(), i4_1_9_tw_g(), i4_1_9_cf_g(),
        i4_2_0_rt_g(), i4_2_0_tw_g(), i4_2_0_cf_g(),
        i4_2_1_rt_g(), i4_2_1_tw_g(), i4_2_1_cf_g(),
        i4_2_2_rt_g(), i4_2_2_tw_g(), i4_2_2_cf_g(),
        i4_2_3_rt_g(), i4_2_3_tw_g(), i4_2_3_cf_g(),
        i4_2_4_rt_g(), i4_2_4_tw_g(), i4_2_4_cf_g(),
    ]
}
