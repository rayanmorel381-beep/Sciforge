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

mod actinides;
mod alkali_metals;
mod alkaline_earth_metals;
mod halogens;
mod lanthanides;
mod metalloids;
mod noble_gases;
mod nonmetals;
mod post_transition_metals;
mod superheavy_elements;
mod transition_metals;

fn init_elements() -> Vec<Element> {
    vec![
        nonmetals::hydrogen::element(),
        noble_gases::helium::element(),
        alkali_metals::lithium::element(),
        alkaline_earth_metals::beryllium::element(),
        metalloids::boron::element(),
        nonmetals::carbon::element(),
        nonmetals::nitrogen::element(),
        nonmetals::oxygen::element(),
        halogens::fluorine::element(),
        noble_gases::neon::element(),
        alkali_metals::sodium::element(),
        alkaline_earth_metals::magnesium::element(),
        post_transition_metals::aluminium::element(),
        metalloids::silicon::element(),
        nonmetals::phosphorus::element(),
        nonmetals::sulfur::element(),
        halogens::chlorine::element(),
        noble_gases::argon::element(),
        alkali_metals::potassium::element(),
        alkaline_earth_metals::calcium::element(),
        transition_metals::scandium::element(),
        transition_metals::titanium::element(),
        transition_metals::vanadium::element(),
        transition_metals::chromium::element(),
        transition_metals::manganese::element(),
        transition_metals::iron::element(),
        transition_metals::cobalt::element(),
        transition_metals::nickel::element(),
        transition_metals::copper::element(),
        transition_metals::zinc::element(),
        post_transition_metals::gallium::element(),
        metalloids::germanium::element(),
        metalloids::arsenic::element(),
        nonmetals::selenium::element(),
        halogens::bromine::element(),
        noble_gases::krypton::element(),
        alkali_metals::rubidium::element(),
        alkaline_earth_metals::strontium::element(),
        transition_metals::yttrium::element(),
        transition_metals::zirconium::element(),
        transition_metals::niobium::element(),
        transition_metals::molybdenum::element(),
        transition_metals::technetium::element(),
        transition_metals::ruthenium::element(),
        transition_metals::rhodium::element(),
        transition_metals::palladium::element(),
        transition_metals::silver::element(),
        transition_metals::cadmium::element(),
        post_transition_metals::indium::element(),
        post_transition_metals::tin::element(),
        metalloids::antimony::element(),
        metalloids::tellurium::element(),
        halogens::iodine::element(),
        noble_gases::xenon::element(),
        alkali_metals::cesium::element(),
        alkaline_earth_metals::barium::element(),
        lanthanides::lanthanum::element(),
        lanthanides::cerium::element(),
        lanthanides::praseodymium::element(),
        lanthanides::neodymium::element(),
        lanthanides::promethium::element(),
        lanthanides::samarium::element(),
        lanthanides::europium::element(),
        lanthanides::gadolinium::element(),
        lanthanides::terbium::element(),
        lanthanides::dysprosium::element(),
        lanthanides::holmium::element(),
        lanthanides::erbium::element(),
        lanthanides::thulium::element(),
        lanthanides::ytterbium::element(),
        lanthanides::lutetium::element(),
        transition_metals::hafnium::element(),
        transition_metals::tantalum::element(),
        transition_metals::tungsten::element(),
        transition_metals::rhenium::element(),
        transition_metals::osmium::element(),
        transition_metals::iridium::element(),
        transition_metals::platinum::element(),
        transition_metals::gold::element(),
        transition_metals::mercury::element(),
        post_transition_metals::thallium::element(),
        post_transition_metals::lead::element(),
        post_transition_metals::bismuth::element(),
        post_transition_metals::polonium::element(),
        halogens::astatine::element(),
        noble_gases::radon::element(),
        alkali_metals::francium::element(),
        alkaline_earth_metals::radium::element(),
        actinides::actinium::element(),
        actinides::thorium::element(),
        actinides::protactinium::element(),
        actinides::uranium::element(),
        actinides::neptunium::element(),
        actinides::plutonium::element(),
        actinides::americium::element(),
        actinides::curium::element(),
        actinides::berkelium::element(),
        actinides::californium::element(),
        actinides::einsteinium::element(),
        actinides::fermium::element(),
        actinides::mendelevium::element(),
        actinides::nobelium::element(),
        actinides::lawrencium::element(),
        superheavy_elements::rutherfordium::element(),
        superheavy_elements::dubnium::element(),
        superheavy_elements::seaborgium::element(),
        superheavy_elements::bohrium::element(),
        superheavy_elements::hassium::element(),
        superheavy_elements::meitnerium::element(),
        superheavy_elements::darmstadtium::element(),
        superheavy_elements::roentgenium::element(),
        superheavy_elements::copernicium::element(),
        superheavy_elements::nihonium::element(),
        superheavy_elements::flerovium::element(),
        superheavy_elements::moscovium::element(),
        superheavy_elements::livermorium::element(),
        halogens::tennessine::element(),
        noble_gases::oganesson::element(),
    ]
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
