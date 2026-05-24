use sciforge_hub::prelude::*;

#[derive(Clone, Copy)]
struct DistortionMachine {
    name: &'static str,
    active_volume_m3: f64,
    cycle_s: f64,
    curvature_index: f64,
    conversion_efficiency: f64,
}

#[derive(Clone, Copy)]
struct Preset {
    name: &'static str,
    coherence_time_s: f64,
    gradient_factor: f64,
}

fn dark_energy_density_kg_m3() -> f64 {
    5.96e-27
}

fn dark_energy_density_j_m3() -> f64 {
    dark_energy_density_kg_m3() * constants::C * constants::C
}

fn half_life_to_seconds(half_life: f64, unit: Option<&str>) -> Option<f64> {
    let u = unit?.to_ascii_lowercase();
    if u.contains("year") || u.contains("yr") || u.contains("an") {
        return Some(half_life * 365.25 * 24.0 * 3600.0);
    }
    if u.contains("day") || u.contains("jour") {
        return Some(half_life * 24.0 * 3600.0);
    }
    if u.contains("hour") || u.contains("heure") || u == "h" {
        return Some(half_life * 3600.0);
    }
    if u.contains("min") {
        return Some(half_life * 60.0);
    }
    if u.contains("ms") {
        return Some(half_life * 1.0e-3);
    }
    if u.contains("us") {
        return Some(half_life * 1.0e-6);
    }
    if u.contains("ns") {
        return Some(half_life * 1.0e-9);
    }
    if u.contains("sec") || u == "s" {
        return Some(half_life);
    }
    None
}

fn isotope_coupling(stable: bool, half_life_s: Option<f64>, mass_number: u32) -> f64 {
    let mass_factor = ((mass_number as f64) / 7.0).sqrt().clamp(0.35, 4.0);
    if stable {
        return 1.0e-13 * mass_factor;
    }
    if let Some(t12) = half_life_s {
        let lambda = chemistry::nuclear::decay::half_life_to_decay_constant(t12);
        let weak_window = (lambda / 1.0e-9).sqrt().clamp(0.1, 1.0e6);
        return 1.0e-11 * weak_window * mass_factor;
    }
    1.0e-12 * mass_factor
}

fn distortion_power_w(volume_m3: f64, curvature_index: f64) -> f64 {
    let base_curvature_cost = 1.0e11;
    base_curvature_cost * volume_m3 * curvature_index.powi(2)
}

fn get_cocktail_coupling() -> Option<(f64, usize)> {
    let targets = [
        ("Be", 13u32),
        ("Li", 12u32),
        ("O", 12u32),
        ("N", 11u32),
        ("He", 10u32),
        ("F", 15u32),
    ];

    let mut couplings = Vec::new();
    for (sym, a) in targets {
        let e = constants::elements::by_symbol(sym)?;
        let iso = e.isotopes.iter().find(|i| i.mass_number == a)?;
        let t12_s = iso
            .half_life
            .and_then(|hl| half_life_to_seconds(hl, iso.half_life_unit));
        couplings.push(isotope_coupling(iso.stable, t12_s, iso.mass_number));
    }

    let mean = couplings.iter().sum::<f64>() / couplings.len() as f64;
    Some((mean, couplings.len()))
}

fn evaluate_preset(
    machine: DistortionMachine,
    preset: Preset,
    coupling_mean: f64,
    cocktail_size: usize,
) {
    let coherent_gain = 1.0 + 0.12 * (cocktail_size as f64 - 1.0);
    let coherence_boost = (preset.coherence_time_s / machine.cycle_s).clamp(0.05, 1.0e9);
    let p_dist = distortion_power_w(machine.active_volume_m3, machine.curvature_index);

    let p_out = dark_energy_density_j_m3()
        * machine.active_volume_m3
        * preset.gradient_factor
        * coherence_boost
        * coupling_mean
        * coherent_gain
        * machine.conversion_efficiency
        / machine.cycle_s;

    let p_net = p_out - p_dist;
    let status = if p_net > 0.0 {
        "AUTO-ALIMENTE"
    } else {
        "DEFICIT"
    };

    let e_start_j = p_dist * machine.cycle_s;
    let m_start_full = e_start_j / (constants::C * constants::C);
    let m_start_fusion = m_start_full / 0.007;

    let deficit_w = if p_net < 0.0 { -p_net } else { 0.0 };
    let m_dot_full = deficit_w / (constants::C * constants::C);
    let m_dot_fusion = m_dot_full / 0.007;

    let e_net_day_j = p_net * 86400.0;

    println!(
        "{:<24} | {:>8.0e} | {:>9.3e} | {:>11.3e} | {:>11.3e} | {:>11.3e} | {:>12}",
        preset.name, preset.coherence_time_s, preset.gradient_factor, p_out, p_dist, p_net, status
    );
    println!(
        "  E_start={:.3e} J | m_start(mc2)={:.3e} kg | m_start(fusion)={:.3e} kg",
        e_start_j, m_start_full, m_start_fusion
    );
    println!(
        "  deficit_mass_flow(mc2)={:.3e} kg/s | deficit_mass_flow(fusion)={:.3e} kg/s | E_net/day={:.3e} J",
        m_dot_full, m_dot_fusion, e_net_day_j
    );
}

