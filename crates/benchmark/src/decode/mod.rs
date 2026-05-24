mod compact;
mod full;

pub use compact::decode_compact;
pub use full::decode_full;

use crate::engine::{BenchmarkEncodeError, BenchmarkMetrics};

pub fn decode<'a>(bytes: &'a [u8]) -> Result<BenchmarkMetrics<'a>, BenchmarkEncodeError> {
    decode_full(bytes).or_else(|_| decode_compact(bytes))
}
