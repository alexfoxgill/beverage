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

#[derive(Component)]
pub struct RememberVis;

fn update_visibility(
    player: Query<(&HexPos, &Facing, &Vision), With<Player>>,
    positioned: Query<(&HexPos, Entity, Option<&Children>)>,
    mut visibilities: Query<(&mut Visibility, Option<&RememberVis>)>,
) {
    if let Ok((&HexPos(player_pos), _facing, vision)) = player.get_single() {
        for (&HexPos(pos), entity, children) in positioned.iter() {
            let visible = player_pos.distance(pos) <= vision.radius;

            if let Ok((mut visibility, remember)) = visibilities.get_mut(entity) {
                match (visibility.is_visible, visible, remember.is_some()) {
                    (x, y, _) if x == y => (),
                    (true, false, true) => (),
                    _ => visibility.is_visible = visible,
                }
            }

            if let Some(children) = children {
                for &entity in children.iter() {
                    if let Ok((mut visibility, _)) = visibilities.get_mut(entity) {
                        visibility.is_visible = visible;
                    }
                }
            }
        }
    }
}
