use crate::components::powertrain::engines::thermals::assemblies::{aspiration::Aspiration, bloc::Bloc, layout::Layout};
use crate::components::powertrain::engines::thermals::parts::culasses::Culasse;
use super::cc;

const L: Layout = Layout::Inline;
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

pub fn i6_2_0_tp_g() -> Bloc { make(cc::_2_0::CC, 73.9, 77.8) }
pub fn i6_2_3_tp_g() -> Bloc { make(cc::_2_3::CC, 77.4, 81.5) }
pub fn i6_2_5_tp_g() -> Bloc { make(cc::_2_5::CC, 79.6, 83.8) }
pub fn i6_2_8_tp_g() -> Bloc { make(cc::_2_8::CC, 82.6, 86.9) }
pub fn i6_3_0_tp_g() -> Bloc { make(cc::_3_0::CC, 84.6, 89.1) }
pub fn i6_3_2_tp_g() -> Bloc { make(cc::_3_2::CC, 86.4, 90.9) }
pub fn i6_3_5_tp_g() -> Bloc { make(cc::_3_5::CC, 89.0, 93.7) }

pub fn all() -> Vec<Bloc> {
    vec![i6_2_0_tp_g(), i6_2_3_tp_g(), i6_2_5_tp_g(), i6_2_8_tp_g(), i6_3_0_tp_g(), i6_3_2_tp_g(), i6_3_5_tp_g()]
}
