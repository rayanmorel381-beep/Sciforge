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
| 001_H | nonmetal | H | Hydrogen | 1s¹ | 1.008 | f32 | 1000 | 90.48 | 0.00 |
| 002_He | noble gas | He | Helium | 1s² | 4.002602 | f64 | 5000 | 169.05 | 0.00 |
| 003_Li | alkali metal | Li | Lithium | 1s² 2s¹ | 6.941 | f32 | 10000 | 249.70 | 5000000.00 |
| 004_Be | alkaline earth metal | Be | Beryllium | 1s² 2s² | 9.0121831 | f64 | 1000 | 318.35 | 0.00 |
| 005_B | metalloid | B | Boron | 1s² 2s² 2p¹ | 10.81 | f32 | 5000 | 402.31 | 2500000.00 |
| 006_C | nonmetal | C | Carbon | 1s² 2s² 2p² | 12.011 | f64 | 10000 | 471.10 | 2500000.00 |
| 007_N | nonmetal | N | Nitrogen | 1s² 2s² 2p³ | 14.007 | f32 | 1000 | 551.92 | 0.00 |
| 008_O | nonmetal | O | Oxygen | 1s² 2s² 2p⁴ | 15.999 | f64 | 5000 | 620.61 | 1666666.62 |
| 009_F | halogen | F | Fluorine | 1s² 2s² 2p⁵ | 18.998403 | f32 | 10000 | 697.91 | 1666666.62 |
| 010_Ne | noble gas | Ne | Neon | 1s² 2s² 2p⁶ | 20.1797 | f64 | 1000 | 775.56 | 0.00 |
| 011_Na | alkali metal | Na | Sodium | [Ne] 3s¹ | 22.98976928 | f32 | 5000 | 846.23 | 1250000.00 |
| 012_Mg | alkaline earth metal | Mg | Magnesium | [Ne] 3s² | 24.305 | f64 | 10000 | 926.44 | 1111111.12 |
| 013_Al | post-transition metal | Al | Aluminium | [Ne] 3s² 3p¹ | 26.9815385 | f32 | 1000 | 1010.41 | 1000000.00 |
| 014_Si | metalloid | Si | Silicon | [Ne] 3s² 3p² | 28.085 | f64 | 5000 | 1140.37 | 1000000.00 |
| 015_P | nonmetal | P | Phosphorus | [Ne] 3s² 3p³ | 30.973762 | f32 | 10000 | 1160.05 | 909090.94 |
| 016_S | nonmetal | S | Sulfur | [Ne] 3s² 3p⁴ | 32.06 | f64 | 1000 | 1249.86 | 1000000.00 |
| 017_Cl | halogen | Cl | Chlorine | [Ne] 3s² 3p⁵ | 35.45 | f32 | 5000 | 1319.70 | 833333.31 |
| 018_Ar | noble gas | Ar | Argon | [Ne] 3s² 3p⁶ | 39.948 | f64 | 10000 | 1392.33 | 769230.75 |
| 019_K | alkali metal | K | Potassium | [Ar] 4s¹ | 39.0983 | f32 | 1000 | 1526.03 | 1000000.00 |
| 020_Ca | alkaline earth metal | Ca | Calcium | [Ar] 4s² | 40.078 | f64 | 5000 | 1662.09 | 625000.00 |
| 021_Sc | transition metal | Sc | Scandium | [Ar] 3d¹ 4s² | 44.955908 | f32 | 10000 | 1636.08 | 625000.00 |
| 022_Ti | transition metal | Ti | Titanium | [Ar] 3d² 4s² | 47.867 | f64 | 1000 | 1694.40 | 1000000.00 |
| 023_V | transition metal | V | Vanadium | [Ar] 3d³ 4s² | 50.9415 | f32 | 5000 | 1794.18 | 625000.00 |
| 024_Cr | transition metal | Cr | Chromium | [Ar] 3d⁵ 4s¹ | 51.9961 | f64 | 10000 | 1952.34 | 526315.81 |
| 025_Mn | transition metal | Mn | Manganese | [Ar] 3d⁵ 4s² | 54.938044 | f32 | 1000 | 1907.84 | 1000000.00 |
| 026_Fe | transition metal | Fe | Iron | [Ar] 3d⁶ 4s² | 55.845 | f64 | 5000 | 2015.35 | 500000.00 |
| 027_Co | transition metal | Co | Cobalt | [Ar] 3d⁷ 4s² | 58.933194 | f32 | 10000 | 2080.07 | 500000.00 |
| 028_Ni | transition metal | Ni | Nickel | [Ar] 3d⁸ 4s² | 58.6934 | f64 | 1000 | 2148.77 | 500000.00 |
| 029_Cu | transition metal | Cu | Copper | [Ar] 3d¹⁰ 4s¹ | 63.546 | f32 | 5000 | 2224.89 | 454545.47 |
| 030_Zn | transition metal | Zn | Zinc | [Ar] 3d¹⁰ 4s² | 65.38 | f64 | 10000 | 2326.91 | 434782.59 |
| 031_Ga | post-transition metal | Ga | Gallium | [Ar] 3d¹⁰ 4s² 4p¹ | 69.723 | f32 | 1000 | 2456.66 | 500000.00 |
| 032_Ge | metalloid | Ge | Germanium | [Ar] 3d¹⁰ 4s² 4p² | 72.63 | f64 | 5000 | 2461.49 | 416666.66 |
| 033_As | metalloid | As | Arsenic | [Ar] 3d¹⁰ 4s² 4p³ | 74.921595 | f32 | 10000 | 2543.32 | 400000.00 |
| 034_Se | nonmetal | Se | Selenium | [Ar] 3d¹⁰ 4s² 4p⁴ | 78.971 | f64 | 1000 | 2592.98 | 500000.00 |
| 035_Br | halogen | Br | Bromine | [Ar] 3d¹⁰ 4s² 4p⁵ | 79.904 | f32 | 5000 | 2714.75 | 384615.38 |
| 036_Kr | noble gas | Kr | Krypton | [Ar] 3d¹⁰ 4s² 4p⁶ | 83.798 | f64 | 10000 | 2785.39 | 370370.38 |
| 037_Rb | alkali metal | Rb | Rubidium | [Kr] 5s¹ | 85.4678 | f32 | 1000 | 2895.13 | 500000.00 |
| 038_Sr | alkaline earth metal | Sr | Strontium | [Kr] 5s² | 87.62 | f64 | 5000 | 2917.53 | 357142.84 |
| 039_Y | transition metal | Y | Yttrium | [Kr] 4d¹ 5s² | 88.90584 | f32 | 10000 | 3073.45 | 333333.34 |
| 040_Zr | transition metal | Zr | Zirconium | [Kr] 4d² 5s² | 91.224 | f64 | 1000 | 3129.57 | 333333.34 |
| 041_Nb | transition metal | Nb | Niobium | [Kr] 4d⁴ 5s¹ | 92.90637 | f32 | 5000 | 3122.35 | 333333.34 |
| 042_Mo | transition metal | Mo | Molybdenum | [Kr] 4d⁵ 5s¹ | 95.95 | f64 | 10000 | 3299.01 | 312500.00 |
| 043_Tc | transition metal | Tc | Technetium | [Kr] 4d⁵ 5s² | 97.907216 | f32 | 1000 | 3284.76 | 333333.34 |
| 044_Ru | transition metal | Ru | Ruthenium | [Kr] 4d⁷ 5s¹ | 101.07 | f64 | 5000 | 3692.70 | 277777.78 |
| 045_Rh | transition metal | Rh | Rhodium | [Kr] 4d⁸ 5s¹ | 102.9055 | f32 | 10000 | 3461.99 | 294117.66 |
| 046_Pd | transition metal | Pd | Palladium | [Kr] 4d¹⁰ | 106.42 | f64 | 1000 | 3541.52 | 333333.34 |
| 047_Ag | transition metal | Ag | Silver | [Kr] 4d¹⁰ 5s¹ | 107.8682 | f32 | 5000 | 3631.66 | 277777.78 |
| 048_Cd | transition metal | Cd | Cadmium | [Kr] 4d¹⁰ 5s² | 112.414 | f64 | 10000 | 3665.32 | 277777.78 |
| 049_In | post-transition metal | In | Indium | [Kr] 4d¹⁰ 5s² 5p¹ | 114.818 | f32 | 1000 | 4352.59 | 250000.00 |
| 050_Sn | post-transition metal | Sn | Tin | [Kr] 4d¹⁰ 5s² 5p² | 118.71 | f64 | 5000 | 3820.33 | 263157.91 |
| 051_Sb | metalloid | Sb | Antimony | [Kr] 4d¹⁰ 5s² 5p³ | 121.76 | f32 | 10000 | 3933.09 | 256410.25 |
| 052_Te | metalloid | Te | Tellurium | [Kr] 4d¹⁰ 5s² 5p⁴ | 127.6 | f64 | 1000 | 3963.67 | 333333.34 |
| 053_I | halogen | I | Iodine | [Kr] 4d¹⁰ 5s² 5p⁵ | 126.90447 | f32 | 5000 | 4075.38 | 250000.00 |
| 054_Xe | noble gas | Xe | Xenon | [Kr] 4d¹⁰ 5s² 5p⁶ | 131.293 | f64 | 10000 | 4163.17 | 243902.44 |
| 055_Cs | alkali metal | Cs | Cesium | [Xe] 6s¹ | 132.90545196 | f32 | 1000 | 4324.45 | 250000.00 |
| 056_Ba | alkaline earth metal | Ba | Barium | [Xe] 6s² | 137.327 | f64 | 5000 | 4288.23 | 238095.23 |
| 057_La | lanthanide | La | Lanthanum | [Xe] 5d¹ 6s² | 138.90547 | f32 | 10000 | 4438.56 | 227272.73 |
| 058_Ce | lanthanide | Ce | Cerium | [Xe] 4f¹ 5d¹ 6s² | 140.116 | f64 | 1000 | 4410.13 | 250000.00 |
| 059_Pr | lanthanide | Pr | Praseodymium | [Xe] 4f³ 6s² | 140.90766 | f32 | 5000 | 4559.97 | 227272.73 |
| 060_Nd | lanthanide | Nd | Neodymium | [Xe] 4f⁴ 6s² | 144.242 | f64 | 10000 | 4696.36 | 217391.30 |
| 061_Pm | lanthanide | Pm | Promethium | [Xe] 4f⁵ 6s² | 144.912749 | f32 | 1000 | 4666.57 | 250000.00 |
| 062_Sm | lanthanide | Sm | Samarium | [Xe] 4f⁶ 6s² | 150.36 | f64 | 5000 | 4767.45 | 217391.30 |
| 063_Eu | lanthanide | Eu | Europium | [Xe] 4f⁷ 6s² | 151.964 | f32 | 10000 | 4832.15 | 208333.33 |
| 064_Gd | lanthanide | Gd | Gadolinium | [Xe] 4f⁷ 5d¹ 6s² | 157.25 | f64 | 1000 | 4872.53 | 250000.00 |
| 065_Tb | lanthanide | Tb | Terbium | [Xe] 4f⁹ 6s² | 158.92535 | f32 | 5000 | 5021.52 | 200000.00 |
| 066_Dy | lanthanide | Dy | Dysprosium | [Xe] 4f¹⁰ 6s² | 162.5 | f64 | 10000 | 5111.67 | 196078.44 |
| 067_Ho | lanthanide | Ho | Holmium | [Xe] 4f¹¹ 6s² | 164.93033 | f32 | 1000 | 5231.69 | 200000.00 |
| 068_Er | lanthanide | Er | Erbium | [Xe] 4f¹² 6s² | 167.259 | f64 | 5000 | 5226.24 | 192307.69 |
| 069_Tm | lanthanide | Tm | Thulium | [Xe] 4f¹³ 6s² | 168.93422 | f32 | 10000 | 5317.33 | 188679.25 |
| 070_Yb | lanthanide | Yb | Ytterbium | [Xe] 4f¹⁴ 6s² | 173.045 | f64 | 1000 | 5331.37 | 200000.00 |
| 071_Lu | lanthanide | Lu | Lutetium | [Xe] 4f¹⁴ 5d¹ 6s² | 174.9668 | f32 | 5000 | 5450.91 | 185185.19 |
| 072_Hf | transition metal | Hf | Hafnium | [Xe] 4f¹⁴ 5d² 6s² | 178.49 | f64 | 10000 | 5537.44 | 181818.19 |
| 073_Ta | transition metal | Ta | Tantalum | [Xe] 4f¹⁴ 5d³ 6s² | 180.94788 | f32 | 1000 | 5554.75 | 200000.00 |
| 074_W | transition metal | W | Tungsten | [Xe] 4f¹⁴ 5d⁴ 6s² | 183.84 | f64 | 5000 | 5673.61 | 178571.42 |
| 075_Re | transition metal | Re | Rhenium | [Xe] 4f¹⁴ 5d⁵ 6s² | 186.207 | f32 | 10000 | 5804.09 | 172413.80 |
| 076_Os | transition metal | Os | Osmium | [Xe] 4f¹⁴ 5d⁶ 6s² | 190.23 | f64 | 1000 | 5805.46 | 200000.00 |
| 077_Ir | transition metal | Ir | Iridium | [Xe] 4f¹⁴ 5d⁷ 6s² | 192.217 | f32 | 5000 | 6319.36 | 161290.33 |
| 078_Pt | transition metal | Pt | Platinum | [Xe] 4f¹⁴ 5d⁹ 6s¹ | 195.084 | f64 | 10000 | 6245.62 | 161290.33 |
| 079_Au | transition metal | Au | Gold | [Xe] 4f¹⁴ 5d¹⁰ 6s¹ | 196.966569 | f32 | 1000 | 6239.05 | 166666.67 |
| 080_Hg | transition metal | Hg | Mercury | [Xe] 4f¹⁴ 5d¹⁰ 6s² | 200.592 | f64 | 5000 | 6227.39 | 161290.33 |
| 081_Tl | post-transition metal | Tl | Thallium | [Xe] 4f¹⁴ 5d¹⁰ 6s² 6p¹ | 204.38 | f32 | 10000 | 6350.10 | 158730.16 |
| 082_Pb | post-transition metal | Pb | Lead | [Xe] 4f¹⁴ 5d¹⁰ 6s² 6p² | 207.2 | f64 | 1000 | 6397.04 | 166666.67 |
| 083_Bi | post-transition metal | Bi | Bismuth | [Xe] 4f¹⁴ 5d¹⁰ 6s² 6p³ | 208.9804 | f32 | 5000 | 6530.20 | 156250.00 |
| 084_Po | post-transition metal | Po | Polonium | [Xe] 4f¹⁴ 5d¹⁰ 6s² 6p⁴ | 208.98243 | f64 | 10000 | 6547.83 | 153846.16 |
| 085_At | halogen | At | Astatine | [Xe] 4f¹⁴ 5d¹⁰ 6s² 6p⁵ | 210 | f32 | 1000 | 6571.25 | 166666.67 |
| 086_Rn | noble gas | Rn | Radon | [Xe] 4f¹⁴ 5d¹⁰ 6s² 6p⁶ | 222 | f64 | 5000 | 6705.93 | 151515.16 |
| 087_Fr | alkali metal | Fr | Francium | [Rn] 7s¹ | 223 | f32 | 10000 | 6769.47 | 149253.73 |
| 088_Ra | alkaline earth metal | Ra | Radium | [Rn] 7s² | 226 | f64 | 1000 | 6927.16 | 166666.67 |
| 089_Ac | actinide | Ac | Actinium | [Rn] 6d¹ 7s² | 227.027752 | f32 | 5000 | 6975.24 | 147058.83 |
| 090_Th | actinide | Th | Thorium | [Rn] 6d² 7s² | 232.0377 | f64 | 10000 | 7089.71 | 142857.14 |
| 091_Pa | actinide | Pa | Protactinium | [Rn] 5f² 6d¹ 7s² | 231.03588 | f32 | 1000 | 7072.09 | 142857.14 |
| 092_U | actinide | U | Uranium | [Rn] 5f³ 6d¹ 7s² | 238.02891 | f64 | 5000 | 7316.16 | 138888.89 |
| 093_Np | actinide | Np | Neptunium | [Rn] 5f⁴ 6d¹ 7s² | 237.048174 | f32 | 10000 | 7245.58 | 138888.89 |
| 094_Pu | actinide | Pu | Plutonium | [Rn] 5f⁶ 7s² | 244.064204 | f64 | 1000 | 7277.76 | 142857.14 |
| 095_Am | actinide | Am | Americium | [Rn] 5f⁷ 7s² | 243.061381 | f32 | 5000 | 7388.27 | 138888.89 |
| 096_Cm | actinide | Cm | Curium | [Rn] 5f⁷ 6d¹ 7s² | 247.070354 | f64 | 10000 | 7501.51 | 133333.33 |
| 097_Bk | actinide | Bk | Berkelium | [Rn] 5f⁹ 7s² | 247.070307 | f32 | 1000 | 7534.16 | 142857.14 |
| 098_Cf | actinide | Cf | Californium | [Rn] 5f¹⁰ 7s² | 251.079587 | f64 | 5000 | 7762.57 | 131578.95 |
| 099_Es | actinide | Es | Einsteinium | [Rn] 5f¹¹ 7s² | 252.08298 | f32 | 10000 | 7814.70 | 128205.12 |
| 100_Fm | actinide | Fm | Fermium | [Rn] 5f¹² 7s² | 257.095105 | f64 | 1000 | 8014.43 | 125000.00 |
| 101_Md | actinide | Md | Mendelevium | [Rn] 5f¹³ 7s² | 258.098431 | f32 | 5000 | 7986.83 | 128205.12 |
| 102_No | actinide | No | Nobelium | [Rn] 5f¹⁴ 7s² | 259.10103 | f64 | 10000 | 8098.21 | 125000.00 |
| 103_Lr | actinide | Lr | Lawrencium | [Rn] 5f¹⁴ 7s² 7p¹ | 266.12 | f32 | 1000 | 7958.24 | 142857.14 |
| 104_Rf | transition_metal | Rf | Rutherfordium | [Rn] 5f¹⁴ 6d² 7s² | 267.122 | f64 | 5000 | 8047.03 | 125000.00 |
| 105_Db | transition_metal | Db | Dubnium | [Rn] 5f¹⁴ 6d³ 7s² | 268.126 | f32 | 10000 | 8232.80 | 121951.22 |
| 106_Sg | transition_metal | Sg | Seaborgium | [Rn] 5f¹⁴ 6d⁴ 7s² | 269.129 | f64 | 1000 | 8401.15 | 125000.00 |
| 107_Bh | transition_metal | Bh | Bohrium | [Rn] 5f¹⁴ 6d⁵ 7s² | 270.134 | f32 | 5000 | 8499.11 | 119047.62 |
| 108_Hs | transition_metal | Hs | Hassium | [Rn] 5f¹⁴ 6d⁶ 7s² | 269.134 | f64 | 10000 | 9304.38 | 107526.88 |
| 109_Mt | unknown | Mt | Meitnerium | [Rn] 5f¹⁴ 6d⁷ 7s² | 278.156 | f32 | 1000 | 8830.90 | 125000.00 |
| 110_Ds | unknown | Ds | Darmstadtium | [Rn] 5f¹⁴ 6d⁸ 7s² | 281.165 | f64 | 5000 | 8897.69 | 113636.37 |
| 111_Rg | unknown | Rg | Roentgenium | [Rn] 5f¹⁴ 6d⁹ 7s² | 282.169 | f32 | 10000 | 8706.14 | 114942.53 |
| 112_Cn | unknown | Cn | Copernicium | [Rn] 5f¹⁴ 6d¹⁰ 7s² | 285.177 | f64 | 1000 | 10072.36 | 100000.00 |
| 113_Nh | unknown | Nh | Nihonium | [Rn] 5f¹⁴ 6d¹⁰ 7s² 7p¹ | 286.182 | f32 | 5000 | 9439.48 | 106382.98 |
| 114_Fl | unknown | Fl | Flerovium | [Rn] 5f¹⁴ 6d¹⁰ 7s² 7p² | 289.19 | f64 | 10000 | 10116.05 | 99009.90 |
| 115_Mc | unknown | Mc | Moscovium | [Rn] 5f¹⁴ 6d¹⁰ 7s² 7p³ | 290.196 | f32 | 1000 | 8778.98 | 125000.00 |
| 116_Lv | unknown | Lv | Livermorium | [Rn] 5f¹⁴ 6d¹⁰ 7s² 7p⁴ | 293.205 | f64 | 5000 | 9629.90 | 104166.66 |
| 117_Ts | halogen | Ts | Tennessine | [Rn] 5f¹⁴ 6d¹⁰ 7s² 7p⁵ (predicted) | 294 | f32 | 10000 | 9498.33 | 106382.98 |
| 118_Og | noble gas (predicted) | Og | Oganesson | [Rn] 5f¹⁴ 6d¹⁰ 7s² 7p⁶ (predicted) | 294 | f64 | 1000 | 9873.80 | 111111.11 |
## Analytics
### Category Distribution
| Category | Count | Avg (ns) | Min (ns) | Max (ns) |
|---|---|---|---|---|
| nonmetal | 7 | 962.43 | 90.48 | 2592.98 |
| noble gas | 6 | 2665.24 | 169.05 | 6705.93 |
| alkali metal | 6 | 2768.50 | 249.70 | 6769.47 |
| alkaline earth metal | 6 | 2839.97 | 318.35 | 6927.16 |
| metalloid | 6 | 2407.38 | 402.31 | 3963.67 |
| halogen | 6 | 4146.22 | 697.91 | 9498.33 |
| post-transition metal | 8 | 4683.15 | 1010.41 | 6547.83 |
| transition metal | 29 | 3692.76 | 1636.08 | 6319.36 |
| lanthanide | 15 | 4928.96 | 4410.13 | 5450.91 |
| actinide | 15 | 7535.69 | 6975.24 | 8098.21 |
| transition_metal | 5 | 8496.89 | 8047.03 | 9304.38 |
| unknown | 8 | 9308.94 | 8706.14 | 10116.05 |
| noble gas (predicted) | 1 | 9873.80 | 9873.80 | 9873.80 |

### Top 10 Fastest Entries
| # | Entry | Avg (ns) |
|---|---|---|
| 1 | 001_H | 90.48 |
| 2 | 002_He | 169.05 |
| 3 | 003_Li | 249.70 |
| 4 | 004_Be | 318.35 |
| 5 | 005_B | 402.31 |
| 6 | 006_C | 471.10 |
| 7 | 007_N | 551.92 |
| 8 | 008_O | 620.61 |
| 9 | 009_F | 697.91 |
| 10 | 010_Ne | 775.56 |

### Top 10 Slowest Entries
| # | Entry | Avg (ns) |
|---|---|---|
| 1 | 114_Fl | 10116.05 |
| 2 | 112_Cn | 10072.36 |
| 3 | 118_Og | 9873.80 |
| 4 | 116_Lv | 9629.90 |
| 5 | 117_Ts | 9498.33 |
| 6 | 113_Nh | 9439.48 |
| 7 | 108_Hs | 9304.38 |
| 8 | 110_Ds | 8897.69 |
| 9 | 109_Mt | 8830.90 |
| 10 | 115_Mc | 8778.98 |

