use crate::components::powertrain::engines::thermals::assemblies::{aspiration::Aspiration, bloc::Bloc, layout::Layout};
use crate::components::powertrain::engines::thermals::parts::culasses::Culasse;
use super::cc;

const L: Layout = Layout::V;
const N: u8 = 6;

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

pub fn v6_2_0_tp_g() -> Bloc { make(cc::_2_0::CC, 76.4, 72.8) }
pub fn v6_2_3_tp_g() -> Bloc { make(cc::_2_3::CC, 80.0, 76.2) }
pub fn v6_2_5_tp_g() -> Bloc { make(cc::_2_5::CC, 82.3, 78.4) }
pub fn v6_2_7_tp_g() -> Bloc { make(cc::_2_7::CC, 84.4, 80.4) }
pub fn v6_2_8_tp_g() -> Bloc { make(cc::_2_8::CC, 85.4, 81.3) }
pub fn v6_3_0_tp_g() -> Bloc { make(cc::_3_0::CC, 87.4, 83.2) }
pub fn v6_3_2_tp_g() -> Bloc { make(cc::_3_2::CC, 89.3, 85.0) }
pub fn v6_3_5_tp_g() -> Bloc { make(cc::_3_5::CC, 92.0, 87.6) }
pub fn v6_3_6_tp_g() -> Bloc { make(cc::_3_6::CC, 92.9, 88.5) }
pub fn v6_3_7_tp_g() -> Bloc { make(cc::_3_7::CC, 93.8, 89.3) }
pub fn v6_3_8_tp_g() -> Bloc { make(cc::_3_8::CC, 94.6, 90.1) }
pub fn v6_4_0_tp_g() -> Bloc { make(cc::_4_0::CC, 96.2, 91.6) }

pub fn all() -> Vec<Bloc> {
    vec![v6_2_0_tp_g(), v6_2_3_tp_g(), v6_2_5_tp_g(), v6_2_7_tp_g(), v6_2_8_tp_g(), v6_3_0_tp_g(), v6_3_2_tp_g(), v6_3_5_tp_g(), v6_3_6_tp_g(), v6_3_7_tp_g(), v6_3_8_tp_g(), v6_4_0_tp_g()]
}
