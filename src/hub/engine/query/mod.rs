//! Function catalog and introspection.
//!
//! The [`Catalog`] registers every available scientific function with its
//! [`FunctionInfo`] metadata (name, domain, parameter list). Used by the
//! CLI `list-functions` command and the HTTP introspection endpoint.

mod astrobiology;
mod astrochemistry;
mod astronomy;
mod astrophysics;
mod atmospheric_chemistry;
mod atmospheric_physics;
mod biochemistry;
mod biology;
mod biomathematics;
mod biophysics;
mod chemistry;
mod geochemistry;
mod geology;
mod geophysics;
mod mathematical_physics;
mod maths;
mod meteorology;
mod physics;
mod planetary_geology;

use crate::hub::engine::experience::experiment::DomainType;

/// Metadata describing a registered scientific function.
#[derive(Debug, Clone)]
pub struct FunctionInfo {
    /// Scientific domain.
    pub domain: DomainType,
    /// Function name.
    pub name: String,
    /// Expected parameter names.
    pub param_names: Vec<String>,
    /// Human-readable description.
    pub description: String,
}

/// Registry of all available scientific functions.
pub struct Catalog {
    entries: Vec<FunctionInfo>,
}

impl Catalog {
    /// Creates a catalog populated with all domain registrations.
    pub fn new() -> Self {
        let mut entries = Vec::new();
        astronomy::register(&mut entries);
        geology::register(&mut entries);
        meteorology::register(&mut entries);
        physics::register(&mut entries);
        chemistry::register(&mut entries);
        biology::register(&mut entries);
        maths::register(&mut entries);
        astrochemistry::register(&mut entries);
        geophysics::register(&mut entries);
        astrophysics::register(&mut entries);
        biochemistry::register(&mut entries);
        biophysics::register(&mut entries);
        geochemistry::register(&mut entries);
        astrobiology::register(&mut entries);
        atmospheric_chemistry::register(&mut entries);
        atmospheric_physics::register(&mut entries);
        planetary_geology::register(&mut entries);
        biomathematics::register(&mut entries);
        mathematical_physics::register(&mut entries);
        Self { entries }
    }

    /// Total number of registered functions.
    pub fn len(&self) -> usize {
        self.entries.len()
    }
    /// Returns `true` if the catalog is empty.
    pub fn is_empty(&self) -> bool {
        self.entries.is_empty()
    }

    /// Returns all functions belonging to the given domain.
    pub fn by_domain(&self, domain: &DomainType) -> Vec<&FunctionInfo> {
        let d = format!("{:?}", domain);
        self.entries
            .iter()
            .filter(|e| format!("{:?}", e.domain) == d)
            .collect()
    }

    /// Searches functions by name substring (case-insensitive).
    pub fn search(&self, pattern: &str) -> Vec<&FunctionInfo> {
        let p = pattern.to_lowercase();
        self.entries
            .iter()
            .filter(|e| e.name.to_lowercase().contains(&p))
            .collect()
    }

    /// Looks up a function by exact name.
    pub fn get(&self, name: &str) -> Option<&FunctionInfo> {
        self.entries.iter().find(|e| e.name == name)
    }

    /// Returns all registered function names.
    pub fn names(&self) -> Vec<&str> {
        self.entries.iter().map(|e| e.name.as_str()).collect()
    }

    /// Returns (domain_name, function_count) pairs for every domain.
    pub fn domain_summary(&self) -> Vec<(String, usize)> {
        let domains = [
            "Maths",
            "Physics",
            "Chemistry",
            "Biology",
            "Astronomy",
            "Geology",
            "Meteorology",
            "Astrochemistry",
            "Geophysics",
            "Astrophysics",
            "Biochemistry",
            "Biophysics",
            "Geochemistry",
            "Astrobiology",
            "AtmosphericChemistry",
            "AtmosphericPhysics",
            "PlanetaryGeology",
            "Biomathematics",
            "MathematicalPhysics",
        ];
        domains
            .iter()
            .map(|&d| {
                let count = self
                    .entries
                    .iter()
                    .filter(|e| format!("{:?}", e.domain) == d)
                    .count();
                (d.to_string(), count)
            })
            .collect()
    }

    /// Returns functions that accept the given parameter name.
    pub fn by_param(&self, param: &str) -> Vec<&FunctionInfo> {
        self.entries
            .iter()
            .filter(|e| e.param_names.iter().any(|p| p == param))
            .collect()
    }

    /// Returns functions with exactly `n` parameters.
    pub fn by_param_count(&self, n: usize) -> Vec<&FunctionInfo> {
        self.entries
            .iter()
            .filter(|e| e.param_names.len() == n)
            .collect()
    }

    /// Renders the catalog as a Markdown table.
    pub fn to_markdown(&self) -> String {
        let mut out = String::from("# SciForge Function Catalog\n\n");
        let summary = self.domain_summary();
        let total: usize = summary.iter().map(|(_, c)| c).sum();
        out.push_str(&format!("**Total functions: {}**\n\n", total));
        out.push_str("| Domain | Functions |\n|--------|----------|\n");
        for (d, c) in &summary {
            out.push_str(&format!("| {} | {} |\n", d, c));
        }
        out
    }
}

impl Default for Catalog {
    fn default() -> Self {
        Self::new()
    }
}

pub(super) fn reg(
    entries: &mut Vec<FunctionInfo>,
    domain: DomainType,
    name: &str,
    params: &[&str],
    desc: &str,
) {
    entries.push(FunctionInfo {
        domain,
        name: name.into(),
        param_names: params.iter().map(|s| s.to_string()).collect(),
        description: desc.into(),
    });
}
