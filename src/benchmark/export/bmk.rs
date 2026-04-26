use super::Entry;
use crate::benchmark::encode::{encode, encoded_size};
use std::collections::BTreeMap;
use std::fs;
use std::path::Path;

pub(super) fn build_bmk(
    entries: &[Entry<'_>],
    dir: &Path,
) -> std::io::Result<(BTreeMap<String, String>, usize)> {
    let bmk_dir = dir.join("bmk");
    fs::create_dir_all(&bmk_dir)?;

    let mut bmk_buf = Vec::new();
    let mut readable = BTreeMap::new();

    for entry in entries {
        let m = entry.metrics;
        let size = encoded_size(m).unwrap();
        let mut buf = vec![0u8; size];
        encode(m, &mut buf).unwrap();
        bmk_buf.extend_from_slice(&buf);

        let mut text = String::with_capacity(512);
        text.push_str(&format!("experiment_name: {}\n", m.experiment_name));
        text.push_str(&format!("precision:       {}\n", m.precision));
        text.push_str(&format!("elapsed_ms:      {}\n", m.elapsed_ms));
        text.push_str(&format!("iterations:      {}\n", m.iterations));
        text.push_str(&format!("avg_time_ns:     {:.2}\n", m.avg_time_ns));
        text.push_str(&format!("min_time_ns:     {:.2}\n", m.min_time_ns));
        text.push_str(&format!("max_time_ns:     {:.2}\n", m.max_time_ns));
        text.push_str(&format!("time_stddev:     {:.4}\n", m.time_stddev));
        text.push_str(&format!("iterations/sec:  {:.2}\n", m.iterations_per_sec));
        text.push_str(&format!("input_samples:   {}\n", m.input_samples));
        text.push_str(&format!("total_flops:     {}\n", m.total_flops));
        text.push_str(&format!("step_count:      {}\n", m.step_count));
        text.push_str(&format!("input_dim:       {}\n", m.input_dim));
        text.push_str(&format!("output_dim:      {}\n", m.output_dim));
        text.push_str(&format!("output_bytes:    {}\n", m.output_bytes));
        text.push_str(&format!("input_bytes:     {}\n", m.input_bytes));
        text.push_str(&format!("result_bytes:    {}\n", m.result_bytes));
        text.push_str(&format!("eval_error:      {:.6}\n", m.eval_error));
        text.push_str(&format!("eval_accuracy:   {:.6}\n", m.eval_accuracy));
        text.push_str(&format!("eval_r_squared:  {:.6}\n", m.eval_r_squared));
        text.push_str(&format!("eval_mae:        {:.6}\n", m.eval_mae));
        text.push_str(&format!("eval_samples:    {}\n", m.eval_samples));
        text.push_str(&format!("logical_cores:   {}\n", m.logical_cores));
        text.push_str(&format!("avg_freq_mhz:    {}\n", m.avg_frequency_mhz));
        text.push_str(&format!("max_freq_mhz:    {}\n", m.max_frequency_mhz));
        text.push_str(&format!("max_workers:     {}\n", m.max_workers));
        text.push_str(&format!(
            "cpu_utilization:  {:.2}\n",
            m.target_cpu_utilization
        ));
        text.push_str(&format!("binary_size:     {} bytes\n", size));

        let key = format!("{}.bmk", entry.label);
        readable.insert(key, text);
    }

    let bmk_name = dir
        .file_name()
        .and_then(|n| n.to_str())
        .unwrap_or("benchmark");
    fs::write(bmk_dir.join(format!("{bmk_name}.bmk")), &bmk_buf)?;

    Ok((readable, 1))
}
