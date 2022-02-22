use bevy::prelude::*;

use self::{
    actor::ActorRenderPlugin, animation::AnimationPlugin, map::MapRenderPlugin,
    player_vision::PlayerVisionPlugin,
};

pub mod actor;
pub mod animation;
pub mod map;
pub mod player_vision;

pub struct DomainRenderPlugin;
impl Plugin for DomainRenderPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugin(MapRenderPlugin)
            .add_plugin(ActorRenderPlugin)
            .add_plugin(PlayerVisionPlugin)
            .add_plugin(AnimationPlugin);
    }
}
