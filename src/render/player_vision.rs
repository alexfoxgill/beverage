use bevy::{prelude::*, utils::HashSet};
use hex2d::{Coordinate, Position};

use crate::{
    domain::{common::*, vision::Vision},
    map::{MapTile, Terrain},
    Player,
};

// updates which entities are visible based on their hex position relative to the player
pub struct PlayerVisionPlugin;
impl Plugin for PlayerVisionPlugin {
    fn build(&self, app: &mut App) {
        app.add_system_to_stage(
            CoreStage::PostUpdate,
            update_player_visibility.label(PlayerVisionUpdate),
        );
    }
}

#[derive(Clone, Debug, Hash, PartialEq, Eq, SystemLabel)]
pub struct PlayerVisionUpdate;

#[derive(Clone, Copy, PartialEq, Eq)]
pub enum VisibilityMemory {
    Transient,
    Persistent { seen: bool },
}

#[derive(Component)]
pub struct PlayerVisibility {
    pub is_visible: bool,
    pub memory: VisibilityMemory,
}

impl PlayerVisibility {
    pub fn new_transient() -> PlayerVisibility {
        PlayerVisibility {
            is_visible: false,
            memory: VisibilityMemory::Transient,
        }
    }
    pub fn new_persistent() -> PlayerVisibility {
        PlayerVisibility {
            is_visible: false,
            memory: VisibilityMemory::Persistent { seen: false },
        }
    }

    pub fn set_visibility(&mut self, is_visible: bool) {
        self.is_visible = is_visible;

        if is_visible {
            if let VisibilityMemory::Persistent { ref mut seen } = self.memory {
                *seen = true;
            }
        }
    }
}

fn update_player_visibility(
    player: Query<(&HexPos, &Facing, &Vision), With<Player>>,
    positioned: Query<(&HexPos, Entity)>,
    map_tiles: Query<(&HexPos, &MapTile)>,
    mut visibilities: Query<&mut PlayerVisibility>,
) {
    if let Ok((&HexPos(player_coord), &Facing(player_dir), vision)) = player.get_single() {
        let player_pos = Position::new(player_coord, player_dir);

        let walls: HashSet<Coordinate> = map_tiles
            .iter()
            .filter_map(|(c, t)| {
                if t.terrain == Terrain::Wall {
                    Some(c.0)
                } else {
                    None
                }
            })
            .collect();

        for (&HexPos(pos), entity) in positioned.iter() {
            let is_visible = vision.can_see_relative(player_pos, pos, |x| walls.contains(&x));

            if let Ok(mut visibility) = visibilities.get_mut(entity) {
                if visibility.is_visible != is_visible {
                    visibility.set_visibility(is_visible);
                }
            }
        }
    }
}
