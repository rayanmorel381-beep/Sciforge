use crate::powertrain::transmissions::differentials::{
    Differential, LimitedSlipDifferential, OpenDifferential, TorqueVectoring,
};
use crate::powertrain::transmissions::drivingwheels::{
    DriveLayout,
    alls::Awd,
    front::Fwd,
    rear::Rwd,
};

use crate::powertrain::transmissions::drivelines::halfshafts::{self, CvJointKind, Halfshaft, HalfshaftKind};
use crate::powertrain::transmissions::drivelines::hubs::{self, HubKind, WheelBoltPattern, WheelHub};
use crate::powertrain::transmissions::drivelines::propshafts::{self, Propshaft, PropshaftKind};
use crate::powertrain::transmissions::drivelines::transfer_cases::{self, LowRange, TransferCase, TransferCaseKind};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum EngineOrientation {
    Transverse,
    Longitudinal,
}

#[derive(Debug, Clone)]
pub struct DrivelineSpec {
    pub engine_orientation: EngineOrientation,
    pub motorpropulsor_position: f64,
    pub layout: DriveLayout,
    pub input_torque_nm: f64,
    pub max_input_speed_rpm: f64,
    pub front_track_mm: f64,
    pub rear_track_mm: f64,
    pub wheelbase_mm: f64,
    pub gearbox_to_rear_axle_mm: f64,
    pub bolt_pattern: WheelBoltPattern,
    pub max_axial_load_n: f64,
    pub max_radial_load_n: f64,
    pub front_drive_pinion_teeth: u16,
    pub front_ring_gear_teeth: u16,
    pub front_spider_gear_count: u8,
    pub front_spider_gear_teeth: u16,
    pub front_side_gear_teeth: u16,
    pub rear_drive_pinion_teeth: u16,
    pub rear_ring_gear_teeth: u16,
    pub rear_spider_gear_count: u8,
    pub rear_spider_gear_teeth: u16,
    pub rear_side_gear_teeth: u16,
    pub center_drive_pinion_teeth: u16,
    pub center_ring_gear_teeth: u16,
    pub center_spider_gear_count: u8,
    pub center_spider_gear_teeth: u16,
    pub center_side_gear_teeth: u16,
}

#[derive(Debug, Clone)]
pub struct Driveline {
    pub layout: DriveLayout,
    pub front_differential: Option<Differential>,
    pub rear_differential: Option<Differential>,
    pub transfer_case: Option<TransferCase>,
    pub front_propshaft: Option<Propshaft>,
    pub rear_propshaft: Option<Propshaft>,
    pub front_halfshafts: Option<(Halfshaft, Halfshaft)>,
    pub rear_halfshafts: Option<(Halfshaft, Halfshaft)>,
    pub front_hubs: (WheelHub, WheelHub),
    pub rear_hubs: (WheelHub, WheelHub),
    pub differential_count: u8,
    pub total_mass_kg: f64,
}

fn ring_gear_diameter_mm(input_torque_nm: f64, final_drive_ratio: f64) -> f64 {
    let axle_torque = input_torque_nm * final_drive_ratio;
    (140.0 + (axle_torque / 100.0).sqrt() * 12.0).clamp(150.0, 320.0)
}

fn ratio_from_teeth(drive_pinion_teeth: u16, ring_gear_teeth: u16) -> f64 {
    ring_gear_teeth as f64 / drive_pinion_teeth as f64
}

fn build_open_diff(
    input_torque_nm: f64,
    drive_pinion_teeth: u16,
    ring_gear_teeth: u16,
    spider_gear_count: u8,
    spider_gear_teeth: u16,
    side_gear_teeth: u16,
) -> Differential {
    let final_drive_ratio = ratio_from_teeth(drive_pinion_teeth, ring_gear_teeth);
    Differential::Open(OpenDifferential::new(
        ring_gear_diameter_mm(input_torque_nm, final_drive_ratio),
        final_drive_ratio,
        drive_pinion_teeth,
        ring_gear_teeth,
        spider_gear_count,
        spider_gear_teeth,
        side_gear_teeth,
    ))
}

