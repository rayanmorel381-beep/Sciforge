use crate::components::powertrain::engines::thermals::assemblies::{aspiration::Aspiration, bloc::Bloc, layout::Layout};
use crate::components::powertrain::engines::thermals::parts::culasses::Culasse;
use super::cc;

const L: Layout = Layout::W;
const N: u8 = 12;

fn make(cc_val: u32, bore: f64, stroke: f64, variant: &'static str, cr: f64) -> Bloc {
    Bloc {
        layout: L, cylinders: N,
        displacement_cc: cc_val, bore_mm: bore, stroke_mm: stroke,
        aspiration: Aspiration::Na, variant,
        compression_ratio: cr, max_boost_bar: 0.0,
        conrod_ratio_lambda: 0.30,
        design_safety_factor: 2.5,
        head_variants: vec![Culasse::ohv_2v_1p_wedge(N), Culasse::sohc_2v_1p_wedge(N), Culasse::dohc_4v_1p_pent_roof(N), Culasse::dohc_4v_2p_hemi(N), Culasse::dohc_5v_1p(N)],
    }
}

pub fn w12_5_0_std_g() -> Bloc { make(cc::_5_0::CC, 83.1, 76.9, "STD_g", 11.0) }
pub fn w12_5_0_hc_g()  -> Bloc { make(cc::_5_0::CC, 83.1, 76.9, "HC_g",  13.0) }
pub fn w12_6_0_std_g() -> Bloc { make(cc::_6_0::CC, 88.3, 81.8, "STD_g", 11.0) }
pub fn w12_6_0_hc_g()  -> Bloc { make(cc::_6_0::CC, 88.3, 81.8, "HC_g",  13.0) }
pub fn w12_6_3_std_g() -> Bloc { make(cc::_6_3::CC, 89.7, 83.1, "STD_g", 11.0) }
pub fn w12_6_3_hc_g()  -> Bloc { make(cc::_6_3::CC, 89.7, 83.1, "HC_g",  13.0) }

pub fn all() -> Vec<Bloc> {
    vec![w12_5_0_std_g(), w12_5_0_hc_g(), w12_6_0_std_g(), w12_6_0_hc_g(), w12_6_3_std_g(), w12_6_3_hc_g()]
}
