use crate::formula::Formula;
use crate::interval::Interval;

pub fn split_global(
    global: &Interval,
    tau: usize,
    horizon: usize,
) -> (
    Option<Interval>,
    Option<Interval>,
    Option<Interval>,
) {
    let before = if tau == 0 {
        None
    } else {
        global.intersection(&Interval {
            lower: 0,
            upper: tau - 1,
        })
    };

    let at_tau = global.intersection(&Interval {
        lower: tau,
        upper: tau,
    });

    let after = if tau + 1 >= horizon {
        None
    } else {
        global.intersection(&Interval {
            lower: tau + 1,
            upper: horizon - 1,
        })
    };

    (before, at_tau, after)
}

pub fn global_separate(
    formula: &Formula,
    tau: usize,
    horizon: usize,
) -> Option<Formula> {
    let Formula::Until {
        local,
        global,
        left,
        right,
    } = formula
    else {
        return None;
    };

    let (k_minus, k_zero, k_plus) =
        split_global(global, tau, horizon);

    let before_branch = k_minus.map(|km| {
        Formula::Until {
            local: local.clone(),
            global: km,
            left: left.clone(),
            right: right.clone(),
        }
    });

    let phi_before_tau = if tau == 0 {
        Formula::True
    } else {
        Formula::Globally {
            local: Interval {
                lower: 0,
                upper: horizon - 1,
            },
            global: Interval {
                lower: 0,
                upper: tau - 1,
            },
            formula: left.clone(),
        }
    };

    let phi_at_tau = Formula::Globally {
        local: Interval {
            lower: 0,
            upper: horizon - 1,
        },
        global: Interval {
            lower: tau,
            upper: tau,
        },
        formula: left.clone(),
    };

    let exact_tau_branch = k_zero.map(|k0| {
        Formula::Eventually {
            local: local.clone(),
            global: k0,
            formula: right.clone(),
        }
    });

    let after_tau_branch = k_plus.map(|kp| {
        let guard = Formula::Globally {
            local: Interval {
                lower: 0,
                upper: 0,
            },
            global: Interval {
                lower: tau + 1,
                upper: horizon - 1,
            },
            formula: left.clone(),
        };

        Formula::Until {
            local: local.clone(),
            global: kp,
            left: Box::new(guard),
            right: right.clone(),
        }
    });

    let witness_after_or_at = match (exact_tau_branch, after_tau_branch) {
        (Some(exact), Some(after)) => {
            Formula::Or(
                Box::new(exact),
                Box::new(after),
            )
        }

        (Some(exact), None) => exact,

        (None, Some(after)) => after,

        (None, None) => {
            return before_branch;
        }
    };

    let main_branch = Formula::And(
        Box::new(phi_before_tau),
        Box::new(
            Formula::And(
                Box::new(phi_at_tau),
                Box::new(witness_after_or_at),
            )
        ),
    );

    let result = match before_branch {
        Some(before) => {
            Formula::Or(
                Box::new(before),
                Box::new(main_branch),
            )
        }

        None => main_branch,
    };

    Some(result)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn split_global_into_three_parts() {
        let global = Interval {
            lower: 1,
            upper: 5,
        };

        let (before, at_tau, after) =
            split_global(&global, 3, 7);

        assert_eq!(
            before,
            Some(Interval {
                lower: 1,
                upper: 2,
            })
        );

        assert_eq!(
            at_tau,
            Some(Interval {
                lower: 3,
                upper: 3,
            })
        );

        assert_eq!(
            after,
            Some(Interval {
                lower: 4,
                upper: 5,
            })
        );
    }

    #[test]
    fn split_global_tau_zero_has_no_before_part() {
        let global = Interval {
            lower: 0,
            upper: 4,
        };

        let (before, at_tau, after) =
            split_global(&global, 0, 5);

        assert_eq!(before, None);

        assert_eq!(
            at_tau,
            Some(Interval {
                lower: 0,
                upper: 0,
            })
        );

        assert_eq!(
            after,
            Some(Interval {
                lower: 1,
                upper: 4,
            })
        );
    }

    #[test]
    fn split_global_tau_outside_global_interval() {
        let global = Interval {
            lower: 0,
            upper: 2,
        };

        let (_, at_tau, after) =
            split_global(&global, 4, 6);

        assert_eq!(at_tau, None);
        assert_eq!(after, None);
    }
}
#[test]
fn global_separation_preserves_semantics() {
    use crate::semantics::satisfies;
    use crate::trace::Trace;

    let mut trace = Trace::new();

    trace.add_signal(
        "p",
        vec![true, true, true, true, false, false],
    );

    trace.add_signal(
        "q",
        vec![false, false, false, true, false, false],
    );

    let original = Formula::Until {
        local: Interval {
            lower: 1,
            upper: 4,
        },

        global: Interval {
            lower: 0,
            upper: 5,
        },

        left: Box::new(
            Formula::Atom("p".to_string())
        ),

        right: Box::new(
            Formula::Atom("q".to_string())
        ),
    };

    let separated =
        global_separate(&original, 2, 6)
            .expect("Expected Until formula");

    let original_result =
        satisfies(&trace, &original, 0);

    let separated_result =
        satisfies(&trace, &separated, 0);

    assert_eq!(
        original_result,
        separated_result
    );
}

#[test]
fn exhaustive_global_separation_check() {
    use crate::semantics::satisfies;
    use crate::trace::Trace;

    let horizon = 4;

    let total_traces = 1usize << horizon;

    for p_mask in 0..total_traces {
        for q_mask in 0..total_traces {
            let p_values: Vec<bool> = (0..horizon)
                .map(|i| (p_mask & (1 << i)) != 0)
                .collect();

            let q_values: Vec<bool> = (0..horizon)
                .map(|i| (q_mask & (1 << i)) != 0)
                .collect();

            let mut trace = Trace::new();

            trace.add_signal("p", p_values);
            trace.add_signal("q", q_values);

            for time in 0..horizon {
                for local_lower in 0..horizon {
                    for local_upper in local_lower..horizon {
                        for global_lower in 0..horizon {
                            for global_upper in global_lower..horizon {
                                for tau in 0..horizon {
                                    let original = Formula::Until {
                                        local: Interval {
                                            lower: local_lower,
                                            upper: local_upper,
                                        },

                                        global: Interval {
                                            lower: global_lower,
                                            upper: global_upper,
                                        },

                                        left: Box::new(
                                            Formula::Atom("p".to_string())
                                        ),

                                        right: Box::new(
                                            Formula::Atom("q".to_string())
                                        ),
                                    };

                                    let separated = global_separate(
                                        &original,
                                        tau,
                                        horizon,
                                    )
                                    .expect("Expected Until formula");

                                    let original_result =
                                        satisfies(&trace, &original, time);

                                    let separated_result =
                                        satisfies(&trace, &separated, time);

                                    assert_eq!(
                                        original_result,
                                        separated_result,
                                        "Mismatch found:
                                        time = {}
                                        local = [{}, {}]
                                        global = [{}, {}]
                                        tau = {}
                                        p_mask = {}
                                        q_mask = {}",
                                        time,
                                        local_lower,
                                        local_upper,
                                        global_lower,
                                        global_upper,
                                        tau,
                                        p_mask,
                                        q_mask
                                    );
                                }
                            }
                        }
                    }
                }
            }
        }
    }
}