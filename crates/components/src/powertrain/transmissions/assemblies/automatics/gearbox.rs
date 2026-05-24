use crate::powertrain::transmissions::assemblies::{
    GearboxFailureMode, GearboxOperatingPoint,
};
use sciforge_hub::prelude::physics_scalar;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AutomaticKind {
    Dct,
    TorqueConverter,
}

#[derive(Debug, Clone, Copy)]
pub struct TorqueConverterState {
    pub speed_ratio: f64,
    pub torque_ratio: f64,
    pub efficiency: f64,
    pub slip_kw: f64,
    pub locked_up: bool,
    pub k_factor: f64,
}

#[derive(Debug, Clone)]
pub struct AutomaticGearbox {
    pub kind: AutomaticKind,
    pub forward_teeth: Vec<(u32, u32)>,
    pub reverse_teeth: (u32, u32),
    pub shift_time_ms: u32,
    pub efficiency: f64,
    pub inertia_kgm2: f64,
    pub max_input_torque_nm: f64,
    pub module_mm: f64,
    pub face_width_mm: f64,
    pub endurance_pa: f64,
    pub mesh_eff_per_pair: f64,
    pub stall_torque_ratio: f64,
    pub k_factor_design: f64,
    pub lockup_speed_ratio: f64,
    pub clutch_friction_radius_m: f64,
    pub clutch_normal_force_n: f64,
    pub clutch_friction_coeff: f64,
    pub hours_run: f64,
    pub mesh_cycles_per_gear: Vec<f64>,
    pub shift_count: u64,
    pub clutch_energy_consumed_j: f64,
    pub oil_temp_k_runtime: f64,
    pub oil_contamination: f64,
    pub wear_factor: f64,
    pub failures: Vec<GearboxFailureMode>,
}

impl AutomaticGearbox {
    pub fn dct(forward_teeth: Vec<(u32, u32)>, reverse_teeth: (u32, u32)) -> Self {
        let module_mm = 2.25;
        let face_width_mm = 24.0;
        let endurance_pa = 720.0e6;
        let safety = 1.2;
        let mesh_eff = 0.97;
        let (efficiency, inertia_kgm2, max_input_torque_nm) =
            compute_metrics(&forward_teeth, module_mm, face_width_mm, endurance_pa, safety, mesh_eff);
        let n = forward_teeth.len();
        let r_clutch = 0.110;
        let mu = 0.13;
        let n_force = max_input_torque_nm / (mu * r_clutch * 0.65);
        Self {
            kind: AutomaticKind::Dct,
            forward_teeth,
            reverse_teeth,
            shift_time_ms: 8,
            efficiency,
            inertia_kgm2,
            max_input_torque_nm,
            module_mm,
            face_width_mm,
            endurance_pa,
            mesh_eff_per_pair: mesh_eff,
            stall_torque_ratio: 1.0,
            k_factor_design: 0.0,
            lockup_speed_ratio: 1.0,
            clutch_friction_radius_m: r_clutch,
            clutch_normal_force_n: n_force,
            clutch_friction_coeff: mu,
            hours_run: 0.0,
            mesh_cycles_per_gear: vec![0.0; n],
            shift_count: 0,
            clutch_energy_consumed_j: 0.0,
            oil_temp_k_runtime: 313.0,
            oil_contamination: 0.0,
            wear_factor: 0.0,
            failures: Vec::new(),
        }
    }

    pub fn torque_converter(forward_teeth: Vec<(u32, u32)>, reverse_teeth: (u32, u32)) -> Self {
        let module_mm = 2.5;
        let face_width_mm = 26.0;
        let endurance_pa = 480.0e6;
        let safety = 1.5;
        let mesh_eff = 0.985;
        let (mech_eff, inertia_kgm2, max_input_torque_nm) =
            compute_metrics(&forward_teeth, module_mm, face_width_mm, endurance_pa, safety, mesh_eff);
        let stall_tr = 2.2;
        let k_factor = 180.0;
        let lockup_sr = 0.92;
        let efficiency = mech_eff * lockup_sr;
        let n = forward_teeth.len();
        let r_lockup = 0.130;
        let mu = 0.11;
        let n_force = max_input_torque_nm / (mu * r_lockup * 0.6);
        Self {
            kind: AutomaticKind::TorqueConverter,
            forward_teeth,
            reverse_teeth,
            shift_time_ms: 180,
            efficiency,
            inertia_kgm2,
            max_input_torque_nm,
            module_mm,
            face_width_mm,
            endurance_pa,
            mesh_eff_per_pair: mesh_eff,
            stall_torque_ratio: stall_tr,
            k_factor_design: k_factor,
            lockup_speed_ratio: lockup_sr,
            clutch_friction_radius_m: r_lockup,
            clutch_normal_force_n: n_force,
            clutch_friction_coeff: mu,
            hours_run: 0.0,
            mesh_cycles_per_gear: vec![0.0; n],
            shift_count: 0,
            clutch_energy_consumed_j: 0.0,
            oil_temp_k_runtime: 313.0,
            oil_contamination: 0.0,
            wear_factor: 0.0,
            failures: Vec::new(),
        }
    }

