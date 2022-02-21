use bevy::prelude::*;

use self::map::MapRenderPlugin;

pub mod map;

pub struct DomainRenderPlugin;
impl Plugin for DomainRenderPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugin(MapRenderPlugin);
    }
}
