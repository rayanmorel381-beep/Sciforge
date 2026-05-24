//! Catalog entries for mathematics functions.

use super::FunctionInfo;
use super::reg;
use crate::engine::experience::experiment::DomainType::Maths;

pub(super) fn register(e: &mut Vec<FunctionInfo>) {
    reg(
        e,
        Maths,
        "complex_add",
        &["re1", "im1", "re2", "im2"],
        "Complex addition",
    );
    reg(
        e,
        Maths,
        "complex_mul",
        &["re1", "im1", "re2", "im2"],
        "Complex multiplication",
    );
    reg(
        e,
        Maths,
        "complex_modulus",
        &["re", "im"],
        "Complex modulus |z|",
    );
    reg(
        e,
        Maths,
        "determinant_2x2",
        &["matrix"],
        "2×2 matrix determinant",
    );
    reg(e, Maths, "dot_product", &["a", "b"], "Vector dot product");
    reg(
        e,
        Maths,
        "cross_product_3d",
        &["a", "b"],
        "3D cross product",
    );
    reg(
        e,
        Maths,
        "polynomial_eval",
        &["coefficients", "x"],
        "Polynomial evaluation (Horner)",
    );
    reg(
        e,
        Maths,
        "trapezoid",
        &["a", "b", "n", "fn_values"],
        "Trapezoidal integration",
    );
    reg(e, Maths, "euler_step", &["y", "dy", "h"], "Euler ODE step");
    reg(e, Maths, "fft_length", &["n"], "FFT valid length check");
    reg(e, Maths, "mean", &["data"], "Arithmetic mean");
    reg(e, Maths, "variance", &["data"], "Variance");
    reg(
        e,
        Maths,
        "standard_deviation",
        &["data"],
        "Standard deviation",
    );
    reg(
        e,
        Maths,
        "linear_regression_slope",
        &["x", "y"],
        "Linear regression slope",
    );
    reg(
        e,
        Maths,
        "pearson_correlation",
        &["x", "y"],
        "Pearson correlation coefficient",
    );
}
