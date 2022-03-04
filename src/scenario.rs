use bevy::prelude::*;

use crate::{
    domain::turn_queue::TurnQueue,
    map::{BasicHex, DrunkardsWalk, MapGenerator},
    spawn::spawn_map_entities,
};

pub struct CaveScenario;
impl Plugin for CaveScenario {
    fn build(&self, app: &mut App) {
        app.add_startup_system(Self::setup);
    }
}
impl CaveScenario {
    fn setup(mut commands: Commands, mut turn_queue: ResMut<TurnQueue>) {
        let map = DrunkardsWalk::example().generate_map();

        spawn_map_entities(&mut commands, &mut turn_queue, &map);
    }
}

pub struct ArenaScenario;
impl Plugin for ArenaScenario {
    fn build(&self, app: &mut App) {
        app.add_startup_system(Self::setup);
    }
}
impl ArenaScenario {
    fn setup(mut commands: Commands, mut turn_queue: ResMut<TurnQueue>) {
        let map = BasicHex::new(5).generate_map();

        spawn_map_entities(&mut commands, &mut turn_queue, &map);
    }
}
