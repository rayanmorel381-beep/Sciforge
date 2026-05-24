//! Dispatch handler for signal processing functions.

use super::super::params::*;
use crate::domain::common::errors::{HubError, HubResult};
use crate::domain::maths;
use crate::engine::experience::runner::RunOutput;

pub(super) fn dispatch(func: &str, p: &Params) -> HubResult<RunOutput> {
    match func {
        "low_pass_rc" => Ok(RunOutput::Vector(maths::signal::filters::low_pass_rc(
            get_v(p, "signal")?,
            get_f(p, "dt")?,
            get_f(p, "rc")?,
        ))),
        "high_pass_rc" => Ok(RunOutput::Vector(maths::signal::filters::high_pass_rc(
            get_v(p, "signal")?,
            get_f(p, "dt")?,
            get_f(p, "rc")?,
        ))),
        "sig_moving_average" => Ok(RunOutput::Vector(maths::signal::filters::moving_average(
            get_v(p, "signal")?,
            get_u(p, "window")?,
        ))),
        "sig_exponential_moving_average" => Ok(RunOutput::Vector(
            maths::signal::filters::exponential_moving_average(
                get_v(p, "signal")?,
                get_f(p, "alpha")?,
            ),
        )),
        "sig_median_filter" => Ok(RunOutput::Vector(maths::signal::filters::median_filter(
            get_v(p, "signal")?,
            get_u(p, "window")?,
        ))),
        "butterworth_gain" => Ok(RunOutput::Scalar(maths::signal::filters::butterworth_gain(
            get_f(p, "freq")?,
            get_f(p, "cutoff")?,
            get_i(p, "order")? as u32,
        ))),
        "chebyshev_gain" => Ok(RunOutput::Scalar(maths::signal::filters::chebyshev_gain(
            get_f(p, "freq")?,
            get_f(p, "cutoff")?,
            get_i(p, "order")? as u32,
            get_f(p, "ripple_db")?,
        ))),
        "sig_savitzky_golay_5" => Ok(RunOutput::Vector(maths::signal::filters::savitzky_golay_5(
            get_v(p, "signal")?,
        ))),
        "notch_filter" => Ok(RunOutput::Vector(maths::signal::filters::notch_filter(
            get_v(p, "signal")?,
            get_f(p, "dt")?,
            get_f(p, "freq")?,
            get_f(p, "bandwidth")?,
        ))),
        "band_pass_filter" => Ok(RunOutput::Vector(maths::signal::filters::band_pass_filter(
            get_v(p, "signal")?,
            get_f(p, "dt")?,
            get_f(p, "low_freq")?,
            get_f(p, "high_freq")?,
        ))),
        "gaussian_filter" => Ok(RunOutput::Vector(maths::signal::filters::gaussian_filter(
            get_v(p, "signal")?,
            get_f(p, "sigma")?,
        ))),
        "wiener_filter_frequency" => Ok(RunOutput::Vector(
            maths::signal::filters::wiener_filter_frequency(
                get_v(p, "signal_power")?,
                get_v(p, "noise_power")?,
            ),
        )),
        "kalman_filter_1d" => Ok(RunOutput::Vector(maths::signal::filters::kalman_filter_1d(
            get_v(p, "measurements")?,
            get_f(p, "process_noise")?,
            get_f(p, "measurement_noise")?,
            get_f(p, "initial_estimate")?,
        ))),
        "adaptive_lms_filter" => Ok(RunOutput::Vector(
            maths::signal::filters::adaptive_lms_filter(
                get_v(p, "input")?,
                get_v(p, "desired")?,
                get_u(p, "filter_length")?,
                get_f(p, "mu")?,
            ),
        )),
        "bessel_gain" => Ok(RunOutput::Scalar(maths::signal::filters::bessel_gain(
            get_f(p, "freq")?,
            get_f(p, "cutoff")?,
            get_i(p, "order")? as u32,
        ))),
        "sig_weighted_moving_average" => Ok(RunOutput::Vector(
            maths::signal::filters::weighted_moving_average(
                get_v(p, "signal")?,
                get_v(p, "weights")?,
            ),
        )),
        "derivative_filter" => Ok(RunOutput::Vector(
            maths::signal::filters::derivative_filter(get_v(p, "signal")?, get_f(p, "dt")?),
        )),

        "sig_convolve" => Ok(RunOutput::Vector(maths::signal::convolution::convolve(
            get_v(p, "a")?,
            get_v(p, "b")?,
        ))),
        "sig_cross_correlate" => Ok(RunOutput::Vector(
            maths::signal::convolution::cross_correlate(get_v(p, "a")?, get_v(p, "b")?),
        )),
        "sig_autocorrelation" => Ok(RunOutput::Vector(
            maths::signal::convolution::autocorrelation(get_v(p, "signal")?),
        )),
        "sig_deconvolve" => Ok(RunOutput::Vector(maths::signal::convolution::deconvolve(
            get_v(p, "signal")?,
            get_v(p, "kernel")?,
        ))),
        "matched_filter" => Ok(RunOutput::Vector(
            maths::signal::convolution::matched_filter(get_v(p, "signal")?, get_v(p, "template")?),
        )),
        "downsample" => Ok(RunOutput::Vector(maths::signal::convolution::downsample(
            get_v(p, "signal")?,
            get_u(p, "factor")?,
        ))),
        "upsample" => Ok(RunOutput::Vector(maths::signal::convolution::upsample(
            get_v(p, "signal")?,
            get_u(p, "factor")?,
        ))),
        "sig_zero_pad" => Ok(RunOutput::Vector(maths::signal::convolution::zero_pad(
            get_v(p, "signal")?,
            get_u(p, "total_length")?,
        ))),
        "convolve_same" => Ok(RunOutput::Vector(
            maths::signal::convolution::convolve_same(get_v(p, "a")?, get_v(p, "b")?),
        )),
        "convolve_valid" => Ok(RunOutput::Vector(
            maths::signal::convolution::convolve_valid(get_v(p, "a")?, get_v(p, "b")?),
        )),
        "circular_convolution" => Ok(RunOutput::Vector(
            maths::signal::convolution::circular_convolution(get_v(p, "a")?, get_v(p, "b")?),
        )),
        "overlap_add" => Ok(RunOutput::Vector(maths::signal::convolution::overlap_add(
            get_v(p, "signal")?,
            get_v(p, "kernel")?,
            get_u(p, "block_size")?,
        ))),
        "normalized_cross_correlation" => Ok(RunOutput::Vector(
            maths::signal::convolution::normalized_cross_correlation(
                get_v(p, "a")?,
                get_v(p, "b")?,
            ),
        )),
        "convolution_2d" => Ok(RunOutput::Matrix(
            maths::signal::convolution::convolution_2d(get_m(p, "image")?, get_m(p, "kernel")?),
        )),
        "wiener_deconvolution" => Ok(RunOutput::Vector(
            maths::signal::convolution::wiener_deconvolution(
                get_v(p, "signal")?,
                get_v(p, "kernel")?,
                get_f(p, "noise_power")?,
            ),
        )),

        "haar_transform" => Ok(RunOutput::Vector(maths::signal::wavelets::haar_transform(
            get_v(p, "signal")?,
        ))),
        "haar_inverse" => Ok(RunOutput::Vector(maths::signal::wavelets::haar_inverse(
            get_v(p, "coeffs")?,
        ))),
        "db4_scaling" => {
            let r = maths::signal::wavelets::db4_scaling();
            Ok(RunOutput::Vector(r.to_vec()))
        }
        "wavelet_energy" => Ok(RunOutput::Scalar(maths::signal::wavelets::wavelet_energy(
            get_v(p, "detail_coeffs")?,
        ))),
        "wavelet_entropy" => {
            let levels = get_m(p, "detail_levels")?.to_vec();
            Ok(RunOutput::Scalar(maths::signal::wavelets::wavelet_entropy(
                &levels,
            )))
        }
        "threshold_hard" => Ok(RunOutput::Vector(maths::signal::wavelets::threshold_hard(
            get_v(p, "coeffs")?,
            get_f(p, "threshold")?,
        ))),
        "threshold_soft" => Ok(RunOutput::Vector(maths::signal::wavelets::threshold_soft(
            get_v(p, "coeffs")?,
            get_f(p, "threshold")?,
        ))),
        "universal_threshold" => Ok(RunOutput::Scalar(
            maths::signal::wavelets::universal_threshold(get_u(p, "n")?, get_f(p, "sigma")?),
        )),
        "noise_estimate_mad" => Ok(RunOutput::Scalar(
            maths::signal::wavelets::noise_estimate_mad(get_v(p, "detail_coeffs")?),
        )),
        "db2_scaling" => {
            let r = maths::signal::wavelets::db2_scaling();
            Ok(RunOutput::Vector(r.to_vec()))
        }
        "morlet_wavelet" => Ok(RunOutput::Scalar(maths::signal::wavelets::morlet_wavelet(
            get_f(p, "t")?,
            get_f(p, "sigma")?,
        ))),
        "mexican_hat_wavelet" => Ok(RunOutput::Scalar(
            maths::signal::wavelets::mexican_hat_wavelet(get_f(p, "t")?, get_f(p, "sigma")?),
        )),
        "continuous_wavelet_transform_morlet" => Ok(RunOutput::Matrix(
            maths::signal::wavelets::continuous_wavelet_transform_morlet(
                get_v(p, "signal")?,
                get_v(p, "scales")?,
                get_f(p, "dt")?,
            ),
        )),
        "multiresolution_decomposition" => {
            let (approx, details) = maths::signal::wavelets::multiresolution_decomposition(
                get_v(p, "signal")?,
                get_u(p, "levels")?,
            );
            let mut result = vec![approx];
            result.extend(details);
            Ok(RunOutput::Matrix(result))
        }
        "wavelet_reconstruction_haar" => {
            let m = get_m(p, "data")?;
            let approx = &m[0];
            let details: Vec<Vec<f64>> = m[1..].to_vec();
            Ok(RunOutput::Vector(
                maths::signal::wavelets::wavelet_reconstruction_haar(approx, &details),
            ))
        }
        "wavelet_shrinkage_denoise" => Ok(RunOutput::Vector(
            maths::signal::wavelets::wavelet_shrinkage_denoise(
                get_v(p, "signal")?,
                get_u(p, "levels")?,
            ),
        )),
        "scalogram_energy" => Ok(RunOutput::Vector(
            maths::signal::wavelets::scalogram_energy(get_m(p, "cwt_coeffs")?),
        )),

        "power_spectral_density" => Ok(RunOutput::Vector(
            maths::signal::spectral::power_spectral_density(
                get_v(p, "signal")?,
                get_f(p, "sample_rate")?,
            ),
        )),
        "spectral_dft" => {
            let (re, im) = maths::signal::spectral::dft(get_v(p, "signal")?);
            Ok(RunOutput::Matrix(vec![re, im]))
        }
        "spectral_idft" => Ok(RunOutput::Vector(maths::signal::spectral::idft(
            get_v(p, "re")?,
            get_v(p, "im")?,
        ))),
        "spectral_magnitude_spectrum" => Ok(RunOutput::Vector(
            maths::signal::spectral::magnitude_spectrum(get_v(p, "re")?, get_v(p, "im")?),
        )),
        "spectral_phase_spectrum" => Ok(RunOutput::Vector(
            maths::signal::spectral::phase_spectrum(get_v(p, "re")?, get_v(p, "im")?),
        )),
        "spectral_frequency_bins" => Ok(RunOutput::Vector(
            maths::signal::spectral::frequency_bins(get_u(p, "n")?, get_f(p, "sample_rate")?),
        )),
        "spectral_centroid" => Ok(RunOutput::Scalar(
            maths::signal::spectral::spectral_centroid(
                get_v(p, "magnitudes")?,
                get_v(p, "frequencies")?,
            ),
        )),
        "spectral_bandwidth" => Ok(RunOutput::Scalar(
            maths::signal::spectral::spectral_bandwidth(
                get_v(p, "magnitudes")?,
                get_v(p, "frequencies")?,
                get_f(p, "centroid")?,
            ),
        )),
        "spectral_rolloff" => Ok(RunOutput::Integer(
            maths::signal::spectral::spectral_rolloff(
                get_v(p, "magnitudes")?,
                get_f(p, "threshold")?,
            ) as i64,
        )),
        "windowed_signal" => Ok(RunOutput::Vector(maths::signal::spectral::windowed_signal(
            get_v(p, "signal")?,
            get_v(p, "window")?,
        ))),
        "hanning_window" => Ok(RunOutput::Vector(maths::signal::spectral::hanning_window(
            get_u(p, "n")?,
        ))),
        "spectral_hamming_window" => Ok(RunOutput::Vector(
            maths::signal::spectral::hamming_window(get_u(p, "n")?),
        )),
        "spectral_blackman_window" => Ok(RunOutput::Vector(
            maths::signal::spectral::blackman_window(get_u(p, "n")?),
        )),
        "spectral_kaiser_window" => Ok(RunOutput::Vector(maths::signal::spectral::kaiser_window(
            get_u(p, "n")?,
            get_f(p, "beta")?,
        ))),
        "spectral_stft" => {
            let r = maths::signal::spectral::stft(
                get_v(p, "signal")?,
                get_u(p, "window_size")?,
                get_u(p, "hop_size")?,
            );
            Ok(RunOutput::Matrix(
                r.into_iter()
                    .map(|(re, im)| {
                        let mut row = re;
                        row.extend(im);
                        row
                    })
                    .collect(),
            ))
        }
        "cross_spectral_density" => Ok(RunOutput::Vector(
            maths::signal::spectral::cross_spectral_density(
                get_v(p, "signal_a")?,
                get_v(p, "signal_b")?,
                get_f(p, "sample_rate")?,
            ),
        )),
        "spectral_flatness" => Ok(RunOutput::Scalar(
            maths::signal::spectral::spectral_flatness(get_v(p, "magnitudes")?),
        )),
        "spectral_flux" => Ok(RunOutput::Scalar(maths::signal::spectral::spectral_flux(
            get_v(p, "current")?,
            get_v(p, "previous")?,
        ))),
        "bark_scale" => Ok(RunOutput::Scalar(maths::signal::spectral::bark_scale(
            get_f(p, "freq")?,
        ))),
        "mel_scale" => Ok(RunOutput::Scalar(maths::signal::spectral::mel_scale(
            get_f(p, "freq")?,
        ))),
        "inverse_mel" => Ok(RunOutput::Scalar(maths::signal::spectral::inverse_mel(
            get_f(p, "mel")?,
        ))),

        _ => Err(HubError::InvalidInput(format!("unknown function: {func}"))),
    }
}
