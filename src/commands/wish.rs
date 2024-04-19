use rand::Rng;

#[cfg(test)]
mod wish_tests {
    use super::*;

    const ROLLS: u32 = 1_000_000;
    const EPS: f64 = 0.001;

    fn test_tol(result: f64, expected: f64) {
        let res = result / ROLLS as f64;
        let high = expected + EPS;
        let low = expected - EPS;

        assert!(
            res < high && res > low,
            "Expected: {}, Got: {}",
            expected,
            res
        );
    }

    #[test]
    fn regular_wish_test() {
        let mut state = RegularState::new(0, 0);
        let mut rng = rand::thread_rng();

        let mut s5 = 0.;
        let mut s4 = 0.;

        let wish = RegularWish {
            weights: Weights::new(0.006, 0.051),
            pity: Pity::new(75, 90, 10),
            five_star_count: 100,
            four_star_count: 100,
            three_star_count: 100,
        };

        for _ in 0..ROLLS {
            let roll;
            (roll, state) = wish.roll(state, &mut rng);
            match roll.kind {
                RollKind::FiveStar => s5 = s5 + 1.,
                RollKind::FourStar => s4 = s4 + 1.,
                _ => (),
            }
        }

        test_tol(s5, 0.016);
        test_tol(s4, 0.13);
    }

    #[test]
    fn featured_wish_test() {
        let mut state = FeaturedState::new(RegularState::new(0, 0), true, true);
        let mut rng = rand::thread_rng();

        let mut s5 = 0.;
        let mut s4 = 0.;

        let wish = FeaturedWish {
            base: RegularWish {
                weights: Weights::new(0.006, 0.051),
                pity: Pity::new(75, 90, 10),
                five_star_count: 100,
                four_star_count: 100,
                three_star_count: 100,
            },
            five_star_featured_count: 100,
            four_star_featured_count: 100,
            featured_chance: 0.5,
        };

        for _ in 0..ROLLS {
            let roll;
            (roll, state) = wish.roll(state, &mut rng);
            match roll.kind {
                RollKind::FiveStar => s5 = s5 + 1.,
                RollKind::FiveStarFeatured => s5 = s5 + 1.,
                RollKind::FourStar => s4 = s4 + 1.,
                RollKind::FourStarFeatured => s4 = s4 + 1.,
                _ => (),
            }
        }

        test_tol(s5, 0.016);
        test_tol(s4, 0.13);
    }
}

#[derive(Debug, Clone)]
struct RegularState {
    since_s5: u32,
    since_s4: u32,
}

impl RegularState {
    fn new(since_s5: u32, since_s4: u32) -> Self {
        Self { since_s5, since_s4 }
    }
}

#[derive(Debug, Clone)]
struct FeaturedState {
    base: RegularState,
    last_s5_featured: bool,
    last_s4_featured: bool,
}

impl FeaturedState {
    fn new(base: RegularState, last_s5_featured: bool, last_s4_featured: bool) -> Self {
        Self {
            base,
            last_s5_featured,
            last_s4_featured,
        }
    }
}

#[derive(Debug, Clone)]
struct Weights {
    s5: f32,
    s4: f32,
}

#[derive(Debug, Clone)]
struct Pity {
    s5_start: u32,
    s5_end: u32,
    s4_proc: u32,
}

impl Pity {
    fn new(s5_start: u32, s5_end: u32, s4_proc: u32) -> Self {
        Self {
            s5_start,
            s5_end,
            s4_proc,
        }
    }
}

impl Weights {
    fn new(s5: f32, s4: f32) -> Self {
        Self { s5, s4 }
    }

    fn get_distribution(&self, pity: &Pity, state: &RegularState) -> [f32; 2] {
        let s5_odds = if state.since_s5 < pity.s5_start {
            self.s5
        } else {
            let inc = (1. - self.s5) / (pity.s5_end - pity.s5_start) as f32;
            self.s5 + inc * (state.since_s5 - pity.s5_start) as f32
        };

        let s4_odds = if state.since_s4 < pity.s4_proc {
            self.s4
        } else {
            1.
        };

        [s5_odds, s4_odds]
    }
}

#[derive(Debug, Clone)]
struct RegularWish {
    weights: Weights,
    pity: Pity,
    five_star_count: u32,
    four_star_count: u32,
    three_star_count: u32,
}

