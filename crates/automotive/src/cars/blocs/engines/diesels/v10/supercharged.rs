use crate::components::powertrain::engines::thermals::assemblies::{aspiration::Aspiration, bloc::Bloc, layout::Layout};
use super::cc;

const L: Layout = Layout::V;
const N: u8 = 10;

fn make(cc_val: u32, bore: f64, stroke: f64, variant: &'static str, cr: f64, boost: f64) -> Bloc {
    Bloc {
        layout: L, cylinders: N,
        displacement_cc: cc_val, bore_mm: bore, stroke_mm: stroke,
        aspiration: Aspiration::Supercharged, variant,
        compression_ratio: cr, max_boost_bar: boost,
        conrod_ratio_lambda: 0.3,
        design_safety_factor: 2.5,
        head_variants: vec![crate::components::powertrain::engines::thermals::parts::culasses::Culasse::ohv_2v_1p_bowl(N), crate::components::powertrain::engines::thermals::parts::culasses::Culasse::sohc_2v_1p_pre_chamber(N), crate::components::powertrain::engines::thermals::parts::culasses::Culasse::dohc_4v_1p_bowl(N), crate::components::powertrain::engines::thermals::parts::culasses::Culasse::dohc_4v_2p_bowl(N)],
    }
}

pub fn v10_4_0_rt_d(bore: f64, stroke: f64) -> Bloc { make(cc::_4_0::CC, bore, stroke, "RT_d", 20.0, 0.5) }
pub fn v10_4_0_tw_d(bore: f64, stroke: f64) -> Bloc { make(cc::_4_0::CC, bore, stroke, "TW_d", 19.0, 0.9) }
pub fn v10_4_0_cf_d(bore: f64, stroke: f64) -> Bloc { make(cc::_4_0::CC, bore, stroke, "CF_d", 18.5, 1.2) }
pub fn v10_4_2_rt_d(bore: f64, stroke: f64) -> Bloc { make(cc::_4_2::CC, bore, stroke, "RT_d", 20.0, 0.5) }
pub fn v10_4_2_tw_d(bore: f64, stroke: f64) -> Bloc { make(cc::_4_2::CC, bore, stroke, "TW_d", 19.0, 0.9) }
pub fn v10_4_2_cf_d(bore: f64, stroke: f64) -> Bloc { make(cc::_4_2::CC, bore, stroke, "CF_d", 18.5, 1.2) }
pub fn v10_5_0_rt_d(bore: f64, stroke: f64) -> Bloc { make(cc::_5_0::CC, bore, stroke, "RT_d", 20.0, 0.5) }
pub fn v10_5_0_tw_d(bore: f64, stroke: f64) -> Bloc { make(cc::_5_0::CC, bore, stroke, "TW_d", 19.0, 0.9) }
pub fn v10_5_0_cf_d(bore: f64, stroke: f64) -> Bloc { make(cc::_5_0::CC, bore, stroke, "CF_d", 18.5, 1.2) }
pub fn v10_5_2_rt_d(bore: f64, stroke: f64) -> Bloc { make(cc::_5_2::CC, bore, stroke, "RT_d", 20.0, 0.5) }
pub fn v10_5_2_tw_d(bore: f64, stroke: f64) -> Bloc { make(cc::_5_2::CC, bore, stroke, "TW_d", 19.0, 0.9) }
pub fn v10_5_2_cf_d(bore: f64, stroke: f64) -> Bloc { make(cc::_5_2::CC, bore, stroke, "CF_d", 18.5, 1.2) }
pub fn v10_5_7_rt_d(bore: f64, stroke: f64) -> Bloc { make(cc::_5_7::CC, bore, stroke, "RT_d", 20.0, 0.5) }
pub fn v10_5_7_tw_d(bore: f64, stroke: f64) -> Bloc { make(cc::_5_7::CC, bore, stroke, "TW_d", 19.0, 0.9) }
pub fn v10_5_7_cf_d(bore: f64, stroke: f64) -> Bloc { make(cc::_5_7::CC, bore, stroke, "CF_d", 18.5, 1.2) }
pub fn v10_8_3_rt_d(bore: f64, stroke: f64) -> Bloc { make(cc::_8_3::CC, bore, stroke, "RT_d", 20.0, 0.5) }
pub fn v10_8_3_tw_d(bore: f64, stroke: f64) -> Bloc { make(cc::_8_3::CC, bore, stroke, "TW_d", 19.0, 0.9) }
pub fn v10_8_3_cf_d(bore: f64, stroke: f64) -> Bloc { make(cc::_8_3::CC, bore, stroke, "CF_d", 18.5, 1.2) }
pub fn v10_8_4_rt_d(bore: f64, stroke: f64) -> Bloc { make(cc::_8_4::CC, bore, stroke, "RT_d", 20.0, 0.5) }
pub fn v10_8_4_tw_d(bore: f64, stroke: f64) -> Bloc { make(cc::_8_4::CC, bore, stroke, "TW_d", 19.0, 0.9) }
pub fn v10_8_4_cf_d(bore: f64, stroke: f64) -> Bloc { make(cc::_8_4::CC, bore, stroke, "CF_d", 18.5, 1.2) }

