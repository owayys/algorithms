use std::fmt;

#[derive(Debug, PartialEq, Eq)]
pub enum CORDICError {
    TooManyIterations,
}

impl fmt::Display for CORDICError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            CORDICError::TooManyIterations => {
                write!(f, "Only up to 15 iterations are supported.")
            }
        }
    }
}

const CORDIC_GAIN: f64 = 0.60725293500888125;

const ARCTAN_TABLE: [f64; 15] = [
    0.7853981633974483,
    0.4636476090008061,
    0.24497866312686414,
    0.12435499454676144,
    0.06241881000216422,
    0.031239833430268277,
    0.015623728620476831,
    0.007812341060101111,
    0.003906230131967011,
    0.001953122516478818,
    0.0009765621895593195,
    0.0004882812111948303,
    0.00024414062014936177,
    0.00012207031189367021,
    0.00006103515617420877,
];

pub fn cordic(target: f64, iterations: u64) -> Result<(f64, f64), CORDICError> {
    if iterations as usize > ARCTAN_TABLE.len() {
        return Err(CORDICError::TooManyIterations);
    }

    let mut x = CORDIC_GAIN;
    let mut y = 0.0;
    let mut z = target;

    for i in 0..iterations {
        let pow2 = 2.0_f64.powi(-(i as i32));

        let sigma = if z >= 0.0 { 1.0 } else { -1.0 };

        let next_x = x - sigma * y * pow2;
        let next_y = y + sigma * x * pow2;

        x = next_x;
        y = next_y;
        z -= sigma * ARCTAN_TABLE[i as usize];
    }

    Ok((x, y))
}

#[cfg(test)]
mod tests {
    use super::*;

    const EPSILON: f64 = 1e-4;

    fn approx_eq(a: f64, b: f64, epsilon: f64) -> bool {
        (a - b).abs() < epsilon
    }

    #[test]
    fn zero_angle_gives_identity() {
        let (cos, sin) = cordic(0.0, 15).unwrap();
        assert!(approx_eq(cos, 1.0, EPSILON), "cos: {cos}");
        assert!(approx_eq(sin, 0.0, EPSILON), "sin: {sin}");
    }

    #[test]
    fn quarter_pi_matches_known_values() {
        let (cos, sin) = cordic(std::f64::consts::FRAC_PI_4, 15).unwrap();
        assert!(
            approx_eq(cos, std::f64::consts::FRAC_1_SQRT_2, EPSILON),
            "cos: {cos}"
        );
        assert!(
            approx_eq(sin, std::f64::consts::FRAC_1_SQRT_2, EPSILON),
            "sin: {sin}"
        );
    }

    #[test]
    fn sixth_pi_matches_known_values() {
        let (cos, sin) = cordic(std::f64::consts::FRAC_PI_6, 15).unwrap();
        assert!(approx_eq(cos, 0.8660254, EPSILON), "cos: {cos}");
        assert!(approx_eq(sin, 0.5, EPSILON), "sin: {sin}");
    }

    #[test]
    fn negative_angle_is_handled() {
        let (cos, sin) = cordic(-std::f64::consts::FRAC_PI_4, 15).unwrap();
        assert!(
            approx_eq(cos, std::f64::consts::FRAC_1_SQRT_2, EPSILON),
            "cos: {cos}"
        );
        assert!(
            approx_eq(sin, -std::f64::consts::FRAC_1_SQRT_2, EPSILON),
            "sin: {sin}"
        );
    }

    #[test]
    fn iterations_within_table_bound_succeeds() {
        assert!(cordic(std::f64::consts::FRAC_PI_4, 15).is_ok());
    }

    #[test]
    fn iterations_exceeding_table_bound_errors() {
        assert!(cordic(std::f64::consts::FRAC_PI_4, 16).is_err());
    }

    #[test]
    fn accuracy_improves_with_more_iterations() {
        let target = std::f64::consts::FRAC_PI_4;
        let (cos_few, _) = cordic(target, 3).unwrap();
        let (cos_many, _) = cordic(target, 15).unwrap();
        let expected = std::f64::consts::FRAC_1_SQRT_2;
        assert!(
            (cos_many - expected).abs() < (cos_few - expected).abs(),
            "Expected more iterations to be more accurate: few={cos_few}, many={cos_many}"
        );
    }
}
