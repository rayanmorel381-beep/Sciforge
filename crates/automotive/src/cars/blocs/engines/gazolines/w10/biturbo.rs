use crate::components::powertrain::engines::thermals::assemblies::{aspiration::Aspiration, bloc::Bloc, layout::Layout};
use crate::components::powertrain::engines::thermals::parts::culasses::Culasse;
use super::cc;

const L: Layout = Layout::W;
const N: u8 = 10;

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

pub fn w10_4_5_tp_g() -> Bloc { make(cc::_4_5::CC, 85.0, 79.4) }
pub fn w10_5_0_tp_g() -> Bloc { make(cc::_5_0::CC, 88.0, 82.2) }
pub fn w10_5_5_tp_g() -> Bloc { make(cc::_5_5::CC, 90.8, 84.9) }

pub fn all() -> Vec<Bloc> { vec![w10_4_5_tp_g(), w10_5_0_tp_g(), w10_5_5_tp_g()] }
