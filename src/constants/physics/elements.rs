use crate::parser::yaml::YamlValue;
use crate::parser::yaml::parse_yaml;
use std::sync::OnceLock;

#[derive(Clone, Debug)]
pub struct DecayMode {
    pub mode: &'static str,
    pub branching_ratio: f64,
    pub daughter: Option<&'static str>,
}

#[derive(Clone, Debug)]
pub struct Isotope {
    pub name: &'static str,
    pub symbol: &'static str,
    pub mass_number: u32,
    pub neutrons: u32,
    pub atomic_mass: f64,
    pub half_life: Option<f64>,
    pub half_life_unit: Option<&'static str>,
    pub stable: bool,
    pub decay_modes: Vec<DecayMode>,
    pub natural_abundance: f64,
    pub nuclear_spin: Option<&'static str>,
}

#[derive(Clone, Debug)]
pub struct Element {
    pub symbol: &'static str,
    pub name: &'static str,
    pub atomic_number: u32,
    pub atomic_mass: f64,
    pub electronegativity: Option<f64>,
    pub group: Option<u32>,
    pub period: u32,
    pub category: &'static str,
    pub electron_configuration: &'static str,
    pub isotopes: Vec<Isotope>,
}

const YAML_SOURCES: [&str; 118] = [
    include_str!("../../../tableau-periodique/non-metaux/hydrogen.yaml"),
    include_str!("../../../tableau-periodique/gaz-nobles/helium.yaml"),
    include_str!("../../../tableau-periodique/metaux-alcalins/lithium.yaml"),
    include_str!("../../../tableau-periodique/metaux-alcalino-terreux/beryllium.yaml"),
    include_str!("../../../tableau-periodique/metalloides/boron.yaml"),
    include_str!("../../../tableau-periodique/non-metaux/carbon.yaml"),
    include_str!("../../../tableau-periodique/non-metaux/nitrogen.yaml"),
    include_str!("../../../tableau-periodique/non-metaux/oxygen.yaml"),
    include_str!("../../../tableau-periodique/halogenes/fluorine.yaml"),
    include_str!("../../../tableau-periodique/gaz-nobles/neon.yaml"),
    include_str!("../../../tableau-periodique/metaux-alcalins/sodium.yaml"),
    include_str!("../../../tableau-periodique/metaux-alcalino-terreux/magnesium.yaml"),
    include_str!("../../../tableau-periodique/metaux-post-transition/aluminium.yaml"),
    include_str!("../../../tableau-periodique/metalloides/silicon.yaml"),
    include_str!("../../../tableau-periodique/non-metaux/phosphorus.yaml"),
    include_str!("../../../tableau-periodique/non-metaux/sulfur.yaml"),
    include_str!("../../../tableau-periodique/halogenes/chlorine.yaml"),
    include_str!("../../../tableau-periodique/gaz-nobles/argon.yaml"),
    include_str!("../../../tableau-periodique/metaux-alcalins/potassium.yaml"),
    include_str!("../../../tableau-periodique/metaux-alcalino-terreux/calcium.yaml"),
    include_str!("../../../tableau-periodique/metaux-de-transition/scandium.yaml"),
    include_str!("../../../tableau-periodique/metaux-de-transition/titanium.yaml"),
    include_str!("../../../tableau-periodique/metaux-de-transition/vanadium.yaml"),
    include_str!("../../../tableau-periodique/metaux-de-transition/chromium.yaml"),
    include_str!("../../../tableau-periodique/metaux-de-transition/manganese.yaml"),
    include_str!("../../../tableau-periodique/metaux-de-transition/iron.yaml"),
    include_str!("../../../tableau-periodique/metaux-de-transition/cobalt.yaml"),
    include_str!("../../../tableau-periodique/metaux-de-transition/nickel.yaml"),
    include_str!("../../../tableau-periodique/metaux-de-transition/copper.yaml"),
    include_str!("../../../tableau-periodique/metaux-de-transition/zinc.yaml"),
    include_str!("../../../tableau-periodique/metaux-post-transition/gallium.yaml"),
    include_str!("../../../tableau-periodique/metalloides/germanium.yaml"),
    include_str!("../../../tableau-periodique/metalloides/arsenic.yaml"),
    include_str!("../../../tableau-periodique/non-metaux/selenium.yaml"),
    include_str!("../../../tableau-periodique/halogenes/bromine.yaml"),
    include_str!("../../../tableau-periodique/gaz-nobles/krypton.yaml"),
    include_str!("../../../tableau-periodique/metaux-alcalins/rubidium.yaml"),
    include_str!("../../../tableau-periodique/metaux-alcalino-terreux/strontium.yaml"),
    include_str!("../../../tableau-periodique/metaux-de-transition/yttrium.yaml"),
    include_str!("../../../tableau-periodique/metaux-de-transition/zirconium.yaml"),
    include_str!("../../../tableau-periodique/metaux-de-transition/niobium.yaml"),
    include_str!("../../../tableau-periodique/metaux-de-transition/molybdenum.yaml"),
    include_str!("../../../tableau-periodique/metaux-de-transition/technetium.yaml"),
    include_str!("../../../tableau-periodique/metaux-de-transition/ruthenium.yaml"),
    include_str!("../../../tableau-periodique/metaux-de-transition/rhodium.yaml"),
    include_str!("../../../tableau-periodique/metaux-de-transition/palladium.yaml"),
    include_str!("../../../tableau-periodique/metaux-de-transition/silver.yaml"),
    include_str!("../../../tableau-periodique/metaux-de-transition/cadmium.yaml"),
    include_str!("../../../tableau-periodique/metaux-post-transition/indium.yaml"),
    include_str!("../../../tableau-periodique/metaux-post-transition/tin.yaml"),
    include_str!("../../../tableau-periodique/metalloides/antimony.yaml"),
    include_str!("../../../tableau-periodique/metalloides/tellurium.yaml"),
    include_str!("../../../tableau-periodique/halogenes/iodine.yaml"),
    include_str!("../../../tableau-periodique/gaz-nobles/xenon.yaml"),
    include_str!("../../../tableau-periodique/metaux-alcalins/cesium.yaml"),
    include_str!("../../../tableau-periodique/metaux-alcalino-terreux/barium.yaml"),
    include_str!("../../../tableau-periodique/lanthanides/lanthanum.yaml"),
    include_str!("../../../tableau-periodique/lanthanides/cerium.yaml"),
    include_str!("../../../tableau-periodique/lanthanides/praseodymium.yaml"),
    include_str!("../../../tableau-periodique/lanthanides/neodymium.yaml"),
    include_str!("../../../tableau-periodique/lanthanides/promethium.yaml"),
    include_str!("../../../tableau-periodique/lanthanides/samarium.yaml"),
    include_str!("../../../tableau-periodique/lanthanides/europium.yaml"),
    include_str!("../../../tableau-periodique/lanthanides/gadolinium.yaml"),
    include_str!("../../../tableau-periodique/lanthanides/terbium.yaml"),
    include_str!("../../../tableau-periodique/lanthanides/dysprosium.yaml"),
    include_str!("../../../tableau-periodique/lanthanides/holmium.yaml"),
    include_str!("../../../tableau-periodique/lanthanides/erbium.yaml"),
    include_str!("../../../tableau-periodique/lanthanides/thulium.yaml"),
    include_str!("../../../tableau-periodique/lanthanides/ytterbium.yaml"),
    include_str!("../../../tableau-periodique/lanthanides/lutetium.yaml"),
    include_str!("../../../tableau-periodique/metaux-de-transition/hafnium.yaml"),
    include_str!("../../../tableau-periodique/metaux-de-transition/tantalum.yaml"),
    include_str!("../../../tableau-periodique/metaux-de-transition/tungsten.yaml"),
    include_str!("../../../tableau-periodique/metaux-de-transition/rhenium.yaml"),
    include_str!("../../../tableau-periodique/metaux-de-transition/osmium.yaml"),
    include_str!("../../../tableau-periodique/metaux-de-transition/iridium.yaml"),
    include_str!("../../../tableau-periodique/metaux-de-transition/platinum.yaml"),
    include_str!("../../../tableau-periodique/metaux-de-transition/gold.yaml"),
    include_str!("../../../tableau-periodique/metaux-de-transition/mercury.yaml"),
    include_str!("../../../tableau-periodique/metaux-post-transition/thallium.yaml"),
    include_str!("../../../tableau-periodique/metaux-post-transition/lead.yaml"),
    include_str!("../../../tableau-periodique/metaux-post-transition/bismuth.yaml"),
    include_str!("../../../tableau-periodique/metaux-post-transition/polonium.yaml"),
    include_str!("../../../tableau-periodique/halogenes/astatine.yaml"),
    include_str!("../../../tableau-periodique/gaz-nobles/radon.yaml"),
    include_str!("../../../tableau-periodique/metaux-alcalins/francium.yaml"),
    include_str!("../../../tableau-periodique/metaux-alcalino-terreux/radium.yaml"),
    include_str!("../../../tableau-periodique/actinides/actinium.yaml"),
    include_str!("../../../tableau-periodique/actinides/thorium.yaml"),
    include_str!("../../../tableau-periodique/actinides/protactinium.yaml"),
    include_str!("../../../tableau-periodique/actinides/uranium.yaml"),
    include_str!("../../../tableau-periodique/actinides/neptunium.yaml"),
    include_str!("../../../tableau-periodique/actinides/plutonium.yaml"),
    include_str!("../../../tableau-periodique/actinides/americium.yaml"),
    include_str!("../../../tableau-periodique/actinides/curium.yaml"),
    include_str!("../../../tableau-periodique/actinides/berkelium.yaml"),
    include_str!("../../../tableau-periodique/actinides/californium.yaml"),
    include_str!("../../../tableau-periodique/actinides/einsteinium.yaml"),
    include_str!("../../../tableau-periodique/actinides/fermium.yaml"),
    include_str!("../../../tableau-periodique/actinides/mendelevium.yaml"),
    include_str!("../../../tableau-periodique/actinides/nobelium.yaml"),
    include_str!("../../../tableau-periodique/actinides/lawrencium.yaml"),
    include_str!("../../../tableau-periodique/elements-superlourds/rutherfordium.yaml"),
    include_str!("../../../tableau-periodique/elements-superlourds/dubnium.yaml"),
    include_str!("../../../tableau-periodique/elements-superlourds/seaborgium.yaml"),
    include_str!("../../../tableau-periodique/elements-superlourds/bohrium.yaml"),
    include_str!("../../../tableau-periodique/elements-superlourds/hassium.yaml"),
    include_str!("../../../tableau-periodique/elements-superlourds/meitnerium.yaml"),
    include_str!("../../../tableau-periodique/elements-superlourds/darmstadtium.yaml"),
    include_str!("../../../tableau-periodique/elements-superlourds/roentgenium.yaml"),
    include_str!("../../../tableau-periodique/elements-superlourds/copernicium.yaml"),
    include_str!("../../../tableau-periodique/elements-superlourds/nihonium.yaml"),
    include_str!("../../../tableau-periodique/elements-superlourds/flerovium.yaml"),
    include_str!("../../../tableau-periodique/elements-superlourds/moscovium.yaml"),
    include_str!("../../../tableau-periodique/elements-superlourds/livermorium.yaml"),
    include_str!("../../../tableau-periodique/halogenes/tennessine.yaml"),
    include_str!("../../../tableau-periodique/gaz-nobles/oganesson.yaml"),
];