fn build_lsd_diff(
    input_torque_nm: f64,
    drive_pinion_teeth: u16,
    ring_gear_teeth: u16,
    spider_gear_count: u8,
    spider_gear_teeth: u16,
    side_gear_teeth: u16,
) -> Differential {
    let final_drive_ratio = ratio_from_teeth(drive_pinion_teeth, ring_gear_teeth);
    Differential::Lsd(LimitedSlipDifferential::mechanical(
        ring_gear_diameter_mm(input_torque_nm, final_drive_ratio),
        final_drive_ratio,
        drive_pinion_teeth,
        ring_gear_teeth,
        spider_gear_count,
        spider_gear_teeth,
        side_gear_teeth,
    ))
}

fn build_tv_diff(
    input_torque_nm: f64,
    drive_pinion_teeth: u16,
    ring_gear_teeth: u16,
    spider_gear_count: u8,
    spider_gear_teeth: u16,
    side_gear_teeth: u16,
) -> Differential {
    let final_drive_ratio = ratio_from_teeth(drive_pinion_teeth, ring_gear_teeth);
    Differential::TorqueVectoring(TorqueVectoring::electric(
        ring_gear_diameter_mm(input_torque_nm, final_drive_ratio),
        final_drive_ratio,
        drive_pinion_teeth,
        ring_gear_teeth,
        spider_gear_count,
        spider_gear_teeth,
        side_gear_teeth,
    ))
}

fn build_axle_pair(
    track_mm: f64,
    axle_torque_nm: f64,
    bolt_pattern: WheelBoltPattern,
    max_axial_load_n: f64,
    max_radial_load_n: f64,
) -> ((Halfshaft, Halfshaft), (WheelHub, WheelHub)) {
    build_axle_pair_kind(
        track_mm,
        axle_torque_nm,
        HalfshaftKind::SolidSteel,
        HubKind::IndependentDriven,
        bolt_pattern,
        max_axial_load_n,
        max_radial_load_n,
    )
}

fn build_axle_pair_kind(
    track_mm: f64,
    axle_torque_nm: f64,
    halfshaft_kind: HalfshaftKind,
    hub_kind: HubKind,
    bolt_pattern: WheelBoltPattern,
    max_axial_load_n: f64,
    max_radial_load_n: f64,
) -> ((Halfshaft, Halfshaft), (WheelHub, WheelHub)) {
    let half_torque = axle_torque_nm / 2.0;
    let length_mm = (track_mm / 2.0 - 60.0).max(150.0);
    let left = halfshafts::assemble(
        halfshaft_kind,
        length_mm,
        half_torque,
        CvJointKind::Tripod,
        CvJointKind::BallRzeppa,
    );
    let right = halfshafts::assemble(
        halfshaft_kind,
        length_mm,
        half_torque,
        CvJointKind::Tripod,
        CvJointKind::BallRzeppa,
    );
    let hub_l = hubs::assemble(
        hub_kind,
        bolt_pattern,
        max_axial_load_n,
        max_radial_load_n,
        half_torque,
    );
    let hub_r = hubs::assemble(
        hub_kind,
        bolt_pattern,
        max_axial_load_n,
        max_radial_load_n,
        half_torque,
    );
    ((left, right), (hub_l, hub_r))
}

fn build_idle_hubs(
    bolt_pattern: WheelBoltPattern,
    max_axial_load_n: f64,
    max_radial_load_n: f64,
) -> (WheelHub, WheelHub) {
    let l = hubs::assemble(
        HubKind::UnitBearing,
        bolt_pattern,
        max_axial_load_n,
        max_radial_load_n,
        0.0,
    );
    let r = hubs::assemble(
        HubKind::UnitBearing,
        bolt_pattern,
        max_axial_load_n,
        max_radial_load_n,
        0.0,
    );
    (l, r)
}

