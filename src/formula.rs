// This file defines the syntax of the discrete STL-GT formulas used in the thesis.
// It does not evaluate formulas; it only provides a structured representation of them.
// The temporal operators explicitly store both the local interval I_d and the global
// interval K_d, matching the discrete semantics where the witness j must satisfy
// j - k ∈ I_d and j ∈ K_d.

use crate::interval::Interval;

#[derive(Debug, Clone)]
pub enum Formula {
    True,

    Atom(String),

    Not(Box<Formula>),

    And(Box<Formula>, Box<Formula>),

    Or(Box<Formula>, Box<Formula>),

    Eventually {
        local: Interval,
        global: Interval,
        formula: Box<Formula>,
    },

    Globally {
        local: Interval,
        global: Interval,
        formula: Box<Formula>,
    },

    Until {
        local: Interval,
        global: Interval,
        left: Box<Formula>,
        right: Box<Formula>,
    },
}
