use crate::powertrain::transmissions::assemblies::GearboxFailureMode;
use sciforge_hub::prelude::physics_scalar;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CvtKind {
    Belt,
    Toroidal,
}

#[derive(Debug, Clone, Copy)]
pub struct CvtOperatingPoint {
    pub ratio: f64,
    pub rpm_in: f64,
    pub rpm_out: f64,
    pub torque_in_nm: f64,
    pub torque_out_nm: f64,
    pub clamp_force_n: f64,
    pub contact_pressure_pa: f64,
    pub traction_coeff: f64,
    pub slip_fraction: f64,
    pub slip_loss_kw: f64,
    pub bearing_loss_kw: f64,
    pub effective_eff: f64,
    pub power_in_kw: f64,
    pub power_out_kw: f64,
    pub oil_temp_k: f64,
}

#[derive(Debug, Clone)]
pub struct Cvt {
    pub kind: CvtKind,
    pub ratio_min: f64,
    pub ratio_max: f64,
    pub efficiency: f64,
    pub inertia_kgm2: f64,
    pub max_input_torque_nm: f64,
    pub primary_radius_max_m: f64,
    pub secondary_radius_max_m: f64,
    pub face_width_m: f64,
    pub belt_or_roller_radius_m: f64,
    pub contact_e_pa: f64,
    pub allowable_pressure_pa: f64,
    pub max_traction_coeff: f64,
    pub clamp_safety_factor: f64,
    pub hours_run: f64,
    pub belt_cycles: f64,
    pub oil_temp_k_runtime: f64,
    pub oil_contamination: f64,
    pub wear_factor: f64,
    pub failures: Vec<GearboxFailureMode>,
}

impl Cvt {
    pub fn belt(ratio_min: f64, ratio_max: f64) -> Self {
        let r_primary_max = 0.060_f64;
        let span = (ratio_max / ratio_min).sqrt();
        let r_secondary_max = r_primary_max * span;
        let face_m = 0.030_f64;
        let belt_radius_m = 0.0009;
        let belt_e_pa = 200.0e9;
        let allowable_pressure_pa = 1300.0e6 / 1.6;
        let max_traction = 0.10;
        let clamp_safety = 1.4;
        let rho = sciforge_core::moleculars::solids::irons::steels::STEEL_4340.density_kg_m3;
        let mass_p = std::f64::consts::PI * r_primary_max.powi(2) * face_m * rho;
        let mass_s = std::f64::consts::PI * r_secondary_max.powi(2) * face_m * rho;
        let inertia_kgm2 = 0.5 * mass_p * r_primary_max.powi(2) + 0.5 * mass_s * r_secondary_max.powi(2);
        let allowable_force_n = physics_scalar(
            "hertz_contact_pressure",
            &["force", "r1", "r2", "e_star"],
            &[1.0, belt_radius_m, r_primary_max, belt_e_pa],
        )
            .map(|p_unit| (allowable_pressure_pa / p_unit).powi(3))
            .unwrap_or(15000.0);
        let max_input_torque_nm = allowable_force_n * r_primary_max * max_traction * 0.85 / clamp_safety;
        Self {
            kind: CvtKind::Belt,
            ratio_min,
            ratio_max,
            efficiency: 0.88,
            inertia_kgm2,
            max_input_torque_nm,
            primary_radius_max_m: r_primary_max,
            secondary_radius_max_m: r_secondary_max,
            face_width_m: face_m,
            belt_or_roller_radius_m: belt_radius_m,
            contact_e_pa: belt_e_pa,
            allowable_pressure_pa,
            max_traction_coeff: max_traction,
            clamp_safety_factor: clamp_safety,
            hours_run: 0.0,
            belt_cycles: 0.0,
            oil_temp_k_runtime: 313.0,
            oil_contamination: 0.0,
            wear_factor: 0.0,
            failures: Vec::new(),
        }
    }

