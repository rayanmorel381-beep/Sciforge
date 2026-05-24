use crate::components::powertrain::engines::thermals::assemblies::{aspiration::Aspiration, bloc::Bloc, layout::Layout};
use crate::components::powertrain::engines::thermals::parts::culasses::Culasse;
use super::cc;

const L: Layout = Layout::V;
const N: u8 = 6;

fn make(cc_val: u32, bore: f64, stroke: f64, variant: &'static str, cr: f64, boost: f64) -> Bloc {
    Bloc {
        layout: L, cylinders: N,
        displacement_cc: cc_val, bore_mm: bore, stroke_mm: stroke,
        aspiration: Aspiration::Supercharged, variant,
        compression_ratio: cr, max_boost_bar: boost,
        conrod_ratio_lambda: 0.3,
        design_safety_factor: 2.5,
        head_variants: vec![Culasse::ohv_2v_1p_bowl(N), Culasse::sohc_2v_1p_pre_chamber(N), Culasse::dohc_4v_1p_bowl(N), Culasse::dohc_4v_2p_bowl(N)],
    }
}

pub fn v6_2_0_rt_d() -> Bloc { make(cc::_2_0::CC, 76.4, 72.8, "RT_d", 20.0, 0.5) }
pub fn v6_2_0_tw_d() -> Bloc { make(cc::_2_0::CC, 76.4, 72.8, "TW_d", 19.0, 0.9) }
pub fn v6_2_0_cf_d() -> Bloc { make(cc::_2_0::CC, 76.4, 72.8, "CF_d", 18.5, 1.2) }
pub fn v6_2_3_rt_d() -> Bloc { make(cc::_2_3::CC, 80.0, 76.2, "RT_d", 20.0, 0.5) }
pub fn v6_2_3_tw_d() -> Bloc { make(cc::_2_3::CC, 80.0, 76.2, "TW_d", 19.0, 0.9) }
pub fn v6_2_3_cf_d() -> Bloc { make(cc::_2_3::CC, 80.0, 76.2, "CF_d", 18.5, 1.2) }
pub fn v6_2_5_rt_d() -> Bloc { make(cc::_2_5::CC, 82.3, 78.4, "RT_d", 20.0, 0.5) }
pub fn v6_2_5_tw_d() -> Bloc { make(cc::_2_5::CC, 82.3, 78.4, "TW_d", 19.0, 0.9) }
pub fn v6_2_5_cf_d() -> Bloc { make(cc::_2_5::CC, 82.3, 78.4, "CF_d", 18.5, 1.2) }
pub fn v6_2_7_rt_d() -> Bloc { make(cc::_2_7::CC, 84.4, 80.4, "RT_d", 20.0, 0.5) }
pub fn v6_2_7_tw_d() -> Bloc { make(cc::_2_7::CC, 84.4, 80.4, "TW_d", 19.0, 0.9) }
pub fn v6_2_7_cf_d() -> Bloc { make(cc::_2_7::CC, 84.4, 80.4, "CF_d", 18.5, 1.2) }
pub fn v6_2_8_rt_d() -> Bloc { make(cc::_2_8::CC, 85.4, 81.3, "RT_d", 20.0, 0.5) }
pub fn v6_2_8_tw_d() -> Bloc { make(cc::_2_8::CC, 85.4, 81.3, "TW_d", 19.0, 0.9) }
pub fn v6_2_8_cf_d() -> Bloc { make(cc::_2_8::CC, 85.4, 81.3, "CF_d", 18.5, 1.2) }
pub fn v6_3_0_rt_d() -> Bloc { make(cc::_3_0::CC, 87.4, 83.2, "RT_d", 20.0, 0.5) }
pub fn v6_3_0_tw_d() -> Bloc { make(cc::_3_0::CC, 87.4, 83.2, "TW_d", 19.0, 0.9) }
pub fn v6_3_0_cf_d() -> Bloc { make(cc::_3_0::CC, 87.4, 83.2, "CF_d", 18.5, 1.2) }
pub fn v6_3_2_rt_d() -> Bloc { make(cc::_3_2::CC, 89.3, 85.0, "RT_d", 20.0, 0.5) }
pub fn v6_3_2_tw_d() -> Bloc { make(cc::_3_2::CC, 89.3, 85.0, "TW_d", 19.0, 0.9) }
pub fn v6_3_2_cf_d() -> Bloc { make(cc::_3_2::CC, 89.3, 85.0, "CF_d", 18.5, 1.2) }
pub fn v6_3_5_rt_d() -> Bloc { make(cc::_3_5::CC, 92.0, 87.6, "RT_d", 20.0, 0.5) }
pub fn v6_3_5_tw_d() -> Bloc { make(cc::_3_5::CC, 92.0, 87.6, "TW_d", 19.0, 0.9) }
pub fn v6_3_5_cf_d() -> Bloc { make(cc::_3_5::CC, 92.0, 87.6, "CF_d", 18.5, 1.2) }
pub fn v6_3_6_rt_d() -> Bloc { make(cc::_3_6::CC, 92.9, 88.5, "RT_d", 20.0, 0.5) }
pub fn v6_3_6_tw_d() -> Bloc { make(cc::_3_6::CC, 92.9, 88.5, "TW_d", 19.0, 0.9) }
pub fn v6_3_6_cf_d() -> Bloc { make(cc::_3_6::CC, 92.9, 88.5, "CF_d", 18.5, 1.2) }
pub fn v6_3_7_rt_d() -> Bloc { make(cc::_3_7::CC, 93.8, 89.3, "RT_d", 20.0, 0.5) }
pub fn v6_3_7_tw_d() -> Bloc { make(cc::_3_7::CC, 93.8, 89.3, "TW_d", 19.0, 0.9) }
pub fn v6_3_7_cf_d() -> Bloc { make(cc::_3_7::CC, 93.8, 89.3, "CF_d", 18.5, 1.2) }
pub fn v6_3_8_rt_d() -> Bloc { make(cc::_3_8::CC, 94.6, 90.1, "RT_d", 20.0, 0.5) }
pub fn v6_3_8_tw_d() -> Bloc { make(cc::_3_8::CC, 94.6, 90.1, "TW_d", 19.0, 0.9) }
pub fn v6_3_8_cf_d() -> Bloc { make(cc::_3_8::CC, 94.6, 90.1, "CF_d", 18.5, 1.2) }
pub fn v6_4_0_rt_d() -> Bloc { make(cc::_4_0::CC, 96.2, 91.6, "RT_d", 20.0, 0.5) }
pub fn v6_4_0_tw_d() -> Bloc { make(cc::_4_0::CC, 96.2, 91.6, "TW_d", 19.0, 0.9) }
pub fn v6_4_0_cf_d() -> Bloc { make(cc::_4_0::CC, 96.2, 91.6, "CF_d", 18.5, 1.2) }
