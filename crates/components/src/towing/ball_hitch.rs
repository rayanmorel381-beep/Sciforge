#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum BallSize {
    Mm50,
    Mm51,
    Mm57,
    Inch2,
}

#[derive(Debug, Clone)]
pub struct BallHitch {
    pub ball_size: BallSize,
    pub max_gross_kg: f64,
    pub max_tongue_kg: f64,
    pub removable: bool,
}

impl BallHitch {
    pub fn standard(ball_size: BallSize, max_gross_kg: f64) -> Self {
        Self { ball_size, max_gross_kg, max_tongue_kg: max_gross_kg * 0.1, removable: false }
    }

    pub fn removable(ball_size: BallSize, max_gross_kg: f64) -> Self {
        Self { ball_size, max_gross_kg, max_tongue_kg: max_gross_kg * 0.1, removable: true }
    }
}