    pub fn speeds(&self) -> usize {
        self.forward_teeth.len()
    }

    pub fn ratio(&self, gear: u8) -> f64 {
        let idx = (gear as usize).saturating_sub(1).min(self.forward_teeth.len() - 1);
        let (p, s) = self.forward_teeth[idx];
        s as f64 / p as f64
    }

    pub fn reverse_ratio(&self) -> f64 {
        let (p, s) = self.reverse_teeth;
        -(s as f64 / p as f64)
    }

    fn oil_density_kg_m3(oil_temp_k: f64) -> f64 {
        (880.0 - 0.6 * (oil_temp_k - 293.0)).max(700.0)
    }

    fn oil_viscosity_pas(oil_temp_k: f64) -> f64 {
        let t_c = oil_temp_k - 273.15;
        (0.30 * (-0.025 * (t_c - 40.0)).exp()).max(0.005)
    }

    pub fn torque_converter_state(
        &self,
        rpm_engine: f64,
        rpm_turbine: f64,
        forced_lockup: bool,
    ) -> TorqueConverterState {
        if self.kind != AutomaticKind::TorqueConverter {
            return TorqueConverterState {
                speed_ratio: 1.0,
                torque_ratio: 1.0,
                efficiency: 1.0,
                slip_kw: 0.0,
                locked_up: true,
                k_factor: 0.0,
            };
        }
        let sr = if rpm_engine.abs() > 1.0 { (rpm_turbine / rpm_engine).clamp(0.0, 1.0) } else { 0.0 };
        let coupling_sr = self.lockup_speed_ratio;
        let locked_up = forced_lockup || sr >= coupling_sr;
        let (tr, eta) = if locked_up {
            (1.0, 1.0)
        } else if sr >= coupling_sr - 0.05 {
            let trc = 1.0;
            (trc, sr)
        } else {
            let trc = self.stall_torque_ratio - (self.stall_torque_ratio - 1.0) * (sr / coupling_sr);
            (trc, sr * trc)
        };
        let omega_in = rpm_engine.abs() * std::f64::consts::PI / 30.0;
        let k = self.k_factor_design;
        let t_capacity = if k > 0.0 { (rpm_engine.abs() / k).powi(2) } else { 0.0 };
        let slip_w = if locked_up { 0.0 } else { t_capacity * omega_in * (1.0 - eta) };
        TorqueConverterState {
            speed_ratio: sr,
            torque_ratio: tr,
            efficiency: eta,
            slip_kw: slip_w / 1000.0,
            locked_up,
            k_factor: k,
        }
    }

