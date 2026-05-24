## 1️⃣ Current State

| Building Block | Location | Description |
|---|---|---|
| Conversion constants | `src/constants/units.rs` | 12 raw constants (EV_TO_JOULE, CALORIE_TO_JOULE, ATM_TO_PASCAL, etc.) |
| Unit enums | `src/hub/domain/common/units.rs` | 7 separate enums (LengthUnit, MassUnit, TimeUnit, TemperatureUnit, EnergyUnit, PressureUnit, AngleUnit) |
| SI conversion | `src/hub/domain/common/units.rs` | `to_si(value, from) → from_si(value, to)` |
| Utility functions | `src/hub/domain/common/units.rs` | convert_length, convert_mass, convert_time, etc. |

No composite dimensions (m/s², kg·m/s², etc.). No type-safety: a velocity (f64) can be added to a mass (f64) without error.

## 2️⃣ Dimensions de Base — `src/maths/units/dimension.rs`

| Component | Description |
|---|---|
| `Dimension` | `{ length: i8, mass: i8, time: i8, temperature: i8, current: i8, amount: i8, luminosity: i8 }` |

| Constant | Exponents (L, M, T, Θ, I, N, J) | SI Unit |
|---|---|---|
| `DIMENSIONLESS` | (0, 0, 0, 0, 0, 0, 0) | — |
| `LENGTH` | (1, 0, 0, 0, 0, 0, 0) | m |
| `MASS` | (0, 1, 0, 0, 0, 0, 0) | kg |
| `TIME` | (0, 0, 1, 0, 0, 0, 0) | s |
| `VELOCITY` | (1, 0, -1, 0, 0, 0, 0) | m/s |
| `ACCELERATION` | (1, 0, -2, 0, 0, 0, 0) | m/s² |
| `FORCE` | (1, 1, -2, 0, 0, 0, 0) | N = kg·m/s² |
| `ENERGY` | (2, 1, -2, 0, 0, 0, 0) | J = kg·m²/s² |
| `POWER` | (2, 1, -3, 0, 0, 0, 0) | W = kg·m²/s³ |
| `PRESSURE` | (-1, 1, -2, 0, 0, 0, 0) | Pa = kg/(m·s²) |
| `CHARGE` | (0, 0, 1, 0, 1, 0, 0) | C = A·s |
| `FREQUENCY` | (0, 0, -1, 0, 0, 0, 0) | Hz = 1/s |

| Operation | Description |
|---|---|
| `mul(a, b)` | Exponent addition |
| `div(a, b)` | Exponent subtraction |
| `pow(d, n)` | Exponent multiplication by n |
| `is_compatible(a, b)` | true if exponents are identical |
| `impl Display` | Formatage "kg·m²·s⁻²" |

## 3️⃣ Quantity — `src/maths/units/quantity.rs`

| Component | Description |
|---|---|
| `Quantity` | `{ value: f64, dimension: Dimension }` |
| `Quantity::new(value, dimension)` | Constructor |
| `Quantity::dimensionless(value)` | Dimensionless value |

| Operation | Rule | Condition |
|---|---|---|
| `Add / Sub` | value + value | Identical dimensions, otherwise DimensionError |
| `Mul` | value × value, dimension.mul(dimension) | Always valid |
| `Div` | value / value, dimension.div(dimension) | Always valid |
| `Pow(n: i8)` | value.powi(n), dimension.pow(n) | Always valid |
| `Neg` | -value, same dimension | Always valid |
| `Sqrt` | value.sqrt(), exponents / 2 | Even exponents required |
| Comparisons | Partial ordering | Identical dimensions, otherwise error |
| `impl Display` | "9.81 m·s⁻²" | — |

## 4️⃣ Erreurs — `src/maths/units/error.rs`

| Variant | Description |
|---|---|
| `IncompatibleDimensions { expected, found }` | Incompatible dimensions for Add/Sub/comparison |
| `NonIntegerExponent` | Non-integer exponent after Sqrt |
| `DivisionByZero` | Division by zero |

## 5️⃣ Constructeurs SI — `src/maths/units/si.rs`

| Function | Dimension | SI Unit |
|---|---|---|
| `meters(v)` | LENGTH | m |
| `kilograms(v)` | MASS | kg |
| `seconds(v)` | TIME | s |
| `kelvin(v)` | TEMPERATURE | K |
| `amperes(v)` | CURRENT | A |
| `moles(v)` | AMOUNT | mol |
| `candelas(v)` | LUMINOSITY | cd |
| `newtons(v)` | FORCE | N |
| `joules(v)` | ENERGY | J |
| `watts(v)` | POWER | W |
| `pascals(v)` | PRESSURE | Pa |
| `hertz(v)` | FREQUENCY | Hz |
| `meters_per_second(v)` | VELOCITY | m/s |
| `meters_per_second_squared(v)` | ACCELERATION | m/s² |

## 6️⃣ Conversion — `src/maths/units/convert.rs`

| Function | Description |
|---|---|
| `convert(qty, from_scale, to_scale)` | Generic conversion by scale factor |
| `from_length(v, unit: LengthUnit)` | Converts to SI then wraps in Quantity |
| `from_mass(v, unit: MassUnit)` | Same for mass |
| `from_time(v, unit: TimeUnit)` | Same for time |
| `from_energy(v, unit: EnergyUnit)` | Same for energy |
| `from_pressure(v, unit: PressureUnit)` | Same for pressure |
| `to_unit(qty, unit: LengthUnit) → f64` | Extracts value in target unit |

## 7️⃣ Module Structure

| File | Description |
|---|---|
| `src/maths/units/mod.rs` | mod dimension, quantity, error, si, convert |
| Re-exports | Dimension, Quantity, DimensionError, constants, SI constructors |
| `src/maths/mod.rs` | Add `pub mod units` |

## 8️⃣ Integration with Existing Constants

| Constante | Quantity | Dimension |
|---|---|---|
| `G_QUANTITY` | `Quantity::new(G, ...)` | m³·kg⁻¹·s⁻² |
| `C_QUANTITY` | `Quantity::new(C, VELOCITY)` | m/s |
| `K_B_QUANTITY` | `Quantity::new(K_B, ENERGY/TEMPERATURE)` | J/K |

Optional module: `src/constants/quantities.rs` for dimensioned versions.

## 9️⃣ Tests

| Test | Description |
|---|---|
| Valid addition | `meters(5.0) + meters(3.0) = meters(8.0)` |
| Addition invalide | `meters(5.0) + kilograms(3.0) → DimensionError` |
| F = m × a | `kilograms(10.0) * meters_per_second_squared(9.81)` → FORCE, valeur 98.1 |
| E = mc² | Verify ENERGY dimension |
| Round-trip | Conversions with existing enums |
| Sqrt | `sqrt(meters(4.0) * meters(4.0)) → meters(4.0)` via dimension check |
