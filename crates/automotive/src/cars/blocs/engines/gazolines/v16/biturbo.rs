use crate::components::powertrain::engines::thermals::assemblies::{aspiration::Aspiration, bloc::Bloc, layout::Layout};
use crate::components::powertrain::engines::thermals::parts::culasses::Culasse;
use super::cc;

const L: Layout = Layout::V;
const N: u8 = 16;

fn make(cc_val: u32, bore: f64, stroke: f64) -> Bloc {
    Bloc {
        layout: L, cylinders: N,
        displacement_cc: cc_val, bore_mm: bore, stroke_mm: stroke,
        aspiration: Aspiration::Turbo(2), variant: "TP_g",
        compression_ratio: 10.0, max_boost_bar: 1.4,
        conrod_ratio_lambda: 0.30,
        design_safety_factor: 2.5,
        head_variants: vec![Culasse::ohv_2v_1p_wedge(N), Culasse::sohc_2v_1p_wedge(N), Culasse::dohc_4v_1p_pent_roof(N), Culasse::dohc_4v_2p_hemi(N), Culasse::dohc_5v_1p(N)],
    }
}

pub fn v16_6_0_tp_g() -> Bloc { make(cc::_6_0::CC, 81.2, 72.5) }
pub fn v16_6_75_tp_g() -> Bloc { make(cc::_6_75::CC, 84.4, 75.4) }
pub fn v16_7_0_tp_g() -> Bloc { make(cc::_7_0::CC, 85.4, 76.2) }
pub fn v16_7_4_tp_g() -> Bloc { make(cc::_7_4::CC, 87.0, 77.7) }
pub fn v16_8_0_tp_g() -> Bloc { make(cc::_8_0::CC, 89.3, 79.7) }
pub fn v16_8_4_tp_g() -> Bloc { make(cc::_8_4::CC, 90.8, 81.1) }
pub fn v16_9_0_tp_g() -> Bloc { make(cc::_9_0::CC, 92.9, 82.9) }
pub fn v16_13_6_tp_g() -> Bloc { make(cc::_13_6::CC, 106.6, 95.2) }

pub fn all() -> Vec<Bloc> {
    vec![v16_6_0_tp_g(), v16_6_75_tp_g(), v16_7_0_tp_g(), v16_7_4_tp_g(), v16_8_0_tp_g(), v16_8_4_tp_g(), v16_9_0_tp_g(), v16_13_6_tp_g()]
}
