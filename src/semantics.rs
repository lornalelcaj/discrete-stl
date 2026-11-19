use crate::formula::Formula;
use crate::trace::Trace;

pub fn satisfies(trace: &Trace, formula: &Formula, time: usize) -> bool {
    match formula {
        Formula::True => true,

        Formula::Atom(name) => {
            trace.value_at(name, time).unwrap_or(false)
        }

        Formula::Not(inner) => {
            !satisfies(trace, inner, time)
        }

        Formula::And(left, right) => {
            satisfies(trace, left, time)
                && satisfies(trace, right, time)
        }

        Formula::Or(left, right) => {
            satisfies(trace, left, time)
                || satisfies(trace, right, time)
        }

        Formula::Eventually {
            local,
            global,
            formula,
        } => {
            let horizon = trace.horizon();

            for witness in time..horizon {
                // Local condition:
                // witness - time ∈ I
                if !local.contains(witness - time) {
                    continue;
                }

                // Global condition:
                // witness ∈ K
                if !global.contains(witness) {
                    continue;
                }

                // φ must hold at the witness
                if satisfies(trace, formula, witness) {
                    return true;
                }
            }

            false
        }

        Formula::Globally {
            local,
            global,
            formula,
        } => {
            let horizon = trace.horizon();

            for position in time..horizon {
                // Local condition:
                // position - time ∈ I
                if !local.contains(position - time) {
                    continue;
                }

                // Global condition:
                // position ∈ K
                if !global.contains(position) {
                    continue;
                }

                // φ must hold at every admissible position
                if !satisfies(trace, formula, position) {
                    return false;
                }
            }

            // If there are no admissible positions,
            // Globally is vacuously true.
            true
        }

        Formula::Until {
            local,
            global,
            left,
            right,
        } => {
            let horizon = trace.horizon();

            for witness in time..horizon {
                // Local witness constraint:
                // witness - time ∈ I
                if !local.contains(witness - time) {
                    continue;
                }

                // Global witness constraint:
                // witness ∈ K
                if !global.contains(witness) {
                    continue;
                }

                // ψ must hold at the witness
                if !satisfies(trace, right, witness) {
                    continue;
                }

                // φ must hold at every position from
                // time up to and INCLUDING the witness.
                let mut left_holds = true;

                for position in time..=witness {
                    if !satisfies(trace, left, position) {
                        left_holds = false;
                        break;
                    }
                }

                if left_holds {
                    return true;
                }
            }

            false
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::formula::Formula;
    use crate::interval::Interval;
    use crate::trace::Trace;

    #[test]
    fn until_is_true_with_valid_witness() {
        let mut trace = Trace::new();

        trace.add_signal(
            "p",
            vec![true, true, true, true, false],
        );

        trace.add_signal(
            "q",
            vec![false, false, false, true, false],
        );

        let formula = Formula::Until {
            local: Interval {
                lower: 1,
                upper: 3,
            },

            global: Interval {
                lower: 0,
                upper: 4,
            },

            left: Box::new(
                Formula::Atom("p".to_string())
            ),

            right: Box::new(
                Formula::Atom("q".to_string())
            ),
        };

        assert!(satisfies(&trace, &formula, 1));
    }

    #[test]
    fn until_is_false_when_psi_never_holds() {
        let mut trace = Trace::new();

        trace.add_signal(
            "p",
            vec![true, true, true, true],
        );

        trace.add_signal(
            "q",
            vec![false, false, false, false],
        );

        let formula = Formula::Until {
            local: Interval {
                lower: 1,
                upper: 3,
            },

            global: Interval {
                lower: 0,
                upper: 3,
            },

            left: Box::new(
                Formula::Atom("p".to_string())
            ),

            right: Box::new(
                Formula::Atom("q".to_string())
            ),
        };

        assert!(!satisfies(&trace, &formula, 0));
    }

    #[test]
    fn until_is_false_when_phi_fails_before_witness() {
        let mut trace = Trace::new();

        trace.add_signal(
            "p",
            vec![true, false, true, true],
        );

        trace.add_signal(
            "q",
            vec![false, false, false, true],
        );

        let formula = Formula::Until {
            local: Interval {
                lower: 1,
                upper: 3,
            },

            global: Interval {
                lower: 0,
                upper: 3,
            },

            left: Box::new(
                Formula::Atom("p".to_string())
            ),

            right: Box::new(
                Formula::Atom("q".to_string())
            ),
        };

        assert!(!satisfies(&trace, &formula, 0));
    }

    #[test]
    fn eventually_is_true_when_formula_holds_at_valid_position() {
        let mut trace = Trace::new();

        trace.add_signal(
            "p",
            vec![false, false, true, false],
        );

        let formula = Formula::Eventually {
            local: Interval {
                lower: 1,
                upper: 3,
            },

            global: Interval {
                lower: 0,
                upper: 3,
            },

            formula: Box::new(
                Formula::Atom("p".to_string())
            ),
        };

        assert!(satisfies(&trace, &formula, 0));
    }

    #[test]
    fn globally_is_true_when_formula_holds_everywhere_required() {
        let mut trace = Trace::new();

        trace.add_signal(
            "p",
            vec![true, true, true, false],
        );

        let formula = Formula::Globally {
            local: Interval {
                lower: 0,
                upper: 2,
            },

            global: Interval {
                lower: 0,
                upper: 3,
            },

            formula: Box::new(
                Formula::Atom("p".to_string())
            ),
        };

        assert!(satisfies(&trace, &formula, 0));
    }

    #[test]
    fn globally_is_false_when_formula_fails_at_required_position() {
        let mut trace = Trace::new();

        trace.add_signal(
            "p",
            vec![true, false, true, true],
        );

        let formula = Formula::Globally {
            local: Interval {
                lower: 0,
                upper: 2,
            },

            global: Interval {
                lower: 0,
                upper: 3,
            },

            formula: Box::new(
                Formula::Atom("p".to_string())
            ),
        };

        assert!(!satisfies(&trace, &formula, 0));
    }

    #[test]
    fn not_and_or_work_correctly() {
        let mut trace = Trace::new();

        trace.add_signal(
            "p",
            vec![true],
        );

        trace.add_signal(
            "q",
            vec![false],
        );

        let p = Formula::Atom("p".to_string());
        let q = Formula::Atom("q".to_string());

        let not_q = Formula::Not(
            Box::new(q.clone())
        );

        let and_formula = Formula::And(
            Box::new(p.clone()),
            Box::new(not_q),
        );

        let or_formula = Formula::Or(
            Box::new(q),
            Box::new(p),
        );

        assert!(satisfies(&trace, &and_formula, 0));
        assert!(satisfies(&trace, &or_formula, 0));
    }
}