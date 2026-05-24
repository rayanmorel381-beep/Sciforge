use crate::components::powertrain::engines::thermals::assemblies::{aspiration::Aspiration, bloc::Bloc, layout::Layout};
use crate::components::powertrain::engines::thermals::parts::culasses::Culasse;
use super::cc;

const L: Layout = Layout::V;
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

pub fn v10_4_0_tp_g() -> Bloc { make(cc::_4_0::CC, 82.4, 74.9) }
pub fn v10_4_2_tp_g() -> Bloc { make(cc::_4_2::CC, 83.8, 76.2) }
pub fn v10_5_0_tp_g() -> Bloc { make(cc::_5_0::CC, 88.8, 80.7) }
pub fn v10_5_2_tp_g() -> Bloc { make(cc::_5_2::CC, 90.0, 81.8) }
pub fn v10_5_7_tp_g() -> Bloc { make(cc::_5_7::CC, 92.8, 84.4) }
pub fn v10_8_3_tp_g() -> Bloc { make(cc::_8_3::CC, 105.1, 95.5) }
pub fn v10_8_4_tp_g() -> Bloc { make(cc::_8_4::CC, 105.6, 96.0) }

pub fn all() -> Vec<Bloc> {
    vec![v10_4_0_tp_g(), v10_4_2_tp_g(), v10_5_0_tp_g(), v10_5_2_tp_g(), v10_5_7_tp_g(), v10_8_3_tp_g(), v10_8_4_tp_g()]
}