    pub fn operating_point(
        &self,
        gear: u8,
        rpm_in: f64,
        torque_in_nm: f64,
        oil_temp_k: f64,
    ) -> GearboxOperatingPoint {
        let ratio = self.ratio(gear);
        let omega_in = rpm_in * std::f64::consts::PI / 30.0;
        let idx = (gear as usize).saturating_sub(1).min(self.forward_teeth.len() - 1);
        let (p_teeth, _) = self.forward_teeth[idx];
        let r_pinion_m = (p_teeth as f64 * self.module_mm) / 2000.0;
        let b_m = self.face_width_mm / 1000.0;
        let rho_oil = Self::oil_density_kg_m3(oil_temp_k);
        let mu_oil = Self::oil_viscosity_pas(oil_temp_k);
        let c_m = match self.kind {
            AutomaticKind::Dct => 0.04,
            AutomaticKind::TorqueConverter => 0.06,
        };
        let churn_w = c_m * 0.5 * rho_oil * omega_in.powi(3) * r_pinion_m.powi(5) * b_m;
        let bearing_torque_nm = mu_oil * 12.0 * r_pinion_m * 0.5;
        let bearing_w = bearing_torque_nm * omega_in;
        let power_in_w = torque_in_nm.abs() * omega_in;
        let mesh_eff = self.efficiency;
        let p_after_mesh = power_in_w * mesh_eff;
        let p_out = (p_after_mesh - churn_w - bearing_w).max(0.0);
        let rpm_out = rpm_in / ratio;
        let effective_eff = if power_in_w > 1.0 { p_out / power_in_w } else { mesh_eff };
        let torque_out_nm = if rpm_out.abs() > 1.0 {
            p_out / (rpm_out.abs() * std::f64::consts::PI / 30.0) * rpm_out.signum()
        } else {
            torque_in_nm * ratio * mesh_eff
        };
        let h_m = (self.module_mm * 2.25) / 1000.0;
        let t_m = (self.module_mm * std::f64::consts::PI / 2.0) / 1000.0;
        let i_m4 = b_m * t_m.powi(3) / 12.0;
        let y_m = t_m / 2.0;
        let m_load = torque_in_nm.abs() * h_m / r_pinion_m;
        let hertz_pressure_pa = physics_scalar(
            "beam_bending_stress",
            &["m", "y", "i"],
            &[m_load, y_m, i_m4],
        )
            .unwrap_or(m_load * y_m / i_m4);
        GearboxOperatingPoint {
            gear,
            ratio,
            rpm_in,
            rpm_out,
            torque_in_nm,
            torque_out_nm,
            mesh_eff,
            churn_loss_kw: churn_w / 1000.0,
            bearing_loss_kw: bearing_w / 1000.0,
            effective_eff,
            power_in_kw: power_in_w / 1000.0,
            power_out_kw: p_out / 1000.0,
            hertz_pressure_pa,
            oil_temp_k,
        }
    }

    pub fn shift_clutch_energy_j(&self, from_gear: u8, to_gear: u8, rpm_in: f64) -> f64 {
        let r_from = self.ratio(from_gear);
        let r_to = self.ratio(to_gear);
        let omega_in = rpm_in * std::f64::consts::PI / 30.0;
        let omega_out = omega_in / r_from;
        let omega_in_after = omega_out * r_to;
        let delta_omega = omega_in - omega_in_after;
        0.5 * self.inertia_kgm2 * delta_omega.powi(2)
    }

    pub fn clutch_thermal_capacity_j(&self) -> f64 {
        let cp_steel = 460.0;
        let m_clutch_kg = (self.max_input_torque_nm / 100.0).max(2.0);
        let dt_max = 200.0;
        m_clutch_kg * cp_steel * dt_max
    }

    pub fn shift(&mut self, from_gear: u8, to_gear: u8, rpm_in: f64) {
        if self.failures.iter().any(|f| f.is_terminal()) {
            return;
        }
        self.shift_count += 1;
        let e = self.shift_clutch_energy_j(from_gear, to_gear, rpm_in);
        self.clutch_energy_consumed_j += e;
        let cap = self.clutch_thermal_capacity_j();
        if e > cap && !self.failures.contains(&GearboxFailureMode::ClutchPackBurnt) {
            self.failures.push(GearboxFailureMode::ClutchPackBurnt);
        }
    }