fn parse_decay_mode(node: &YamlValue<'static>) -> DecayMode {
    DecayMode {
        mode: node.get("mode").and_then(|v| v.as_str()).unwrap_or(""),
        branching_ratio: node
            .get("branching_ratio")
            .and_then(|v| v.as_number())
            .unwrap_or(0.0),
        daughter: node.get("daughter").and_then(|v| v.as_str()),
    }
}

fn parse_isotope(node: &YamlValue<'static>) -> Isotope {
    let abundance_val = node.get("natural_abundance");
    let natural_abundance = match abundance_val {
        Some(YamlValue::String("trace")) => 0.0,
        Some(v) => v.as_number().unwrap_or(0.0),
        None => 0.0,
    };

    Isotope {
        name: node.get("name").and_then(|v| v.as_str()).unwrap_or(""),
        symbol: node.get("symbol").and_then(|v| v.as_str()).unwrap_or(""),
        mass_number: node
            .get("mass_number")
            .and_then(|v| v.as_u32())
            .unwrap_or(0),
        neutrons: node.get("neutrons").and_then(|v| v.as_u32()).unwrap_or(0),
        atomic_mass: node
            .get("atomic_mass")
            .and_then(|v| v.as_number())
            .unwrap_or(0.0),
        half_life: node.get("half_life").and_then(|v| v.as_number()),
        half_life_unit: node.get("half_life_unit").and_then(|v| v.as_str()),
        stable: node
            .get("stable")
            .and_then(|v| v.as_bool())
            .unwrap_or(false),
        decay_modes: node
            .get("decay_modes")
            .map(|v| v.items().iter().map(parse_decay_mode).collect())
            .unwrap_or_default(),
        natural_abundance,
        nuclear_spin: node.get("nuclear_spin").and_then(|v| v.as_str()),
    }
}

