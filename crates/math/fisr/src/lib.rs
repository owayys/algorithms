pub fn fast_inverse_sqrt(x: f32) -> f32 {
    let x2 = x * 0.5;
    let i = x.to_bits();
    let i = 0x5f3759df - (i >> 1);
    let y = f32::from_bits(i);

    y * (1.5 - (x2 * y * y))
}

#[cfg(test)]
mod tests {
    use super::*;

    const RELATIVE_TOLERANCE: f32 = 0.005;

    fn relative_error(actual: f32, expected: f32) -> f32 {
        (actual - expected).abs() / expected
    }

    #[test]
    fn one_is_identity() {
        let result = fast_inverse_sqrt(1.0);
        assert!(
            relative_error(result, 1.0) < RELATIVE_TOLERANCE,
            "got {result}"
        );
    }

    #[test]
    fn four_gives_one_half() {
        let result = fast_inverse_sqrt(4.0);
        assert!(
            relative_error(result, 0.5) < RELATIVE_TOLERANCE,
            "got {result}"
        );
    }

    #[test]
    fn matches_std_across_a_range() {
        for x in [0.1_f32, 2.0, 10.0, 100.0, 1000.0, 65536.0] {
            let expected = 1.0 / x.sqrt();
            let result = fast_inverse_sqrt(x);
            assert!(
                relative_error(result, expected) < RELATIVE_TOLERANCE,
                "x={x}: got {result}, expected {expected}"
            );
        }
    }

    #[test]
    fn zero_does_not_panic() {
        let result = fast_inverse_sqrt(0.0);
        assert!(
            result.is_finite(),
            "expected a finite (if meaningless) value, got {result}"
        );
    }

    #[test]
    #[should_panic]
    fn negative_input_panics_on_underflow() {
        let _ = fast_inverse_sqrt(-4.0);
    }
}
