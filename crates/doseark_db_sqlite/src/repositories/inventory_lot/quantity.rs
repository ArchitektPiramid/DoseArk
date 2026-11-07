//! Normalizacja ilości magazynowych do precyzji używanej w repozytoriach

/// Rounds floating-point stock amounts to six decimal places
pub(crate) fn normalize_stock_quantity(value: f64) -> f64 {
    const SCALE: f64 = 1_000_000.0;

    if !value.is_finite() {
        return value;
    }

    let rounded = (value * SCALE).round() / SCALE;

    if rounded.abs() < 0.000_001 {
        0.0
    } else {
        rounded
    }
}
