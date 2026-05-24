use crate::components::powertrain::engines::thermals::assemblies::{aspiration::Aspiration, bloc::Bloc, layout::Layout};
use super::cc;

const L: Layout = Layout::V;
const N: u8 = 8;

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

pub fn v8_3_5_tp_d(bore: f64, stroke: f64) -> Bloc { make(cc::_3_5::CC, bore, stroke) }
pub fn v8_3_9_tp_d(bore: f64, stroke: f64) -> Bloc { make(cc::_3_9::CC, bore, stroke) }
pub fn v8_4_0_tp_d(bore: f64, stroke: f64) -> Bloc { make(cc::_4_0::CC, bore, stroke) }
pub fn v8_4_4_tp_d(bore: f64, stroke: f64) -> Bloc { make(cc::_4_4::CC, bore, stroke) }
pub fn v8_4_5_tp_d(bore: f64, stroke: f64) -> Bloc { make(cc::_4_5::CC, bore, stroke) }
pub fn v8_4_6_tp_d(bore: f64, stroke: f64) -> Bloc { make(cc::_4_6::CC, bore, stroke) }
pub fn v8_4_8_tp_d(bore: f64, stroke: f64) -> Bloc { make(cc::_4_8::CC, bore, stroke) }
pub fn v8_5_0_tp_d(bore: f64, stroke: f64) -> Bloc { make(cc::_5_0::CC, bore, stroke) }
pub fn v8_5_3_tp_d(bore: f64, stroke: f64) -> Bloc { make(cc::_5_3::CC, bore, stroke) }
pub fn v8_5_5_tp_d(bore: f64, stroke: f64) -> Bloc { make(cc::_5_5::CC, bore, stroke) }
pub fn v8_5_7_tp_d(bore: f64, stroke: f64) -> Bloc { make(cc::_5_7::CC, bore, stroke) }
pub fn v8_6_0_tp_d(bore: f64, stroke: f64) -> Bloc { make(cc::_6_0::CC, bore, stroke) }
pub fn v8_6_2_tp_d(bore: f64, stroke: f64) -> Bloc { make(cc::_6_2::CC, bore, stroke) }
pub fn v8_6_4_tp_d(bore: f64, stroke: f64) -> Bloc { make(cc::_6_4::CC, bore, stroke) }
pub fn v8_7_0_tp_d(bore: f64, stroke: f64) -> Bloc { make(cc::_7_0::CC, bore, stroke) }

