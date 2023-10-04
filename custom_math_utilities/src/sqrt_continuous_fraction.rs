pub struct SqrtContinuousFractionCoefficients {
    pub a0: u64,
    pub period: Vec<u64>,
}

impl SqrtContinuousFractionCoefficients {
    pub fn new(n: u64) -> SqrtContinuousFractionCoefficients {
        determine_sqrt_continuous_fraction_coefficients(n)
    }
}

#[derive(PartialEq, Eq)]
struct CoefficientState {
    a: u64,
    i: u64,
    j: u64,
}

fn compute_next_coefficient(a0: u64, n: u64, curr_state: &CoefficientState) -> CoefficientState {
    let i = curr_state.a * curr_state.j - curr_state.i;
    let j = (n - i * i) / curr_state.j;
    let a = (i + a0) / j;

    CoefficientState { a, i, j }
}

fn determine_sqrt_continuous_fraction_coefficients(n: u64) -> SqrtContinuousFractionCoefficients {
    let a0 = (n as f64).sqrt().floor() as u64;

    if a0 * a0 == n {
        return SqrtContinuousFractionCoefficients { a0, period: vec![] };
    }

    let mut coeffiecient_states = vec![CoefficientState { a: a0, i: 0, j: 1 }];

    loop {
        let next_state = compute_next_coefficient(a0, n, coeffiecient_states.last().unwrap());

        if coeffiecient_states.contains(&next_state) {
            break;
        }

        coeffiecient_states.push(next_state);
    }

    SqrtContinuousFractionCoefficients {
        a0,
        period: coeffiecient_states
            .iter()
            .skip(1)
            .map(|state| state.a)
            .collect(),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn case(n: u64, coefficients: SqrtContinuousFractionCoefficients) {
        let calculated_coefficients = SqrtContinuousFractionCoefficients::new(n);

        assert_eq!(calculated_coefficients.a0, coefficients.a0);
        assert_eq!(calculated_coefficients.period, coefficients.period);
    }

    #[test]
    fn sqrt_continuous_fraction_cases() {
        case(
            2,
            SqrtContinuousFractionCoefficients {
                a0: 1,
                period: vec![2],
            },
        );
        case(
            3,
            SqrtContinuousFractionCoefficients {
                a0: 1,
                period: vec![1, 2],
            },
        );
        case(
            4,
            SqrtContinuousFractionCoefficients {
                a0: 2,
                period: vec![],
            },
        );
        case(
            5,
            SqrtContinuousFractionCoefficients {
                a0: 2,
                period: vec![4],
            },
        );
        case(
            6,
            SqrtContinuousFractionCoefficients {
                a0: 2,
                period: vec![2, 4],
            },
        );
        case(
            7,
            SqrtContinuousFractionCoefficients {
                a0: 2,
                period: vec![1, 1, 1, 4],
            },
        );
        case(
            8,
            SqrtContinuousFractionCoefficients {
                a0: 2,
                period: vec![1, 4],
            },
        );
        case(
            9,
            SqrtContinuousFractionCoefficients {
                a0: 3,
                period: vec![],
            },
        );
        case(
            10,
            SqrtContinuousFractionCoefficients {
                a0: 3,
                period: vec![6],
            },
        );
        case(
            11,
            SqrtContinuousFractionCoefficients {
                a0: 3,
                period: vec![3, 6],
            },
        );
        case(
            12,
            SqrtContinuousFractionCoefficients {
                a0: 3,
                period: vec![2, 6],
            },
        );
        case(
            13,
            SqrtContinuousFractionCoefficients {
                a0: 3,
                period: vec![1, 1, 1, 1, 6],
            },
        );
    }
}
