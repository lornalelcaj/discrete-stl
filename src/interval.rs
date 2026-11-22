#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Interval {
    pub lower: usize,
    pub upper: usize,
}

impl Interval {
    pub fn contains(&self, value: usize) -> bool {
        self.lower <= value && value <= self.upper
    }

    pub fn intersection(&self, other: &Interval) -> Option<Interval> {
        let lower = self.lower.max(other.lower);
        let upper = self.upper.min(other.upper);

        if lower <= upper {
            Some(Interval { lower, upper })
        } else {
            None
        }
    }

    pub fn shift_left(&self, tau: usize) -> Option<Interval> {
        if self.upper < tau {
            return None;
        }

        let lower = self.lower.saturating_sub(tau);
        let upper = self.upper - tau;

        Some(Interval { lower, upper })
    }
}
