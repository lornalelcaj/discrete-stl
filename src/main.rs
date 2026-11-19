mod interval;
mod formula;
mod trace;
mod semantics;
mod separation;

use formula::Formula;
use interval::Interval;
use semantics::satisfies;
use trace::Trace;
use separation::split_global;

fn main() {
    let k = Interval {
        lower: 1,
        upper: 5,
    };

    let tau = 3;
    let horizon = 7;

    let (k_minus, k_zero, k_plus) =
        split_global(&k, tau, horizon);

    println!("K- = {:?}", k_minus);
    println!("K0 = {:?}", k_zero);
    println!("K+ = {:?}", k_plus);
}