fn parse_element(yaml: &'static str) -> Element {
    let root = parse_yaml(yaml.as_bytes()).expect("invalid element yaml");
    let elem = root.get("element").expect("missing element key");

    Element {
        name: elem.get("name").and_then(|v| v.as_str()).unwrap_or(""),
        symbol: elem.get("symbol").and_then(|v| v.as_str()).unwrap_or(""),
        atomic_number: elem
            .get("atomic_number")
            .and_then(|v| v.as_u32())
            .unwrap_or(0),
        atomic_mass: elem
            .get("atomic_mass")
            .and_then(|v| v.as_number())
            .unwrap_or(0.0),
        electronegativity: elem.get("electronegativity").and_then(|v| v.as_number()),
        group: elem.get("group").and_then(|v| v.as_u32()),
        period: elem.get("period").and_then(|v| v.as_u32()).unwrap_or(0),
        category: elem
            .get("category")
            .and_then(|v| v.as_str())
            .unwrap_or("unknown"),
        electron_configuration: elem
            .get("electron_configuration")
            .and_then(|v| v.as_str())
            .unwrap_or(""),
        isotopes: root
            .get("isotopes")
            .map(|v| v.items().iter().map(parse_isotope).collect())
            .unwrap_or_default(),
    }
}

fn init_elements() -> Vec<Element> {
    YAML_SOURCES.iter().map(|src| parse_element(src)).collect()
}

fn elements() -> &'static Vec<Element> {
    static ELEMENTS: OnceLock<Vec<Element>> = OnceLock::new();
    ELEMENTS.get_or_init(init_elements)
}

pub fn by_atomic_number(z: u32) -> Option<&'static Element> {
    if (1..=118).contains(&z) {
        Some(&elements()[(z - 1) as usize])
    } else {
        None
    }
}

pub fn all() -> &'static [Element] {
    elements()
}

pub fn by_symbol(sym: &str) -> Option<&'static Element> {
    elements().iter().find(|e| e.symbol == sym)
}

pub fn atomic_mass(z: u32) -> f64 {
    by_atomic_number(z).map_or(0.0, |e| e.atomic_mass)
}

pub fn electronegativity(z: u32) -> Option<f64> {
    by_atomic_number(z).and_then(|e| e.electronegativity)
}
