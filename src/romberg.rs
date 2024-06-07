const PRECISION: u64 = 15;
const ERROR_PRECISION: u64 = 20;

#[inline(always)]
fn riemann_trapezoid(f: &impl Fn(f64) -> f64, a: f64, b: f64, n: u64) -> f64 {
    let h = (b - a) / (2 * n) as f64;
    let mut total = 0.0;
    for i in 0..n {
        let x = a + (h * (2 * i + 1) as f64);
        total += 2.0 * f(x)
    }
    return total * (h / 2.0);
}

pub fn integrate(f: impl Fn(f64) -> f64, a: f64, b: f64) -> Option<f64> {
    assert!(b > a);
    let mut matrix = [[0.0; ERROR_PRECISION as usize + 1]; ERROR_PRECISION as usize + 1];
    matrix[1][1] = riemann_trapezoid(&f, a, b, 1);

    let mut i = 1;
    let mut error = 1.0;
    while i < PRECISION || error > 0.0001 {
        if i == ERROR_PRECISION {
            return None;
        }

        let n = 2u64.pow(i as u32);
        matrix[i as usize + 1][1] = matrix[i as usize][1] / 2.0 + riemann_trapezoid(&f, a, b, n);
        for k in 2..(i + 2) {
            let j = (2 + i - k) as usize;
            matrix[j][k as usize] = ((4u64.pow(k as u32 - 1) as f64
                * matrix[j + 1][k as usize - 1])
                - matrix[j][k as usize - 1])
                / (4u64.pow(k as u32 - 1) as f64 - 1.0)
        }

        error = ((matrix[1][i as usize] - matrix[1][i as usize - 1])
            / (matrix[1][i as usize] + matrix[1][i as usize - 1]))
            .abs();
        i += 1;
    }

    let result = matrix[1][i as usize];
    if result.is_nan() || result.is_infinite() {
        return None;
    }
    Some(result)
}

#[cfg(test)]
mod tests {
    use super::*;

    macro_rules! assert_in_range {
        ($e:expr, $expected:expr) => {{
            const EPSILON: f64 = 0.001;
            assert!((($expected - EPSILON)..=($expected + EPSILON)).contains(&$e));
        }};
    }

    #[test]
    fn basic_function() {
        let area = integrate(|x| x.powi(2), -2.5, 3.0).unwrap();
        assert_in_range!(area, 14.2083333333)
    }

    #[test]
    fn interesting_function() {
        let area = integrate(
            |x| x.powi(2) - x.sqrt() / 42.0 - x.sin() * 20.0 - x.powi(3) / 10.0,
            0.4,
            10.5,
        )
        .unwrap();
        assert_in_range!(area, 53.5097395925)
    }

    #[test]
    fn simple_improper_integral() {
        let result = integrate(|x| 1.0 / x, 0.0, 3.0);
        assert!(result.is_none());
    }

    #[test]
    fn improper_integral() {
        let result = integrate(|x| x.tan(), 0.0, 3.0);
        assert!(result.is_none());
    }
}