    pub fn toroidal(ratio_min: f64, ratio_max: f64) -> Self {
        let r_primary_max = 0.072_f64;
        let span = (ratio_max / ratio_min).sqrt();
        let r_secondary_max = r_primary_max * span;
        let face_m = 0.040_f64;
        let r_roller = 0.012;
        let e_pa = 205.0e9;
        let allowable_pressure_pa = 3000.0e6 / 1.4;
        let max_traction = 0.085;
        let clamp_safety = 1.3;
        let rho = sciforge_core::moleculars::solids::irons::steels::STEEL_4340.density_kg_m3;
        let mass_p = std::f64::consts::PI * r_primary_max.powi(2) * face_m * rho;
        let mass_s = std::f64::consts::PI * r_secondary_max.powi(2) * face_m * rho;
        let inertia_kgm2 = 0.5 * mass_p * r_primary_max.powi(2) + 0.5 * mass_s * r_secondary_max.powi(2);
        let allowable_force_n = physics_scalar(
            "hertz_contact_pressure",
            &["force", "r1", "r2", "e_star"],
            &[1.0, r_roller, r_primary_max, e_pa],
        )
            .map(|p_unit| (allowable_pressure_pa / p_unit).powi(3))
            .unwrap_or(28000.0);
        let max_input_torque_nm = allowable_force_n * r_primary_max * max_traction * 0.92 / clamp_safety;
        Self {
            kind: CvtKind::Toroidal,
            ratio_min,
            ratio_max,
            efficiency: 0.92,
            inertia_kgm2,
            max_input_torque_nm,
            primary_radius_max_m: r_primary_max,
            secondary_radius_max_m: r_secondary_max,
            face_width_m: face_m,
            belt_or_roller_radius_m: r_roller,
            contact_e_pa: e_pa,
            allowable_pressure_pa,
            max_traction_coeff: max_traction,
            clamp_safety_factor: clamp_safety,
            hours_run: 0.0,
            belt_cycles: 0.0,
            oil_temp_k_runtime: 313.0,
            oil_contamination: 0.0,
            wear_factor: 0.0,
            failures: Vec::new(),
        }
    }

    pub fn clamp(&self, ratio: f64) -> f64 {
        ratio.clamp(self.ratio_min, self.ratio_max)
    }

    fn working_radii(&self, ratio: f64) -> (f64, f64) {
        let r = self.clamp(ratio);
        let r_p = self.primary_radius_max_m / r.sqrt();
        let r_s = r_p * r;
        (r_p, r_s)
    }

    fn oil_viscosity_pas(oil_temp_k: f64) -> f64 {
        let t_c = oil_temp_k - 273.15;
        (0.30 * (-0.025 * (t_c - 40.0)).exp()).max(0.005)
    }

    pub fn operating_point(
        &self,
        ratio: f64,
        rpm_in: f64,
        torque_in_nm: f64,
        oil_temp_k: f64,
    ) -> CvtOperatingPoint {
        let r = self.clamp(ratio);
        let (r_p, r_s) = self.working_radii(r);
        let rpm_out = rpm_in / r;
        let omega_in = rpm_in * std::f64::consts::PI / 30.0;
        let force_required_n = torque_in_nm.abs() / (r_p * self.max_traction_coeff).max(1.0e-6);
        let clamp_force_n = force_required_n * self.clamp_safety_factor;
        let contact_pressure_pa = physics_scalar(
            "hertz_contact_pressure",
            &["force", "r1", "r2", "e_star"],
            &[clamp_force_n, self.belt_or_roller_radius_m, r_p, self.contact_e_pa],
        )
            .unwrap_or((clamp_force_n / (self.belt_or_roller_radius_m * r_p)).sqrt() * (self.contact_e_pa / std::f64::consts::PI).sqrt());
        let utilization = (contact_pressure_pa / self.allowable_pressure_pa).clamp(0.0, 1.0);
        let traction_coeff = self.max_traction_coeff * utilization.sqrt();
        let demanded_traction = torque_in_nm.abs() / (clamp_force_n * r_p).max(1.0e-6);
        let slip_fraction = if demanded_traction <= traction_coeff {
            (demanded_traction / traction_coeff.max(1.0e-6)) * 0.02
        } else {
            (0.02 + (demanded_traction - traction_coeff) * 5.0).min(1.0)
        };
        let mu_oil = Self::oil_viscosity_pas(oil_temp_k);
        let bearing_torque_nm = mu_oil * 12.0 * (r_p + r_s) * 0.5;
        let bearing_w = bearing_torque_nm * omega_in;
        let power_in_w = torque_in_nm.abs() * omega_in;
        let slip_loss_w = power_in_w * slip_fraction;
        let mech_eff = self.efficiency;
        let p_after_mech = power_in_w * mech_eff;
        let p_out = (p_after_mech - slip_loss_w - bearing_w).max(0.0);
        let effective_eff = if power_in_w > 1.0 { p_out / power_in_w } else { mech_eff };
        let torque_out_nm = if rpm_out.abs() > 1.0 {
            p_out / (rpm_out.abs() * std::f64::consts::PI / 30.0) * rpm_out.signum()
        } else {
            torque_in_nm * r * mech_eff
        };
        CvtOperatingPoint {
            ratio: r,
            rpm_in,
            rpm_out,
            torque_in_nm,
            torque_out_nm,
            clamp_force_n,
            contact_pressure_pa,
            traction_coeff,
            slip_fraction,
            slip_loss_kw: slip_loss_w / 1000.0,
            bearing_loss_kw: bearing_w / 1000.0,
            effective_eff,
            power_in_kw: power_in_w / 1000.0,
            power_out_kw: p_out / 1000.0,
            oil_temp_k,
        }
    }

