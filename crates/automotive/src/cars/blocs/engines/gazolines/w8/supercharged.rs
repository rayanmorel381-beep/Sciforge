use crate::components::powertrain::engines::thermals::assemblies::{aspiration::Aspiration, bloc::Bloc, layout::Layout};
use crate::components::powertrain::engines::thermals::parts::culasses::Culasse;
use super::cc;

const L: Layout = Layout::W;
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

pub fn w8_3_6_rt_g() -> Bloc { make(cc::_3_6::CC, 84.4, 80.4, "RT_g", 0.8) }
pub fn w8_3_6_tw_g() -> Bloc { make(cc::_3_6::CC, 84.4, 80.4, "TW_g", 0.9) }
pub fn w8_3_6_cf_g() -> Bloc { make(cc::_3_6::CC, 84.4, 80.4, "CF_g", 0.7) }
pub fn w8_4_0_rt_g() -> Bloc { make(cc::_4_0::CC, 87.4, 83.2, "RT_g", 0.8) }
pub fn w8_4_0_tw_g() -> Bloc { make(cc::_4_0::CC, 87.4, 83.2, "TW_g", 0.9) }
pub fn w8_4_0_cf_g() -> Bloc { make(cc::_4_0::CC, 87.4, 83.2, "CF_g", 0.7) }
pub fn w8_4_2_rt_g() -> Bloc { make(cc::_4_2::CC, 88.9, 84.7, "RT_g", 0.8) }
pub fn w8_4_2_tw_g() -> Bloc { make(cc::_4_2::CC, 88.9, 84.7, "TW_g", 0.9) }
pub fn w8_4_2_cf_g() -> Bloc { make(cc::_4_2::CC, 88.9, 84.7, "CF_g", 0.7) }

pub fn all() -> Vec<Bloc> {
    vec![w8_3_6_rt_g(), w8_3_6_tw_g(), w8_3_6_cf_g(), w8_4_0_rt_g(), w8_4_0_tw_g(), w8_4_0_cf_g(), w8_4_2_rt_g(), w8_4_2_tw_g(), w8_4_2_cf_g()]
}
