mod wish;

use rand::prelude::*;
use sqlx::sqlite::SqliteQueryResult;

struct FiveStarPity {
    start: u32,
    end: u32,
}

struct FourStarPity {
    proc: u32,
}

impl FiveStarPity {
    // Assumes pity increase is linear
    fn get_increase(&self, base: f64) -> f64 {
        (1 - base) / (self.end - self.start)
    }
}


struct RegularPool {
    items: [u32],
}

struct FeaturedPool {
    items: [u32],
    featured: [u32],
}


struct PityState {
    rolls_since_drop: u32,
    last_was_featured: bool,
}

pub struct PityStates {
    s5: PityState,
    s4: PityState,
}

impl PityStates {
    pub fn new() -> Self {
        PityStates(PityState(0, false), PityState(0, false))
    }
}

struct Weights {
    s5: f64,
    s4: f64,
}

struct Pities {
    s5: FiveStarPity,
    s4: FourStarPity,
}

impl Weights {
    fn get_distribution(&self, pities: &Pities, pstates: &PityStates) -> (f64, f64, f64) {
        let s5_pityindex = pstates.s5.rolls_since_drop - pities.s5.start;
        let s5_odds = if s5_pityindex < 0 {
            self.s5
        } else {
            self.s5 + pities.s5.get_increase(self.s5) * s5_pityindex;
        };

        let s4_odds = if pities.s4.proc <= pstates.s4.rolls_since_drop {
            1 - s5_odds
        } else {
            self.s4
        };

        (s5_odds, s4_odds, 1 - s5_odds - s4_odds)
    }
}

pub struct Wish {
    weights: Weights,
    pools: (FeaturedPool, FeaturedPool, RegularPool),
    pities: Pities,
}

impl Wish {
    pub fn roll(&self, pstate: PityStates) -> (u32, PityStates) {
        let odds = self.weights.get_distribution(self.pities, pstate);
        let mut rng = thread_rng();
        let dist = WeightedIndex::new(&odds).unwrap();
        let index = dist.sample(&mut rng);
        return match index {
            0 => {
                // 5 star item, yay!
                // TODO: featured items
                let i = rng.gen_range(0..self.pools.0.items.len());
                (
                    self.pools.0[i],
                    PityStates(
                        PityState(0, true),
                        PityState(pstate.s4.rolls_since_drop + 1, pstate.s4.last_was_featured),
                    ),
                )
            }
            1 => {
                // 4 star item, cool.
                // TODO: featured items
                let i = rng.gen_range(0..self.pools.1.items.len());
                (
                    self.pools.1[i],
                    PityStates(
                        PityState(pstate.s5.rolls_since_drop + 1, pstate.s5.last_was_featured),
                        PityState(0, true),
                    ),
                )
            }
            2 => {
                // 3 star item, meh.
                let i = rng.gen_range(0..self.pools.2.items.len());
                (
                    self.pools.2[i],
                    PityStates(
                        PityState(pstate.s5.rolls_since_drop + 1, pstate.s5.last_was_featured),
                        PityState(pstate.s4.rolls_since_drop + 1, pstate.s4.last_was_featured),
                    ),
                )
            }
        };
    }
 
    // TODO: Actually query the DB
    pub fn query(wish_id: u32) -> Self {
        Wish(
            Weights(0.2, 0.7),
            Pools(
                FeaturedPool([1, 2, 3], [2]),
                FeaturedPool([4, 5, 6], [4, 5]),
                RegularPool([7, 8, 9]),
            ),
        )
    }
}
