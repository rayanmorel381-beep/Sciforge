use crate::components::powertrain::engines::thermals::assemblies::{aspiration::Aspiration, bloc::Bloc, layout::Layout};
use super::cc;

const L: Layout = Layout::V;
const N: u8 = 12;

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

pub fn v12_5_0_rt_d(bore: f64, stroke: f64) -> Bloc { make(cc::_5_0::CC, bore, stroke, "RT_d", 20.0, 0.5) }
pub fn v12_5_0_tw_d(bore: f64, stroke: f64) -> Bloc { make(cc::_5_0::CC, bore, stroke, "TW_d", 19.0, 0.9) }
pub fn v12_5_0_cf_d(bore: f64, stroke: f64) -> Bloc { make(cc::_5_0::CC, bore, stroke, "CF_d", 18.5, 1.2) }
pub fn v12_5_5_rt_d(bore: f64, stroke: f64) -> Bloc { make(cc::_5_5::CC, bore, stroke, "RT_d", 20.0, 0.5) }
pub fn v12_5_5_tw_d(bore: f64, stroke: f64) -> Bloc { make(cc::_5_5::CC, bore, stroke, "TW_d", 19.0, 0.9) }
pub fn v12_5_5_cf_d(bore: f64, stroke: f64) -> Bloc { make(cc::_5_5::CC, bore, stroke, "CF_d", 18.5, 1.2) }
pub fn v12_6_0_rt_d(bore: f64, stroke: f64) -> Bloc { make(cc::_6_0::CC, bore, stroke, "RT_d", 20.0, 0.5) }
pub fn v12_6_0_tw_d(bore: f64, stroke: f64) -> Bloc { make(cc::_6_0::CC, bore, stroke, "TW_d", 19.0, 0.9) }
pub fn v12_6_0_cf_d(bore: f64, stroke: f64) -> Bloc { make(cc::_6_0::CC, bore, stroke, "CF_d", 18.5, 1.2) }
pub fn v12_6_2_rt_d(bore: f64, stroke: f64) -> Bloc { make(cc::_6_2::CC, bore, stroke, "RT_d", 20.0, 0.5) }
pub fn v12_6_2_tw_d(bore: f64, stroke: f64) -> Bloc { make(cc::_6_2::CC, bore, stroke, "TW_d", 19.0, 0.9) }
pub fn v12_6_2_cf_d(bore: f64, stroke: f64) -> Bloc { make(cc::_6_2::CC, bore, stroke, "CF_d", 18.5, 1.2) }
pub fn v12_6_3_rt_d(bore: f64, stroke: f64) -> Bloc { make(cc::_6_3::CC, bore, stroke, "RT_d", 20.0, 0.5) }
pub fn v12_6_3_tw_d(bore: f64, stroke: f64) -> Bloc { make(cc::_6_3::CC, bore, stroke, "TW_d", 19.0, 0.9) }
pub fn v12_6_3_cf_d(bore: f64, stroke: f64) -> Bloc { make(cc::_6_3::CC, bore, stroke, "CF_d", 18.5, 1.2) }
pub fn v12_6_5_rt_d(bore: f64, stroke: f64) -> Bloc { make(cc::_6_5::CC, bore, stroke, "RT_d", 20.0, 0.5) }
pub fn v12_6_5_tw_d(bore: f64, stroke: f64) -> Bloc { make(cc::_6_5::CC, bore, stroke, "TW_d", 19.0, 0.9) }
pub fn v12_6_5_cf_d(bore: f64, stroke: f64) -> Bloc { make(cc::_6_5::CC, bore, stroke, "CF_d", 18.5, 1.2) }
pub fn v12_6_75_rt_d(bore: f64, stroke: f64) -> Bloc { make(cc::_6_75::CC, bore, stroke, "RT_d", 20.0, 0.5) }
pub fn v12_6_75_tw_d(bore: f64, stroke: f64) -> Bloc { make(cc::_6_75::CC, bore, stroke, "TW_d", 19.0, 0.9) }
pub fn v12_6_75_cf_d(bore: f64, stroke: f64) -> Bloc { make(cc::_6_75::CC, bore, stroke, "CF_d", 18.5, 1.2) }
pub fn v12_7_0_rt_d(bore: f64, stroke: f64) -> Bloc { make(cc::_7_0::CC, bore, stroke, "RT_d", 20.0, 0.5) }
pub fn v12_7_0_tw_d(bore: f64, stroke: f64) -> Bloc { make(cc::_7_0::CC, bore, stroke, "TW_d", 19.0, 0.9) }
pub fn v12_7_0_cf_d(bore: f64, stroke: f64) -> Bloc { make(cc::_7_0::CC, bore, stroke, "CF_d", 18.5, 1.2) }