fn pair_mass(p: &(Halfshaft, Halfshaft)) -> f64 {
    p.0.mass_kg + p.1.mass_kg
}

fn hub_pair_mass(p: &(WheelHub, WheelHub)) -> f64 {
    p.0.mass_kg + p.1.mass_kg
}

fn diff_mass_kg(d: &Differential) -> f64 {
    let ring_mm = match d {
        Differential::Open(o) => o.ring_gear_diameter_mm,
        Differential::Lsd(l) => l.ring_gear_diameter_mm,
        Differential::Locked(_) => 220.0,
        Differential::TorqueVectoring(_) => 240.0,
    };
    let base = (ring_mm / 1000.0).powi(2) * 250.0;
    let extra = match d {
        Differential::Open(_) => 0.0,
        Differential::Lsd(_) => 6.0,
        Differential::Locked(_) => 4.0,
        Differential::TorqueVectoring(_) => 14.0,
    };
    base + extra
}

pub fn assemble(spec: &DrivelineSpec) -> Driveline {
    match &spec.layout {
        DriveLayout::Fwd(fwd) => assemble_fwd(spec, fwd),
        DriveLayout::Rwd(rwd) => assemble_rwd(spec, rwd),
        DriveLayout::Awd(awd) => {
            let (kind, front_split_pct, has_center_diff) = match awd {
                Awd::FullTime(ft) => (
                    TransferCaseKind::FullTimeChain,
                    ft.front_torque_split_pct as f64,
                    ft.centre_differential,
                ),
                Awd::PartTime(pt) => (
                    TransferCaseKind::PartTimeChain,
                    pt.front_torque_split_pct as f64,
                    false,
                ),
                Awd::OnDemand(od) => (
                    TransferCaseKind::Active,
                    od.max_front_torque_split_pct as f64,
                    false,
                ),
            };
            let center_diff = if has_center_diff {
                Some(build_open_diff(
                    spec.input_torque_nm,
                    spec.center_drive_pinion_teeth,
                    spec.center_ring_gear_teeth,
                    spec.center_spider_gear_count,
                    spec.center_spider_gear_teeth,
                    spec.center_side_gear_teeth,
                ))
            } else {
                None
            };
            assemble_awd(spec, kind, front_split_pct, center_diff)
        }
    }
}

fn assemble_fwd(spec: &DrivelineSpec, fwd: &Fwd) -> Driveline {
    let limited_slip = match fwd {
        Fwd::Transverse(t) => t.limited_slip,
        Fwd::Longitudinal(l) => l.limited_slip,
    };
    let front_ratio = ratio_from_teeth(spec.front_drive_pinion_teeth, spec.front_ring_gear_teeth);
    let front_diff = if limited_slip {
        build_lsd_diff(
            spec.input_torque_nm,
            spec.front_drive_pinion_teeth,
            spec.front_ring_gear_teeth,
            spec.front_spider_gear_count,
            spec.front_spider_gear_teeth,
            spec.front_side_gear_teeth,
        )
    } else {
        build_open_diff(
            spec.input_torque_nm,
            spec.front_drive_pinion_teeth,
            spec.front_ring_gear_teeth,
            spec.front_spider_gear_count,
            spec.front_spider_gear_teeth,
            spec.front_side_gear_teeth,
        )
    };
    let axle_torque = spec.input_torque_nm * front_ratio;
    let (front_hs, front_driven_hubs) = build_axle_pair(
        spec.front_track_mm,
        axle_torque,
        spec.bolt_pattern,
        spec.max_axial_load_n,
        spec.max_radial_load_n,
    );
    let rear_hubs = build_idle_hubs(spec.bolt_pattern, spec.max_axial_load_n, spec.max_radial_load_n);
    let total_mass_kg = diff_mass_kg(&front_diff)
        + pair_mass(&front_hs)
        + hub_pair_mass(&front_driven_hubs)
        + hub_pair_mass(&rear_hubs);
    Driveline {
        layout: spec.layout.clone(),
        front_differential: Some(front_diff),
        rear_differential: None,
        transfer_case: None,
        front_propshaft: None,
        rear_propshaft: None,
        front_halfshafts: Some(front_hs),
        rear_halfshafts: None,
        front_hubs: front_driven_hubs,
        rear_hubs,
        differential_count: 1,
        total_mass_kg: (total_mass_kg * 100.0).round() / 100.0,
    }
}

