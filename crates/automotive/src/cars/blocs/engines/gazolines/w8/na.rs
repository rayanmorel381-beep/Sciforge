use crate::components::powertrain::engines::thermals::assemblies::{aspiration::Aspiration, bloc::Bloc, layout::Layout};
use crate::components::powertrain::engines::thermals::parts::culasses::Culasse;
use super::cc;

const L: Layout = Layout::W;
const N: u8 = 8;

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

pub fn w8_3_6_std_g() -> Bloc { make(cc::_3_6::CC, 84.4, 80.4, "STD_g", 11.0) }
pub fn w8_3_6_hc_g()  -> Bloc { make(cc::_3_6::CC, 84.4, 80.4, "HC_g",  13.0) }
pub fn w8_4_0_std_g() -> Bloc { make(cc::_4_0::CC, 87.4, 83.2, "STD_g", 11.0) }
pub fn w8_4_0_hc_g()  -> Bloc { make(cc::_4_0::CC, 87.4, 83.2, "HC_g",  13.0) }
pub fn w8_4_2_std_g() -> Bloc { make(cc::_4_2::CC, 88.9, 84.7, "STD_g", 11.0) }
pub fn w8_4_2_hc_g()  -> Bloc { make(cc::_4_2::CC, 88.9, 84.7, "HC_g",  13.0) }

pub fn all() -> Vec<Bloc> {
    vec![w8_3_6_std_g(), w8_3_6_hc_g(), w8_4_0_std_g(), w8_4_0_hc_g(), w8_4_2_std_g(), w8_4_2_hc_g()]
}
