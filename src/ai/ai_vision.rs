use bevy::prelude::*;
use hex2d::Position;

use crate::{
    domain::{
        common::{Facing, HexPos},
        vision::Vision,
    },
    map::MapTiles,
    Player,
};

#[derive(Component)]
pub struct CanSeePlayer;

pub fn update_can_see_player(
    mut commands: Commands,
    seers: Query<(Entity, &HexPos, &Facing, &Vision)>,
    players: Query<&HexPos, With<Player>>,
    map: MapTiles,
) {
    if let Ok(&HexPos(player_pos)) = players.get_single() {
        let walls = map.get_walls();

        for (e, &HexPos(coord), &Facing(dir), vis) in seers.iter() {
            let pos = Position::new(coord, dir);

            let visible = vis.can_see_relative(pos, player_pos, |x| walls.contains(&x));

            if visible {
                commands.entity(e).insert(CanSeePlayer);
            } else {
                commands.entity(e).remove::<CanSeePlayer>();
            }
        }
    }
}
