use crate::components::powertrain::engines::thermals::assemblies::{aspiration::Aspiration, bloc::Bloc, layout::Layout};
use crate::components::powertrain::engines::thermals::parts::culasses::Culasse;
use super::cc;

const L: Layout = Layout::Inline;
const N: u8 = 1;

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

pub fn i1_0_1_rt_g() -> Bloc { make(cc::_0_1::CC, 47.7, 56.1, "RT_g", 0.8) }
pub fn i1_0_1_tw_g() -> Bloc { make(cc::_0_1::CC, 47.7, 56.1, "TW_g", 0.9) }
pub fn i1_0_1_cf_g() -> Bloc { make(cc::_0_1::CC, 47.7, 56.1, "CF_g", 0.7) }
pub fn i1_0_2_rt_g() -> Bloc { make(cc::_0_2::CC, 60.0, 70.6, "RT_g", 0.8) }
pub fn i1_0_2_tw_g() -> Bloc { make(cc::_0_2::CC, 60.0, 70.6, "TW_g", 0.9) }
pub fn i1_0_2_cf_g() -> Bloc { make(cc::_0_2::CC, 60.0, 70.6, "CF_g", 0.7) }
pub fn i1_0_3_rt_g() -> Bloc { make(cc::_0_3::CC, 68.7, 80.8, "RT_g", 0.8) }
pub fn i1_0_3_tw_g() -> Bloc { make(cc::_0_3::CC, 68.7, 80.8, "TW_g", 0.9) }
pub fn i1_0_3_cf_g() -> Bloc { make(cc::_0_3::CC, 68.7, 80.8, "CF_g", 0.7) }
pub fn i1_0_4_rt_g() -> Bloc { make(cc::_0_4::CC, 75.6, 88.9, "RT_g", 0.8) }
pub fn i1_0_4_tw_g() -> Bloc { make(cc::_0_4::CC, 75.6, 88.9, "TW_g", 0.9) }
pub fn i1_0_4_cf_g() -> Bloc { make(cc::_0_4::CC, 75.6, 88.9, "CF_g", 0.7) }
pub fn i1_0_5_rt_g() -> Bloc { make(cc::_0_5::CC, 81.5, 95.9, "RT_g", 0.8) }
pub fn i1_0_5_tw_g() -> Bloc { make(cc::_0_5::CC, 81.5, 95.9, "TW_g", 0.9) }
pub fn i1_0_5_cf_g() -> Bloc { make(cc::_0_5::CC, 81.5, 95.9, "CF_g", 0.7) }
pub fn i1_0_6_rt_g() -> Bloc { make(cc::_0_6::CC, 86.6, 101.9, "RT_g", 0.8) }
pub fn i1_0_6_tw_g() -> Bloc { make(cc::_0_6::CC, 86.6, 101.9, "TW_g", 0.9) }
pub fn i1_0_6_cf_g() -> Bloc { make(cc::_0_6::CC, 86.6, 101.9, "CF_g", 0.7) }
pub fn i1_0_7_rt_g() -> Bloc { make(cc::_0_7::CC, 91.2, 107.3, "RT_g", 0.8) }
pub fn i1_0_7_tw_g() -> Bloc { make(cc::_0_7::CC, 91.2, 107.3, "TW_g", 0.9) }
pub fn i1_0_7_cf_g() -> Bloc { make(cc::_0_7::CC, 91.2, 107.3, "CF_g", 0.7) }
pub fn i1_0_8_rt_g() -> Bloc { make(cc::_0_8::CC, 95.3, 112.1, "RT_g", 0.8) }
pub fn i1_0_8_tw_g() -> Bloc { make(cc::_0_8::CC, 95.3, 112.1, "TW_g", 0.9) }
pub fn i1_0_8_cf_g() -> Bloc { make(cc::_0_8::CC, 95.3, 112.1, "CF_g", 0.7) }

pub fn all() -> Vec<Bloc> {
    vec![
        i1_0_1_rt_g(), i1_0_1_tw_g(), i1_0_1_cf_g(),
        i1_0_2_rt_g(), i1_0_2_tw_g(), i1_0_2_cf_g(),
        i1_0_3_rt_g(), i1_0_3_tw_g(), i1_0_3_cf_g(),
        i1_0_4_rt_g(), i1_0_4_tw_g(), i1_0_4_cf_g(),
        i1_0_5_rt_g(), i1_0_5_tw_g(), i1_0_5_cf_g(),
        i1_0_6_rt_g(), i1_0_6_tw_g(), i1_0_6_cf_g(),
        i1_0_7_rt_g(), i1_0_7_tw_g(), i1_0_7_cf_g(),
        i1_0_8_rt_g(), i1_0_8_tw_g(), i1_0_8_cf_g(),
    ]
}