fn main() {
    println!("====================================================================");
    println!(" CHRONOS - Presets utilisables moteur energie noire (cocktail fixe)");
    println!("====================================================================\n");

    let Some((coupling_mean, cocktail_size)) = get_cocktail_coupling() else {
        println!("Impossible de charger les isotopes du cocktail dans la table SciForge.");
        return;
    };

    println!("Cocktail utilise: ¹³Be, ¹²Li, ¹²O, ¹¹N, ¹⁰He, ¹⁵F");
    println!(
        "Coupling moyen = {:.3e}, taille cocktail = {}",
        coupling_mean, cocktail_size
    );
    println!(
        "Densite energie noire = {:.3e} J/m^3\n",
        dark_energy_density_j_m3()
    );

    let prototype = DistortionMachine {
        name: "Prototype labo realiste",
        active_volume_m3: 10.0,
        cycle_s: 1.0,
        curvature_index: 1.0e-10,
        conversion_efficiency: 0.30,
    };

    let ring = DistortionMachine {
        name: "Anneau orbital optimiste",
        active_volume_m3: 1.0e6,
        cycle_s: 1.0,
        curvature_index: 1.0e-9,
        conversion_efficiency: 0.45,
    };

    let prototype_presets = [
        Preset {
            name: "P1",
            coherence_time_s: 1.0e4,
            gradient_factor: 3.547e1,
        },
        Preset {
            name: "P2",
            coherence_time_s: 1.0e5,
            gradient_factor: 3.547e0,
        },
        Preset {
            name: "P3",
            coherence_time_s: 1.0e6,
            gradient_factor: 3.547e-1,
        },
        Preset {
            name: "P4",
            coherence_time_s: 1.0e7,
            gradient_factor: 3.547e-2,
        },
        Preset {
            name: "P5",
            coherence_time_s: 1.0e8,
            gradient_factor: 3.547e-3,
        },
        Preset {
            name: "P6",
            coherence_time_s: 1.0e9,
            gradient_factor: 3.547e-4,
        },
    ];

    let ring_presets = [
        Preset {
            name: "R1",
            coherence_time_s: 1.0e5,
            gradient_factor: 2.365e2,
        },
        Preset {
            name: "R2",
            coherence_time_s: 1.0e6,
            gradient_factor: 2.365e1,
        },
        Preset {
            name: "R3",
            coherence_time_s: 1.0e7,
            gradient_factor: 2.365e0,
        },
        Preset {
            name: "R4",
            coherence_time_s: 1.0e8,
            gradient_factor: 2.365e-1,
        },
        Preset {
            name: "R5",
            coherence_time_s: 1.0e9,
            gradient_factor: 2.365e-2,
        },
    ];

    println!(
        "====================== {} ======================",
        prototype.name
    );
    println!(
        "{:24} | {:>8} | {:>9} | {:>11} | {:>11} | {:>11} | {:>12}",
        "Preset", "C(s)", "G", "P_out(W)", "P_dist(W)", "P_net(W)", "Etat"
    );
    println!(
        "{:-<24}-+-{:-<8}-+-{:-<9}-+-{:-<11}-+-{:-<11}-+-{:-<11}-+-{:-<12}",
        "", "", "", "", "", "", ""
    );
    for p in prototype_presets {
        evaluate_preset(prototype, p, coupling_mean, cocktail_size);
    }

    println!(
        "\n====================== {} ======================",
        ring.name
    );
    println!(
        "{:24} | {:>8} | {:>9} | {:>11} | {:>11} | {:>11} | {:>12}",
        "Preset", "C(s)", "G", "P_out(W)", "P_dist(W)", "P_net(W)", "Etat"
    );
    println!(
        "{:-<24}-+-{:-<8}-+-{:-<9}-+-{:-<11}-+-{:-<11}-+-{:-<11}-+-{:-<12}",
        "", "", "", "", "", "", ""
    );
    for p in ring_presets {
        evaluate_preset(ring, p, coupling_mean, cocktail_size);
    }

    println!("\nConclusion:");
    println!("1) Cette version teste uniquement les presets utilisables identifies.");
    println!(
        "2) Pour chaque preset, l'exemple donne la puissance nette et la masse/energie necessaires."
    );
    println!("3) Si P_net > 0, le moteur est auto-alimente apres amorcage.");
}
