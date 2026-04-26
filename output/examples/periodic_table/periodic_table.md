# Periodic Table of Elements — Data from IUPAC, 2024 edition

118 benchmark entries across 5 formats.

---

## Grid
```
  nonmetal        :   H    C    N    O    P    S   Se
  noble gas       :  He   Ne   Ar   Kr   Xe   Rn
  alkali metal    :  Li   Na    K   Rb   Cs   Fr
  alkaline earth metal:  Be   Mg   Ca   Sr   Ba   Ra
  metalloid       :   B   Si   Ge   As   Sb   Te
  halogen         :   F   Cl   Br    I   At   Ts
  post-transition metal:  Al   Ga   In   Sn   Tl   Pb   Bi   Po
  transition metal:  Sc   Ti    V   Cr   Mn   Fe   Co   Ni   Cu   Zn    Y   Zr   Nb   Mo   Tc   Ru   Rh   Pd   Ag   Cd   Hf   Ta    W   Re   Os   Ir   Pt   Au   Hg
  lanthanide      :  La   Ce   Pr   Nd   Pm   Sm   Eu   Gd   Tb   Dy   Ho   Er   Tm   Yb   Lu
  actinide        :  Ac   Th   Pa    U   Np   Pu   Am   Cm   Bk   Cf   Es   Fm   Md   No   Lr
  transition_metal:  Rf   Db   Sg   Bh   Hs
  unknown         :  Mt   Ds   Rg   Cn   Nh   Fl   Mc   Lv
  noble gas (predicted):  Og
```
---

## Directory Layout
```
csv/
  all.csv
json/
  {label}.json
  summary.json
yaml/
  {label}.yaml
  summary.yaml
toml/
  {label}.toml
  summary.toml
bmk/
  {label}.bmk
benchmark.html
benchmark.md
```

