use crate::interval::Interval;

#[derive(Debug, Clone)]
pub enum Formula {
    True,

    Atom(String),

    Not(Box<Formula>),

    And(
        Box<Formula>,
        Box<Formula>,
    ),

    Or(
        Box<Formula>,
        Box<Formula>,
    ),

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