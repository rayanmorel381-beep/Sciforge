use crate::components::powertrain::engines::thermals::assemblies::{aspiration::Aspiration, bloc::Bloc, layout::Layout};
use crate::components::powertrain::engines::thermals::parts::culasses::Culasse;
use super::cc;

const L: Layout = Layout::Inline;
const N: u8 = 5;

fn make(cc_val: u32, bore: f64, stroke: f64, variant: &'static str, cr: f64, boost: f64) -> Bloc {
    Bloc {
        layout: L, cylinders: N,
        displacement_cc: cc_val, bore_mm: bore, stroke_mm: stroke,
        aspiration: Aspiration::Turbo(1), variant,
        compression_ratio: cr, max_boost_bar: boost,
        conrod_ratio_lambda: 0.3,
        design_safety_factor: 2.5,
        head_variants: vec![Culasse::ohv_2v_1p_bowl(N), Culasse::sohc_2v_1p_pre_chamber(N), Culasse::dohc_4v_1p_bowl(N), Culasse::dohc_4v_2p_bowl(N)],
    }
}

pub fn i5_2_0_ss_d() -> Bloc { make(cc::_2_0::CC, 77.7, 84.5, "SS_d", 17.5, 1.5) }
pub fn i5_2_0_ts_d() -> Bloc { make(cc::_2_0::CC, 77.7, 84.5, "TS_d", 16.5, 1.9) }
pub fn i5_2_0_vg_d() -> Bloc { make(cc::_2_0::CC, 77.7, 84.5, "VG_d", 16.0, 2.2) }
pub fn i5_2_1_ss_d() -> Bloc { make(cc::_2_1::CC, 78.9, 85.8, "SS_d", 17.5, 1.5) }
pub fn i5_2_1_ts_d() -> Bloc { make(cc::_2_1::CC, 78.9, 85.8, "TS_d", 16.5, 1.9) }
pub fn i5_2_1_vg_d() -> Bloc { make(cc::_2_1::CC, 78.9, 85.8, "VG_d", 16.0, 2.2) }
