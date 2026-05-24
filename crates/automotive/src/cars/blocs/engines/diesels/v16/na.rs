use crate::components::powertrain::engines::thermals::assemblies::{aspiration::Aspiration, bloc::Bloc, layout::Layout};
use super::cc;

const L: Layout = Layout::V;
const N: u8 = 16;

fn make(cc_val: u32, bore: f64, stroke: f64, variant: &'static str, cr: f64) -> Bloc {
    Bloc {
        layout: L, cylinders: N,
        displacement_cc: cc_val, bore_mm: bore, stroke_mm: stroke,
        aspiration: Aspiration::Na, variant,
        compression_ratio: cr, max_boost_bar: 0.0,
        conrod_ratio_lambda: 0.3,
        design_safety_factor: 2.5,
        head_variants: vec![crate::components::powertrain::engines::thermals::parts::culasses::Culasse::ohv_2v_1p_bowl(N), crate::components::powertrain::engines::thermals::parts::culasses::Culasse::sohc_2v_1p_pre_chamber(N), crate::components::powertrain::engines::thermals::parts::culasses::Culasse::dohc_4v_1p_bowl(N), crate::components::powertrain::engines::thermals::parts::culasses::Culasse::dohc_4v_2p_bowl(N)],
    }
}

pub fn v16_6_0_std_d(bore: f64, stroke: f64) -> Bloc { make(cc::_6_0::CC, bore, stroke, "STD_d", 18.5) }
pub fn v16_6_0_hc_d(bore: f64, stroke: f64) -> Bloc { make(cc::_6_0::CC, bore, stroke, "HC_d",  21.0) }
pub fn v16_6_75_std_d(bore: f64, stroke: f64) -> Bloc { make(cc::_6_75::CC, bore, stroke, "STD_d", 18.5) }
pub fn v16_6_75_hc_d(bore: f64, stroke: f64) -> Bloc { make(cc::_6_75::CC, bore, stroke, "HC_d",  21.0) }
pub fn v16_7_0_std_d(bore: f64, stroke: f64) -> Bloc { make(cc::_7_0::CC, bore, stroke, "STD_d", 18.5) }
pub fn v16_7_0_hc_d(bore: f64, stroke: f64) -> Bloc { make(cc::_7_0::CC, bore, stroke, "HC_d",  21.0) }
pub fn v16_7_4_std_d(bore: f64, stroke: f64) -> Bloc { make(cc::_7_4::CC, bore, stroke, "STD_d", 18.5) }
pub fn v16_7_4_hc_d(bore: f64, stroke: f64) -> Bloc { make(cc::_7_4::CC, bore, stroke, "HC_d",  21.0) }
pub fn v16_8_0_std_d(bore: f64, stroke: f64) -> Bloc { make(cc::_8_0::CC, bore, stroke, "STD_d", 18.5) }
pub fn v16_8_0_hc_d(bore: f64, stroke: f64) -> Bloc { make(cc::_8_0::CC, bore, stroke, "HC_d",  21.0) }
pub fn v16_8_4_std_d(bore: f64, stroke: f64) -> Bloc { make(cc::_8_4::CC, bore, stroke, "STD_d", 18.5) }
pub fn v16_8_4_hc_d(bore: f64, stroke: f64) -> Bloc { make(cc::_8_4::CC, bore, stroke, "HC_d",  21.0) }
pub fn v16_9_0_std_d(bore: f64, stroke: f64) -> Bloc { make(cc::_9_0::CC, bore, stroke, "STD_d", 18.5) }
pub fn v16_9_0_hc_d(bore: f64, stroke: f64) -> Bloc { make(cc::_9_0::CC, bore, stroke, "HC_d",  21.0) }
pub fn v16_13_6_std_d(bore: f64, stroke: f64) -> Bloc { make(cc::_13_6::CC, bore, stroke, "STD_d", 18.5) }
pub fn v16_13_6_hc_d(bore: f64, stroke: f64) -> Bloc { make(cc::_13_6::CC, bore, stroke, "HC_d",  21.0) }

