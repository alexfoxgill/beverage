use bevy::prelude::*;

use crate::{
    domain::{common::*, vision::Vision},
    Player,
};

// updates which entities are visible based on their hex position relative to the player
pub struct PlayerVisionPlugin;
impl Plugin for PlayerVisionPlugin {
    fn build(&self, app: &mut App) {
        app.add_system_to_stage(CoreStage::PostUpdate, update_visibility);
    }
}

fn update_visibility(
    player: Query<(&HexPos, &Facing, &Vision), With<Player>>,
    mut viewable_things: Query<(&HexPos, &mut Visibility)>,
) {
    if let Ok((&HexPos(player_pos), _facing, vision)) = player.get_single() {
        for (&HexPos(pos), mut visibility) in viewable_things.iter_mut() {
            visibility.is_visible = player_pos.distance(pos) <= vision.radius;
        }
    }
}
