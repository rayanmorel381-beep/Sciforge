use crate::components::powertrain::engines::thermals::assemblies::{aspiration::Aspiration, bloc::Bloc, layout::Layout};
use crate::components::powertrain::engines::thermals::parts::culasses::Culasse;
use super::cc;

const L: Layout = Layout::Flat;
const N: u8 = 2;

fn make(cc_val: u32, bore: f64, stroke: f64, variant: &'static str, boost: f64) -> Bloc {
    Bloc {
        layout: L,
        cylinders: N,
        displacement_cc: cc_val,
        bore_mm: bore,
        stroke_mm: stroke,
        aspiration: Aspiration::Supercharged,
        variant,
        compression_ratio: 9.2,
        max_boost_bar: boost,
        conrod_ratio_lambda: 0.30,
        design_safety_factor: 2.5,
        head_variants: vec![Culasse::ohv_2v_1p_wedge(N), Culasse::sohc_2v_1p_wedge(N), Culasse::dohc_4v_1p_pent_roof(N), Culasse::dohc_4v_2p_hemi(N), Culasse::dohc_5v_1p(N)],
    }
}

pub fn f2_0_6_rt_g() -> Bloc { make(cc::_0_6::CC, 70.6, 76.7, "RT_g", 0.8) }
pub fn f2_0_6_tw_g() -> Bloc { make(cc::_0_6::CC, 70.6, 76.7, "TW_g", 0.9) }
pub fn f2_0_6_cf_g() -> Bloc { make(cc::_0_6::CC, 70.6, 76.7, "CF_g", 0.7) }
pub fn f2_0_8_rt_g() -> Bloc { make(cc::_0_8::CC, 77.7, 84.5, "RT_g", 0.8) }
pub fn f2_0_8_tw_g() -> Bloc { make(cc::_0_8::CC, 77.7, 84.5, "TW_g", 0.9) }
pub fn f2_0_8_cf_g() -> Bloc { make(cc::_0_8::CC, 77.7, 84.5, "CF_g", 0.7) }
pub fn f2_1_0_rt_g() -> Bloc { make(cc::_1_0::CC, 83.7, 91.0, "RT_g", 0.8) }
pub fn f2_1_0_tw_g() -> Bloc { make(cc::_1_0::CC, 83.7, 91.0, "TW_g", 0.9) }
pub fn f2_1_0_cf_g() -> Bloc { make(cc::_1_0::CC, 83.7, 91.0, "CF_g", 0.7) }
pub fn f2_1_2_rt_g() -> Bloc { make(cc::_1_2::CC, 88.9, 96.6, "RT_g", 0.8) }
pub fn f2_1_2_tw_g() -> Bloc { make(cc::_1_2::CC, 88.9, 96.6, "TW_g", 0.9) }
pub fn f2_1_2_cf_g() -> Bloc { make(cc::_1_2::CC, 88.9, 96.6, "CF_g", 0.7) }
pub fn f2_1_4_rt_g() -> Bloc { make(cc::_1_4::CC, 93.6, 101.7, "RT_g", 0.8) }
pub fn f2_1_4_tw_g() -> Bloc { make(cc::_1_4::CC, 93.6, 101.7, "TW_g", 0.9) }
pub fn f2_1_4_cf_g() -> Bloc { make(cc::_1_4::CC, 93.6, 101.7, "CF_g", 0.7) }
pub fn f2_1_6_rt_g() -> Bloc { make(cc::_1_6::CC, 97.9, 106.4, "RT_g", 0.8) }
pub fn f2_1_6_tw_g() -> Bloc { make(cc::_1_6::CC, 97.9, 106.4, "TW_g", 0.9) }
pub fn f2_1_6_cf_g() -> Bloc { make(cc::_1_6::CC, 97.9, 106.4, "CF_g", 0.7) }
pub fn f2_1_8_rt_g() -> Bloc { make(cc::_1_8::CC, 101.8, 110.7, "RT_g", 0.8) }
pub fn f2_1_8_tw_g() -> Bloc { make(cc::_1_8::CC, 101.8, 110.7, "TW_g", 0.9) }
pub fn f2_1_8_cf_g() -> Bloc { make(cc::_1_8::CC, 101.8, 110.7, "CF_g", 0.7) }
pub fn f2_2_0_rt_g() -> Bloc { make(cc::_2_0::CC, 105.4, 114.6, "RT_g", 0.8) }
pub fn f2_2_0_tw_g() -> Bloc { make(cc::_2_0::CC, 105.4, 114.6, "TW_g", 0.9) }
pub fn f2_2_0_cf_g() -> Bloc { make(cc::_2_0::CC, 105.4, 114.6, "CF_g", 0.7) }

pub fn all() -> Vec<Bloc> {
    vec![
        f2_0_6_rt_g(), f2_0_6_tw_g(), f2_0_6_cf_g(),
        f2_0_8_rt_g(), f2_0_8_tw_g(), f2_0_8_cf_g(),
        f2_1_0_rt_g(), f2_1_0_tw_g(), f2_1_0_cf_g(),
        f2_1_2_rt_g(), f2_1_2_tw_g(), f2_1_2_cf_g(),
        f2_1_4_rt_g(), f2_1_4_tw_g(), f2_1_4_cf_g(),
        f2_1_6_rt_g(), f2_1_6_tw_g(), f2_1_6_cf_g(),
        f2_1_8_rt_g(), f2_1_8_tw_g(), f2_1_8_cf_g(),
        f2_2_0_rt_g(), f2_2_0_tw_g(), f2_2_0_cf_g(),
    ]
}
