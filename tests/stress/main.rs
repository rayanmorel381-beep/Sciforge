use sciforge::benchmark::engine::BenchmarkMetrics;
use sciforge::benchmark::engine::bench;
use sciforge::benchmark::export::{Entry, export};
use std::fs;
use std::path::PathBuf;
use std::process::Command;
use std::sync::Arc;
use std::sync::atomic::{AtomicU64, Ordering};
use std::thread;
use std::time::Instant;

#[inline]
fn xorshift64(mut x: u64) -> u64 {
    x ^= x << 13;
    x ^= x >> 7;
    x ^= x << 17;
    x
}

fn stress_output_dir() -> PathBuf {
    let dir = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("output")
        .join("stress");
    fs::create_dir_all(&dir).expect("failed to create output/stress directory");
    dir
}

fn reset_stress_output_dir() -> PathBuf {
    let dir = stress_output_dir();
    if dir.exists() {
        fs::remove_dir_all(&dir).expect("failed to reset output/stress directory");
    }
    fs::create_dir_all(&dir).expect("failed to create output/stress directory");
    dir
}

fn metric<'a>(name: &'a str, elapsed_ms: u64, avg_time_ns: f32, ips: f32) -> BenchmarkMetrics<'a> {
    BenchmarkMetrics {
        experiment_name: name,
        precision: "f64",
        elapsed_ms,
        iterations: 1,
        input_samples: 0,
        avg_time_ns,
        last_time_ns: avg_time_ns,
        output_bytes: 0,
        total_flops: 0,
        step_count: 0,
        input_dim: 0,
        output_dim: 0,
        benchmark_flags: 0,
        input_bytes: 0,
        result_bytes: 0,
        min_time_ns: avg_time_ns,
        max_time_ns: avg_time_ns,
        time_stddev: 0.0,
        iterations_per_sec: ips,
        samples_per_sec: 0.0,
        eval_error: 0.0,
        eval_accuracy: 0.0,
        eval_r_squared: 0.0,
        eval_mae: 0.0,
        eval_samples: 0,
        eval_dataset_hash: 0,
        logical_cores: 0,
        avg_frequency_mhz: 0,
        max_frequency_mhz: 0,
        max_workers: 0,
        target_cpu_utilization: 0.0,
    }
}

fn sanitize_label(name: &str) -> String {
    let mut label = String::with_capacity(name.len());
    for ch in name.chars() {
        if ch.is_ascii_alphanumeric() {
            label.push(ch);
        } else {
            label.push('_');
        }
    }
    while label.contains("__") {
        label = label.replace("__", "_");
    }
    label.trim_matches('_').to_string()
}

fn make_symbol(name: &str) -> String {
    let parts: Vec<&str> = name.split('_').filter(|part| !part.is_empty()).collect();
    if parts.len() <= 1 {
        let mut chars = name.chars().filter(|ch| ch.is_ascii_alphanumeric());
        let first = chars.next().unwrap_or('?').to_ascii_uppercase();
        let second = chars.next().unwrap_or('x').to_ascii_lowercase();
        return format!("{first}{second}");
    }
    parts
        .iter()
        .take(3)
        .map(|part| part.chars().next().unwrap_or('?').to_ascii_uppercase())
        .collect()
}

fn target_code(target: &str) -> &'static str {
    match target {
        "benchmark" => "bm",
        "parser" => "pr",
        "constants" => "ct",
        "maths" => "ma",
        "physics" => "ph",
        "chemistry" => "ch",
        "biology" => "bi",
        "geology" => "ge",
        "astronomy" => "as",
        "meteorology" => "me",
        "hub" => "hb",
        "scientific_validation" => "sv",
        "scientific_properties" => "sp",
        _ => "ot",
    }
}

#[derive(Clone, Debug)]
struct StressJob {
    target: String,
    module: String,
    label: String,
    symbol: String,
    test_name: String,
    cargo_args: Vec<String>,
}

