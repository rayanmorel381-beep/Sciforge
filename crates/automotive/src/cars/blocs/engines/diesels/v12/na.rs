use crate::components::powertrain::engines::thermals::assemblies::{aspiration::Aspiration, bloc::Bloc, layout::Layout};
use super::cc;

const L: Layout = Layout::V;
const N: u8 = 12;

fn make(cc_val: u32, bore: f64, stroke: f64, variant: &'static str, cr: f64) -> Bloc {
    Bloc {
        layout: L, cylinders: N,
        displacement_cc: cc_val, bore_mm: bore, stroke_mm: stroke,
        aspiration: Aspiration::Na, variant,
        compression_ratio: cr, max_boost_bar: 0.0,
        conrod_ratio_lambda: 0.3,
        design_safety_factor: 2.5,
        head_variants: vec![crate::components::powertrain::engines::thermals::parts::culasses::Culasse::ohv_2v_1p_bowl(N), crate::components::powertrain::engines::thermals::parts::culasses::Culasse::sohc_2v_1p_pre_chamber(N), crate::components::powertrain::engines::thermals::parts::culasses::Culasse::dohc_4v_1p_bowl(N), crate::components::powertrain::engines::thermals::parts::culasses::Culasse::dohc_4v_2p_bowl(N)],
    }
}

pub fn v12_5_0_std_d(bore: f64, stroke: f64) -> Bloc { make(cc::_5_0::CC, bore, stroke, "STD_d", 18.5) }
pub fn v12_5_0_hc_d(bore: f64, stroke: f64) -> Bloc { make(cc::_5_0::CC, bore, stroke, "HC_d",  21.0) }
pub fn v12_5_5_std_d(bore: f64, stroke: f64) -> Bloc { make(cc::_5_5::CC, bore, stroke, "STD_d", 18.5) }
pub fn v12_5_5_hc_d(bore: f64, stroke: f64) -> Bloc { make(cc::_5_5::CC, bore, stroke, "HC_d",  21.0) }
pub fn v12_6_0_std_d(bore: f64, stroke: f64) -> Bloc { make(cc::_6_0::CC, bore, stroke, "STD_d", 18.5) }
pub fn v12_6_0_hc_d(bore: f64, stroke: f64) -> Bloc { make(cc::_6_0::CC, bore, stroke, "HC_d",  21.0) }
pub fn v12_6_2_std_d(bore: f64, stroke: f64) -> Bloc { make(cc::_6_2::CC, bore, stroke, "STD_d", 18.5) }
pub fn v12_6_2_hc_d(bore: f64, stroke: f64) -> Bloc { make(cc::_6_2::CC, bore, stroke, "HC_d",  21.0) }
pub fn v12_6_3_std_d(bore: f64, stroke: f64) -> Bloc { make(cc::_6_3::CC, bore, stroke, "STD_d", 18.5) }
pub fn v12_6_3_hc_d(bore: f64, stroke: f64) -> Bloc { make(cc::_6_3::CC, bore, stroke, "HC_d",  21.0) }
pub fn v12_6_5_std_d(bore: f64, stroke: f64) -> Bloc { make(cc::_6_5::CC, bore, stroke, "STD_d", 18.5) }
pub fn v12_6_5_hc_d(bore: f64, stroke: f64) -> Bloc { make(cc::_6_5::CC, bore, stroke, "HC_d",  21.0) }
pub fn v12_6_75_std_d(bore: f64, stroke: f64) -> Bloc { make(cc::_6_75::CC, bore, stroke, "STD_d", 18.5) }
pub fn v12_6_75_hc_d(bore: f64, stroke: f64) -> Bloc { make(cc::_6_75::CC, bore, stroke, "HC_d",  21.0) }
pub fn v12_7_0_std_d(bore: f64, stroke: f64) -> Bloc { make(cc::_7_0::CC, bore, stroke, "STD_d", 18.5) }
pub fn v12_7_0_hc_d(bore: f64, stroke: f64) -> Bloc { make(cc::_7_0::CC, bore, stroke, "HC_d",  21.0) }

