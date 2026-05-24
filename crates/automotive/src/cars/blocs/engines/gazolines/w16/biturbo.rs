use crate::components::powertrain::engines::thermals::assemblies::{aspiration::Aspiration, bloc::Bloc, layout::Layout};
use crate::components::powertrain::engines::thermals::parts::culasses::Culasse;
use super::cc;

const L: Layout = Layout::W;
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

pub fn w16_7_0_tp_g() -> Bloc { make(cc::_7_0::CC, 84.9, 77.2) }
pub fn w16_8_0_tp_g() -> Bloc { make(cc::_8_0::CC, 88.8, 80.7) }
pub fn w16_8_4_tp_g() -> Bloc { make(cc::_8_4::CC, 90.3, 82.1) }

pub fn all() -> Vec<Bloc> { vec![w16_7_0_tp_g(), w16_8_0_tp_g(), w16_8_4_tp_g()] }
