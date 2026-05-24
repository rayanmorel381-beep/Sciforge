use sciforge_hub::benchmark::decode::decode;
use sciforge_hub::benchmark::engine::bench;
use sciforge_hub::benchmark::export::{Entry, export};
use sciforge_hub::benchmark::report::generate;
use sciforge_hub::prelude::chemistry::molecular::bonding;
use sciforge_hub::prelude::chemistry::stoichiometry::calculations;
use sciforge_hub::prelude::constants::elements;
use std::fs;
use std::path::Path;

struct Reaction {
    name: &'static str,
    formula: &'static str,
    category: &'static str,
    reactants: &'static [(&'static str, u32)],
    products: &'static [(&'static str, u32)],
}

const REACTIONS: [Reaction; 8] = [
    Reaction {
        name: "Water synthesis",
        formula: "2H\u{2082} + O\u{2082} \u{2192} 2H\u{2082}O",
        category: "Synthesis",
        reactants: &[("H", 4), ("O", 2)],
        products: &[("H", 4), ("O", 2)],
    },
    Reaction {
        name: "Combustion of methane",
        formula: "CH\u{2084} + 2O\u{2082} \u{2192} CO\u{2082} + 2H\u{2082}O",
        category: "Combustion",
        reactants: &[("C", 1), ("H", 4), ("O", 4)],
        products: &[("C", 1), ("O", 4), ("H", 4)],
    },
    Reaction {
        name: "Rust formation",
        formula: "4Fe + 3O\u{2082} \u{2192} 2Fe\u{2082}O\u{2083}",
        category: "Oxidation",
        reactants: &[("Fe", 4), ("O", 6)],
        products: &[("Fe", 4), ("O", 6)],
    },
    Reaction {
        name: "Photosynthesis",
        formula: "6CO\u{2082} + 6H\u{2082}O \u{2192} C\u{2086}H\u{2081}\u{2082}O\u{2086} + 6O\u{2082}",
        category: "Biochemistry",
        reactants: &[("C", 6), ("O", 18), ("H", 12)],
        products: &[("C", 6), ("H", 12), ("O", 18)],
    },
    Reaction {
        name: "Ammonia synthesis (Haber)",
        formula: "N\u{2082} + 3H\u{2082} \u{2192} 2NH\u{2083}",
        category: "Synthesis",
        reactants: &[("N", 2), ("H", 6)],
        products: &[("N", 2), ("H", 6)],
    },
    Reaction {
        name: "Sodium chloride",
        formula: "2Na + Cl\u{2082} \u{2192} 2NaCl",
        category: "Synthesis",
        reactants: &[("Na", 2), ("Cl", 2)],
        products: &[("Na", 2), ("Cl", 2)],
    },
    Reaction {
        name: "Sulfuric acid neutralization",
        formula: "H\u{2082}SO\u{2084} + 2NaOH \u{2192} Na\u{2082}SO\u{2084} + 2H\u{2082}O",
        category: "Acid-Base",
        reactants: &[("H", 4), ("S", 1), ("O", 6), ("Na", 2)],
        products: &[("Na", 2), ("S", 1), ("O", 6), ("H", 4)],
    },
    Reaction {
        name: "Thermite reaction",
        formula: "2Al + Fe\u{2082}O\u{2083} \u{2192} Al\u{2082}O\u{2083} + 2Fe",
        category: "Oxidation",
        reactants: &[("Al", 2), ("Fe", 2), ("O", 3)],
        products: &[("Al", 2), ("O", 3), ("Fe", 2)],
    },
];

fn molar_mass_from_composition(composition: &[(&str, u32)]) -> f64 {
    let all = elements::all();
    let mut masses = Vec::new();
    let mut counts = Vec::new();
    for &(sym, count) in composition {
        let e = all.iter().find(|e| e.symbol == sym).unwrap();
        masses.push(e.atomic_mass);
        counts.push(count);
    }
    bonding::molar_mass(&masses, &counts)
}

fn main() {
    let dir = Path::new("output/examples/chemical_reactions");
    fs::create_dir_all(dir).unwrap();

    let react_mass_strs: Vec<String> = REACTIONS
        .iter()
        .map(|r| format!("{:.4} g/mol", molar_mass_from_composition(r.reactants)))
        .collect();

    let prod_mass_strs: Vec<String> = REACTIONS
        .iter()
        .map(|r| format!("{:.4} g/mol", molar_mass_from_composition(r.products)))
        .collect();

    let atom_econ_strs: Vec<String> = REACTIONS
        .iter()
        .map(|r| {
            let prod_m = molar_mass_from_composition(r.products);
            let react_m = molar_mass_from_composition(r.reactants);
            let ae = calculations::atom_economy(prod_m, &[react_m]);
            format!("{:.1}%", ae)
        })
        .collect();

    let formula_strs: Vec<String> = REACTIONS.iter().map(|r| r.formula.to_string()).collect();

    let mut metrics_store: Vec<(String, String, _)> = Vec::new();

    let exp_names: Vec<String> = REACTIONS
        .iter()
        .map(|r| format!("{}_calc", r.name))
        .collect();

    for (idx, r) in REACTIONS.iter().enumerate() {
        let comp: Vec<(&str, u32)> = r.reactants.to_vec();
        let m = bench(&exp_names[idx], "f64", 5000, move || {
            let mut v = 0u64;
            for _ in 0..50 {
                let all = elements::all();
                let mut masses = Vec::new();
                let mut counts = Vec::new();
                for &(sym, count) in &comp {
                    let e = all.iter().find(|e| e.symbol == sym).unwrap();
                    masses.push(e.atomic_mass);
                    counts.push(count);
                }
                let mm = bonding::molar_mass(&masses, &counts);
                v = v.wrapping_add(mm.to_bits());
            }
            v
        });
        let result = format!(
            "{} | {} | M(react)={} | M(prod)={} | Atom economy={}",
            r.name, r.formula, react_mass_strs[idx], prod_mass_strs[idx], atom_econ_strs[idx]
        );
        let rpt = generate(&m, &result);
        assert!(rpt.csv.starts_with("experiment_name"));
        metrics_store.push((r.name.to_string(), result, m));
    }

    let cat_strs: Vec<String> = REACTIONS.iter().map(|r| r.category.to_string()).collect();

    let entries: Vec<Entry<'_>> = metrics_store
        .iter()
        .enumerate()
        .map(|(idx, (label, result, m))| Entry {
            metrics: m,
            result: result.as_str(),
            label: label.as_str(),
            tags: vec![
                ("category", cat_strs[idx].as_str()),
                ("name", REACTIONS[idx].name),
                ("formula", formula_strs[idx].as_str()),
                ("reactant_mass", react_mass_strs[idx].as_str()),
                ("product_mass", prod_mass_strs[idx].as_str()),
                ("atom_economy", atom_econ_strs[idx].as_str()),
            ],
            grid_row: Some((idx as u8 / 4) + 1),
            grid_col: Some((idx as u8 % 4) + 1),
        })
        .collect();

    let summary = export(
        "Chemical Reactions \u{2014} Molar Masses & Atom Economy",
        &entries,
        dir,
    )
    .unwrap();
    assert!(summary.files_written > 0);

    let bmk_dir = dir.join("bmk");
    let last_bmk = fs::read(bmk_dir.join("chemical_reactions.bmk")).unwrap();
    let view = decode(&last_bmk).unwrap();
    assert!(!view.experiment_name.is_empty());
}
