use crate::components::powertrain::engines::thermals::assemblies::{aspiration::Aspiration, bloc::Bloc, layout::Layout};
use crate::components::powertrain::engines::thermals::parts::culasses::Culasse;
use super::cc;

const L: Layout = Layout::Inline;
const N: u8 = 2;

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

pub fn i2_0_6_tp_g() -> Bloc { make(cc::_0_6::CC, 70.1, 77.9) }
pub fn i2_0_8_tp_g() -> Bloc { make(cc::_0_8::CC, 77.1, 85.7) }
pub fn i2_1_0_tp_g() -> Bloc { make(cc::_1_0::CC, 83.1, 92.3) }
pub fn i2_1_2_tp_g() -> Bloc { make(cc::_1_2::CC, 88.3, 98.1) }
pub fn i2_1_4_tp_g() -> Bloc { make(cc::_1_4::CC, 92.9, 103.2) }
pub fn i2_1_6_tp_g() -> Bloc { make(cc::_1_6::CC, 97.1, 107.9) }
pub fn i2_1_8_tp_g() -> Bloc { make(cc::_1_8::CC, 101.0, 112.2) }
pub fn i2_2_0_tp_g() -> Bloc { make(cc::_2_0::CC, 104.6, 116.2) }

pub fn all() -> Vec<Bloc> {
    vec![i2_0_6_tp_g(), i2_0_8_tp_g(), i2_1_0_tp_g(), i2_1_2_tp_g(), i2_1_4_tp_g(), i2_1_6_tp_g(), i2_1_8_tp_g(), i2_2_0_tp_g()]
}
