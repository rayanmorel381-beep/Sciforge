//! Experiment data model.
//!
//! [`DomainType`] enumerates every supported scientific discipline;
//! [`Experiment`] pairs a domain with a function name and a list of
//! [`ParameterValue`]s.

/// Supported scientific domains.
#[derive(Debug, Clone)]
pub enum DomainType {
    Maths,
    Physics,
    Chemistry,
    Biology,
    Astronomy,
    Geology,
    Meteorology,
    Astrochemistry,
    Geophysics,
    Astrophysics,
    Biochemistry,
    Biophysics,
    Geochemistry,
    Astrobiology,
    AtmosphericChemistry,
    AtmosphericPhysics,
    PlanetaryGeology,
    Biomathematics,
    MathematicalPhysics,
}

/// Typed parameter value passed to a computation.
#[derive(Debug, Clone)]
pub enum ParameterValue {
    Scalar(f64),
    Integer(i64),
    Vector(Vec<f64>),
    Matrix(Vec<Vec<f64>>),
    Boolean(bool),
    Text(String),
    Complex(f64, f64),
    ComplexVector(Vec<(f64, f64)>),
    Polynomial(Vec<f64>),
    IntVector(Vec<usize>),
    IntMatrix(Vec<Vec<usize>>),
    EdgeList(Vec<(usize, usize, f64)>),
    Sparse {
        rows: usize,
        cols: usize,
        row_ptr: Vec<usize>,
        col_idx: Vec<usize>,
        values: Vec<f64>,
    },
    Tensor {
        data: Vec<f64>,
        shape: Vec<usize>,
    },
}

/// A computation request targeting a domain function with parameters.
#[derive(Debug, Clone)]
pub struct Experiment {
    /// Scientific domain.
    pub domain: DomainType,
    /// Function to invoke.
    pub function_name: String,
    /// Named parameters.
    pub parameters: Vec<(String, ParameterValue)>,
}

impl Experiment {
    /// Creates an experiment with no parameters.
    pub fn new(domain: DomainType, function_name: &str) -> Self {
        Self {
            domain,
            function_name: function_name.to_string(),
            parameters: Vec::new(),
        }
    }

    /// Appends a named parameter.
    pub fn param(mut self, name: &str, value: ParameterValue) -> Self {
        self.parameters.push((name.to_string(), value));
        self
    }
}