fn cargo_output(args: &[String]) -> std::process::Output {
    Command::new("cargo")
        .args(args)
        .current_dir(env!("CARGO_MANIFEST_DIR"))
        .output()
        .expect("failed to spawn cargo process")
}

fn is_artifact_test(target: &str, test_name: &str) -> bool {
    match target {
        "benchmark" => matches!(
            test_name,
            "export_generates_files" | "export_html_contains_entries" | "bmk_roundtrip_from_export"
        ),
        "parser" => matches!(
            test_name,
            "generate_csv_file"
                | "generate_json_file"
                | "generate_yaml_file"
                | "generate_markdown_file"
                | "generate_html_file"
        ),
        "scientific_validation" => test_name == "scientific_reference_validation_passes",
        _ => test_name == "generate_output_files",
    }
}

fn discover_test_jobs(target: &str, selector: &[&str]) -> Vec<StressJob> {
    let mut list_args: Vec<String> = selector.iter().map(|arg| (*arg).to_string()).collect();
    list_args.push("--".to_string());
    list_args.push("--list".to_string());

    let output = cargo_output(&list_args);
    assert!(
        output.status.success(),
        "failed to list tests for target {target}: {}",
        String::from_utf8_lossy(&output.stderr)
    );

    let stdout = String::from_utf8_lossy(&output.stdout);
    let mut jobs = Vec::new();

    for line in stdout.lines() {
        let Some((name, kind)) = line.rsplit_once(": ") else {
            continue;
        };
        if kind.trim() != "test" {
            continue;
        }

        let test_name = name.trim();
        if test_name.is_empty() {
            continue;
        }
        if is_artifact_test(target, test_name) {
            continue;
        }

        let leaf = test_name.rsplit("::").next().unwrap_or(test_name);
        let module = test_name.split("::").next().unwrap_or("root").to_string();
        let label = sanitize_label(&format!("{target}_{test_name}"));

        let mut cargo_args: Vec<String> = selector.iter().map(|arg| (*arg).to_string()).collect();
        cargo_args.push(test_name.to_string());
        cargo_args.push("--".to_string());
        cargo_args.push("--exact".to_string());
        cargo_args.push("--test-threads=1".to_string());

        jobs.push(StressJob {
            target: target.to_string(),
            module,
            label,
            symbol: make_symbol(leaf),
            test_name: test_name.to_string(),
            cargo_args,
        });
    }

    jobs.sort_by(|left, right| left.test_name.cmp(&right.test_name));
    for (idx, job) in jobs.iter_mut().enumerate() {
        job.label = format!("{target}_{:03}", idx + 1);
    }
    jobs
}

fn discover_all_test_jobs() -> (Vec<String>, Vec<StressJob>) {
    let selectors = [
        ("lib", vec!["test", "--lib"]),
        ("benchmark", vec!["test", "--test", "benchmark"]),
        ("parser", vec!["test", "--test", "parser"]),
        ("constants", vec!["test", "--test", "constants"]),
        ("maths", vec!["test", "--test", "maths"]),
        ("physics", vec!["test", "--test", "physics"]),
        ("chemistry", vec!["test", "--test", "chemistry"]),
        ("biology", vec!["test", "--test", "biology"]),
        ("geology", vec!["test", "--test", "geology"]),
        ("astronomy", vec!["test", "--test", "astronomy"]),
        ("meteorology", vec!["test", "--test", "meteorology"]),
        ("hub", vec!["test", "--test", "hub"]),
        (
            "scientific_validation",
            vec!["test", "--test", "scientific_validation"],
        ),
        (
            "scientific_properties",
            vec!["test", "--test", "scientific_properties"],
        ),
    ];

    let target_order = selectors
        .iter()
        .map(|(target, _)| (*target).to_string())
        .collect::<Vec<_>>();
    let mut jobs = Vec::new();

    for (target, selector) in selectors {
        jobs.extend(discover_test_jobs(target, &selector));
    }

    (target_order, jobs)
}

