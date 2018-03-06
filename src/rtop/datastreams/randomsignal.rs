extern crate rand;
use self::rand::distributions::{IndependentSample, Range};
use self::rand::thread_rng;

#[derive(Clone)]
pub struct RandomSignal {
    range: Range<u64>,
    rng: rand::ThreadRng,
}

impl RandomSignal {
    pub fn new(lower: u64, upper: u64) -> RandomSignal {
        RandomSignal {
            range: Range::new(lower, upper),
            rng: thread_rng(),
        }
    }
}

impl Iterator for RandomSignal {
    type Item = u64;
    fn next(&mut self) -> Option<u64> {
        Some(self.range.ind_sample(&mut self.rng))
    }
}
