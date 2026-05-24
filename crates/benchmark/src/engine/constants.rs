pub const CSV_HEADER: &str = "experiment_name,precision,elapsed_ms,iterations,avg_time_ns,min_time_ns,max_time_ns,time_stddev,iterations_per_sec,result";

pub const BENCHMARK_MAGIC: [u8; 4] = [b'B', b'M', b'K', 0x01];
pub const BENCHMARK_VERSION: u16 = 5;
pub const BENCHMARK_HEADER_SIZE: usize = 4
    + 2
    + 2
    + 8
    + 8
    + 8
    + 8
    + 8
    + 8
    + 8
    + 4
    + 4
    + 4
    + 8
    + 8
    + 8
    + 8
    + 8
    + 8
    + 8
    + 8
    + 8
    + 8
    + 8
    + 8
    + 8
    + 8
    + 4
    + 4
    + 4
    + 4
    + 8
    + 2
    + 2;
