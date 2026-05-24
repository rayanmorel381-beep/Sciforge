use crate::components::powertrain::engines::thermals::assemblies::{aspiration::Aspiration, bloc::Bloc, layout::Layout};
use crate::components::powertrain::engines::thermals::parts::culasses::Culasse;
use super::cc;

const L: Layout = Layout::Inline;
const N: u8 = 5;

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

pub fn i5_2_0_std_g() -> Bloc { make(cc::_2_0::CC, 77.7, 84.5, "STD_g", 11.0) }
pub fn i5_2_0_hc_g()  -> Bloc { make(cc::_2_0::CC, 77.7, 84.5, "HC_g",  13.0) }
pub fn i5_2_1_std_g() -> Bloc { make(cc::_2_1::CC, 78.9, 85.8, "STD_g", 11.0) }
pub fn i5_2_1_hc_g()  -> Bloc { make(cc::_2_1::CC, 78.9, 85.8, "HC_g",  13.0) }
pub fn i5_2_2_std_g() -> Bloc { make(cc::_2_2::CC, 80.2, 87.2, "STD_g", 11.0) }
pub fn i5_2_2_hc_g()  -> Bloc { make(cc::_2_2::CC, 80.2, 87.2, "HC_g",  13.0) }
pub fn i5_2_3_std_g() -> Bloc { make(cc::_2_3::CC, 81.4, 88.5, "STD_g", 11.0) }
pub fn i5_2_3_hc_g()  -> Bloc { make(cc::_2_3::CC, 81.4, 88.5, "HC_g",  13.0) }
pub fn i5_2_4_std_g() -> Bloc { make(cc::_2_4::CC, 82.5, 89.7, "STD_g", 11.0) }
pub fn i5_2_4_hc_g()  -> Bloc { make(cc::_2_4::CC, 82.5, 89.7, "HC_g",  13.0) }
pub fn i5_2_5_std_g() -> Bloc { make(cc::_2_5::CC, 83.7, 91.0, "STD_g", 11.0) }
pub fn i5_2_5_hc_g()  -> Bloc { make(cc::_2_5::CC, 83.7, 91.0, "HC_g",  13.0) }

pub fn all() -> Vec<Bloc> {
    vec![
        i5_2_0_std_g(), i5_2_0_hc_g(), i5_2_1_std_g(), i5_2_1_hc_g(),
        i5_2_2_std_g(), i5_2_2_hc_g(), i5_2_3_std_g(), i5_2_3_hc_g(),
        i5_2_4_std_g(), i5_2_4_hc_g(), i5_2_5_std_g(), i5_2_5_hc_g(),
    ]
}
