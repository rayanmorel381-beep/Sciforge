use crate::components::powertrain::engines::thermals::assemblies::{aspiration::Aspiration, bloc::Bloc, layout::Layout};
use crate::components::powertrain::engines::thermals::parts::culasses::Culasse;
use super::cc;

const L: Layout = Layout::Flat;
const N: u8 = 8;

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

pub fn f8_2_5_tp_g() -> Bloc { make(cc::_2_5::CC, 75.5, 69.9) }
pub fn f8_3_0_tp_g() -> Bloc { make(cc::_3_0::CC, 80.2, 74.3) }
pub fn f8_3_3_tp_g() -> Bloc { make(cc::_3_3::CC, 82.8, 76.7) }
pub fn f8_3_5_tp_g() -> Bloc { make(cc::_3_5::CC, 84.4, 78.1) }
pub fn f8_4_0_tp_g() -> Bloc { make(cc::_4_0::CC, 88.3, 81.8) }
pub fn f8_4_5_tp_g() -> Bloc { make(cc::_4_5::CC, 91.8, 85.0) }
pub fn f8_4_9_tp_g() -> Bloc { make(cc::_4_9::CC, 94.4, 87.4) }
pub fn f8_5_0_tp_g() -> Bloc { make(cc::_5_0::CC, 95.1, 88.1) }

pub fn all() -> Vec<Bloc> {
    vec![
        f8_2_5_tp_g(), f8_3_0_tp_g(), f8_3_3_tp_g(), f8_3_5_tp_g(),
        f8_4_0_tp_g(), f8_4_5_tp_g(), f8_4_9_tp_g(), f8_5_0_tp_g(),
    ]
}
