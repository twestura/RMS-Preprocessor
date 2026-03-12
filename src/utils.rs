//! Utility functions.

pub type Pointf64 = (f64, f64);
pub type Pointu32 = (u32, u32);

/// Returns `theta.sin()`.
pub fn sin(theta: f64) -> f64 {
    theta.sin()
}

/// Returns `theta.cos()`.
pub fn cos(theta: f64) -> f64 {
    theta.cos()
}

/// Returns `x.round()`.
pub fn round(x: f64) -> u32 {
    x.round() as u32
}