#[derive(Debug, Clone)]
struct FeaturedWish {
    base: RegularWish,
    five_star_featured_count: u32,
    four_star_featured_count: u32,
    featured_chance: f64,
}

#[derive(Debug, Clone)]
enum RollKind {
    FiveStar,
    FiveStarFeatured,
    FourStar,
    FourStarFeatured,
    ThreeStar,
}

#[derive(Debug, Clone)]
struct Roll {
    kind: RollKind,
    index: u32,
}

impl Roll {
    fn new(kind: RollKind, index: u32) -> Self {
        Roll { kind, index }
    }
}

impl RegularWish {
    fn make_s3_roll<R: Rng>(&self, state: RegularState, rng: &mut R) -> (Roll, RegularState) {
        (
            Roll::new(RollKind::ThreeStar, rng.gen_range(0..self.three_star_count)),
            RegularState::new(state.since_s5 + 1, state.since_s4 + 1),
        )
    }

    fn make_s4_roll<R: Rng>(&self, state: RegularState, rng: &mut R) -> (Roll, RegularState) {
        (
            Roll::new(RollKind::FourStar, rng.gen_range(0..self.four_star_count)),
            RegularState::new(state.since_s5 + 1, 0),
        )
    }

    fn make_s5_roll<R: Rng>(&self, state: RegularState, rng: &mut R) -> (Roll, RegularState) {
        (
            Roll::new(RollKind::FiveStar, rng.gen_range(0..self.five_star_count)),
            RegularState::new(0, state.since_s4 + 1),
        )
    }

    fn roll<R: Rng>(&self, state: RegularState, rng: &mut R) -> (Roll, RegularState) {
        let roll: f32 = rng.gen();
        let dist = self.weights.get_distribution(&self.pity, &state);
        if roll <= dist[0] {
            self.make_s5_roll(state, rng)
        } else if roll <= dist[1] {
            self.make_s4_roll(state, rng)
        } else {
            self.make_s3_roll(state, rng)
        }
    }
}

impl FeaturedWish {
    fn make_s3_roll<R: Rng>(&self, state: FeaturedState, rng: &mut R) -> (Roll, FeaturedState) {
        let (roll, base) = self.base.make_s3_roll(state.base, rng);
        (
            roll,
            FeaturedState::new(base, state.last_s5_featured, state.last_s4_featured),
        )
    }

    fn make_s4_roll<R: Rng>(&self, state: FeaturedState, rng: &mut R) -> (Roll, FeaturedState) {
        if !state.last_s4_featured || rng.gen_bool(self.featured_chance) {
            (
                Roll::new(
                    RollKind::FourStarFeatured,
                    rng.gen_range(0..self.four_star_featured_count),
                ),
                FeaturedState::new(
                    RegularState::new(state.base.since_s5, 0),
                    state.last_s5_featured,
                    false,
                ),
            )
        } else {
            let (roll, base) = self.base.make_s4_roll(state.base, rng);
            (
                roll,
                FeaturedState::new(base, state.last_s5_featured, false),
            )
        }
    }

    fn make_s5_roll<R: Rng>(&self, state: FeaturedState, rng: &mut R) -> (Roll, FeaturedState) {
        if !state.last_s4_featured || rng.gen_bool(self.featured_chance) {
            (
                Roll::new(
                    RollKind::FiveStarFeatured,
                    rng.gen_range(0..self.five_star_featured_count),
                ),
                FeaturedState::new(
                    RegularState::new(0, state.base.since_s4),
                    true,
                    state.last_s4_featured,
                ),
            )
        } else {
            let (roll, base) = self.base.make_s5_roll(state.base, rng);
            (
                roll,
                FeaturedState::new(base, false, state.last_s4_featured),
            )
        }
    }

    fn roll<R: Rng>(&self, state: FeaturedState, rng: &mut R) -> (Roll, FeaturedState) {
        let roll: f32 = rng.gen();
        let dist = self
            .base
            .weights
            .get_distribution(&self.base.pity, &state.base);
        if roll <= dist[0] {
            self.make_s5_roll(state, rng)
        } else if roll <= dist[1] {
            self.make_s4_roll(state, rng)
        } else {
            self.make_s3_roll(state, rng)
        }
    }
}
