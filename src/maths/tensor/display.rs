use super::storage::Tensor;
use std::fmt;

impl fmt::Display for Tensor {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if self.rank() == 0 {
            write!(f, "{:.6}", self.data[0])
        } else if self.rank() == 1 {
            write!(f, "[")?;
            for i in 0..self.shape[0] {
                if i > 0 {
                    write!(f, ", ")?;
                }
                write!(f, "{:.6}", self.get(&[i]))?;
            }
            write!(f, "]")
        } else if self.rank() == 2 {
            for i in 0..self.shape[0] {
                write!(f, "[")?;
                for j in 0..self.shape[1] {
                    if j > 0 {
                        write!(f, ", ")?;
                    }
                    write!(f, "{:>12.6}", self.get(&[i, j]))?;
                }
                writeln!(f, "]")?;
            }
            Ok(())
        } else {
            write!(f, "Tensor(shape={:?})", self.shape)
        }
    }
}
