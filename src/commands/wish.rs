use crate::{Context, Error};
use rand::{rngs, Rng, SeedableRng};

#[poise::command(
    prefix_command,
    slash_command,
    category = "Wish",
    user_cooldown = 5,
    required_bot_permissions = "SEND_MESSAGES"
)]
pub async fn regular_wish_test(
    context: Context<'_>,
    #[description = "The seed for the prng"] seed: Option<u64>,
    #[description = "The amount of rolls"] n: u32,
) -> Result<(), Error> {
    let mut state = RegularState::new(0, 0);
    let mut rng = rngs::StdRng::seed_from_u64(seed.unwrap_or(374829654398254837u64));

    let mut s5 = 0f64;
    let mut s4 = 0f64;
    let mut s3 = 0f64;

    let wish = RegularWish {
        weights: Weights::new(0.006, 0.051),
        pity: Pity::new(75, 90, 10),
        five_star_count: 100,
        four_star_count: 100,
        three_star_count: 100,
    };

    for _ in 0..n {
        let (roll, nstate) = wish.roll(state, &mut rng);
        state = nstate;
        match roll.kind {
            RollKind::FiveStar => s5 = s5 + 1f64,
            RollKind::FourStar => s4 = s4 + 1f64,
            RollKind::ThreeStar => s3 = s3 + 1f64,
            _ => (), // How did we even get here
        }
    }

    let _ = context
        .say(format!(
            "s5: {}, s4: {}, s3: {}",
            s5 / n as f64,
            s4 / n as f64,
            s3 / n as f64
        ))
        .await;

    Ok(())
}

#[poise::command(
    prefix_command,
    slash_command,
    category = "Wish",
    user_cooldown = 5,
    required_bot_permissions = "SEND_MESSAGES"
)]
pub async fn featured_wish_test(
    context: Context<'_>,
    #[description = "The seed for the prng"] seed: Option<u64>,
    #[description = "The amount of rolls"] n: u32,
) -> Result<(), Error> {
    let mut state = FeaturedState::new(RegularState::new(0, 0), true, true);
    let mut rng = rngs::StdRng::seed_from_u64(seed.unwrap_or(374829654398254837u64));

    let mut s5 = 0f64;
    let mut s5_f = 0f64;
    let mut s4 = 0f64;
    let mut s4_f = 0f64;
    let mut s3 = 0f64;

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

    for _ in 0..n {
        let (roll, nstate) = wish.roll(state, &mut rng);
        state = nstate;
        match roll.kind {
            RollKind::FiveStar => s5 = s5 + 1f64,
            RollKind::FiveStarFeatured => s5_f = s5_f + 1f64,
            RollKind::FourStar => s4 = s4 + 1f64,
            RollKind::FourStarFeatured => s4_f = s4_f + 1f64,
            RollKind::ThreeStar => s3 = s3 + 1f64,
        }
    }

    let _ = context
        .say(format!(
            "s5: {}, s5_f: {}, s4: {}, s4_f: {}, s3: {}",
            s5 / n as f64,
            s5_f / n as f64,
            s4 / n as f64,
            s4_f / n as f64,
            s3 / n as f64
        ))
        .await;

    Ok(())
}

struct RegularState {
    since_s5: u32,
    since_s4: u32,
}

impl RegularState {
    fn new(since_s5: u32, since_s4: u32) -> Self {
        Self { since_s5, since_s4 }
    }
}

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

struct Weights {
    s5: f32,
    s4: f32,
}

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
            let inc = (1f32 - self.s5) / (pity.s5_end - pity.s5_start) as f32;
            self.s5 + inc * (state.since_s5 - pity.s5_start) as f32
        };

        let s4_odds = if pity.s4_proc <= state.since_s4 {
            1f32 - s5_odds
        } else {
            self.s4
        };

        [s5_odds, s4_odds + s5_odds]
    }
}

struct RegularWish {
    weights: Weights,
    pity: Pity,
    five_star_count: u32,
    four_star_count: u32,
    three_star_count: u32,
}

struct FeaturedWish {
    base: RegularWish,
    five_star_featured_count: u32,
    four_star_featured_count: u32,
    featured_chance: f64,
}

enum RollKind {
    FiveStar,
    FiveStarFeatured,
    FourStar,
    FourStarFeatured,
    ThreeStar,
}

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
