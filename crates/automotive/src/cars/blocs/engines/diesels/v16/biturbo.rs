use crate::components::powertrain::engines::thermals::assemblies::{aspiration::Aspiration, bloc::Bloc, layout::Layout};
use super::cc;

const L: Layout = Layout::V;
const N: u8 = 16;

fn make(cc_val: u32, bore: f64, stroke: f64) -> Bloc {
    Bloc {
        layout: L, cylinders: N,
        displacement_cc: cc_val, bore_mm: bore, stroke_mm: stroke,
        aspiration: Aspiration::Turbo(2), variant: "TP_d",
        compression_ratio: 15.5, max_boost_bar: 2.5,
        conrod_ratio_lambda: 0.3,
        design_safety_factor: 2.5,
        head_variants: vec![crate::components::powertrain::engines::thermals::parts::culasses::Culasse::ohv_2v_1p_bowl(N), crate::components::powertrain::engines::thermals::parts::culasses::Culasse::sohc_2v_1p_pre_chamber(N), crate::components::powertrain::engines::thermals::parts::culasses::Culasse::dohc_4v_1p_bowl(N), crate::components::powertrain::engines::thermals::parts::culasses::Culasse::dohc_4v_2p_bowl(N)],
    }
}

pub fn v16_6_0_tp_d(bore: f64, stroke: f64) -> Bloc { make(cc::_6_0::CC, bore, stroke) }
pub fn v16_6_75_tp_d(bore: f64, stroke: f64) -> Bloc { make(cc::_6_75::CC, bore, stroke) }
pub fn v16_7_0_tp_d(bore: f64, stroke: f64) -> Bloc { make(cc::_7_0::CC, bore, stroke) }
pub fn v16_7_4_tp_d(bore: f64, stroke: f64) -> Bloc { make(cc::_7_4::CC, bore, stroke) }
pub fn v16_8_0_tp_d(bore: f64, stroke: f64) -> Bloc { make(cc::_8_0::CC, bore, stroke) }
pub fn v16_8_4_tp_d(bore: f64, stroke: f64) -> Bloc { make(cc::_8_4::CC, bore, stroke) }
pub fn v16_9_0_tp_d(bore: f64, stroke: f64) -> Bloc { make(cc::_9_0::CC, bore, stroke) }
pub fn v16_13_6_tp_d(bore: f64, stroke: f64) -> Bloc { make(cc::_13_6::CC, bore, stroke) }