fn assemble_rwd(spec: &DrivelineSpec, rwd: &Rwd) -> Driveline {
    let rear_pinion = spec.rear_drive_pinion_teeth;
    let rear_ring = spec.rear_ring_gear_teeth;
    let rear_spider_count = spec.rear_spider_gear_count;
    let rear_spider = spec.rear_spider_gear_teeth;
    let rear_side = spec.rear_side_gear_teeth;
    let rear_ratio = ratio_from_teeth(rear_pinion, rear_ring);
    let rear_diff = match rwd {
        Rwd::LiveAxle(_) => build_open_diff(spec.input_torque_nm, rear_pinion, rear_ring, rear_spider_count, rear_spider, rear_side),
        Rwd::Independent(i) => {
            if i.torque_vectoring {
                build_tv_diff(spec.input_torque_nm, rear_pinion, rear_ring, rear_spider_count, rear_spider, rear_side)
            } else if i.limited_slip {
                build_lsd_diff(spec.input_torque_nm, rear_pinion, rear_ring, rear_spider_count, rear_spider, rear_side)
            } else {
                build_open_diff(spec.input_torque_nm, rear_pinion, rear_ring, rear_spider_count, rear_spider, rear_side)
            }
        }
        Rwd::DeDion(_) => build_open_diff(spec.input_torque_nm, rear_pinion, rear_ring, rear_spider_count, rear_spider, rear_side),
    };
    let halfshaft_kind = match rwd {
        Rwd::LiveAxle(_) => HalfshaftKind::SolidSteel,
        Rwd::Independent(_) | Rwd::DeDion(_) => HalfshaftKind::SolidSteel,
    };
    let axle_torque = spec.input_torque_nm * rear_ratio;
    let (rear_hs, rear_driven_hubs) = build_axle_pair_kind(
        spec.rear_track_mm,
        axle_torque,
        halfshaft_kind,
        match rwd {
            Rwd::LiveAxle(_) => HubKind::LiveAxle,
            _ => HubKind::IndependentDriven,
        },
        spec.bolt_pattern,
        spec.max_axial_load_n,
        spec.max_radial_load_n,
    );
    let propshaft_kind = if spec.gearbox_to_rear_axle_mm > 1500.0 {
        PropshaftKind::TwoPieceWithCenterBearing
    } else {
        PropshaftKind::SinglePiece
    };
    let propshaft = propshafts::assemble(
        propshaft_kind,
        spec.gearbox_to_rear_axle_mm,
        spec.input_torque_nm,
        spec.max_input_speed_rpm,
    );
    let front_hubs = build_idle_hubs(spec.bolt_pattern, spec.max_axial_load_n, spec.max_radial_load_n);
    let total_mass_kg = diff_mass_kg(&rear_diff)
        + pair_mass(&rear_hs)
        + hub_pair_mass(&rear_driven_hubs)
        + hub_pair_mass(&front_hubs)
        + propshaft.mass_kg;
    Driveline {
        layout: spec.layout.clone(),
        front_differential: None,
        rear_differential: Some(rear_diff),
        transfer_case: None,
        front_propshaft: None,
        rear_propshaft: Some(propshaft),
        front_halfshafts: None,
        rear_halfshafts: Some(rear_hs),
        front_hubs,
        rear_hubs: rear_driven_hubs,
        differential_count: 1,
        total_mass_kg: (total_mass_kg * 100.0).round() / 100.0,
    }
}

