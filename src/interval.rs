/*
The interval component implements the discrete temporal-set operations required by the continuous to discrete translation of syntactic separation.
The temporal sets I_d and K_d represent the local and global timing constraints of STL-GT.
Membership implements the admissibility conditions j-k in I_d and j in K_d.
Intersection is used to create the before-, at-, and after tau regions used by global and local separation.
the shift operation implements the change of local delay when the evaluation point moves from k to k+tau
*/

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Interval {
    pub lower: usize,
    pub upper: usize,
}

impl Interval {
    //checking whether a position satisfies a temporal-set constraint
    pub fn contains(&self, value: usize) -> bool {
        self.lower <= value && value <= self.upper
    }

    //implements the restriction of the original global or local timing set to the regions created by syntactic separation
    pub fn intersection(&self, other: &Interval) -> Option<Interval> {
        let lower = self.lower.max(other.lower);
        let upper = self.upper.min(other.upper);

        if lower <= upper {
            Some(Interval { lower, upper })
        } else {
            None
        }
    }

    /*
    When local separation moves the reference point from k to k+tau the remaining delay to the same witness decreases by tau.
    So the admissible local interval has to be shifted left by tau
    */
    pub fn shift_left(&self, tau: usize) -> Option<Interval> {
        if self.upper < tau {
            return None;
        }

        let lower = self.lower.saturating_sub(tau);
        let upper = self.upper - tau;

        Some(Interval { lower, upper })
    }
}
