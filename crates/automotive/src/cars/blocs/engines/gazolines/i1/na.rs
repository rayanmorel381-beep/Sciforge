use crate::components::powertrain::engines::thermals::assemblies::{aspiration::Aspiration, bloc::Bloc, layout::Layout};
use crate::components::powertrain::engines::thermals::parts::culasses::Culasse;
use super::cc;

const L: Layout = Layout::Inline;
const N: u8 = 1;

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

pub fn i1_0_1_std_g() -> Bloc { make(cc::_0_1::CC, 47.7, 56.1, "STD_g", 11.0) }
pub fn i1_0_1_hc_g()  -> Bloc { make(cc::_0_1::CC, 47.7, 56.1, "HC_g",  13.0) }
pub fn i1_0_2_std_g() -> Bloc { make(cc::_0_2::CC, 60.0, 70.6, "STD_g", 11.0) }
pub fn i1_0_2_hc_g()  -> Bloc { make(cc::_0_2::CC, 60.0, 70.6, "HC_g",  13.0) }
pub fn i1_0_3_std_g() -> Bloc { make(cc::_0_3::CC, 68.7, 80.8, "STD_g", 11.0) }
pub fn i1_0_3_hc_g()  -> Bloc { make(cc::_0_3::CC, 68.7, 80.8, "HC_g",  13.0) }
pub fn i1_0_4_std_g() -> Bloc { make(cc::_0_4::CC, 75.6, 88.9, "STD_g", 11.0) }
pub fn i1_0_4_hc_g()  -> Bloc { make(cc::_0_4::CC, 75.6, 88.9, "HC_g",  13.0) }
pub fn i1_0_5_std_g() -> Bloc { make(cc::_0_5::CC, 81.5, 95.9, "STD_g", 11.0) }
pub fn i1_0_5_hc_g()  -> Bloc { make(cc::_0_5::CC, 81.5, 95.9, "HC_g",  13.0) }
pub fn i1_0_6_std_g() -> Bloc { make(cc::_0_6::CC, 86.6, 101.9, "STD_g", 11.0) }
pub fn i1_0_6_hc_g()  -> Bloc { make(cc::_0_6::CC, 86.6, 101.9, "HC_g",  13.0) }
pub fn i1_0_7_std_g() -> Bloc { make(cc::_0_7::CC, 91.2, 107.3, "STD_g", 11.0) }
pub fn i1_0_7_hc_g()  -> Bloc { make(cc::_0_7::CC, 91.2, 107.3, "HC_g",  13.0) }
pub fn i1_0_8_std_g() -> Bloc { make(cc::_0_8::CC, 95.3, 112.1, "STD_g", 11.0) }
pub fn i1_0_8_hc_g()  -> Bloc { make(cc::_0_8::CC, 95.3, 112.1, "HC_g",  13.0) }

pub fn all() -> Vec<Bloc> {
    vec![
        i1_0_1_std_g(), i1_0_1_hc_g(), i1_0_2_std_g(), i1_0_2_hc_g(),
        i1_0_3_std_g(), i1_0_3_hc_g(), i1_0_4_std_g(), i1_0_4_hc_g(),
        i1_0_5_std_g(), i1_0_5_hc_g(), i1_0_6_std_g(), i1_0_6_hc_g(),
        i1_0_7_std_g(), i1_0_7_hc_g(), i1_0_8_std_g(), i1_0_8_hc_g(),
    ]
}
