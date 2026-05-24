//! Dispatch handler for FFT functions.

use super::super::params::*;
use super::helpers::*;
use crate::domain::common::errors::{HubError, HubResult};
use crate::domain::maths;
use crate::engine::experience::runner::RunOutput;

pub(super) fn dispatch(func: &str, p: &Params) -> HubResult<RunOutput> {
    match func {
        "fft" => Ok(out_cv(maths::fft::fft(&get_cv(p, "input")?))),
        "ifft" => Ok(out_cv(maths::fft::ifft(&get_cv(p, "input")?))),
        "fft_real" => Ok(out_cv(maths::fft::fft_real(get_v(p, "input")?))),
        "power_spectrum" => Ok(RunOutput::Vector(maths::fft::power_spectrum(get_v(
            p, "input",
        )?))),
        "fft_frequency_bins" => Ok(RunOutput::Vector(maths::fft::frequency_bins(
            get_u(p, "n")?,
            get_f(p, "sample_rate")?,
        ))),
        "fft_convolve" => Ok(RunOutput::Vector(maths::fft::convolve(
            get_v(p, "a")?,
            get_v(p, "b")?,
        ))),
        "fft_cross_correlate" => Ok(RunOutput::Vector(maths::fft::cross_correlate(
            get_v(p, "a")?,
            get_v(p, "b")?,
        ))),
        "fft_autocorrelation" => Ok(RunOutput::Vector(maths::fft::autocorrelation(get_v(
            p, "input",
        )?))),
        "fft_magnitude_spectrum" => Ok(RunOutput::Vector(maths::fft::magnitude_spectrum(get_v(
            p, "input",
        )?))),
        "fft_phase_spectrum" => Ok(RunOutput::Vector(maths::fft::phase_spectrum(get_v(
            p, "input",
        )?))),
        "cepstrum" => Ok(RunOutput::Vector(maths::fft::cepstrum(get_v(p, "input")?))),
        "fft_spectral_centroid" => Ok(RunOutput::Scalar(maths::fft::spectral_centroid(
            get_v(p, "magnitudes")?,
            get_f(p, "sample_rate")?,
        ))),
        "fft_spectral_rolloff" => Ok(RunOutput::Integer(maths::fft::spectral_rolloff(
            get_v(p, "magnitudes")?,
            get_f(p, "threshold")?,
        ) as i64)),
        "fft_spectral_flatness" => Ok(RunOutput::Scalar(maths::fft::spectral_flatness(get_v(
            p,
            "magnitudes",
        )?))),
        "hann_window" => Ok(RunOutput::Vector(maths::fft::hann_window(get_u(p, "n")?))),
        "hamming_window" => Ok(RunOutput::Vector(maths::fft::hamming_window(get_u(
            p, "n",
        )?))),
        "blackman_window" => Ok(RunOutput::Vector(maths::fft::blackman_window(get_u(
            p, "n",
        )?))),
        "kaiser_window" => Ok(RunOutput::Vector(maths::fft::kaiser_window(
            get_u(p, "n")?,
            get_f(p, "beta")?,
        ))),
        "windowed_fft" => Ok(out_cv(maths::fft::windowed_fft(
            get_v(p, "input")?,
            get_v(p, "window")?,
        ))),
        "fft_stft" => {
            let r = maths::fft::stft(
                get_v(p, "input")?,
                get_u(p, "window_size")?,
                get_u(p, "hop_size")?,
            );
            Ok(RunOutput::ComplexMatrix(
                r.into_iter()
                    .map(|frame| frame.iter().map(|c| (c.re, c.im)).collect())
                    .collect(),
            ))
        }
        "zero_pad" => Ok(RunOutput::Vector(maths::fft::zero_pad(
            get_v(p, "input")?,
            get_u(p, "target_len")?,
        ))),
        "deconvolve" => Ok(RunOutput::Vector(maths::fft::deconvolve(
            get_v(p, "signal")?,
            get_v(p, "kernel")?,
            get_f(p, "reg")?,
        ))),
        "fft_spectral_bandwidth" => Ok(RunOutput::Scalar(maths::fft::spectral_bandwidth(
            get_v(p, "magnitudes")?,
            get_f(p, "sample_rate")?,
        ))),
        "fft_spectral_entropy" => Ok(RunOutput::Scalar(maths::fft::spectral_entropy(get_v(
            p,
            "magnitudes",
        )?))),
        "bluestein_fft" => Ok(out_cv(maths::fft::bluestein_fft(&get_cv(p, "input")?))),
        "bluestein_ifft" => Ok(out_cv(maths::fft::bluestein_ifft(&get_cv(p, "input")?))),
        "fft_arbitrary" => Ok(out_cv(maths::fft::fft_arbitrary(&get_cv(p, "input")?))),
        "ifft_arbitrary" => Ok(out_cv(maths::fft::ifft_arbitrary(&get_cv(p, "input")?))),
        "goertzel" => {
            let r = maths::fft::goertzel(get_v(p, "input")?, get_f(p, "freq_bin")?);
            Ok(out_c(r))
        }
        "goertzel_mag" => Ok(RunOutput::Scalar(maths::fft::goertzel_mag(
            get_v(p, "input")?,
            get_f(p, "freq_bin")?,
        ))),
        "chirp_z_transform" => Ok(out_cv(maths::fft::chirp_z_transform(
            &get_cv(p, "input")?,
            get_u(p, "m")?,
            get_c(p, "w")?,
            get_c(p, "a")?,
        ))),
        "fft_shift" => Ok(out_cv(maths::fft::fft_shift(&get_cv(p, "input")?))),
        "ifft_shift" => Ok(out_cv(maths::fft::ifft_shift(&get_cv(p, "input")?))),
        "hilbert_transform" => Ok(out_cv(maths::fft::hilbert_transform(get_v(p, "input")?))),
        "analytic_signal" => Ok(out_cv(maths::fft::analytic_signal(get_v(p, "input")?))),
        "instantaneous_frequency" => Ok(RunOutput::Vector(maths::fft::instantaneous_frequency(
            get_v(p, "input")?,
            get_f(p, "sample_rate")?,
        ))),
        "sliding_dft" => Ok(out_cv(maths::fft::sliding_dft(
            get_v(p, "input")?,
            get_u(p, "k")?,
            get_u(p, "window")?,
        ))),
        "dct_ii" => Ok(RunOutput::Vector(maths::fft::dct_ii(get_v(p, "input")?))),
        "idct_ii" => Ok(RunOutput::Vector(maths::fft::idct_ii(get_v(p, "input")?))),
        "dst_i" => Ok(RunOutput::Vector(maths::fft::dst_i(get_v(p, "input")?))),
        "dct_i" => Ok(RunOutput::Vector(maths::fft::dct_i(get_v(p, "input")?))),
        "dct_iii" => Ok(RunOutput::Vector(maths::fft::dct_iii(get_v(p, "input")?))),
        "dct_iv" => Ok(RunOutput::Vector(maths::fft::dct_iv(get_v(p, "input")?))),
        "dst_ii" => Ok(RunOutput::Vector(maths::fft::dst_ii(get_v(p, "input")?))),
        "dst_iii" => Ok(RunOutput::Vector(maths::fft::dst_iii(get_v(p, "input")?))),
        "dst_iv" => Ok(RunOutput::Vector(maths::fft::dst_iv(get_v(p, "input")?))),
        "mdct" => Ok(RunOutput::Vector(maths::fft::mdct(get_v(p, "input")?))),
        "imdct" => Ok(RunOutput::Vector(maths::fft::imdct(get_v(p, "input")?))),
        "dct_2d" => Ok(RunOutput::Matrix(maths::fft::dct_2d(get_m(p, "input")?))),
        "idct_2d" => Ok(RunOutput::Matrix(maths::fft::idct_2d(get_m(p, "input")?))),
        "hartley_transform" => Ok(RunOutput::Vector(maths::fft::hartley_transform(get_v(
            p, "input",
        )?))),

        _ => Err(HubError::InvalidInput(format!("unknown function: {func}"))),
    }
}