fn assemble_awd(
    spec: &DrivelineSpec,
    transfer_kind: TransferCaseKind,
    front_split_pct: f64,
    center_differential: Option<Differential>,
) -> Driveline {
    let rear_split_pct = 100.0 - front_split_pct;
    let front_torque_in = spec.input_torque_nm * front_split_pct / 100.0;
    let rear_torque_in = spec.input_torque_nm * rear_split_pct / 100.0;
    let front_ratio = ratio_from_teeth(spec.front_drive_pinion_teeth, spec.front_ring_gear_teeth);
    let rear_ratio = ratio_from_teeth(spec.rear_drive_pinion_teeth, spec.rear_ring_gear_teeth);
    let front_axle_torque = front_torque_in * front_ratio;
    let rear_axle_torque = rear_torque_in * rear_ratio;

    let has_center = center_differential.is_some();
    let transfer_case = transfer_cases::assemble(
        transfer_kind,
        LowRange::None,
        front_split_pct,
        center_differential,
        spec.input_torque_nm,
    );

    let front_diff = build_open_diff(
        front_torque_in.max(1.0),
        spec.front_drive_pinion_teeth,
        spec.front_ring_gear_teeth,
        spec.front_spider_gear_count,
        spec.front_spider_gear_teeth,
        spec.front_side_gear_teeth,
    );
    let rear_diff = build_open_diff(
        rear_torque_in.max(1.0),
        spec.rear_drive_pinion_teeth,
        spec.rear_ring_gear_teeth,
        spec.rear_spider_gear_count,
        spec.rear_spider_gear_teeth,
        spec.rear_side_gear_teeth,
    );

    let (front_hs, front_hubs) = build_axle_pair(
        spec.front_track_mm,
        front_axle_torque,
        spec.bolt_pattern,
        spec.max_axial_load_n,
        spec.max_radial_load_n,
    );
    let (rear_hs, rear_hubs) = build_axle_pair(
        spec.rear_track_mm,
        rear_axle_torque,
        spec.bolt_pattern,
        spec.max_axial_load_n,
        spec.max_radial_load_n,
    );

    let front_prop_len_mm = (spec.wheelbase_mm * 0.35).max(400.0);
    let rear_prop_len_mm = spec.gearbox_to_rear_axle_mm.max(spec.wheelbase_mm * 0.55);
    let front_propshaft = propshafts::assemble(
        PropshaftKind::SinglePiece,
        front_prop_len_mm,
        front_torque_in.max(1.0),
        spec.max_input_speed_rpm,
    );
    let rear_propshaft_kind = if rear_prop_len_mm > 1500.0 {
        PropshaftKind::TwoPieceWithCenterBearing
    } else {
        PropshaftKind::SinglePiece
    };
    let rear_propshaft = propshafts::assemble(
        rear_propshaft_kind,
        rear_prop_len_mm,
        rear_torque_in.max(1.0),
        spec.max_input_speed_rpm,
    );

    let mut differential_count: u8 = 2;
    if has_center {
        differential_count += 1;
    }

    let total_mass_kg = transfer_case.mass_kg
        + diff_mass_kg(&front_diff)
        + diff_mass_kg(&rear_diff)
        + front_propshaft.mass_kg
        + rear_propshaft.mass_kg
        + pair_mass(&front_hs)
        + pair_mass(&rear_hs)
        + hub_pair_mass(&front_hubs)
        + hub_pair_mass(&rear_hubs);

    Driveline {
        layout: spec.layout.clone(),
        front_differential: Some(front_diff),
        rear_differential: Some(rear_diff),
        transfer_case: Some(transfer_case),
        front_propshaft: Some(front_propshaft),
        rear_propshaft: Some(rear_propshaft),
        front_halfshafts: Some(front_hs),
        rear_halfshafts: Some(rear_hs),
        front_hubs,
        rear_hubs,
        differential_count,
        total_mass_kg: (total_mass_kg * 100.0).round() / 100.0,
    }
}
