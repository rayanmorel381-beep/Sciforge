use crate::components::powertrain::engines::thermals::assemblies::{aspiration::Aspiration, bloc::Bloc, layout::Layout};
use crate::components::powertrain::engines::thermals::parts::culasses::Culasse;
use super::cc;

const L: Layout = Layout::Inline;
const N: u8 = 3;

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

pub fn i3_0_6_std_g() -> Bloc { make(cc::_0_6::CC, 61.6, 67.0, "STD_g", 11.0) }
pub fn i3_0_6_hc_g()  -> Bloc { make(cc::_0_6::CC, 61.6, 67.0, "HC_g",  13.0) }
pub fn i3_0_7_std_g() -> Bloc { make(cc::_0_7::CC, 64.9, 70.5, "STD_g", 11.0) }
pub fn i3_0_7_hc_g()  -> Bloc { make(cc::_0_7::CC, 64.9, 70.5, "HC_g",  13.0) }
pub fn i3_0_8_std_g() -> Bloc { make(cc::_0_8::CC, 67.9, 73.8, "STD_g", 11.0) }
pub fn i3_0_8_hc_g()  -> Bloc { make(cc::_0_8::CC, 67.9, 73.8, "HC_g",  13.0) }
pub fn i3_0_9_std_g() -> Bloc { make(cc::_0_9::CC, 70.6, 76.7, "STD_g", 11.0) }
pub fn i3_0_9_hc_g()  -> Bloc { make(cc::_0_9::CC, 70.6, 76.7, "HC_g",  13.0) }
pub fn i3_1_0_std_g() -> Bloc { make(cc::_1_0::CC, 73.1, 79.5, "STD_g", 11.0) }
pub fn i3_1_0_hc_g()  -> Bloc { make(cc::_1_0::CC, 73.1, 79.5, "HC_g",  13.0) }
pub fn i3_1_1_std_g() -> Bloc { make(cc::_1_1::CC, 75.4, 82.0, "STD_g", 11.0) }
pub fn i3_1_1_hc_g()  -> Bloc { make(cc::_1_1::CC, 75.4, 82.0, "HC_g",  13.0) }
pub fn i3_1_2_std_g() -> Bloc { make(cc::_1_2::CC, 77.7, 84.5, "STD_g", 11.0) }
pub fn i3_1_2_hc_g()  -> Bloc { make(cc::_1_2::CC, 77.7, 84.5, "HC_g",  13.0) }
pub fn i3_1_3_std_g() -> Bloc { make(cc::_1_3::CC, 79.8, 86.7, "STD_g", 11.0) }
pub fn i3_1_3_hc_g()  -> Bloc { make(cc::_1_3::CC, 79.8, 86.7, "HC_g",  13.0) }
pub fn i3_1_4_std_g() -> Bloc { make(cc::_1_4::CC, 81.8, 88.9, "STD_g", 11.0) }
pub fn i3_1_4_hc_g()  -> Bloc { make(cc::_1_4::CC, 81.8, 88.9, "HC_g",  13.0) }
pub fn i3_1_5_std_g() -> Bloc { make(cc::_1_5::CC, 83.7, 91.0, "STD_g", 11.0) }
pub fn i3_1_5_hc_g()  -> Bloc { make(cc::_1_5::CC, 83.7, 91.0, "HC_g",  13.0) }

pub fn all() -> Vec<Bloc> {
    vec![
        i3_0_6_std_g(), i3_0_6_hc_g(), i3_0_7_std_g(), i3_0_7_hc_g(),
        i3_0_8_std_g(), i3_0_8_hc_g(), i3_0_9_std_g(), i3_0_9_hc_g(),
        i3_1_0_std_g(), i3_1_0_hc_g(), i3_1_1_std_g(), i3_1_1_hc_g(),
        i3_1_2_std_g(), i3_1_2_hc_g(), i3_1_3_std_g(), i3_1_3_hc_g(),
        i3_1_4_std_g(), i3_1_4_hc_g(), i3_1_5_std_g(), i3_1_5_hc_g(),
    ]
}
