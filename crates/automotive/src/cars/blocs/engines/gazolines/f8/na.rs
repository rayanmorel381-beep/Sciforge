use crate::components::powertrain::engines::thermals::assemblies::{aspiration::Aspiration, bloc::Bloc, layout::Layout};
use crate::components::powertrain::engines::thermals::parts::culasses::Culasse;
use super::cc;

const L: Layout = Layout::Flat;
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

pub fn f8_2_5_std_g() -> Bloc { make(cc::_2_5::CC, 75.5, 69.9, "STD_g", 11.0) }
pub fn f8_2_5_hc_g()  -> Bloc { make(cc::_2_5::CC, 75.5, 69.9, "HC_g",  13.0) }
pub fn f8_3_0_std_g() -> Bloc { make(cc::_3_0::CC, 80.2, 74.3, "STD_g", 11.0) }
pub fn f8_3_0_hc_g()  -> Bloc { make(cc::_3_0::CC, 80.2, 74.3, "HC_g",  13.0) }
pub fn f8_3_3_std_g() -> Bloc { make(cc::_3_3::CC, 82.8, 76.7, "STD_g", 11.0) }
pub fn f8_3_3_hc_g()  -> Bloc { make(cc::_3_3::CC, 82.8, 76.7, "HC_g",  13.0) }
pub fn f8_3_5_std_g() -> Bloc { make(cc::_3_5::CC, 84.4, 78.1, "STD_g", 11.0) }
pub fn f8_3_5_hc_g()  -> Bloc { make(cc::_3_5::CC, 84.4, 78.1, "HC_g",  13.0) }
pub fn f8_4_0_std_g() -> Bloc { make(cc::_4_0::CC, 88.3, 81.8, "STD_g", 11.0) }
pub fn f8_4_0_hc_g()  -> Bloc { make(cc::_4_0::CC, 88.3, 81.8, "HC_g",  13.0) }
pub fn f8_4_5_std_g() -> Bloc { make(cc::_4_5::CC, 91.8, 85.0, "STD_g", 11.0) }
pub fn f8_4_5_hc_g()  -> Bloc { make(cc::_4_5::CC, 91.8, 85.0, "HC_g",  13.0) }
pub fn f8_4_9_std_g() -> Bloc { make(cc::_4_9::CC, 94.4, 87.4, "STD_g", 11.0) }
pub fn f8_4_9_hc_g()  -> Bloc { make(cc::_4_9::CC, 94.4, 87.4, "HC_g",  13.0) }
pub fn f8_5_0_std_g() -> Bloc { make(cc::_5_0::CC, 95.1, 88.1, "STD_g", 11.0) }
pub fn f8_5_0_hc_g()  -> Bloc { make(cc::_5_0::CC, 95.1, 88.1, "HC_g",  13.0) }

pub fn all() -> Vec<Bloc> {
    vec![
        f8_2_5_std_g(), f8_2_5_hc_g(), f8_3_0_std_g(), f8_3_0_hc_g(),
        f8_3_3_std_g(), f8_3_3_hc_g(), f8_3_5_std_g(), f8_3_5_hc_g(),
        f8_4_0_std_g(), f8_4_0_hc_g(), f8_4_5_std_g(), f8_4_5_hc_g(),
        f8_4_9_std_g(), f8_4_9_hc_g(), f8_5_0_std_g(), f8_5_0_hc_g(),
    ]
}