## Results
| Label | category | symbol | name | config | mass | Precision | Iterations | Avg (ns) | Iters/s |
|---|---|---|---|---|---|---|---|---|---|
| 001_H | nonmetal | H | Hydrogen | 1s¹ | 1.008 | f32 | 1000 | 95.70 | 0.00 |
| 002_He | noble gas | He | Helium | 1s² | 4.002602 | f64 | 5000 | 194.93 | 0.00 |
| 003_Li | alkali metal | Li | Lithium | 1s² 2s¹ | 6.941 | f32 | 10000 | 265.72 | 5000000.00 |
| 004_Be | alkaline earth metal | Be | Beryllium | 1s² 2s² | 9.0121831 | f64 | 1000 | 346.21 | 0.00 |
| 005_B | metalloid | B | Boron | 1s² 2s² 2p¹ | 10.81 | f32 | 5000 | 463.13 | 2500000.00 |
| 006_C | nonmetal | C | Carbon | 1s² 2s² 2p² | 12.011 | f64 | 10000 | 514.79 | 2000000.00 |
| 007_N | nonmetal | N | Nitrogen | 1s² 2s² 2p³ | 14.007 | f32 | 1000 | 557.65 | 0.00 |
| 008_O | nonmetal | O | Oxygen | 1s² 2s² 2p⁴ | 15.999 | f64 | 5000 | 639.57 | 1666666.67 |
| 009_F | halogen | F | Fluorine | 1s² 2s² 2p⁵ | 18.998403 | f32 | 10000 | 804.65 | 1250000.00 |
| 010_Ne | noble gas | Ne | Neon | 1s² 2s² 2p⁶ | 20.1797 | f64 | 1000 | 785.88 | 0.00 |
| 011_Na | alkali metal | Na | Sodium | [Ne] 3s¹ | 22.98976928 | f32 | 5000 | 901.85 | 1250000.00 |
| 012_Mg | alkaline earth metal | Mg | Magnesium | [Ne] 3s² | 24.305 | f64 | 10000 | 991.70 | 1111111.11 |
| 013_Al | post-transition metal | Al | Aluminium | [Ne] 3s² 3p¹ | 26.9815385 | f32 | 1000 | 1041.63 | 1000000.00 |
| 014_Si | metalloid | Si | Silicon | [Ne] 3s² 3p² | 28.085 | f64 | 5000 | 1108.43 | 1000000.00 |
| 015_P | nonmetal | P | Phosphorus | [Ne] 3s² 3p³ | 30.973762 | f32 | 10000 | 1193.05 | 909090.91 |
| 016_S | nonmetal | S | Sulfur | [Ne] 3s² 3p⁴ | 32.06 | f64 | 1000 | 1291.49 | 1000000.00 |
| 017_Cl | halogen | Cl | Chlorine | [Ne] 3s² 3p⁵ | 35.45 | f32 | 5000 | 1321.72 | 833333.33 |
| 018_Ar | noble gas | Ar | Argon | [Ne] 3s² 3p⁶ | 39.948 | f64 | 10000 | 1447.26 | 714285.71 |
| 019_K | alkali metal | K | Potassium | [Ar] 4s¹ | 39.0983 | f32 | 1000 | 1459.22 | 1000000.00 |
| 020_Ca | alkaline earth metal | Ca | Calcium | [Ar] 4s² | 40.078 | f64 | 5000 | 1625.94 | 625000.00 |
| 021_Sc | transition metal | Sc | Scandium | [Ar] 3d¹ 4s² | 44.955908 | f32 | 10000 | 1643.54 | 625000.00 |
| 022_Ti | transition metal | Ti | Titanium | [Ar] 3d² 4s² | 47.867 | f64 | 1000 | 1805.70 | 1000000.00 |
| 023_V | transition metal | V | Vanadium | [Ar] 3d³ 4s² | 50.9415 | f32 | 5000 | 1804.14 | 555555.56 |
| 024_Cr | transition metal | Cr | Chromium | [Ar] 3d⁵ 4s¹ | 51.9961 | f64 | 10000 | 1871.85 | 555555.56 |
| 025_Mn | transition metal | Mn | Manganese | [Ar] 3d⁵ 4s² | 54.938044 | f32 | 1000 | 1950.00 | 1000000.00 |
| 026_Fe | transition metal | Fe | Iron | [Ar] 3d⁶ 4s² | 55.845 | f64 | 5000 | 2007.70 | 500000.00 |
| 027_Co | transition metal | Co | Cobalt | [Ar] 3d⁷ 4s² | 58.933194 | f32 | 10000 | 2112.24 | 476190.48 |
| 028_Ni | transition metal | Ni | Nickel | [Ar] 3d⁸ 4s² | 58.6934 | f64 | 1000 | 2195.53 | 500000.00 |
| 029_Cu | transition metal | Cu | Copper | [Ar] 3d¹⁰ 4s¹ | 63.546 | f32 | 5000 | 2287.10 | 454545.45 |
| 030_Zn | transition metal | Zn | Zinc | [Ar] 3d¹⁰ 4s² | 65.38 | f64 | 10000 | 2340.08 | 434782.61 |
| 031_Ga | post-transition metal | Ga | Gallium | [Ar] 3d¹⁰ 4s² 4p¹ | 69.723 | f32 | 1000 | 2465.75 | 500000.00 |
| 032_Ge | metalloid | Ge | Germanium | [Ar] 3d¹⁰ 4s² 4p² | 72.63 | f64 | 5000 | 2456.93 | 416666.67 |
| 033_As | metalloid | As | Arsenic | [Ar] 3d¹⁰ 4s² 4p³ | 74.921595 | f32 | 10000 | 2569.77 | 400000.00 |
| 034_Se | nonmetal | Se | Selenium | [Ar] 3d¹⁰ 4s² 4p⁴ | 78.971 | f64 | 1000 | 2641.96 | 500000.00 |
| 035_Br | halogen | Br | Bromine | [Ar] 3d¹⁰ 4s² 4p⁵ | 79.904 | f32 | 5000 | 2709.80 | 384615.38 |
| 036_Kr | noble gas | Kr | Krypton | [Ar] 3d¹⁰ 4s² 4p⁶ | 83.798 | f64 | 10000 | 2877.84 | 357142.86 |
| 037_Rb | alkali metal | Rb | Rubidium | [Kr] 5s¹ | 85.4678 | f32 | 1000 | 2849.42 | 500000.00 |
| 038_Sr | alkaline earth metal | Sr | Strontium | [Kr] 5s² | 87.62 | f64 | 5000 | 2973.56 | 357142.86 |
| 039_Y | transition metal | Y | Yttrium | [Kr] 4d¹ 5s² | 88.90584 | f32 | 10000 | 3136.35 | 322580.65 |
| 040_Zr | transition metal | Zr | Zirconium | [Kr] 4d² 5s² | 91.224 | f64 | 1000 | 3056.81 | 333333.33 |
| 041_Nb | transition metal | Nb | Niobium | [Kr] 4d⁴ 5s¹ | 92.90637 | f32 | 5000 | 3230.24 | 312500.00 |
| 042_Mo | transition metal | Mo | Molybdenum | [Kr] 4d⁵ 5s¹ | 95.95 | f64 | 10000 | 3338.47 | 303030.30 |
| 043_Tc | transition metal | Tc | Technetium | [Kr] 4d⁵ 5s² | 97.907216 | f32 | 1000 | 3412.72 | 333333.33 |
| 044_Ru | transition metal | Ru | Ruthenium | [Kr] 4d⁷ 5s¹ | 101.07 | f64 | 5000 | 3504.29 | 294117.65 |
| 045_Rh | transition metal | Rh | Rhodium | [Kr] 4d⁸ 5s¹ | 102.9055 | f32 | 10000 | 3583.89 | 285714.29 |
| 046_Pd | transition metal | Pd | Palladium | [Kr] 4d¹⁰ | 106.42 | f64 | 1000 | 3616.91 | 333333.33 |
| 047_Ag | transition metal | Ag | Silver | [Kr] 4d¹⁰ 5s¹ | 107.8682 | f32 | 5000 | 3690.34 | 277777.78 |
| 048_Cd | transition metal | Cd | Cadmium | [Kr] 4d¹⁰ 5s² | 112.414 | f64 | 10000 | 3763.21 | 270270.27 |
| 049_In | post-transition metal | In | Indium | [Kr] 4d¹⁰ 5s² 5p¹ | 114.818 | f32 | 1000 | 3738.75 | 333333.33 |
| 050_Sn | post-transition metal | Sn | Tin | [Kr] 4d¹⁰ 5s² 5p² | 118.71 | f64 | 5000 | 3923.69 | 263157.89 |
| 051_Sb | metalloid | Sb | Antimony | [Kr] 4d¹⁰ 5s² 5p³ | 121.76 | f32 | 10000 | 3969.11 | 256410.26 |
| 052_Te | metalloid | Te | Tellurium | [Kr] 4d¹⁰ 5s² 5p⁴ | 127.6 | f64 | 1000 | 4146.63 | 250000.00 |
| 053_I | halogen | I | Iodine | [Kr] 4d¹⁰ 5s² 5p⁵ | 126.90447 | f32 | 5000 | 4227.59 | 238095.24 |
| 054_Xe | noble gas | Xe | Xenon | [Kr] 4d¹⁰ 5s² 5p⁶ | 131.293 | f64 | 10000 | 4253.64 | 238095.24 |
| 055_Cs | alkali metal | Cs | Cesium | [Xe] 6s¹ | 132.90545196 | f32 | 1000 | 4203.19 | 250000.00 |
| 056_Ba | alkaline earth metal | Ba | Barium | [Xe] 6s² | 137.327 | f64 | 5000 | 4674.12 | 217391.30 |
| 057_La | lanthanide | La | Lanthanum | [Xe] 5d¹ 6s² | 138.90547 | f32 | 10000 | 4510.37 | 222222.22 |
| 058_Ce | lanthanide | Ce | Cerium | [Xe] 4f¹ 5d¹ 6s² | 140.116 | f64 | 1000 | 4563.78 | 250000.00 |
| 059_Pr | lanthanide | Pr | Praseodymium | [Xe] 4f³ 6s² | 140.90766 | f32 | 5000 | 4661.17 | 217391.30 |
| 060_Nd | lanthanide | Nd | Neodymium | [Xe] 4f⁴ 6s² | 144.242 | f64 | 10000 | 4894.73 | 208333.33 |
| 061_Pm | lanthanide | Pm | Promethium | [Xe] 4f⁵ 6s² | 144.912749 | f32 | 1000 | 4768.08 | 250000.00 |
| 062_Sm | lanthanide | Sm | Samarium | [Xe] 4f⁶ 6s² | 150.36 | f64 | 5000 | 5493.32 | 185185.19 |
| 063_Eu | lanthanide | Eu | Europium | [Xe] 4f⁷ 6s² | 151.964 | f32 | 10000 | 5127.62 | 196078.43 |
| 064_Gd | lanthanide | Gd | Gadolinium | [Xe] 4f⁷ 5d¹ 6s² | 157.25 | f64 | 1000 | 4909.90 | 250000.00 |
| 065_Tb | lanthanide | Tb | Terbium | [Xe] 4f⁹ 6s² | 158.92535 | f32 | 5000 | 5044.69 | 200000.00 |
| 066_Dy | lanthanide | Dy | Dysprosium | [Xe] 4f¹⁰ 6s² | 162.5 | f64 | 10000 | 5624.03 | 178571.43 |
| 067_Ho | lanthanide | Ho | Holmium | [Xe] 4f¹¹ 6s² | 164.93033 | f32 | 1000 | 5394.09 | 200000.00 |
| 068_Er | lanthanide | Er | Erbium | [Xe] 4f¹² 6s² | 167.259 | f64 | 5000 | 5426.33 | 185185.19 |
| 069_Tm | lanthanide | Tm | Thulium | [Xe] 4f¹³ 6s² | 168.93422 | f32 | 10000 | 5381.33 | 188679.25 |
| 070_Yb | lanthanide | Yb | Ytterbium | [Xe] 4f¹⁴ 6s² | 173.045 | f64 | 1000 | 5344.40 | 200000.00 |
| 071_Lu | lanthanide | Lu | Lutetium | [Xe] 4f¹⁴ 5d¹ 6s² | 174.9668 | f32 | 5000 | 5551.34 | 185185.19 |
| 072_Hf | transition metal | Hf | Hafnium | [Xe] 4f¹⁴ 5d² 6s² | 178.49 | f64 | 10000 | 5876.18 | 172413.79 |
| 073_Ta | transition metal | Ta | Tantalum | [Xe] 4f¹⁴ 5d³ 6s² | 180.94788 | f32 | 1000 | 5761.57 | 200000.00 |
| 074_W | transition metal | W | Tungsten | [Xe] 4f¹⁴ 5d⁴ 6s² | 183.84 | f64 | 5000 | 5828.79 | 172413.79 |
| 075_Re | transition metal | Re | Rhenium | [Xe] 4f¹⁴ 5d⁵ 6s² | 186.207 | f32 | 10000 | 5931.54 | 169491.53 |
| 076_Os | transition metal | Os | Osmium | [Xe] 4f¹⁴ 5d⁶ 6s² | 190.23 | f64 | 1000 | 6005.12 | 166666.67 |
| 077_Ir | transition metal | Ir | Iridium | [Xe] 4f¹⁴ 5d⁷ 6s² | 192.217 | f32 | 5000 | 6176.86 | 166666.67 |
| 078_Pt | transition metal | Pt | Platinum | [Xe] 4f¹⁴ 5d⁹ 6s¹ | 195.084 | f64 | 10000 | 6110.85 | 163934.43 |
| 079_Au | transition metal | Au | Gold | [Xe] 4f¹⁴ 5d¹⁰ 6s¹ | 196.966569 | f32 | 1000 | 6071.55 | 166666.67 |
| 080_Hg | transition metal | Hg | Mercury | [Xe] 4f¹⁴ 5d¹⁰ 6s² | 200.592 | f64 | 5000 | 6237.34 | 161290.32 |
| 081_Tl | post-transition metal | Tl | Thallium | [Xe] 4f¹⁴ 5d¹⁰ 6s² 6p¹ | 204.38 | f32 | 10000 | 6636.80 | 151515.15 |
| 082_Pb | post-transition metal | Pb | Lead | [Xe] 4f¹⁴ 5d¹⁰ 6s² 6p² | 207.2 | f64 | 1000 | 6287.12 | 166666.67 |
| 083_Bi | post-transition metal | Bi | Bismuth | [Xe] 4f¹⁴ 5d¹⁰ 6s² 6p³ | 208.9804 | f32 | 5000 | 6630.27 | 151515.15 |
| 084_Po | post-transition metal | Po | Polonium | [Xe] 4f¹⁴ 5d¹⁰ 6s² 6p⁴ | 208.98243 | f64 | 10000 | 6550.27 | 153846.15 |
| 085_At | halogen | At | Astatine | [Xe] 4f¹⁴ 5d¹⁰ 6s² 6p⁵ | 210 | f32 | 1000 | 6666.44 | 166666.67 |
| 086_Rn | noble gas | Rn | Radon | [Xe] 4f¹⁴ 5d¹⁰ 6s² 6p⁶ | 222 | f64 | 5000 | 6607.90 | 151515.15 |
| 087_Fr | alkali metal | Fr | Francium | [Rn] 7s¹ | 223 | f32 | 10000 | 6706.94 | 149253.73 |
| 088_Ra | alkaline earth metal | Ra | Radium | [Rn] 7s² | 226 | f64 | 1000 | 6715.18 | 166666.67 |
| 089_Ac | actinide | Ac | Actinium | [Rn] 6d¹ 7s² | 227.027752 | f32 | 5000 | 6847.07 | 147058.82 |
| 090_Th | actinide | Th | Thorium | [Rn] 6d² 7s² | 232.0377 | f64 | 10000 | 7229.11 | 138888.89 |
| 091_Pa | actinide | Pa | Protactinium | [Rn] 5f² 6d¹ 7s² | 231.03588 | f32 | 1000 | 7288.52 | 142857.14 |
| 092_U | actinide | U | Uranium | [Rn] 5f³ 6d¹ 7s² | 238.02891 | f64 | 5000 | 7130.27 | 142857.14 |
| 093_Np | actinide | Np | Neptunium | [Rn] 5f⁴ 6d¹ 7s² | 237.048174 | f32 | 10000 | 7185.77 | 140845.07 |
| 094_Pu | actinide | Pu | Plutonium | [Rn] 5f⁶ 7s² | 244.064204 | f64 | 1000 | 7347.30 | 142857.14 |
| 095_Am | actinide | Am | Americium | [Rn] 5f⁷ 7s² | 243.061381 | f32 | 5000 | 7340.23 | 138888.89 |
| 096_Cm | actinide | Cm | Curium | [Rn] 5f⁷ 6d¹ 7s² | 247.070354 | f64 | 10000 | 7677.89 | 131578.95 |
| 097_Bk | actinide | Bk | Berkelium | [Rn] 5f⁹ 7s² | 247.070307 | f32 | 1000 | 7817.07 | 142857.14 |
| 098_Cf | actinide | Cf | Californium | [Rn] 5f¹⁰ 7s² | 251.079587 | f64 | 5000 | 7892.61 | 128205.13 |
| 099_Es | actinide | Es | Einsteinium | [Rn] 5f¹¹ 7s² | 252.08298 | f32 | 10000 | 7817.05 | 128205.13 |
| 100_Fm | actinide | Fm | Fermium | [Rn] 5f¹² 7s² | 257.095105 | f64 | 1000 | 8230.76 | 125000.00 |
| 101_Md | actinide | Md | Mendelevium | [Rn] 5f¹³ 7s² | 258.098431 | f32 | 5000 | 8117.14 | 125000.00 |
| 102_No | actinide | No | Nobelium | [Rn] 5f¹⁴ 7s² | 259.10103 | f64 | 10000 | 8344.92 | 120481.93 |
| 103_Lr | actinide | Lr | Lawrencium | [Rn] 5f¹⁴ 7s² 7p¹ | 266.12 | f32 | 1000 | 8910.24 | 125000.00 |
| 104_Rf | transition_metal | Rf | Rutherfordium | [Rn] 5f¹⁴ 6d² 7s² | 267.122 | f64 | 5000 | 8290.02 | 121951.22 |
| 105_Db | transition_metal | Db | Dubnium | [Rn] 5f¹⁴ 6d³ 7s² | 268.126 | f32 | 10000 | 8299.34 | 121951.22 |
| 106_Sg | transition_metal | Sg | Seaborgium | [Rn] 5f¹⁴ 6d⁴ 7s² | 269.129 | f64 | 1000 | 8248.20 | 125000.00 |
| 107_Bh | transition_metal | Bh | Bohrium | [Rn] 5f¹⁴ 6d⁵ 7s² | 270.134 | f32 | 5000 | 8444.88 | 119047.62 |
| 108_Hs | transition_metal | Hs | Hassium | [Rn] 5f¹⁴ 6d⁶ 7s² | 269.134 | f64 | 10000 | 8561.10 | 117647.06 |
| 109_Mt | unknown | Mt | Meitnerium | [Rn] 5f¹⁴ 6d⁷ 7s² | 278.156 | f32 | 1000 | 8466.37 | 125000.00 |
| 110_Ds | unknown | Ds | Darmstadtium | [Rn] 5f¹⁴ 6d⁸ 7s² | 281.165 | f64 | 5000 | 8539.74 | 119047.62 |
| 111_Rg | unknown | Rg | Roentgenium | [Rn] 5f¹⁴ 6d⁹ 7s² | 282.169 | f32 | 10000 | 8503.51 | 117647.06 |
| 112_Cn | unknown | Cn | Copernicium | [Rn] 5f¹⁴ 6d¹⁰ 7s² | 285.177 | f64 | 1000 | 10129.10 | 100000.00 |
| 113_Nh | unknown | Nh | Nihonium | [Rn] 5f¹⁴ 6d¹⁰ 7s² 7p¹ | 286.182 | f32 | 5000 | 8676.41 | 116279.07 |
| 114_Fl | unknown | Fl | Flerovium | [Rn] 5f¹⁴ 6d¹⁰ 7s² 7p² | 289.19 | f64 | 10000 | 8943.08 | 112359.55 |
| 115_Mc | unknown | Mc | Moscovium | [Rn] 5f¹⁴ 6d¹⁰ 7s² 7p³ | 290.196 | f32 | 1000 | 8776.39 | 125000.00 |
| 116_Lv | unknown | Lv | Livermorium | [Rn] 5f¹⁴ 6d¹⁰ 7s² 7p⁴ | 293.205 | f64 | 5000 | 8886.08 | 113636.36 |
| 117_Ts | halogen | Ts | Tennessine | [Rn] 5f¹⁴ 6d¹⁰ 7s² 7p⁵ (predicted) | 294 | f32 | 10000 | 9014.73 | 111111.11 |
| 118_Og | noble gas (predicted) | Og | Oganesson | [Rn] 5f¹⁴ 6d¹⁰ 7s² 7p⁶ (predicted) | 294 | f64 | 1000 | 9093.12 | 111111.11 |
## Analytics
### Category Distribution
| Category | Count | Avg (ns) | Min (ns) | Max (ns) |
|---|---|---|---|---|
| nonmetal | 7 | 990.60 | 95.70 | 2641.96 |
| noble gas | 6 | 2694.57 | 194.93 | 6607.90 |
| alkali metal | 6 | 2731.06 | 265.72 | 6706.94 |
| alkaline earth metal | 6 | 2887.79 | 346.21 | 6715.18 |
| metalloid | 6 | 2452.33 | 463.13 | 4146.63 |
| halogen | 6 | 4124.16 | 804.65 | 9014.73 |
| post-transition metal | 8 | 4659.29 | 1041.63 | 6636.80 |
| transition metal | 29 | 3736.24 | 1643.54 | 6237.34 |
| lanthanide | 15 | 5113.01 | 4510.37 | 5624.03 |
| actinide | 15 | 7678.40 | 6847.07 | 8910.24 |
| transition_metal | 5 | 8368.71 | 8248.20 | 8561.10 |
| unknown | 8 | 8865.09 | 8466.37 | 10129.10 |
| noble gas (predicted) | 1 | 9093.12 | 9093.12 | 9093.12 |

### Top 10 Fastest Entries
| # | Entry | Avg (ns) |
|---|---|---|
| 1 | 001_H | 95.70 |
| 2 | 002_He | 194.93 |
| 3 | 003_Li | 265.72 |
| 4 | 004_Be | 346.21 |
| 5 | 005_B | 463.13 |
| 6 | 006_C | 514.79 |
| 7 | 007_N | 557.65 |
| 8 | 008_O | 639.57 |
| 9 | 010_Ne | 785.88 |
| 10 | 009_F | 804.65 |

### Top 10 Slowest Entries
| # | Entry | Avg (ns) |
|---|---|---|
| 1 | 112_Cn | 10129.10 |
| 2 | 118_Og | 9093.12 |
| 3 | 117_Ts | 9014.73 |
| 4 | 114_Fl | 8943.08 |
| 5 | 103_Lr | 8910.24 |
| 6 | 116_Lv | 8886.08 |
| 7 | 115_Mc | 8776.39 |
| 8 | 113_Nh | 8676.41 |
| 9 | 108_Hs | 8561.10 |
| 10 | 110_Ds | 8539.74 |

