use bevy::prelude::*;

use self::{actor::ActorRenderPlugin, map::MapRenderPlugin};

pub mod actor;
pub mod animation;
pub mod map;

pub struct DomainRenderPlugin;
impl Plugin for DomainRenderPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugin(MapRenderPlugin)
            .add_plugin(ActorRenderPlugin);
    }
}
