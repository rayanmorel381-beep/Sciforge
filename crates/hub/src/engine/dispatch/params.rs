//! Typed parameter extraction helpers for dispatch handlers.
//!
//! Provides [`get_f`], [`get_i`], [`get_s`], and related accessors
//! that pull values from a `Params` map with proper error reporting.

use crate::domain::common::errors::{HubError, HubResult};
use crate::engine::experience::experiment::ParameterValue;
#[cfg(feature = "maths")]
use sciforge_lib::maths::complex::Complex;

/// Alias for a named parameter list.
pub type Params = Vec<(String, ParameterValue)>;

/// Extracts a scalar (`f64`) parameter by name.
pub fn get_f(p: &Params, name: &str) -> HubResult<f64> {
    for (k, v) in p {
        if k == name {
            return match v {
                ParameterValue::Scalar(x) => Ok(*x),
                ParameterValue::Integer(i) => Ok(*i as f64),
                _ => Err(HubError::InvalidInput(format!("{name}: expected scalar"))),
            };
        }
    }
    Err(HubError::InvalidInput(format!("missing parameter: {name}")))
}

/// Extracts an integer (`i64`) parameter by name.
pub fn get_i(p: &Params, name: &str) -> HubResult<i64> {
    for (k, v) in p {
        if k == name {
            return match v {
                ParameterValue::Integer(i) => Ok(*i),
                ParameterValue::Scalar(x) => Ok(*x as i64),
                _ => Err(HubError::InvalidInput(format!("{name}: expected integer"))),
            };
        }
    }
    Err(HubError::InvalidInput(format!("missing parameter: {name}")))
}

/// Extracts a `usize` parameter by name.
pub fn get_u(p: &Params, name: &str) -> HubResult<usize> {
    get_i(p, name).map(|i| i as usize)
}

/// Extracts a vector (`&[f64]`) parameter by name.
pub fn get_v<'a>(p: &'a Params, name: &str) -> HubResult<&'a [f64]> {
    for (k, v) in p {
        if k == name {
            return match v {
                ParameterValue::Vector(vec) => Ok(vec),
                _ => Err(HubError::InvalidInput(format!("{name}: expected vector"))),
            };
        }
    }
    Err(HubError::InvalidInput(format!("missing parameter: {name}")))
}

/// Extracts a matrix (`&[Vec<f64>]`) parameter by name.
pub fn get_m<'a>(p: &'a Params, name: &str) -> HubResult<&'a [Vec<f64>]> {
    for (k, v) in p {
        if k == name {
            return match v {
                ParameterValue::Matrix(mat) => Ok(mat),
                _ => Err(HubError::InvalidInput(format!("{name}: expected matrix"))),
            };
        }
    }
    Err(HubError::InvalidInput(format!("missing parameter: {name}")))
}

/// Extracts a boolean parameter by name.
pub fn get_b(p: &Params, name: &str) -> HubResult<bool> {
    for (k, v) in p {
        if k == name {
            return match v {
                ParameterValue::Boolean(b) => Ok(*b),
                _ => Err(HubError::InvalidInput(format!("{name}: expected boolean"))),
            };
        }
    }
    Err(HubError::InvalidInput(format!("missing parameter: {name}")))
}

/// Extracts a text (`&str`) parameter by name.
pub fn get_str<'a>(p: &'a Params, name: &str) -> HubResult<&'a str> {
    for (k, v) in p {
        if k == name {
            return match v {
                ParameterValue::Text(s) => Ok(s),
                _ => Err(HubError::InvalidInput(format!("{name}: expected text"))),
            };
        }
    }
    Err(HubError::InvalidInput(format!("missing parameter: {name}")))
}

/// Extracts a complex number parameter by name.
#[cfg(feature = "maths")]
pub fn get_c(p: &Params, name: &str) -> HubResult<Complex> {
    for (k, v) in p {
        if k == name {
            return match v {
                ParameterValue::Complex(re, im) => Ok(Complex::new(*re, *im)),
                ParameterValue::Scalar(x) => Ok(Complex::new(*x, 0.0)),
                _ => Err(HubError::InvalidInput(format!("{name}: expected complex"))),
            };
        }
    }
    Err(HubError::InvalidInput(format!("missing parameter: {name}")))
}

