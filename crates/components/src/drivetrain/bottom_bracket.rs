#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum BottomBracketType {
    ThreadedBsa,
    Pressfit,
    T47,
    SpindleThru,
}

#[derive(Debug, Clone)]
pub struct BottomBracket {
    pub bracket_type: BottomBracketType,
    pub spindle_diameter_mm: f64,
    pub shell_width_mm: f64,
    pub ceramic_bearings: bool,
}

impl BottomBracket {
    pub fn threaded(shell_width_mm: f64) -> Self {
        Self { bracket_type: BottomBracketType::ThreadedBsa, spindle_diameter_mm: 24.0, shell_width_mm, ceramic_bearings: false }
    }

    pub fn pressfit(shell_width_mm: f64) -> Self {
        Self { bracket_type: BottomBracketType::Pressfit, spindle_diameter_mm: 30.0, shell_width_mm, ceramic_bearings: false }
    }

    pub fn ceramic(bracket_type: BottomBracketType, shell_width_mm: f64) -> Self {
        Self { bracket_type, spindle_diameter_mm: 30.0, shell_width_mm, ceramic_bearings: true }
    }
}