    pub fn tick(
        &mut self,
        rpm_in: f64,
        torque_in_nm: f64,
        gear: u8,
        ambient_k: f64,
        rpm_turbine: Option<f64>,
        forced_lockup: bool,
        dt_s: f64,
    ) {
        if self.failures.iter().any(|f| f.is_terminal()) {
            return;
        }
        let op = self.operating_point(gear, rpm_in, torque_in_nm, self.oil_temp_k_runtime);
        let idx = (gear as usize).saturating_sub(1).min(self.mesh_cycles_per_gear.len() - 1);
        let cycles = (rpm_in.abs() / 60.0) * dt_s;
        self.mesh_cycles_per_gear[idx] += cycles;
        self.hours_run += dt_s / 3600.0;
        let tc_slip_w = if self.kind == AutomaticKind::TorqueConverter {
            let n_t = rpm_turbine.unwrap_or(rpm_in);
            self.torque_converter_state(rpm_in, n_t, forced_lockup).slip_kw * 1000.0
        } else {
            0.0
        };
        let p_loss_w = (op.churn_loss_kw + op.bearing_loss_kw + op.power_in_kw * (1.0 - op.mesh_eff)) * 1000.0 + tc_slip_w;
        let oil_mass_kg = match self.kind {
            AutomaticKind::Dct => 6.5,
            AutomaticKind::TorqueConverter => 9.0,
        };
        let cp_oil = 2100.0;
        let h_cool = 35.0;
        let area_case = 0.7;
        let q_out = h_cool * area_case * (self.oil_temp_k_runtime - ambient_k);
        let d_temp = (p_loss_w - q_out) / (oil_mass_kg * cp_oil) * dt_s;
        self.oil_temp_k_runtime = (self.oil_temp_k_runtime + d_temp).max(ambient_k);
        self.oil_contamination += (op.power_in_kw / 100.0).max(0.0) * dt_s / 3600.0 * 1.0e-4;
        let overload = (torque_in_nm.abs() / self.max_input_torque_nm - 1.0).max(0.0);
        let overheat = ((self.oil_temp_k_runtime - 393.0) / 30.0).max(0.0);
        let pitting_rate = (op.hertz_pressure_pa / self.endurance_pa).powi(9);
        self.wear_factor += (pitting_rate * cycles / 1.0e9 + overload * 0.012 + overheat * 0.001 + self.oil_contamination * 0.001) * dt_s;
        if torque_in_nm.abs() > self.max_input_torque_nm * 1.5 {
            self.failures.push(GearboxFailureMode::GearToothBendingBroken);
            return;
        }
        if torque_in_nm.abs() > self.max_input_torque_nm * 1.15 && !self.failures.contains(&GearboxFailureMode::OverTorqueDamage) {
            self.failures.push(GearboxFailureMode::OverTorqueDamage);
        }
        if self.oil_temp_k_runtime > 423.0 && !self.failures.contains(&GearboxFailureMode::OilOverheat) {
            self.failures.push(GearboxFailureMode::OilOverheat);
        }
        if self.kind == AutomaticKind::TorqueConverter && self.oil_temp_k_runtime > 443.0 && !self.failures.contains(&GearboxFailureMode::TorqueConverterOverheat) {
            self.failures.push(GearboxFailureMode::TorqueConverterOverheat);
        }
        if self.oil_temp_k_runtime > 463.0 && !self.failures.contains(&GearboxFailureMode::BearingSeized) {
            self.failures.push(GearboxFailureMode::BearingSeized);
        }
        if self.wear_factor > 1.0 && !self.failures.contains(&GearboxFailureMode::GearToothPittingFatigue) {
            self.failures.push(GearboxFailureMode::GearToothPittingFatigue);
        }
        if self.clutch_energy_consumed_j > self.clutch_thermal_capacity_j() * 1.0e4 && !self.failures.contains(&GearboxFailureMode::ClutchPackBurnt) {
            self.failures.push(GearboxFailureMode::ClutchPackBurnt);
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

fn compute_metrics(
    forward_teeth: &[(u32, u32)],
    module_mm: f64,
    face_width_mm: f64,
    endurance_pa: f64,
    safety: f64,
    mesh_eff: f64,
) -> (f64, f64, f64) {
    let rho = sciforge_core::moleculars::solids::irons::steels::STEEL_4340.density_kg_m3;
    let b_m = face_width_mm / 1000.0;
    let inertia_kgm2: f64 = forward_teeth.iter().flat_map(|(p, s)| [*p, *s]).map(|t| {
        let r_m = (t as f64 * module_mm) / 2000.0;
        let mass = std::f64::consts::PI * r_m.powi(2) * b_m * rho;
        0.5 * mass * r_m.powi(2)
    }).sum();
    let h_m = (module_mm * 2.25) / 1000.0;
    let t_m = (module_mm * std::f64::consts::PI / 2.0) / 1000.0;
    let i_m4 = b_m * t_m.powi(3) / 12.0;
    let y_m = t_m / 2.0;
    let allowable = endurance_pa / safety;
    let max_input_torque_nm = forward_teeth.iter().map(|(p, _)| {
        let r_m = (*p as f64 * module_mm) / 2000.0;
        let unit_m = h_m / r_m;
        let stress = physics_scalar("beam_bending_stress", &["m", "y", "i"], &[unit_m, y_m, i_m4])
            .unwrap_or(unit_m * y_m / i_m4);
        allowable / stress
    }).fold(f64::INFINITY, f64::min);
    let efficiency = mesh_eff.powi(forward_teeth.len() as i32);
    (efficiency, inertia_kgm2, max_input_torque_nm)
}
