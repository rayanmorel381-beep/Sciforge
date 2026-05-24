use crate::components::powertrain::engines::thermals::assemblies::{aspiration::Aspiration, bloc::Bloc, layout::Layout};
use super::cc;

const L: Layout = Layout::V;
const N: u8 = 6;

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

pub fn v6_2_0_tp_d(bore: f64, stroke: f64) -> Bloc { make(cc::_2_0::CC, bore, stroke) }
pub fn v6_2_3_tp_d(bore: f64, stroke: f64) -> Bloc { make(cc::_2_3::CC, bore, stroke) }
pub fn v6_2_5_tp_d(bore: f64, stroke: f64) -> Bloc { make(cc::_2_5::CC, bore, stroke) }
pub fn v6_2_7_tp_d(bore: f64, stroke: f64) -> Bloc { make(cc::_2_7::CC, bore, stroke) }
pub fn v6_2_8_tp_d(bore: f64, stroke: f64) -> Bloc { make(cc::_2_8::CC, bore, stroke) }
pub fn v6_3_0_tp_d(bore: f64, stroke: f64) -> Bloc { make(cc::_3_0::CC, bore, stroke) }
pub fn v6_3_2_tp_d(bore: f64, stroke: f64) -> Bloc { make(cc::_3_2::CC, bore, stroke) }
pub fn v6_3_5_tp_d(bore: f64, stroke: f64) -> Bloc { make(cc::_3_5::CC, bore, stroke) }
pub fn v6_3_6_tp_d(bore: f64, stroke: f64) -> Bloc { make(cc::_3_6::CC, bore, stroke) }
pub fn v6_3_7_tp_d(bore: f64, stroke: f64) -> Bloc { make(cc::_3_7::CC, bore, stroke) }
pub fn v6_3_8_tp_d(bore: f64, stroke: f64) -> Bloc { make(cc::_3_8::CC, bore, stroke) }
pub fn v6_4_0_tp_d(bore: f64, stroke: f64) -> Bloc { make(cc::_4_0::CC, bore, stroke) }

