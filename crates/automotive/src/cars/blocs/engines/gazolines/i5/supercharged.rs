use crate::components::powertrain::engines::thermals::assemblies::{aspiration::Aspiration, bloc::Bloc, layout::Layout};
use crate::components::powertrain::engines::thermals::parts::culasses::Culasse;
use super::cc;

const L: Layout = Layout::Inline;
const N: u8 = 5;

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

pub fn i5_2_0_rt_g() -> Bloc { make(cc::_2_0::CC, 77.7, 84.5, "RT_g", 0.8) }
pub fn i5_2_0_tw_g() -> Bloc { make(cc::_2_0::CC, 77.7, 84.5, "TW_g", 0.9) }
pub fn i5_2_0_cf_g() -> Bloc { make(cc::_2_0::CC, 77.7, 84.5, "CF_g", 0.7) }
pub fn i5_2_1_rt_g() -> Bloc { make(cc::_2_1::CC, 78.9, 85.8, "RT_g", 0.8) }
pub fn i5_2_1_tw_g() -> Bloc { make(cc::_2_1::CC, 78.9, 85.8, "TW_g", 0.9) }
pub fn i5_2_1_cf_g() -> Bloc { make(cc::_2_1::CC, 78.9, 85.8, "CF_g", 0.7) }
pub fn i5_2_2_rt_g() -> Bloc { make(cc::_2_2::CC, 80.2, 87.2, "RT_g", 0.8) }
pub fn i5_2_2_tw_g() -> Bloc { make(cc::_2_2::CC, 80.2, 87.2, "TW_g", 0.9) }
pub fn i5_2_2_cf_g() -> Bloc { make(cc::_2_2::CC, 80.2, 87.2, "CF_g", 0.7) }
pub fn i5_2_3_rt_g() -> Bloc { make(cc::_2_3::CC, 81.4, 88.5, "RT_g", 0.8) }
pub fn i5_2_3_tw_g() -> Bloc { make(cc::_2_3::CC, 81.4, 88.5, "TW_g", 0.9) }
pub fn i5_2_3_cf_g() -> Bloc { make(cc::_2_3::CC, 81.4, 88.5, "CF_g", 0.7) }
pub fn i5_2_4_rt_g() -> Bloc { make(cc::_2_4::CC, 82.5, 89.7, "RT_g", 0.8) }
pub fn i5_2_4_tw_g() -> Bloc { make(cc::_2_4::CC, 82.5, 89.7, "TW_g", 0.9) }
pub fn i5_2_4_cf_g() -> Bloc { make(cc::_2_4::CC, 82.5, 89.7, "CF_g", 0.7) }
pub fn i5_2_5_rt_g() -> Bloc { make(cc::_2_5::CC, 83.7, 91.0, "RT_g", 0.8) }
pub fn i5_2_5_tw_g() -> Bloc { make(cc::_2_5::CC, 83.7, 91.0, "TW_g", 0.9) }
pub fn i5_2_5_cf_g() -> Bloc { make(cc::_2_5::CC, 83.7, 91.0, "CF_g", 0.7) }

pub fn all() -> Vec<Bloc> {
    vec![
        i5_2_0_rt_g(), i5_2_0_tw_g(), i5_2_0_cf_g(),
        i5_2_1_rt_g(), i5_2_1_tw_g(), i5_2_1_cf_g(),
        i5_2_2_rt_g(), i5_2_2_tw_g(), i5_2_2_cf_g(),
        i5_2_3_rt_g(), i5_2_3_tw_g(), i5_2_3_cf_g(),
        i5_2_4_rt_g(), i5_2_4_tw_g(), i5_2_4_cf_g(),
        i5_2_5_rt_g(), i5_2_5_tw_g(), i5_2_5_cf_g(),
    ]
}
