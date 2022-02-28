use bevy::prelude::*;

use self::{
    actor::ActorRenderPlugin, animation::AnimationPlugin, map::MapRenderPlugin,
    player_vision::PlayerVisionPlugin,
};

pub mod actor;
pub mod animation;
pub mod map;
pub mod player_vision;

pub struct GameRenderPlugin;
impl Plugin for GameRenderPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(Msaa { samples: 4 })
            .insert_resource(ClearColor(Color::rgb(0.2, 0.2, 0.25)))
            .add_plugin(MapRenderPlugin)
            .add_plugin(ActorRenderPlugin)
            .add_plugin(PlayerVisionPlugin)
            .add_plugin(AnimationPlugin);
    }
}