#[test]
#[ignore = "stress"]
fn stress_noise() {
    let logical_cores = thread::available_parallelism()
        .map(|n| n.get())
        .unwrap_or(1);
    let worker_count = logical_cores.saturating_mul(2).max(2);
    let rounds_per_worker: u64 = 1_000_000;

    let checksum = Arc::new(AtomicU64::new(0));
    let start = Instant::now();

    let mut handles = Vec::with_capacity(worker_count);
    for worker_id in 0..worker_count {
        let checksum = Arc::clone(&checksum);
        handles.push(thread::spawn(move || {
            let mut seed = 0x9E3779B97F4A7C15_u64
                ^ ((worker_id as u64 + 1).wrapping_mul(0xA24BAED4963EE407_u64));
            let mut acc = (worker_id as f64 + 1.0) * 0.123_456_789;
            let mut local_hash = 0u64;

            for i in 0..rounds_per_worker {
                seed = xorshift64(seed ^ i.rotate_left((worker_id % 32) as u32));
                let noise = ((seed >> 11) as f64) * (1.0 / ((1u64 << 53) as f64));
                let wave = ((i as f64 * 1e-6) + noise * 64.0).sin()
                    * ((i as f64 * 2e-6) + noise * 32.0).cos();
                acc = (acc + wave).mul_add(noise + 1e-12, (acc.abs() + 1.0).ln_1p());
                if !acc.is_finite() {
                    acc = noise + 1.0;
                }
                local_hash ^= seed.wrapping_mul((acc.to_bits() >> 1) ^ i.rotate_left(17));
            }

            checksum.fetch_xor(local_hash, Ordering::Relaxed);
            acc
        }));
    }

    let mut finite_count = 0usize;
    for h in handles {
        let result = h.join().expect("worker thread panicked");
        if result.is_finite() {
            finite_count += 1;
        }
    }

    let elapsed = start.elapsed();
    let elapsed_secs = elapsed.as_secs_f64();
    let total_rounds = rounds_per_worker.saturating_mul(worker_count as u64);
    let rounds_per_sec = if elapsed_secs > 0.0 {
        total_rounds as f64 / elapsed_secs
    } else {
        0.0
    };

    let bench_metrics = bench("stress_noise_pipeline", "f64", 200_000, || {
        let mut x = 1.0_f64;
        for i in 1..=64 {
            let t = i as f64 * 1e-3;
            x = (x + t.sin()).mul_add(t.cos(), (x.abs() + 1.0).ln_1p());
        }
        x
    });

    println!(
        "stress workers={}, rounds_total={}, elapsed_ms={}, rounds_per_sec={:.2}, checksum={}, avg_ns={:.2}, ips={:.2}",
        worker_count,
        total_rounds,
        elapsed.as_millis(),
        rounds_per_sec,
        checksum.load(Ordering::Relaxed),
        bench_metrics.avg_time_ns,
        bench_metrics.iterations_per_sec,
    );

    let elapsed_ms = elapsed.as_millis() as u64;
    let summary_metric = metric(
        "noise",
        elapsed_ms,
        if elapsed_ms > 0 {
            (elapsed_ms as f32 * 1_000_000.0) / total_rounds as f32
        } else {
            0.0
        },
        rounds_per_sec as f32,
    );

    let noise_pipeline_metric = metric(
        "noise_bench",
        bench_metrics.elapsed_ms,
        bench_metrics.avg_time_ns,
        bench_metrics.iterations_per_sec,
    );

    let worker_count_str = worker_count.to_string();
    let noise_result = format!(
        "workers={} rounds_per_worker={} rounds_total={} checksum={} rounds_per_sec={:.2}",
        worker_count,
        rounds_per_worker,
        total_rounds,
        checksum.load(Ordering::Relaxed),
        rounds_per_sec,
    );
    let bench_result = String::from("bench(stress_noise_pipeline)");

    let noise_entries = vec![
        Entry {
            metrics: &summary_metric,
            result: noise_result.as_str(),
            label: "noise_summary",
            tags: vec![
                ("category", "noise"),
                ("symbol", "NS"),
                ("name", "Noise Summary"),
                ("kind", "saturation"),
                ("workers", worker_count_str.as_str()),
            ],
            grid_row: Some(1),
            grid_col: Some(1),
        },
        Entry {
            metrics: &noise_pipeline_metric,
            result: bench_result.as_str(),
            label: "noise_pipeline",
            tags: vec![
                ("category", "noise"),
                ("symbol", "NB"),
                ("name", "Noise Pipeline"),
                ("kind", "bench"),
                ("workers", "1"),
            ],
            grid_row: Some(1),
            grid_col: Some(2),
        },
    ];
    export("Stress Noise", &noise_entries, &reset_stress_output_dir())
        .expect("failed to export stress report");

    assert_eq!(finite_count, worker_count);
    assert!(checksum.load(Ordering::Relaxed) != 0);
    assert!(elapsed_secs > 0.0);
    assert!(rounds_per_sec > 10_000.0);
    assert!(bench_metrics.avg_time_ns > 0.0);
    assert!(bench_metrics.iterations_per_sec > 0.0);
}

