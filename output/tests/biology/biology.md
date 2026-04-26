# SciForge Biology Benchmarks

183 benchmark entries across 5 formats.

---

## Grid
```
  aging           : CRL  GMR   MD  RSS   TS
  bioelectricity  : CES   EL   IT  SDW
  bioenergetics   : AEC  GNA  POR  RCI
  biogeography    : LDG  RSV  RSL  WLE
  bioinformatics  : AGC  EDI   SI  SWS
  biomechanics    : BCO  GSL  GRF   JM
  biophysics      :  FE   HB   LJ   MT
  biostatistics   :  FS  NNT   OR   Se   Sp
  cancer_biology  : CLQ  CKL  TDT  TGE
  cell            : CDT  FFL  LRB   OP
  chronobiology   : CAD  JLR   Ph   ZS
  cryobiology     : CCR  FPD  SDA   VP
  developmental   : FFM   LI  MGS
  dispatch        :  CF  DMP  DUF  GMR  MRK   SR   TS
  ecology         :  IS   SD   SD   SR
  endocrinology   :  HI  HHL  ISI   RS
  enzyme          : CII   HE  HHC   LB  MMH  MMM  MMS
  epigenetics     : CML  EIP   NO  RKE
  ethology        : HRB  HDP  IFD   TS
  evolution       :  AR  FPN  SRB   SR
  genetics        : BSH  EPS   HW  MSB
  genomics        : CAI   GC   GD  SAF
  immunology      :  AT   CE  HIF   VE
  marine_biology  : BTT  FGV  MSY  OAP
  microbiology    :  BF  CSS  GTB   MG
  mycology        : CUE   DR  HGR  SGP
  neuroscience    :  FF   FR  RWU  SWC
  nutrition       : BMR  BMI   NB  RER
  paleobiology    : ERD   ER  NDR   RA
  parasitology    : FOI   PR  VBR
  pharmacology    :  Bi  DRH   HL   TI
  phylogenetics   : JCD   PS   SI
  physiology      : AGE  FSM  GFR   OD
  plant_biology   : BLC  PLR   TR   WP
  population      : CCF  CGL  LVC  LVP
  proteomics      :  GI  MAP   MR   PC
  radiobiology    : OER  RBE  RLQ   Tc
  reproduction    :  FP   HD  ISR  SMS
  stem_cell       :  NO   RE  SRP
  structural      : AHP  BFE  ROG   Rm
  synthetic_biology:  AG  COT   HA   HR
  systems_biology : MCC   OP   UI
  tissue_engineering: CPO  DRF  OTR   SP
  toxicology      :  BR   LP   RQ   TW
  virology        : BRN   BS  HIT  VLD
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
| Label | category | symbol | name | domain | test | Precision | Iterations | Avg (ns) | Iters/s |
|---|---|---|---|---|---|---|---|---|---|
| aging_caloric_restriction_lifespan | aging | CRL | Caloric restriction lifespan | biology | aging::caloric_restriction_lifespan | f64 | 1 | 2388739.00 | 500.00 |
| aging_gompertz_mortality_rate | aging | GMR | Gompertz mortality rate | biology | aging::gompertz_mortality_rate | f64 | 1 | 2356569.00 | 500.00 |
| aging_mitochondrial_damage | aging | MD | Mitochondrial damage | biology | aging::mitochondrial_damage | f64 | 1 | 2735439.00 | 500.00 |
| aging_ros_steady_state | aging | RSS | Ros steady state | biology | aging::ros_steady_state | f64 | 1 | 6062787.00 | 166.67 |
| aging_telomere_shortening | aging | TS | Telomere shortening | biology | aging::telomere_shortening | f64 | 1 | 4275508.00 | 250.00 |
| bioelectricity_cable_equation_steady | bioelectricity | CES | Cable equation steady | biology | bioelectricity::cable_equation_steady | f64 | 1 | 4533278.00 | 250.00 |
| bioelectricity_electrotonic_length | bioelectricity | EL | Electrotonic length | biology | bioelectricity::electrotonic_length | f64 | 1 | 10498526.00 | 100.00 |
| bioelectricity_impedance_tissue | bioelectricity | IT | Impedance tissue | biology | bioelectricity::impedance_tissue | f64 | 1 | 6269367.00 | 166.67 |
| bioelectricity_strength_duration_weiss | bioelectricity | SDW | Strength duration weiss | biology | bioelectricity::strength_duration_weiss | f64 | 1 | 2738959.00 | 500.00 |
| bioenergetics_adenylate_energy_charge | bioenergetics | AEC | Adenylate energy charge | biology | bioenergetics::adenylate_energy_charge | f64 | 1 | 2351309.00 | 500.00 |
| bioenergetics_glycolysis_net_atp | bioenergetics | GNA | Glycolysis net atp | biology | bioenergetics::glycolysis_net_atp | f64 | 1 | 2380898.00 | 500.00 |
| bioenergetics_p_o_ratio | bioenergetics | POR | P o ratio | biology | bioenergetics::p_o_ratio | f64 | 1 | 2384720.00 | 500.00 |
| bioenergetics_respiratory_control_index | bioenergetics | RCI | Respiratory control index | biology | bioenergetics::respiratory_control_index | f64 | 1 | 5900607.00 | 200.00 |
| biogeography_latitudinal_diversity_gradient | biogeography | LDG | Latitudinal diversity gradient | biology | biogeography::latitudinal_diversity_gradient | f64 | 1 | 3486678.00 | 333.33 |
| biogeography_range_shift_velocity | biogeography | RSV | Range shift velocity | biology | biogeography::range_shift_velocity | f64 | 1 | 2481589.00 | 500.00 |
| biogeography_range_size_latitude | biogeography | RSL | Range size latitude | biology | biogeography::range_size_latitude | f64 | 1 | 2274999.00 | 500.00 |
| biogeography_wallace_line_effect | biogeography | WLE | Wallace line effect | biology | biogeography::wallace_line_effect | f64 | 1 | 4666338.00 | 250.00 |
| bioinformatics_alignment_gc_content | bioinformatics | AGC | Alignment gc content | biology | bioinformatics::alignment_gc_content | f64 | 1 | 2438749.00 | 500.00 |
| bioinformatics_edit_distance_identical | bioinformatics | EDI | Edit distance identical | biology | bioinformatics::edit_distance_identical | f64 | 1 | 2452219.00 | 500.00 |
| bioinformatics_sequence_identity | bioinformatics | SI | Sequence identity | biology | bioinformatics::sequence_identity | f64 | 1 | 2011779.00 | 500.00 |
| bioinformatics_smith_waterman_score | bioinformatics | SWS | Smith waterman score | biology | bioinformatics::smith_waterman_score | f64 | 1 | 1977209.00 | 1000.00 |
| biomechanics_biomech_cardiac_output | biomechanics | BCO | Biomech cardiac output | biology | biomechanics::biomech_cardiac_output | f64 | 1 | 1778790.00 | 1000.00 |
| biomechanics_gait_stride_length | biomechanics | GSL | Gait stride length | biology | biomechanics::gait_stride_length | f64 | 1 | 1790669.00 | 1000.00 |
| biomechanics_ground_reaction_force | biomechanics | GRF | Ground reaction force | biology | biomechanics::ground_reaction_force | f64 | 1 | 1807159.00 | 1000.00 |
| biomechanics_joint_moment | biomechanics | JM | Joint moment | biology | biomechanics::joint_moment | f64 | 1 | 2017939.00 | 500.00 |
| biophysics_fret_efficiency | biophysics | FE | Fret efficiency | biology | biophysics::fret_efficiency | f64 | 1 | 2613499.00 | 500.00 |
| biophysics_harmonic_bond | biophysics | HB | Harmonic bond | biology | biophysics::harmonic_bond | f64 | 1 | 3845128.00 | 333.33 |
| biophysics_lennard_jones | biophysics | LJ | Lennard jones | biology | biophysics::lennard_jones | f64 | 1 | 3076909.00 | 333.33 |
| biophysics_membrane_tension | biophysics | MT | Membrane tension | biology | biophysics::membrane_tension | f64 | 1 | 2739149.00 | 500.00 |
| biostatistics_f1_score | biostatistics | FS | F1 score | biology | biostatistics::f1_score | f64 | 1 | 2407649.00 | 500.00 |
| biostatistics_number_needed_to_treat | biostatistics | NNT | Number needed to treat | biology | biostatistics::number_needed_to_treat | f64 | 1 | 1936359.00 | 1000.00 |
| biostatistics_odds_ratio | biostatistics | OR | Odds ratio | biology | biostatistics::odds_ratio | f64 | 1 | 1923639.00 | 1000.00 |
| biostatistics_sensitivity | biostatistics | Se | Sensitivity | biology | biostatistics::sensitivity | f64 | 1 | 2504969.00 | 500.00 |
| biostatistics_specificity | biostatistics | Sp | Specificity | biology | biostatistics::specificity | f64 | 1 | 2157639.00 | 500.00 |
| cancer_biology_cancer_linear_quadratic_survival | cancer_biology | CLQ | Cancer linear quadratic survival | biology | cancer_biology::cancer_linear_quadratic_survival | f64 | 1 | 1898459.00 | 1000.00 |
| cancer_biology_cell_kill_log | cancer_biology | CKL | Cell kill log | biology | cancer_biology::cell_kill_log | f64 | 1 | 1829739.00 | 1000.00 |
| cancer_biology_tumor_doubling_time | cancer_biology | TDT | Tumor doubling time | biology | cancer_biology::tumor_doubling_time | f64 | 1 | 1934869.00 | 1000.00 |
| cancer_biology_tumor_growth_exponential | cancer_biology | TGE | Tumor growth exponential | biology | cancer_biology::tumor_growth_exponential | f64 | 1 | 1873090.00 | 1000.00 |
| cell_cell_doubling_time | cell | CDT | Cell doubling time | biology | cell::cell_doubling_time | f64 | 1 | 1920959.00 | 1000.00 |
| cell_fick_first_law | cell | FFL | Fick first law | biology | cell::fick_first_law | f64 | 1 | 1816119.00 | 1000.00 |
| cell_ligand_receptor_binding | cell | LRB | Ligand receptor binding | biology | cell::ligand_receptor_binding | f64 | 1 | 1868889.00 | 1000.00 |
| cell_osmotic_pressure | cell | OP | Osmotic pressure | biology | cell::osmotic_pressure | f64 | 1 | 2372089.00 | 500.00 |
| chronobiology_circadian_amplitude_damping | chronobiology | CAD | Circadian amplitude damping | biology | chronobiology::circadian_amplitude_damping | f64 | 1 | 2000019.00 | 500.00 |
| chronobiology_jet_lag_recovery | chronobiology | JLR | Jet lag recovery | biology | chronobiology::jet_lag_recovery | f64 | 1 | 1860429.00 | 1000.00 |
| chronobiology_photoperiod | chronobiology | Ph | Photoperiod | biology | chronobiology::photoperiod | f64 | 1 | 2133809.00 | 500.00 |
| chronobiology_zeitgeber_strength | chronobiology | ZS | Zeitgeber strength | biology | chronobiology::zeitgeber_strength | f64 | 1 | 1878409.00 | 1000.00 |
| cryobiology_critical_cooling_rate | cryobiology | CCR | Critical cooling rate | biology | cryobiology::critical_cooling_rate | f64 | 1 | 2121980.00 | 500.00 |
| cryobiology_freezing_point_depression | cryobiology | FPD | Freezing point depression | biology | cryobiology::freezing_point_depression | f64 | 1 | 1963989.00 | 1000.00 |
| cryobiology_storage_decay_arrhenius | cryobiology | SDA | Storage decay arrhenius | biology | cryobiology::storage_decay_arrhenius | f64 | 1 | 2451709.00 | 500.00 |
| cryobiology_vitrification_probability | cryobiology | VP | Vitrification probability | biology | cryobiology::vitrification_probability | f64 | 1 | 2126599.00 | 500.00 |
| developmental_french_flag_model | developmental | FFM | French flag model | biology | developmental::french_flag_model | f64 | 1 | 1895879.00 | 1000.00 |
| developmental_lateral_inhibition | developmental | LI | Lateral inhibition | biology | developmental::lateral_inhibition | f64 | 1 | 1866539.00 | 1000.00 |
| developmental_morphogen_gradient_steady | developmental | MGS | Morphogen gradient steady | biology | developmental::morphogen_gradient_steady | f64 | 1 | 1886909.00 | 1000.00 |
| dispatch_codon_frequency | dispatch | CF | Codon frequency | biology | dispatch::codon_frequency | f64 | 1 | 1860779.00 | 1000.00 |
| dispatch_dispatch_missing_parameter_returns_error | dispatch | DMP | Dispatch missing parameter returns error | biology | dispatch::dispatch_missing_parameter_returns_error | f64 | 1 | 1907299.00 | 1000.00 |
| dispatch_dispatch_unknown_function_returns_error | dispatch | DUF | Dispatch unknown function returns error | biology | dispatch::dispatch_unknown_function_returns_error | f64 | 1 | 1868900.00 | 1000.00 |
| dispatch_gompertz_mortality_rate | dispatch | GMR | Gompertz mortality rate | biology | dispatch::gompertz_mortality_rate | f64 | 1 | 1763889.00 | 1000.00 |
| dispatch_metabolic_rate_kleiber | dispatch | MRK | Metabolic rate kleiber | biology | dispatch::metabolic_rate_kleiber | f64 | 1 | 1806629.00 | 1000.00 |
| dispatch_soil_respiration | dispatch | SR | Soil respiration | biology | dispatch::soil_respiration | f64 | 1 | 1778969.00 | 1000.00 |
| dispatch_telomere_shortening | dispatch | TS | Telomere shortening | biology | dispatch::telomere_shortening | f64 | 1 | 1682540.00 | 1000.00 |
| ecology_inverse_simpson | ecology | IS | Inverse simpson | biology | ecology::inverse_simpson | f64 | 1 | 1800289.00 | 1000.00 |
| ecology_shannon_diversity | ecology | SD | Shannon diversity | biology | ecology::shannon_diversity | f64 | 1 | 1807199.00 | 1000.00 |
| ecology_simpson_diversity | ecology | SD | Simpson diversity | biology | ecology::simpson_diversity | f64 | 1 | 1871009.00 | 1000.00 |
| ecology_species_richness | ecology | SR | Species richness | biology | ecology::species_richness | f64 | 1 | 1806889.00 | 1000.00 |
| endocrinology_homa_ir | endocrinology | HI | Homa ir | biology | endocrinology::homa_ir | f64 | 1 | 1825890.00 | 1000.00 |
| endocrinology_hormone_half_life_clearance | endocrinology | HHL | Hormone half life clearance | biology | endocrinology::hormone_half_life_clearance | f64 | 1 | 1803669.00 | 1000.00 |
| endocrinology_insulin_sensitivity_index | endocrinology | ISI | Insulin sensitivity index | biology | endocrinology::insulin_sensitivity_index | f64 | 1 | 1869779.00 | 1000.00 |
| endocrinology_receptor_saturation | endocrinology | RS | Receptor saturation | biology | endocrinology::receptor_saturation | f64 | 1 | 1801319.00 | 1000.00 |
| enzyme_competitive_inhibition_increases_apparent_km | enzyme | CII | Competitive inhibition increases apparent km | biology | enzyme::competitive_inhibition_increases_apparent_km | f64 | 1 | 1819649.00 | 1000.00 |
| enzyme_hill_equation | enzyme | HE | Hill equation | biology | enzyme::hill_equation | f64 | 1 | 1988950.00 | 1000.00 |
| enzyme_hill_higher_cooperativity_reduces_rate_below_k | enzyme | HHC | Hill higher cooperativity reduces rate below k | biology | enzyme::hill_higher_cooperativity_reduces_rate_below_k | f64 | 1 | 1838419.00 | 1000.00 |
| enzyme_lineweaver_burk | enzyme | LB | Lineweaver burk | biology | enzyme::lineweaver_burk | f64 | 1 | 1836799.00 | 1000.00 |
| enzyme_michaelis_menten_half_vmax | enzyme | MMH | Michaelis menten half vmax | biology | enzyme::michaelis_menten_half_vmax | f64 | 1 | 1787419.00 | 1000.00 |
| enzyme_michaelis_menten_monotonic_with_substrate | enzyme | MMM | Michaelis menten monotonic with substrate | biology | enzyme::michaelis_menten_monotonic_with_substrate | f64 | 1 | 1969159.00 | 1000.00 |
| enzyme_michaelis_menten_saturation | enzyme | MMS | Michaelis menten saturation | biology | enzyme::michaelis_menten_saturation | f64 | 1 | 1820739.00 | 1000.00 |
| epigenetics_cpg_methylation_level | epigenetics | CML | Cpg methylation level | biology | epigenetics::cpg_methylation_level | f64 | 1 | 1843710.00 | 1000.00 |
| epigenetics_epigenetic_inheritance_probability | epigenetics | EIP | Epigenetic inheritance probability | biology | epigenetics::epigenetic_inheritance_probability | f64 | 1 | 1963089.00 | 1000.00 |
| epigenetics_nucleosome_occupancy | epigenetics | NO | Nucleosome occupancy | biology | epigenetics::nucleosome_occupancy | f64 | 1 | 1942579.00 | 1000.00 |
| epigenetics_rnai_knockdown_efficiency | epigenetics | RKE | Rnai knockdown efficiency | biology | epigenetics::rnai_knockdown_efficiency | f64 | 1 | 1803699.00 | 1000.00 |
| ethology_hamilton_relatedness_benefit | ethology | HRB | Hamilton relatedness benefit | biology | ethology::hamilton_relatedness_benefit | f64 | 1 | 1914779.00 | 1000.00 |
| ethology_hawk_dove_payoff | ethology | HDP | Hawk dove payoff | biology | ethology::hawk_dove_payoff | f64 | 1 | 1912059.00 | 1000.00 |
| ethology_ideal_free_distribution | ethology | IFD | Ideal free distribution | biology | ethology::ideal_free_distribution | f64 | 1 | 1985480.00 | 1000.00 |
| ethology_territory_size | ethology | TS | Territory size | biology | ethology::territory_size | f64 | 1 | 3792648.00 | 333.33 |
| evolution_adaptation_rate | evolution | AR | Adaptation rate | biology | evolution::adaptation_rate | f64 | 1 | 2163739.00 | 500.00 |
| evolution_fixation_probability_neutral | evolution | FPN | Fixation probability neutral | biology | evolution::fixation_probability_neutral | f64 | 1 | 2308969.00 | 500.00 |
| evolution_speciation_rate_bdi | evolution | SRB | Speciation rate bdi | biology | evolution::speciation_rate_bdi | f64 | 1 | 2089979.00 | 500.00 |
| evolution_substitution_rate | evolution | SR | Substitution rate | biology | evolution::substitution_rate | f64 | 1 | 1925159.00 | 1000.00 |
| genetics_broad_sense_heritability | genetics | BSH | Broad sense heritability | biology | genetics::broad_sense_heritability | f64 | 1 | 2833199.00 | 500.00 |
| genetics_effective_population_size | genetics | EPS | Effective population size | biology | genetics::effective_population_size | f64 | 1 | 2696249.00 | 500.00 |
| genetics_hardy_weinberg | genetics | HW | Hardy weinberg | biology | genetics::hardy_weinberg | f64 | 1 | 2315369.00 | 500.00 |
| genetics_mutation_selection_balance | genetics | MSB | Mutation selection balance | biology | genetics::mutation_selection_balance | f64 | 1 | 2644719.00 | 500.00 |
| genomics_codon_adaptation_index | genomics | CAI | Codon adaptation index | biology | genomics::codon_adaptation_index | f64 | 1 | 2450358.00 | 500.00 |
| genomics_gc_content | genomics | GC | Gc content | biology | genomics::gc_content | f64 | 1 | 1887440.00 | 1000.00 |
| genomics_gene_density | genomics | GD | Gene density | biology | genomics::gene_density | f64 | 1 | 2362429.00 | 500.00 |
| genomics_snp_allele_frequency | genomics | SAF | Snp allele frequency | biology | genomics::snp_allele_frequency | f64 | 1 | 2466358.00 | 500.00 |
| immunology_antibody_titer | immunology | AT | Antibody titer | biology | immunology::antibody_titer | f64 | 1 | 2071250.00 | 500.00 |
| immunology_clonal_expansion | immunology | CE | Clonal expansion | biology | immunology::clonal_expansion | f64 | 1 | 1935659.00 | 1000.00 |
| immunology_herd_immunity_fraction | immunology | HIF | Herd immunity fraction | biology | immunology::herd_immunity_fraction | f64 | 1 | 2346389.00 | 500.00 |
| immunology_vaccine_efficacy | immunology | VE | Vaccine efficacy | biology | immunology::vaccine_efficacy | f64 | 1 | 2118319.00 | 500.00 |
| marine_biology_bleaching_thermal_threshold | marine_biology | BTT | Bleaching thermal threshold | biology | marine_biology::bleaching_thermal_threshold | f64 | 1 | 2267799.00 | 500.00 |
| marine_biology_fish_growth_von_bertalanffy | marine_biology | FGV | Fish growth von bertalanffy | biology | marine_biology::fish_growth_von_bertalanffy | f64 | 1 | 2000309.00 | 500.00 |
| marine_biology_maximum_sustainable_yield | marine_biology | MSY | Maximum sustainable yield | biology | marine_biology::maximum_sustainable_yield | f64 | 1 | 2086209.00 | 500.00 |
| marine_biology_ocean_acidification_ph | marine_biology | OAP | Ocean acidification ph | biology | marine_biology::ocean_acidification_ph | f64 | 1 | 2526989.00 | 500.00 |
| microbiology_biofilm_formation | microbiology | BF | Biofilm formation | biology | microbiology::biofilm_formation | f64 | 1 | 3084289.00 | 333.33 |
| microbiology_chemostat_steady_state_biomass | microbiology | CSS | Chemostat steady state biomass | biology | microbiology::chemostat_steady_state_biomass | f64 | 1 | 2725198.00 | 500.00 |
| microbiology_generation_time_bacteria | microbiology | GTB | Generation time bacteria | biology | microbiology::generation_time_bacteria | f64 | 1 | 2246459.00 | 500.00 |
| microbiology_monod_growth | microbiology | MG | Monod growth | biology | microbiology::monod_growth | f64 | 1 | 1968609.00 | 1000.00 |
| mycology_carbon_use_efficiency | mycology | CUE | Carbon use efficiency | biology | mycology::carbon_use_efficiency | f64 | 1 | 2549219.00 | 500.00 |
| mycology_decomposition_rate | mycology | DR | Decomposition rate | biology | mycology::decomposition_rate | f64 | 1 | 2068769.00 | 500.00 |
| mycology_hyphal_growth_rate | mycology | HGR | Hyphal growth rate | biology | mycology::hyphal_growth_rate | f64 | 1 | 1888570.00 | 1000.00 |
| mycology_spore_germination_probability | mycology | SGP | Spore germination probability | biology | mycology::spore_germination_probability | f64 | 1 | 1850959.00 | 1000.00 |
| neuroscience_fano_factor | neuroscience | FF | Fano factor | biology | neuroscience::fano_factor | f64 | 1 | 1910339.00 | 1000.00 |
| neuroscience_firing_rate | neuroscience | FR | Firing rate | biology | neuroscience::firing_rate | f64 | 1 | 1881669.00 | 1000.00 |
| neuroscience_rescorla_wagner_update | neuroscience | RWU | Rescorla wagner update | biology | neuroscience::rescorla_wagner_update | f64 | 1 | 1852379.00 | 1000.00 |
| neuroscience_stdp_weight_change | neuroscience | SWC | Stdp weight change | biology | neuroscience::stdp_weight_change | f64 | 1 | 1871889.00 | 1000.00 |
| nutrition_basal_metabolic_rate_mifflin | nutrition | BMR | Basal metabolic rate mifflin | biology | nutrition::basal_metabolic_rate_mifflin | f64 | 1 | 1904810.00 | 1000.00 |
| nutrition_body_mass_index | nutrition | BMI | Body mass index | biology | nutrition::body_mass_index | f64 | 1 | 1865179.00 | 1000.00 |
| nutrition_nitrogen_balance | nutrition | NB | Nitrogen balance | biology | nutrition::nitrogen_balance | f64 | 1 | 2030039.00 | 500.00 |
| nutrition_respiratory_exchange_ratio | nutrition | RER | Respiratory exchange ratio | biology | nutrition::respiratory_exchange_ratio | f64 | 1 | 1894079.00 | 1000.00 |
| paleobiology_evolutionary_rate_darwin | paleobiology | ERD | Evolutionary rate darwin | biology | paleobiology::evolutionary_rate_darwin | f64 | 1 | 1842559.00 | 1000.00 |
| paleobiology_extinction_rate | paleobiology | ER | Extinction rate | biology | paleobiology::extinction_rate | f64 | 1 | 1863559.00 | 1000.00 |
| paleobiology_net_diversification_rate | paleobiology | NDR | Net diversification rate | biology | paleobiology::net_diversification_rate | f64 | 1 | 1971260.00 | 1000.00 |
| paleobiology_radiometric_age | paleobiology | RA | Radiometric age | biology | paleobiology::radiometric_age | f64 | 1 | 1869729.00 | 1000.00 |
| parasitology_force_of_infection | parasitology | FOI | Force of infection | biology | parasitology::force_of_infection | f64 | 1 | 2076819.00 | 500.00 |
| parasitology_parasite_r0 | parasitology | PR | Parasite r0 | biology | parasitology::parasite_r0 | f64 | 1 | 2180789.00 | 500.00 |
| parasitology_vector_borne_r0 | parasitology | VBR | Vector borne r0 | biology | parasitology::vector_borne_r0 | f64 | 1 | 1930639.00 | 1000.00 |
| pharmacology_bioavailability | pharmacology | Bi | Bioavailability | biology | pharmacology::bioavailability | f64 | 1 | 1904209.00 | 1000.00 |
| pharmacology_dose_response_hill | pharmacology | DRH | Dose response hill | biology | pharmacology::dose_response_hill | f64 | 1 | 1935179.00 | 1000.00 |
| pharmacology_half_life | pharmacology | HL | Half life | biology | pharmacology::half_life | f64 | 1 | 2048649.00 | 500.00 |
| pharmacology_therapeutic_index | pharmacology | TI | Therapeutic index | biology | pharmacology::therapeutic_index | f64 | 1 | 1950580.00 | 1000.00 |
| phylogenetics_jukes_cantor_distance | phylogenetics | JCD | Jukes cantor distance | biology | phylogenetics::jukes_cantor_distance | f64 | 1 | 2288709.00 | 500.00 |
| phylogenetics_parsimony_score | phylogenetics | PS | Parsimony score | biology | phylogenetics::parsimony_score | f64 | 1 | 2182899.00 | 500.00 |
| phylogenetics_sackin_index | phylogenetics | SI | Sackin index | biology | phylogenetics::sackin_index | f64 | 1 | 1973689.00 | 1000.00 |
| physiology_alveolar_gas_equation | physiology | AGE | Alveolar gas equation | biology | physiology::alveolar_gas_equation | f64 | 1 | 1923229.00 | 1000.00 |
| physiology_frank_starling_mechanism | physiology | FSM | Frank starling mechanism | biology | physiology::frank_starling_mechanism | f64 | 1 | 1882189.00 | 1000.00 |
| physiology_glomerular_filtration_rate | physiology | GFR | Glomerular filtration rate | biology | physiology::glomerular_filtration_rate | f64 | 1 | 1904839.00 | 1000.00 |
| physiology_oxygen_delivery | physiology | OD | Oxygen delivery | biology | physiology::oxygen_delivery | f64 | 1 | 1975999.00 | 1000.00 |
| plant_biology_beer_lambert_canopy | plant_biology | BLC | Beer lambert canopy | biology | plant_biology::beer_lambert_canopy | f64 | 1 | 1915580.00 | 1000.00 |
| plant_biology_photosynthesis_light_response | plant_biology | PLR | Photosynthesis light response | biology | plant_biology::photosynthesis_light_response | f64 | 1 | 2052199.00 | 500.00 |
| plant_biology_transpiration_rate | plant_biology | TR | Transpiration rate | biology | plant_biology::transpiration_rate | f64 | 1 | 1957179.00 | 1000.00 |
| plant_biology_water_potential | plant_biology | WP | Water potential | biology | plant_biology::water_potential | f64 | 1 | 1887439.00 | 1000.00 |
| population_carrying_capacity_from_resources | population | CCF | Carrying capacity from resources | biology | population::carrying_capacity_from_resources | f64 | 1 | 1833369.00 | 1000.00 |
| population_cell_growth_logistic | population | CGL | Cell growth logistic | biology | population::cell_growth_logistic | f64 | 1 | 1889609.00 | 1000.00 |
| population_lotka_volterra_competition | population | LVC | Lotka volterra competition | biology | population::lotka_volterra_competition | f64 | 1 | 1850320.00 | 1000.00 |
| population_lotka_volterra_predator_prey | population | LVP | Lotka volterra predator prey | biology | population::lotka_volterra_predator_prey | f64 | 1 | 2126848.00 | 500.00 |
| proteomics_gravy_index | proteomics | GI | Gravy index | biology | proteomics::gravy_index | f64 | 1 | 2137160.00 | 500.00 |
| proteomics_mass_accuracy_ppm | proteomics | MAP | Mass accuracy ppm | biology | proteomics::mass_accuracy_ppm | f64 | 1 | 2516799.00 | 500.00 |
| proteomics_mz_ratio | proteomics | MR | Mz ratio | biology | proteomics::mz_ratio | f64 | 1 | 2079879.00 | 500.00 |
| proteomics_protein_coverage | proteomics | PC | Protein coverage | biology | proteomics::protein_coverage | f64 | 1 | 2017939.00 | 500.00 |
| radiobiology_oxygen_enhancement_ratio | radiobiology | OER | Oxygen enhancement ratio | biology | radiobiology::oxygen_enhancement_ratio | f64 | 1 | 1931849.00 | 1000.00 |
| radiobiology_radio_biologically_effective_dose | radiobiology | RBE | Radio biologically effective dose | biology | radiobiology::radio_biologically_effective_dose | f64 | 1 | 2311359.00 | 500.00 |
| radiobiology_radio_linear_quadratic_survival | radiobiology | RLQ | Radio linear quadratic survival | biology | radiobiology::radio_linear_quadratic_survival | f64 | 1 | 2226039.00 | 500.00 |
| radiobiology_tcp | radiobiology | Tc | Tcp | biology | radiobiology::tcp | f64 | 1 | 2067209.00 | 500.00 |
| reproduction_fertilization_probability | reproduction | FP | Fertilization probability | biology | reproduction::fertilization_probability | f64 | 1 | 2159619.00 | 500.00 |
| reproduction_hcg_doubling | reproduction | HD | Hcg doubling | biology | reproduction::hcg_doubling | f64 | 1 | 2158919.00 | 500.00 |
| reproduction_ivf_success_rate | reproduction | ISR | Ivf success rate | biology | reproduction::ivf_success_rate | f64 | 1 | 2084539.00 | 500.00 |
| reproduction_sperm_motility_score | reproduction | SMS | Sperm motility score | biology | reproduction::sperm_motility_score | f64 | 1 | 1892929.00 | 1000.00 |
| stem_cell_niche_occupancy | stem_cell | NO | Niche occupancy | biology | stem_cell::niche_occupancy | f64 | 1 | 1925019.00 | 1000.00 |
| stem_cell_reprogramming_efficiency | stem_cell | RE | Reprogramming efficiency | biology | stem_cell::reprogramming_efficiency | f64 | 1 | 1896170.00 | 1000.00 |
| stem_cell_self_renewal_probability | stem_cell | SRP | Self renewal probability | biology | stem_cell::self_renewal_probability | f64 | 1 | 1880849.00 | 1000.00 |
| structural_alpha_helix_propensity | structural | AHP | Alpha helix propensity | biology | structural::alpha_helix_propensity | f64 | 1 | 1891369.00 | 1000.00 |
| structural_binding_free_energy | structural | BFE | Binding free energy | biology | structural::binding_free_energy | f64 | 1 | 1896969.00 | 1000.00 |
| structural_radius_of_gyration | structural | ROG | Radius of gyration | biology | structural::radius_of_gyration | f64 | 1 | 2033779.00 | 500.00 |
| structural_rmsd | structural | Rm | Rmsd | biology | structural::rmsd | f64 | 1 | 1977709.00 | 1000.00 |
| synthetic_biology_and_gate | synthetic_biology | AG | And gate | biology | synthetic_biology::and_gate | f64 | 1 | 1884149.00 | 1000.00 |
| synthetic_biology_crispr_on_target_score | synthetic_biology | COT | Crispr on target score | biology | synthetic_biology::crispr_on_target_score | f64 | 1 | 2236349.00 | 500.00 |
| synthetic_biology_hill_activation | synthetic_biology | HA | Hill activation | biology | synthetic_biology::hill_activation | f64 | 1 | 2132950.00 | 500.00 |
| synthetic_biology_hill_repression | synthetic_biology | HR | Hill repression | biology | synthetic_biology::hill_repression | f64 | 1 | 1895859.00 | 1000.00 |
| systems_biology_metabolic_control_coefficient | systems_biology | MCC | Metabolic control coefficient | biology | systems_biology::metabolic_control_coefficient | f64 | 1 | 1841209.00 | 1000.00 |
| systems_biology_oscillation_period | systems_biology | OP | Oscillation period | biology | systems_biology::oscillation_period | f64 | 1 | 2125069.00 | 500.00 |
| systems_biology_ultrasensitivity_index | systems_biology | UI | Ultrasensitivity index | biology | systems_biology::ultrasensitivity_index | f64 | 1 | 1995669.00 | 1000.00 |
| tissue_engineering_cell_proliferation_on_scaffold | tissue_engineering | CPO | Cell proliferation on scaffold | biology | tissue_engineering::cell_proliferation_on_scaffold | f64 | 1 | 2974159.00 | 500.00 |
| tissue_engineering_degradation_rate_first_order | tissue_engineering | DRF | Degradation rate first order | biology | tissue_engineering::degradation_rate_first_order | f64 | 1 | 2174659.00 | 500.00 |
| tissue_engineering_oxygen_transfer_rate | tissue_engineering | OTR | Oxygen transfer rate | biology | tissue_engineering::oxygen_transfer_rate | f64 | 1 | 2201789.00 | 500.00 |
| tissue_engineering_scaffold_porosity | tissue_engineering | SP | Scaffold porosity | biology | tissue_engineering::scaffold_porosity | f64 | 1 | 2120669.00 | 500.00 |
| toxicology_bcf_ratio | toxicology | BR | Bcf ratio | biology | toxicology::bcf_ratio | f64 | 1 | 2049629.00 | 500.00 |
| toxicology_ld50_probit | toxicology | LP | Ld50 probit | biology | toxicology::ld50_probit | f64 | 1 | 1986869.00 | 1000.00 |
| toxicology_risk_quotient | toxicology | RQ | Risk quotient | biology | toxicology::risk_quotient | f64 | 1 | 1919359.00 | 1000.00 |
| toxicology_therapeutic_window | toxicology | TW | Therapeutic window | biology | toxicology::therapeutic_window | f64 | 1 | 2086099.00 | 500.00 |
| virology_basic_reproduction_number_viral | virology | BRN | Basic reproduction number viral | biology | virology::basic_reproduction_number_viral | f64 | 1 | 1903999.00 | 1000.00 |
| virology_burst_size | virology | BS | Burst size | biology | virology::burst_size | f64 | 1 | 1872230.00 | 1000.00 |
| virology_herd_immunity_threshold | virology | HIT | Herd immunity threshold | biology | virology::herd_immunity_threshold | f64 | 1 | 1835129.00 | 1000.00 |
| virology_viral_load_decay | virology | VLD | Viral load decay | biology | virology::viral_load_decay | f64 | 1 | 1944849.00 | 1000.00 |
## Analytics
### Category Distribution
| Category | Count | Avg (ns) | Min (ns) | Max (ns) |
|---|---|---|---|---|
| aging | 5 | 3563808.50 | 2356569.00 | 6062787.00 |
| bioelectricity | 4 | 6010033.00 | 2738959.00 | 10498526.00 |
| bioenergetics | 4 | 3254383.50 | 2351309.00 | 5900607.00 |
| biogeography | 4 | 3227401.00 | 2274999.00 | 4666338.00 |
| bioinformatics | 4 | 2219989.00 | 1977209.00 | 2452219.00 |
| biomechanics | 4 | 1848639.25 | 1778790.00 | 2017939.00 |
| biophysics | 4 | 3068671.25 | 2613499.00 | 3845128.00 |
| biostatistics | 5 | 2186051.00 | 1923639.00 | 2504969.00 |
| cancer_biology | 4 | 1884039.25 | 1829739.00 | 1934869.00 |
| cell | 4 | 1994514.00 | 1816119.00 | 2372089.00 |
| chronobiology | 4 | 1968166.50 | 1860429.00 | 2133809.00 |
| cryobiology | 4 | 2166069.25 | 1963989.00 | 2451709.00 |
| developmental | 3 | 1883109.00 | 1866539.00 | 1895879.00 |
| dispatch | 7 | 1809857.88 | 1682540.00 | 1907299.00 |
| ecology | 4 | 1821346.50 | 1800289.00 | 1871009.00 |
| endocrinology | 4 | 1825164.25 | 1801319.00 | 1869779.00 |
| enzyme | 7 | 1865876.25 | 1787419.00 | 1988950.00 |
| epigenetics | 4 | 1888269.25 | 1803699.00 | 1963089.00 |
| ethology | 4 | 2401241.50 | 1912059.00 | 3792648.00 |
| evolution | 4 | 2121961.50 | 1925159.00 | 2308969.00 |
| genetics | 4 | 2622384.00 | 2315369.00 | 2833199.00 |
| genomics | 4 | 2291646.25 | 1887440.00 | 2466358.00 |
| immunology | 4 | 2117904.25 | 1935659.00 | 2346389.00 |
| marine_biology | 4 | 2220326.50 | 2000309.00 | 2526989.00 |
| microbiology | 4 | 2506138.75 | 1968609.00 | 3084289.00 |
| mycology | 4 | 2089379.25 | 1850959.00 | 2549219.00 |
| neuroscience | 4 | 1879069.00 | 1852379.00 | 1910339.00 |
| nutrition | 4 | 1923526.75 | 1865179.00 | 2030039.00 |
| paleobiology | 4 | 1886776.75 | 1842559.00 | 1971260.00 |
| parasitology | 3 | 2062749.00 | 1930639.00 | 2180789.00 |
| pharmacology | 4 | 1959654.25 | 1904209.00 | 2048649.00 |
| phylogenetics | 3 | 2148432.25 | 1973689.00 | 2288709.00 |
| physiology | 4 | 1921564.00 | 1882189.00 | 1975999.00 |
| plant_biology | 4 | 1953099.25 | 1887439.00 | 2052199.00 |
| population | 4 | 1925036.50 | 1833369.00 | 2126848.00 |
| proteomics | 4 | 2187944.25 | 2017939.00 | 2516799.00 |
| radiobiology | 4 | 2134114.00 | 1931849.00 | 2311359.00 |
| reproduction | 4 | 2074001.50 | 1892929.00 | 2159619.00 |
| stem_cell | 3 | 1900679.38 | 1880849.00 | 1925019.00 |
| structural | 4 | 1949956.50 | 1891369.00 | 2033779.00 |
| synthetic_biology | 4 | 2037326.75 | 1884149.00 | 2236349.00 |
| systems_biology | 3 | 1987315.62 | 1841209.00 | 2125069.00 |
| tissue_engineering | 4 | 2367819.00 | 2120669.00 | 2974159.00 |
| toxicology | 4 | 2010489.00 | 1919359.00 | 2086099.00 |
| virology | 4 | 1889051.75 | 1835129.00 | 1944849.00 |

### Top 10 Fastest Entries
| # | Entry | Avg (ns) |
|---|---|---|
| 1 | dispatch_telomere_shortening | 1682540.00 |
| 2 | dispatch_gompertz_mortality_rate | 1763889.00 |
| 3 | biomechanics_biomech_cardiac_output | 1778790.00 |
| 4 | dispatch_soil_respiration | 1778969.00 |
| 5 | enzyme_michaelis_menten_half_vmax | 1787419.00 |
| 6 | biomechanics_gait_stride_length | 1790669.00 |
| 7 | ecology_inverse_simpson | 1800289.00 |
| 8 | endocrinology_receptor_saturation | 1801319.00 |
| 9 | endocrinology_hormone_half_life_clearance | 1803669.00 |
| 10 | epigenetics_rnai_knockdown_efficiency | 1803699.00 |

### Top 10 Slowest Entries
| # | Entry | Avg (ns) |
|---|---|---|
| 1 | bioelectricity_electrotonic_length | 10498526.00 |
| 2 | bioelectricity_impedance_tissue | 6269367.00 |
| 3 | aging_ros_steady_state | 6062787.00 |
| 4 | bioenergetics_respiratory_control_index | 5900607.00 |
| 5 | biogeography_wallace_line_effect | 4666338.00 |
| 6 | bioelectricity_cable_equation_steady | 4533278.00 |
| 7 | aging_telomere_shortening | 4275508.00 |
| 8 | biophysics_harmonic_bond | 3845128.00 |
| 9 | ethology_territory_size | 3792648.00 |
| 10 | biogeography_latitudinal_diversity_gradient | 3486678.00 |

