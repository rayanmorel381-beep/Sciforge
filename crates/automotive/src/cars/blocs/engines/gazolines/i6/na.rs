use crate::components::powertrain::engines::thermals::assemblies::{aspiration::Aspiration, bloc::Bloc, layout::Layout};
use crate::components::powertrain::engines::thermals::parts::culasses::Culasse;
use super::cc;

const L: Layout = Layout::Inline;
const N: u8 = 6;

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

pub fn i6_2_0_std_g() -> Bloc { make(cc::_2_0::CC, 73.9, 77.8, "STD_g", 11.0) }
pub fn i6_2_0_hc_g()  -> Bloc { make(cc::_2_0::CC, 73.9, 77.8, "HC_g",  13.0) }
pub fn i6_2_3_std_g() -> Bloc { make(cc::_2_3::CC, 77.4, 81.5, "STD_g", 11.0) }
pub fn i6_2_3_hc_g()  -> Bloc { make(cc::_2_3::CC, 77.4, 81.5, "HC_g",  13.0) }
pub fn i6_2_5_std_g() -> Bloc { make(cc::_2_5::CC, 79.6, 83.8, "STD_g", 11.0) }
pub fn i6_2_5_hc_g()  -> Bloc { make(cc::_2_5::CC, 79.6, 83.8, "HC_g",  13.0) }
pub fn i6_2_8_std_g() -> Bloc { make(cc::_2_8::CC, 82.6, 86.9, "STD_g", 11.0) }
pub fn i6_2_8_hc_g()  -> Bloc { make(cc::_2_8::CC, 82.6, 86.9, "HC_g",  13.0) }
pub fn i6_3_0_std_g() -> Bloc { make(cc::_3_0::CC, 84.6, 89.1, "STD_g", 11.0) }
pub fn i6_3_0_hc_g()  -> Bloc { make(cc::_3_0::CC, 84.6, 89.1, "HC_g",  13.0) }
pub fn i6_3_2_std_g() -> Bloc { make(cc::_3_2::CC, 86.4, 90.9, "STD_g", 11.0) }
pub fn i6_3_2_hc_g()  -> Bloc { make(cc::_3_2::CC, 86.4, 90.9, "HC_g",  13.0) }
pub fn i6_3_5_std_g() -> Bloc { make(cc::_3_5::CC, 89.0, 93.7, "STD_g", 11.0) }
pub fn i6_3_5_hc_g()  -> Bloc { make(cc::_3_5::CC, 89.0, 93.7, "HC_g",  13.0) }

pub fn all() -> Vec<Bloc> {
    vec![
        i6_2_0_std_g(), i6_2_0_hc_g(), i6_2_3_std_g(), i6_2_3_hc_g(),
        i6_2_5_std_g(), i6_2_5_hc_g(), i6_2_8_std_g(), i6_2_8_hc_g(),
        i6_3_0_std_g(), i6_3_0_hc_g(), i6_3_2_std_g(), i6_3_2_hc_g(),
        i6_3_5_std_g(), i6_3_5_hc_g(),
    ]
}
