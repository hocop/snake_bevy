use bevy::prelude::*;
use rand::SeedableRng;
use rand_chacha::ChaCha8Rng;

pub struct RngPlugin;

impl Plugin for RngPlugin {
    fn build(&self, app: &mut App) {
        app
            .init_resource::<RandomSource>()
        ;
    }
}

#[derive(Resource)]
pub struct RandomSource(pub ChaCha8Rng);

impl Default for RandomSource {
    fn default() -> Self {
        Self(ChaCha8Rng::seed_from_u64(19878367467712))
    }
}