    pub fn tick(
        &mut self,
        ratio: f64,
        rpm_in: f64,
        torque_in_nm: f64,
        ambient_k: f64,
        dt_s: f64,
    ) {
        if self.failures.iter().any(|f| f.is_terminal()) {
            return;
        }
        let op = self.operating_point(ratio, rpm_in, torque_in_nm, self.oil_temp_k_runtime);
        let cycles = (rpm_in.abs() / 60.0) * dt_s;
        self.belt_cycles += cycles;
        self.hours_run += dt_s / 3600.0;
        let p_loss_w = (op.slip_loss_kw + op.bearing_loss_kw + op.power_in_kw * (1.0 - self.efficiency)) * 1000.0;
        let oil_mass_kg = 5.0;
        let cp_oil = 2100.0;
        let h_cool = 30.0;
        let area_case = 0.6;
        let q_out = h_cool * area_case * (self.oil_temp_k_runtime - ambient_k);
        let d_temp = (p_loss_w - q_out) / (oil_mass_kg * cp_oil) * dt_s;
        self.oil_temp_k_runtime = (self.oil_temp_k_runtime + d_temp).max(ambient_k);
        self.oil_contamination += (op.power_in_kw / 100.0).max(0.0) * dt_s / 3600.0 * 1.5e-4;
        let overload = (torque_in_nm.abs() / self.max_input_torque_nm - 1.0).max(0.0);
        let overheat = ((self.oil_temp_k_runtime - 393.0) / 30.0).max(0.0);
        let pitting_rate = (op.contact_pressure_pa / self.allowable_pressure_pa).powi(9);
        self.wear_factor += (pitting_rate * cycles / 1.0e9 + overload * 0.02 + overheat * 0.0015 + self.oil_contamination * 0.001 + op.slip_fraction * 0.001) * dt_s;
        if torque_in_nm.abs() > self.max_input_torque_nm * 1.6 {
            self.failures.push(GearboxFailureMode::OverTorqueDamage);
            return;
        }
        if op.slip_fraction > 0.10 {
            match self.kind {
                CvtKind::Belt => {
                    if !self.failures.contains(&GearboxFailureMode::BeltSlipExcessive) {
                        self.failures.push(GearboxFailureMode::BeltSlipExcessive);
                    }
                }
                CvtKind::Toroidal => {
                    if !self.failures.contains(&GearboxFailureMode::RollerSkid) {
                        self.failures.push(GearboxFailureMode::RollerSkid);
                    }
                }
            }
        }
        if self.oil_temp_k_runtime > 423.0 && !self.failures.contains(&GearboxFailureMode::OilOverheat) {
            self.failures.push(GearboxFailureMode::OilOverheat);
        }
        if self.oil_temp_k_runtime > 463.0 && !self.failures.contains(&GearboxFailureMode::BearingSeized) {
            self.failures.push(GearboxFailureMode::BearingSeized);
        }
        if self.wear_factor > 1.0 && !self.failures.contains(&GearboxFailureMode::GearToothPittingFatigue) {
            self.failures.push(GearboxFailureMode::GearToothPittingFatigue);
        }
    }

    pub fn current_efficiency(&self) -> f64 {
        if self.failures.iter().any(|f| f.is_terminal()) {
            return 0.0;
        }
        let loss: f64 = self.failures.iter().map(|f| f.power_loss_fraction()).sum();
        (self.efficiency * (1.0 - loss).max(0.0)).max(0.0)
    }

    pub fn is_running(&self) -> bool {
        !self.failures.iter().any(|f| f.is_terminal())
    }
}
