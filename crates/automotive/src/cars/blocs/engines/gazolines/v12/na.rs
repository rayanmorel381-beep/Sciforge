use crate::components::powertrain::engines::thermals::assemblies::{aspiration::Aspiration, bloc::Bloc, layout::Layout};
use crate::components::powertrain::engines::thermals::parts::culasses::Culasse;
use super::cc;

const L: Layout = Layout::V;
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

pub fn v12_5_0_std_g() -> Bloc { make(cc::_5_0::CC, 83.6, 76.0, "STD_g", 11.0) }
pub fn v12_5_0_hc_g()  -> Bloc { make(cc::_5_0::CC, 83.6, 76.0, "HC_g",  13.0) }
pub fn v12_5_5_std_g() -> Bloc { make(cc::_5_5::CC, 86.3, 78.5, "STD_g", 11.0) }
pub fn v12_5_5_hc_g()  -> Bloc { make(cc::_5_5::CC, 86.3, 78.5, "HC_g",  13.0) }
pub fn v12_6_0_std_g() -> Bloc { make(cc::_6_0::CC, 88.8, 80.7, "STD_g", 11.0) }
pub fn v12_6_0_hc_g()  -> Bloc { make(cc::_6_0::CC, 88.8, 80.7, "HC_g",  13.0) }
pub fn v12_6_2_std_g() -> Bloc { make(cc::_6_2::CC, 89.8, 81.6, "STD_g", 11.0) }
pub fn v12_6_2_hc_g()  -> Bloc { make(cc::_6_2::CC, 89.8, 81.6, "HC_g",  13.0) }
pub fn v12_6_3_std_g() -> Bloc { make(cc::_6_3::CC, 90.3, 82.1, "STD_g", 11.0) }
pub fn v12_6_3_hc_g()  -> Bloc { make(cc::_6_3::CC, 90.3, 82.1, "HC_g",  13.0) }
pub fn v12_6_5_std_g() -> Bloc { make(cc::_6_5::CC, 91.2, 82.9, "STD_g", 11.0) }
pub fn v12_6_5_hc_g()  -> Bloc { make(cc::_6_5::CC, 91.2, 82.9, "HC_g",  13.0) }
pub fn v12_6_75_std_g() -> Bloc { make(cc::_6_75::CC, 92.4, 84.0, "STD_g", 11.0) }
pub fn v12_6_75_hc_g()  -> Bloc { make(cc::_6_75::CC, 92.4, 84.0, "HC_g",  13.0) }
pub fn v12_7_0_std_g() -> Bloc { make(cc::_7_0::CC, 93.5, 85.0, "STD_g", 11.0) }
pub fn v12_7_0_hc_g()  -> Bloc { make(cc::_7_0::CC, 93.5, 85.0, "HC_g",  13.0) }

pub fn all() -> Vec<Bloc> {
    vec![
        v12_5_0_std_g(), v12_5_0_hc_g(), v12_5_5_std_g(), v12_5_5_hc_g(),
        v12_6_0_std_g(), v12_6_0_hc_g(), v12_6_2_std_g(), v12_6_2_hc_g(),
        v12_6_3_std_g(), v12_6_3_hc_g(), v12_6_5_std_g(), v12_6_5_hc_g(),
        v12_6_75_std_g(), v12_6_75_hc_g(), v12_7_0_std_g(), v12_7_0_hc_g(),
    ]
}
