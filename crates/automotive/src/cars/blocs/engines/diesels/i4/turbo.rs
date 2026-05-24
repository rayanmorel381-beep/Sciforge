use crate::components::powertrain::engines::thermals::assemblies::{aspiration::Aspiration, bloc::Bloc, layout::Layout};
use crate::components::powertrain::engines::thermals::parts::culasses::Culasse;
use super::cc;

const L: Layout = Layout::Inline;
const N: u8 = 4;

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

pub fn i4_1_0_ss_d() -> Bloc { make(cc::_1_0::CC, 66.4, 72.2, "SS_d", 17.5, 1.5) }
pub fn i4_1_0_ts_d() -> Bloc { make(cc::_1_0::CC, 66.4, 72.2, "TS_d", 16.5, 1.9) }
pub fn i4_1_0_vg_d() -> Bloc { make(cc::_1_0::CC, 66.4, 72.2, "VG_d", 16.0, 2.2) }
pub fn i4_1_1_ss_d() -> Bloc { make(cc::_1_1::CC, 68.6, 74.6, "SS_d", 17.5, 1.5) }
pub fn i4_1_1_ts_d() -> Bloc { make(cc::_1_1::CC, 68.6, 74.6, "TS_d", 16.5, 1.9) }
pub fn i4_1_1_vg_d() -> Bloc { make(cc::_1_1::CC, 68.6, 74.6, "VG_d", 16.0, 2.2) }
pub fn i4_1_2_ss_d() -> Bloc { make(cc::_1_2::CC, 70.6, 76.7, "SS_d", 17.5, 1.5) }
pub fn i4_1_2_ts_d() -> Bloc { make(cc::_1_2::CC, 70.6, 76.7, "TS_d", 16.5, 1.9) }
pub fn i4_1_2_vg_d() -> Bloc { make(cc::_1_2::CC, 70.6, 76.7, "VG_d", 16.0, 2.2) }
pub fn i4_1_3_ss_d() -> Bloc { make(cc::_1_3::CC, 72.5, 78.8, "SS_d", 17.5, 1.5) }
pub fn i4_1_3_ts_d() -> Bloc { make(cc::_1_3::CC, 72.5, 78.8, "TS_d", 16.5, 1.9) }
pub fn i4_1_3_vg_d() -> Bloc { make(cc::_1_3::CC, 72.5, 78.8, "VG_d", 16.0, 2.2) }
pub fn i4_1_4_ss_d() -> Bloc { make(cc::_1_4::CC, 74.3, 80.8, "SS_d", 17.5, 1.5) }
pub fn i4_1_4_ts_d() -> Bloc { make(cc::_1_4::CC, 74.3, 80.8, "TS_d", 16.5, 1.9) }
pub fn i4_1_4_vg_d() -> Bloc { make(cc::_1_4::CC, 74.3, 80.8, "VG_d", 16.0, 2.2) }
