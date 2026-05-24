#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum BenchmarkEncodeError {
    BufferTooSmall,
    InvalidFormat,
}
