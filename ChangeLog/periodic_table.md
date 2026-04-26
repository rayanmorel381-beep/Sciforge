# Periodic Table — ChangeLog

---

## v0.0.3

No changes — identical to v0.0.1.

---

## v0.0.2

No changes — identical to v0.0.1.

---

## v0.0.1

### 1️⃣ YAML Data — `tableau-periodique/` — 118 elements, 11 categories

| Category | Directory | Elements | Count |
|---|---|---|---|
| Non-metals | `non-metaux/` | H (1.008), C (12.011), N (14.007), O (15.999), P (30.974), S (32.06), Se (78.971) | 7 |
| Noble gases | `gaz-nobles/` | He (4.003), Ne (20.180), Ar (39.948), Kr (83.798), Xe (131.293), Rn (222.0), Og (294.0) | 7 |
| Halogens | `halogenes/` | F (18.998), Cl (35.45), Br (79.904), I (126.904), At (210.0), Ts (294.0) | 6 |
| Metalloids | `metalloides/` | B (10.81), Si (28.085), Ge (72.630), As (74.922), Sb (121.760), Te (127.60) | 6 |
| Alkali metals | `metaux-alcalins/` | Li (6.941), Na (22.990), K (39.098), Rb (85.468), Cs (132.905), Fr (223.0) | 6 |
| Alkaline earth metals | `metaux-alcalino-terreux/` | Be (9.012), Mg (24.305), Ca (40.078), Sr (87.62), Ba (137.327), Ra (226.0) | 6 |
| Transition metals | `metaux-de-transition/` | Sc, Ti, V, Cr, Mn, Fe, Co, Ni, Cu, Zn, Y, Zr, Nb, Mo, Tc, Ru, Rh, Pd, Ag, Cd, Hf, Ta, W, Re, Os, Ir, Pt, Au, Hg, Rf, Db, Sg, Bh, Hs, Mt, Ds, Rg, Cn | 38 |
| Post-transition metals | `metaux-post-transition/` | Al, Ga, In, Sn, Tl, Pb, Bi, Nh, Fl, Mc, Lv | 11 |
| Lanthanides | `lanthanides/` | La, Ce, Pr, Nd, Pm, Sm, Eu, Gd, Tb, Dy, Ho, Er, Tm, Yb, Lu | 15 |
| Actinides | `actinides/` | Ac, Th, Pa, U, Np, Pu, Am, Cm, Bk, Cf, Es, Fm, Md, No, Lr | 15 |
| Superheavy | `elements-superlourds/` | Og (cross-reference) | 1 |

### 2️⃣ YAML Schema — per-element format

| Field | Type | Example | Description |
|---|---|---|---|
| `symbol` | `&'static str` | `"Fe"` | Element symbol |
| `name` | `&'static str` | `"Iron"` | Element name |
| `atomic_number` | `u32` | `26` | Atomic number $Z$ |
| `atomic_mass` | `f64` | `55.845` | Atomic mass (u) |
| `electronegativity` | `Option<f64>` | `1.83` | Pauling electronegativity |
| `group` | `Option<u32>` | `8` | IUPAC group |
| `period` | `u32` | `4` | Period |

### 3️⃣ Integration

| Component | Path | Description |
|---|---|---|
| Loader | `src/constants/elements.rs` | `include_str!` compile-time loading |
| Parser | `src/parser/yaml/` | Zero-copy `YamlParser` |
| API | `sciforge::constants::elements` | `Vec<Element>` public access |
| Consumers | `benchmark`, `hub` | 118-element simulation and dispatch |
