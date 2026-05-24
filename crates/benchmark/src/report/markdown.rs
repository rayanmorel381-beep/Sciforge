use crate::engine::BenchmarkMetrics;
use sciforge_parser::markdown::{write_md_code_block, write_md_heading, write_md_row};

pub fn generate_markdown(metrics: &BenchmarkMetrics<'_>, result: &str) -> String {
    let mut out = String::with_capacity(1024);
    out.push_str(&write_md_heading(
        1,
        &format!("Benchmark Report: {}", metrics.experiment_name),
    ));
    out.push_str(&write_md_heading(2, "Summary"));
    out.push_str("| Metric | Value |\n|---|---|\n");
    write_md_row(&mut out, "Experiment", metrics.experiment_name);
    write_md_row(&mut out, "Precision", metrics.precision);
    write_md_row(&mut out, "Elapsed (ms)", &metrics.elapsed_ms.to_string());
    write_md_row(&mut out, "Iterations", &metrics.iterations.to_string());
    write_md_row(
        &mut out,
        "Avg time (ns)",
        &format!("{:.2}", metrics.avg_time_ns),
    );
    write_md_row(
        &mut out,
        "Min time (ns)",
        &format!("{:.2}", metrics.min_time_ns),
    );
    write_md_row(
        &mut out,
        "Max time (ns)",
        &format!("{:.2}", metrics.max_time_ns),
    );
    write_md_row(&mut out, "Std dev", &format!("{:.4}", metrics.time_stddev));
    write_md_row(
        &mut out,
        "Iterations/sec",
        &format!("{:.2}", metrics.iterations_per_sec),
    );

    if metrics.total_flops > 0 {
        write_md_row(&mut out, "Total FLOPS", &metrics.total_flops.to_string());
    }
    if metrics.step_count > 0 {
        write_md_row(&mut out, "Steps", &metrics.step_count.to_string());
    }
    if metrics.eval_accuracy > 0.0 {
        write_md_row(
            &mut out,
            "Accuracy",
            &format!("{:.6}", metrics.eval_accuracy),
        );
    }
    if metrics.eval_error > 0.0 {
        write_md_row(&mut out, "Error", &format!("{:.6}", metrics.eval_error));
    }
    if metrics.eval_r_squared > 0.0 {
        write_md_row(&mut out, "R²", &format!("{:.6}", metrics.eval_r_squared));
    }

    out.push_str(&write_md_heading(2, "Result"));
    out.push_str(&write_md_code_block(result));

    out
}
