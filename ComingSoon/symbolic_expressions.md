## 1️⃣ Current State

| Building Block | Location | Description |
|---|---|---|
| Polynomial | `src/maths/polynomial/` | `{ coeffs: Vec<f64> }` with eval, derivative, integral, compose, div_rem, gcd, from_roots |
| Root solvers | `src/maths/polynomial/roots.rs` | newton_raphson, bisection, brent, durand_kerner, laguerre, find_all_roots_real |
| Special polynomials | `src/maths/polynomial/special.rs` | legendre, chebyshev, hermite, laguerre, bernstein, jacobi, etc. |
| Complex | `src/maths/complex/` | Complex, Quaternion, polynomial_eval |
| LaTeX in hypotheses | `demo/hypotheses/` | Inline formulas `\(10^{10}\)`, `\(\eta \approx 6.1 \times 10^{-10}\)` |

No expression tree, no symbolic manipulation. All computation is purely numerical (f64).

## 2️⃣ Expression Tree — `src/maths/symbolic/expr.rs`

| Variant | Description |
|---|---|
| `Const(f64)` | Numeric constant |
| `Var(String)` | Named variable |
| `Neg(Box<Expr>)` | Negation |
| `Add(Box<Expr>, Box<Expr>)` | Addition |
| `Sub(Box<Expr>, Box<Expr>)` | Subtraction |
| `Mul(Box<Expr>, Box<Expr>)` | Multiplication |
| `Div(Box<Expr>, Box<Expr>)` | Division |
| `Pow(Box<Expr>, Box<Expr>)` | Power |
| `Fn { name, args }` | Function call (sin, cos, exp, ln, sqrt, abs, etc.) |

| Trait / Method | Description |
|---|---|
| `impl Display` | Readable infix notation rendering |
| `impl PartialEq` | Structural equality |
| `Expr::var("x")` | Ergonomic variable constructor |
| `Expr::con(3.14)` | Ergonomic constant constructor |
| `Expr::sin(e)`, `Expr::add(a, b)` | Ergonomic function constructors |

## 3️⃣ Numerical Evaluation — `src/maths/symbolic/eval.rs`

| Component | Description |
|---|---|
| `eval(expr, vars: &HashMap<String, f64>)` | → `Result<f64, EvalError>` — recursive evaluation |
| `EvalError::UndefinedVariable` | Variable not defined in vars |
| `EvalError::DivisionByZero` | Division by zero |
| `EvalError::DomainError` | Invalid domain (negative sqrt, negative log) |

| Function | Description |
|---|---|
| sin, cos, tan | Trigonometric functions |
| asin, acos, atan | Inverse trigonometric functions |
| exp, ln, log10 | Exponential and logarithms |
| sqrt, abs | Square root and absolute value |
| floor, ceil | Rounding |
| pi, e, inf | Special constants |

## 4️⃣ Simplification — `src/maths/symbolic/simplify.rs`

| Rule | Transformation |
|---|---|
| `x + 0`, `0 + x` | → `x` |
| `x * 1`, `1 * x` | → `x` |
| `x * 0`, `0 * x` | → `0` |
| `x - 0` | → `x` |
| `x - x` | → `0` |
| `x / 1` | → `x` |
| `x / x` | → `1` |
| `x^0` | → `1` |
| `x^1` | → `x` |
| `0^x` | → `0` |
| `1^x` | → `1` |
| `Const(a) op Const(b)` | → `Const(result)` (constant folding) |
| `-(-x)` | → `x` (double negation) |
| Associativity | Group constants |

## 5️⃣ Symbolic Differentiation — `src/maths/symbolic/differentiate.rs`

| Rule | Formula |
|---|---|
| `d/dx(c)` | `0` |
| `d/dx(x)` | `1` |
| `d/dx(y)` | `0` (different variable) |
| `d/dx(f + g)` | `f' + g'` |
| `d/dx(f * g)` | `f'*g + f*g'` (product) |
| `d/dx(f / g)` | `(f'*g - f*g') / g²` (quotient) |
| `d/dx(f^n)` | `n * f^(n-1) * f'` (chain rule) |
| `d/dx(sin(f))` | `cos(f) * f'` |
| `d/dx(cos(f))` | `-sin(f) * f'` |
| `d/dx(exp(f))` | `exp(f) * f'` |
| `d/dx(ln(f))` | `f' / f` |
| `d/dx(sqrt(f))` | `f' / (2*sqrt(f))` |

Applies `simplify()` on the result.

## 6️⃣ Text Parser — `src/maths/symbolic/parse.rs`

| Grammar | Rule |
|---|---|
| `expr` | `term (('+' \| '-') term)*` |
| `term` | `factor (('*' \| '/') factor)*` |
| `factor` | `base ('^' factor)?` |
| `base` | `NUMBER \| IDENT \| IDENT '(' args ')' \| '(' expr ')' \| '-' base` |

| Support | Description |
|---|---|
| Numbers | Integers, floats, scientific notation (1e-10) |
| Variables | Alphanumeric names + underscore |
| Function calls | `sin(x)`, `exp(2*x)` |
| Named constants | pi, e |

## 7️⃣ LaTeX Conversion — `src/maths/symbolic/latex.rs`

| Function | Description |
|---|---|
| `expr_to_latex(expr) → String` | Expr → LaTeX |
| `latex_to_expr(input) → Result<Expr, ParseError>` | LaTeX → Expr |

| Supported LaTeX | Conversion |
|---|---|
| `\frac{a}{b}` | → Div(a, b) |
| `x^{n}` | → Pow(x, n) |
| `\sqrt{x}` | → Fn("sqrt", x) |
| `\sin`, `\cos`, `\exp`, `\ln` | → Corresponding Fn |
| `\times`, `\cdot` | → Mul |
| `\approx` | → Ignored |
| `10^{10}` | → Pow(Const(10), Const(10)) |

Useful for interpreting formulas from hypotheses/ .md files.

## 8️⃣ Module Structure

| File | Description |
|---|---|
| `src/maths/symbolic/mod.rs` | mod expr, eval, simplify, differentiate, parse, latex |
| Re-exports | Expr, eval, simplify, differentiate, parse_expr, expr_to_latex, latex_to_expr |
| `src/maths/mod.rs` | Add `pub mod symbolic` |

## 9️⃣ Integration with Polynomial

| Function | Description |
|---|---|
| `Polynomial::to_expr(var)` | Converts coefficients to expression tree |
| `Expr::to_polynomial(var)` | → `Option<Polynomial>` (if the expression is polynomial) |

Existing root solvers remain numerical, but one can differentiate symbolically then evaluate.

## 🔟 Tests

| Test | Description |
|---|---|
| parse_expr | `"2*x^3 + sin(x)"` → verify the tree |
| Differentiation | `differentiate(2*x^3, "x")` → simplify → `6*x^2` |
| Evaluation | Multiple variables |
| Round-trip | Expr → latex → parse → same Expr |
| Simplification | Degenerate cases (0+x, x*1, x/x) |
| LaTeX hypotheses | Parsing formulas from hypotheses/ .md files |