/// Extracts a complex vector parameter by name.
#[cfg(feature = "maths")]
pub fn get_cv(p: &Params, name: &str) -> HubResult<Vec<Complex>> {
    for (k, v) in p {
        if k == name {
            return match v {
                ParameterValue::ComplexVector(cv) => {
                    Ok(cv.iter().map(|&(re, im)| Complex::new(re, im)).collect())
                }
                _ => Err(HubError::InvalidInput(format!(
                    "{name}: expected complex vector"
                ))),
            };
        }
    }
    Err(HubError::InvalidInput(format!("missing parameter: {name}")))
}

/// Extracts a polynomial coefficient slice by name.
pub fn get_poly<'a>(p: &'a Params, name: &str) -> HubResult<&'a [f64]> {
    for (k, v) in p {
        if k == name {
            return match v {
                ParameterValue::Polynomial(c) => Ok(c),
                ParameterValue::Vector(c) => Ok(c),
                _ => Err(HubError::InvalidInput(format!(
                    "{name}: expected polynomial"
                ))),
            };
        }
    }
    Err(HubError::InvalidInput(format!("missing parameter: {name}")))
}

/// Extracts a `usize` vector parameter by name.
pub fn get_uv<'a>(p: &'a Params, name: &str) -> HubResult<&'a [usize]> {
    for (k, v) in p {
        if k == name {
            return match v {
                ParameterValue::IntVector(iv) => Ok(iv),
                _ => Err(HubError::InvalidInput(format!(
                    "{name}: expected int vector"
                ))),
            };
        }
    }
    Err(HubError::InvalidInput(format!("missing parameter: {name}")))
}

/// Extracts a `usize` matrix parameter by name.
pub fn get_um<'a>(p: &'a Params, name: &str) -> HubResult<&'a [Vec<usize>]> {
    for (k, v) in p {
        if k == name {
            return match v {
                ParameterValue::IntMatrix(im) => Ok(im),
                _ => Err(HubError::InvalidInput(format!(
                    "{name}: expected int matrix"
                ))),
            };
        }
    }
    Err(HubError::InvalidInput(format!("missing parameter: {name}")))
}

/// Extracts a weighted edge list parameter by name.
pub fn get_edges<'a>(p: &'a Params, name: &str) -> HubResult<&'a [(usize, usize, f64)]> {
    for (k, v) in p {
        if k == name {
            return match v {
                ParameterValue::EdgeList(el) => Ok(el),
                _ => Err(HubError::InvalidInput(format!(
                    "{name}: expected edge list"
                ))),
            };
        }
    }
    Err(HubError::InvalidInput(format!("missing parameter: {name}")))
}

type SparseData<'a> = (usize, usize, &'a [usize], &'a [usize], &'a [f64]);

/// Extracts a CSR sparse matrix parameter by name.
pub fn get_sparse<'a>(p: &'a Params, name: &str) -> HubResult<SparseData<'a>> {
    for (k, v) in p {
        if k == name {
            return match v {
                ParameterValue::Sparse {
                    rows,
                    cols,
                    row_ptr,
                    col_idx,
                    values,
                } => Ok((
                    *rows,
                    *cols,
                    row_ptr.as_slice(),
                    col_idx.as_slice(),
                    values.as_slice(),
                )),
                _ => Err(HubError::InvalidInput(format!("{name}: expected sparse"))),
            };
        }
    }
    Err(HubError::InvalidInput(format!("missing parameter: {name}")))
}

/// Extracts a tensor (data + shape) parameter by name.
pub fn get_tensor<'a>(p: &'a Params, name: &str) -> HubResult<(&'a [f64], &'a [usize])> {
    for (k, v) in p {
        if k == name {
            return match v {
                ParameterValue::Tensor { data, shape } => Ok((data, shape)),
                _ => Err(HubError::InvalidInput(format!("{name}: expected tensor"))),
            };
        }
    }
    Err(HubError::InvalidInput(format!("missing parameter: {name}")))
}
