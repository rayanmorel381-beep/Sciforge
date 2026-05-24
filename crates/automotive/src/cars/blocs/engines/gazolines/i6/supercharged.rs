use crate::components::powertrain::engines::thermals::assemblies::{aspiration::Aspiration, bloc::Bloc, layout::Layout};
use crate::components::powertrain::engines::thermals::parts::culasses::Culasse;
use super::cc;

const L: Layout = Layout::Inline;
const N: u8 = 6;

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

pub fn i6_2_0_rt_g() -> Bloc { make(cc::_2_0::CC, 73.9, 77.8, "RT_g", 0.8) }
pub fn i6_2_0_tw_g() -> Bloc { make(cc::_2_0::CC, 73.9, 77.8, "TW_g", 0.9) }
pub fn i6_2_0_cf_g() -> Bloc { make(cc::_2_0::CC, 73.9, 77.8, "CF_g", 0.7) }
pub fn i6_2_3_rt_g() -> Bloc { make(cc::_2_3::CC, 77.4, 81.5, "RT_g", 0.8) }
pub fn i6_2_3_tw_g() -> Bloc { make(cc::_2_3::CC, 77.4, 81.5, "TW_g", 0.9) }
pub fn i6_2_3_cf_g() -> Bloc { make(cc::_2_3::CC, 77.4, 81.5, "CF_g", 0.7) }
pub fn i6_2_5_rt_g() -> Bloc { make(cc::_2_5::CC, 79.6, 83.8, "RT_g", 0.8) }
pub fn i6_2_5_tw_g() -> Bloc { make(cc::_2_5::CC, 79.6, 83.8, "TW_g", 0.9) }
pub fn i6_2_5_cf_g() -> Bloc { make(cc::_2_5::CC, 79.6, 83.8, "CF_g", 0.7) }
pub fn i6_2_8_rt_g() -> Bloc { make(cc::_2_8::CC, 82.6, 86.9, "RT_g", 0.8) }
pub fn i6_2_8_tw_g() -> Bloc { make(cc::_2_8::CC, 82.6, 86.9, "TW_g", 0.9) }
pub fn i6_2_8_cf_g() -> Bloc { make(cc::_2_8::CC, 82.6, 86.9, "CF_g", 0.7) }
pub fn i6_3_0_rt_g() -> Bloc { make(cc::_3_0::CC, 84.6, 89.1, "RT_g", 0.8) }
pub fn i6_3_0_tw_g() -> Bloc { make(cc::_3_0::CC, 84.6, 89.1, "TW_g", 0.9) }
pub fn i6_3_0_cf_g() -> Bloc { make(cc::_3_0::CC, 84.6, 89.1, "CF_g", 0.7) }
pub fn i6_3_2_rt_g() -> Bloc { make(cc::_3_2::CC, 86.4, 90.9, "RT_g", 0.8) }
pub fn i6_3_2_tw_g() -> Bloc { make(cc::_3_2::CC, 86.4, 90.9, "TW_g", 0.9) }
pub fn i6_3_2_cf_g() -> Bloc { make(cc::_3_2::CC, 86.4, 90.9, "CF_g", 0.7) }
pub fn i6_3_5_rt_g() -> Bloc { make(cc::_3_5::CC, 89.0, 93.7, "RT_g", 0.8) }
pub fn i6_3_5_tw_g() -> Bloc { make(cc::_3_5::CC, 89.0, 93.7, "TW_g", 0.9) }
pub fn i6_3_5_cf_g() -> Bloc { make(cc::_3_5::CC, 89.0, 93.7, "CF_g", 0.7) }

pub fn all() -> Vec<Bloc> {
    vec![
        i6_2_0_rt_g(), i6_2_0_tw_g(), i6_2_0_cf_g(),
        i6_2_3_rt_g(), i6_2_3_tw_g(), i6_2_3_cf_g(),
        i6_2_5_rt_g(), i6_2_5_tw_g(), i6_2_5_cf_g(),
        i6_2_8_rt_g(), i6_2_8_tw_g(), i6_2_8_cf_g(),
        i6_3_0_rt_g(), i6_3_0_tw_g(), i6_3_0_cf_g(),
        i6_3_2_rt_g(), i6_3_2_tw_g(), i6_3_2_cf_g(),
        i6_3_5_rt_g(), i6_3_5_tw_g(), i6_3_5_cf_g(),
    ]
}