#[derive(Debug)]
struct RunResult {
    elapsed_ms: u128,
    success: bool,
    code: Option<i32>,
    stdout: String,
    stderr: String,
}

fn run_cargo(args: Vec<String>) -> RunResult {
    let start = Instant::now();
    let output = cargo_output(&args);

    RunResult {
        elapsed_ms: start.elapsed().as_millis(),
        success: output.status.success(),
        code: output.status.code(),
        stdout: String::from_utf8_lossy(&output.stdout).into_owned(),
        stderr: String::from_utf8_lossy(&output.stderr).into_owned(),
    }
}

#[test]
#[ignore = "stress"]
fn stress_global() {
    let (target_order, jobs) = discover_all_test_jobs();
    let total_jobs = jobs.len();
    let launch_start = Instant::now();

    let mut handles = Vec::with_capacity(total_jobs);
    for job in jobs {
        handles.push(thread::spawn(move || {
            let result = run_cargo(job.cargo_args.clone());
            (job, result)
        }));
    }

    let mut results = Vec::with_capacity(total_jobs);
    for handle in handles {
        results.push(handle.join().expect("stress worker panicked"));
    }

    let total_elapsed_ms = launch_start.elapsed().as_millis();

    let successful_jobs = results.iter().filter(|(_, r)| r.success).count();
    let failed_jobs: Vec<&(StressJob, RunResult)> =
        results.iter().filter(|(_, r)| !r.success).collect();

    let sum_job_elapsed_ms: u128 = results.iter().map(|(_, r)| r.elapsed_ms).sum();
    let avg_job_elapsed_ms = if total_jobs > 0 {
        sum_job_elapsed_ms as f64 / total_jobs as f64
    } else {
        0.0
    };
    let max_job_elapsed_ms = results.iter().map(|(_, r)| r.elapsed_ms).max().unwrap_or(0);
    let min_job_elapsed_ms = results.iter().map(|(_, r)| r.elapsed_ms).min().unwrap_or(0);
    let saturation_factor = if total_elapsed_ms > 0 {
        sum_job_elapsed_ms as f64 / total_elapsed_ms as f64
    } else {
        0.0
    };

    println!(
        "saturation jobs_total={}, jobs_ok={}, jobs_failed={}, wall_ms={}, jobs_sum_ms={}, avg_job_ms={:.2}, min_job_ms={}, max_job_ms={}, saturation_factor={:.2}",
        total_jobs,
        successful_jobs,
        failed_jobs.len(),
        total_elapsed_ms,
        sum_job_elapsed_ms,
        avg_job_elapsed_ms,
        min_job_elapsed_ms,
        max_job_elapsed_ms,
        saturation_factor,
    );

    for (job, result) in &results {
        println!(
            "target={} test={} success={} code={:?} elapsed_ms={}",
            job.target, job.test_name, result.success, result.code, result.elapsed_ms
        );
    }

    let mut labels = Vec::with_capacity(results.len() + target_order.len() + 1);
    let mut categories = Vec::with_capacity(results.len() + target_order.len() + 1);
    let mut symbols = Vec::with_capacity(results.len() + target_order.len() + 1);
    let mut names = Vec::with_capacity(results.len() + target_order.len() + 1);
    let mut targets = Vec::with_capacity(results.len() + target_order.len() + 1);
    let mut modules = Vec::with_capacity(results.len() + target_order.len() + 1);
    let mut statuses = Vec::with_capacity(results.len() + target_order.len() + 1);
    let mut elapsed_tags = Vec::with_capacity(results.len() + target_order.len() + 1);
    let mut texts = Vec::with_capacity(results.len() + target_order.len() + 1);
    let mut elapsed_ms_vals = Vec::with_capacity(results.len() + target_order.len() + 1);
    let mut avg_ns_vals = Vec::with_capacity(results.len() + target_order.len() + 1);
    let mut ips_vals = Vec::with_capacity(results.len() + target_order.len() + 1);
    let mut grid_positions = Vec::with_capacity(results.len() + target_order.len() + 1);

    labels.push(String::from("stress_summary"));
    categories.push(String::from("overview"));
    symbols.push(String::from("SUM"));
    names.push(String::from("Stress Summary"));
    targets.push(String::from("stress"));
    modules.push(String::from("overview"));
    statuses.push(if failed_jobs.is_empty() {
        String::from("ok")
    } else {
        String::from("fail")
    });
    elapsed_tags.push(total_elapsed_ms.to_string());
    texts.push(format!(
        "jobs_total={} jobs_ok={} jobs_failed={} jobs_sum_ms={} min_job_ms={} max_job_ms={} saturation_factor={:.2}",
        total_jobs,
        successful_jobs,
        failed_jobs.len(),
        sum_job_elapsed_ms,
        min_job_elapsed_ms,
        max_job_elapsed_ms,
        saturation_factor
    ));
    elapsed_ms_vals.push(total_elapsed_ms as u64);
    avg_ns_vals.push(avg_job_elapsed_ms as f32 * 1_000_000.0);
    ips_vals.push(if total_elapsed_ms > 0 {
        1000.0 / total_elapsed_ms as f32
    } else {
        0.0
    });
    grid_positions.push(None);

    let cols_per_row: usize = 10;
    let mut current_row: u8 = 2;

    for target in &target_order {
        let mut target_results: Vec<&(StressJob, RunResult)> = results
            .iter()
            .filter(|(job, _)| &job.target == target)
            .collect();
        target_results.sort_by(|left, right| {
            left.0
                .module
                .cmp(&right.0.module)
                .then_with(|| left.0.test_name.cmp(&right.0.test_name))
        });
        if target_results.is_empty() {
            continue;
        }
        let target_count = target_results.len();
        let target_success = target_results.iter().filter(|(_, r)| r.success).count();
        let target_failures = target_count.saturating_sub(target_success);
        let target_sum_ms: u128 = target_results.iter().map(|(_, r)| r.elapsed_ms).sum();
        let target_avg_ms = if target_count > 0 {
            target_sum_ms as f32 / target_count as f32
        } else {
            0.0
        };
        let target_max_ms = target_results
            .iter()
            .map(|(_, r)| r.elapsed_ms)
            .max()
            .unwrap_or(0);
        let target_min_ms = target_results
            .iter()
            .map(|(_, r)| r.elapsed_ms)
            .min()
            .unwrap_or(0);
        let rows_needed = target_results.len().div_ceil(cols_per_row) as u8;

        labels.push(format!("{target}_summary"));
        categories.push(target.clone());
        symbols.push(String::from("SUM"));
        names.push(format!("{} Summary", target_code(target).to_uppercase()));
        targets.push(target.clone());
        modules.push(String::from("summary"));
        statuses.push(if target_failures == 0 {
            String::from("ok")
        } else {
            String::from("fail")
        });
        elapsed_tags.push(target_sum_ms.to_string());
        texts.push(format!(
            "tests_total={} tests_ok={} tests_failed={} elapsed_sum_ms={} min_ms={} max_ms={} avg_ms={:.2}",
            target_count,
            target_success,
            target_failures,
            target_sum_ms,
            target_min_ms,
            target_max_ms,
            target_avg_ms
        ));
        elapsed_ms_vals.push(target_sum_ms as u64);
        avg_ns_vals.push(target_avg_ms * 1_000_000.0);
        ips_vals.push(if target_sum_ms > 0 {
            1000.0 / target_sum_ms as f32
        } else {
            0.0
        });
        grid_positions.push(None);

        for (col_idx, (job, result)) in target_results.iter().enumerate() {
            let row = current_row + (col_idx / cols_per_row) as u8;
            let col = (col_idx % cols_per_row) as u8 + 1;
            let chunk = (col_idx / cols_per_row) + 1;
            labels.push(job.label.clone());
            categories.push(format!("{}{:02}", target_code(&job.target), chunk));
            symbols.push(job.symbol.clone());
            names.push(format!("{}{:03}", target_code(&job.target), col_idx + 1));
            targets.push(job.target.clone());
            modules.push(job.module.clone());
            statuses.push(if result.success {
                String::from("ok")
            } else {
                String::from("fail")
            });
            elapsed_tags.push(result.elapsed_ms.to_string());
            texts.push(format!(
                "success={} code={:?} stdout_bytes={} stderr_bytes={}",
                result.success,
                result.code,
                result.stdout.len(),
                result.stderr.len()
            ));
            elapsed_ms_vals.push(result.elapsed_ms as u64);
            avg_ns_vals.push(result.elapsed_ms as f32 * 1_000_000.0);
            ips_vals.push(if result.elapsed_ms > 0 {
                1000.0 / result.elapsed_ms as f32
            } else {
                0.0
            });
            grid_positions.push(Some((row, col)));
        }

        current_row = current_row.saturating_add(rows_needed + 1);
    }

    let metrics: Vec<BenchmarkMetrics<'_>> = labels
        .iter()
        .enumerate()
        .map(|(i, name)| {
            metric(
                name.as_str(),
                elapsed_ms_vals[i],
                avg_ns_vals[i],
                ips_vals[i],
            )
        })
        .collect();

    let entries: Vec<Entry<'_>> = metrics
        .iter()
        .enumerate()
        .map(|(i, m)| Entry {
            metrics: m,
            result: texts[i].as_str(),
            label: labels[i].as_str(),
            tags: vec![
                ("category", categories[i].as_str()),
                ("symbol", symbols[i].as_str()),
                ("name", names[i].as_str()),
                ("target", targets[i].as_str()),
                ("module", modules[i].as_str()),
                ("status", statuses[i].as_str()),
                ("elapsed_ms", elapsed_tags[i].as_str()),
            ],
            grid_row: grid_positions[i].map(|(row, _)| row),
            grid_col: grid_positions[i].map(|(_, col)| col),
        })
        .collect();

    export("Stress Global", &entries, &reset_stress_output_dir())
        .expect("failed to export stress report");

    assert_eq!(successful_jobs, total_jobs);
    assert!(total_elapsed_ms > 0);
    assert!(saturation_factor >= 1.2);
}
