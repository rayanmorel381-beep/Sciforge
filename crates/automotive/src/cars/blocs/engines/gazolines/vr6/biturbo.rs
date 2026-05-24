use crate::components::powertrain::engines::thermals::assemblies::{aspiration::Aspiration, bloc::Bloc, layout::Layout};
use crate::components::powertrain::engines::thermals::parts::culasses::Culasse;
use super::cc;

const L: Layout = Layout::Vr;
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

pub fn vr6_2_8_tp_g() -> Bloc { make(cc::_2_8::CC, 84.9, 82.4) }
pub fn vr6_3_2_tp_g() -> Bloc { make(cc::_3_2::CC, 88.8, 86.2) }
pub fn vr6_3_6_tp_g() -> Bloc { make(cc::_3_6::CC, 92.3, 89.6) }

pub fn all() -> Vec<Bloc> { vec![vr6_2_8_tp_g(), vr6_3_2_tp_g(), vr6_3_6_tp_g()] }
