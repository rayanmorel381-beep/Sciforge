use crate::components::powertrain::engines::thermals::assemblies::{aspiration::Aspiration, bloc::Bloc, layout::Layout};
use crate::components::powertrain::engines::thermals::parts::culasses::Culasse;
use super::cc;

const L: Layout = Layout::V;
const N: u8 = 12;

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

pub fn v12_5_0_rt_g() -> Bloc { make(cc::_5_0::CC, 83.6, 76.0, "RT_g", 0.8) }
pub fn v12_5_0_tw_g() -> Bloc { make(cc::_5_0::CC, 83.6, 76.0, "TW_g", 0.9) }
pub fn v12_5_0_cf_g() -> Bloc { make(cc::_5_0::CC, 83.6, 76.0, "CF_g", 0.7) }
pub fn v12_5_5_rt_g() -> Bloc { make(cc::_5_5::CC, 86.3, 78.5, "RT_g", 0.8) }
pub fn v12_5_5_tw_g() -> Bloc { make(cc::_5_5::CC, 86.3, 78.5, "TW_g", 0.9) }
pub fn v12_5_5_cf_g() -> Bloc { make(cc::_5_5::CC, 86.3, 78.5, "CF_g", 0.7) }
pub fn v12_6_0_rt_g() -> Bloc { make(cc::_6_0::CC, 88.8, 80.7, "RT_g", 0.8) }
pub fn v12_6_0_tw_g() -> Bloc { make(cc::_6_0::CC, 88.8, 80.7, "TW_g", 0.9) }
pub fn v12_6_0_cf_g() -> Bloc { make(cc::_6_0::CC, 88.8, 80.7, "CF_g", 0.7) }
pub fn v12_6_2_rt_g() -> Bloc { make(cc::_6_2::CC, 89.8, 81.6, "RT_g", 0.8) }
pub fn v12_6_2_tw_g() -> Bloc { make(cc::_6_2::CC, 89.8, 81.6, "TW_g", 0.9) }
pub fn v12_6_2_cf_g() -> Bloc { make(cc::_6_2::CC, 89.8, 81.6, "CF_g", 0.7) }
pub fn v12_6_3_rt_g() -> Bloc { make(cc::_6_3::CC, 90.3, 82.1, "RT_g", 0.8) }
pub fn v12_6_3_tw_g() -> Bloc { make(cc::_6_3::CC, 90.3, 82.1, "TW_g", 0.9) }
pub fn v12_6_3_cf_g() -> Bloc { make(cc::_6_3::CC, 90.3, 82.1, "CF_g", 0.7) }
pub fn v12_6_5_rt_g() -> Bloc { make(cc::_6_5::CC, 91.2, 82.9, "RT_g", 0.8) }
pub fn v12_6_5_tw_g() -> Bloc { make(cc::_6_5::CC, 91.2, 82.9, "TW_g", 0.9) }
pub fn v12_6_5_cf_g() -> Bloc { make(cc::_6_5::CC, 91.2, 82.9, "CF_g", 0.7) }
pub fn v12_6_75_rt_g() -> Bloc { make(cc::_6_75::CC, 92.4, 84.0, "RT_g", 0.8) }
pub fn v12_6_75_tw_g() -> Bloc { make(cc::_6_75::CC, 92.4, 84.0, "TW_g", 0.9) }
pub fn v12_6_75_cf_g() -> Bloc { make(cc::_6_75::CC, 92.4, 84.0, "CF_g", 0.7) }
pub fn v12_7_0_rt_g() -> Bloc { make(cc::_7_0::CC, 93.5, 85.0, "RT_g", 0.8) }
pub fn v12_7_0_tw_g() -> Bloc { make(cc::_7_0::CC, 93.5, 85.0, "TW_g", 0.9) }
pub fn v12_7_0_cf_g() -> Bloc { make(cc::_7_0::CC, 93.5, 85.0, "CF_g", 0.7) }

pub fn all() -> Vec<Bloc> {
    vec![
        v12_5_0_rt_g(), v12_5_0_tw_g(), v12_5_0_cf_g(), v12_5_5_rt_g(), v12_5_5_tw_g(), v12_5_5_cf_g(),
        v12_6_0_rt_g(), v12_6_0_tw_g(), v12_6_0_cf_g(), v12_6_2_rt_g(), v12_6_2_tw_g(), v12_6_2_cf_g(),
        v12_6_3_rt_g(), v12_6_3_tw_g(), v12_6_3_cf_g(), v12_6_5_rt_g(), v12_6_5_tw_g(), v12_6_5_cf_g(),
        v12_6_75_rt_g(), v12_6_75_tw_g(), v12_6_75_cf_g(), v12_7_0_rt_g(), v12_7_0_tw_g(), v12_7_0_cf_g(),
    ]
}
