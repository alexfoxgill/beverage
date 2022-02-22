use bevy_prototype_lyon::{entity::ShapeBundle, prelude::*};
use hex2d::Coordinate;

use crate::{
    domain::common::HEX_SPACING,
    map::{MapTile, Terrain},
    player_vision::{PlayerVisibility, PlayerVisionUpdate, VisibilityMemory},
};

use bevy::prelude::*;

pub struct MapRenderPlugin;
impl Plugin for MapRenderPlugin {
    fn build(&self, app: &mut App) {
        app.add_system_to_stage(
            CoreStage::PostUpdate,
            update_map_visibility.after(PlayerVisionUpdate),
        );
    }
}

fn update_map_visibility(
    mut query: Query<(&MapTile, &PlayerVisibility, &mut DrawMode), Changed<PlayerVisibility>>,
) {
    for (tile, vis, mut draw) in query.iter_mut() {
        *draw = get_draw_mode(tile, TileVisibility::from_vis(vis));
    }
}

#[derive(Clone, Copy, PartialEq, Eq)]
pub enum TileVisibility {
    Visible,
    Seen,
    Undiscovered,
}

impl TileVisibility {
    pub fn from_vis(vis: &PlayerVisibility) -> TileVisibility {
        if vis.is_visible {
            TileVisibility::Visible
        } else if let VisibilityMemory::Persistent { seen: true } = vis.memory {
            TileVisibility::Seen
        } else {
            TileVisibility::Undiscovered
        }
    }
}

fn get_draw_mode(tile: &MapTile, vis: TileVisibility) -> DrawMode {
    let mut color = match tile.terrain {
        Terrain::Floor => Color::OLIVE,
        Terrain::Wall => Color::MIDNIGHT_BLUE,
    };

    match vis {
        TileVisibility::Visible => DrawMode::Outlined {
            fill_mode: FillMode::color(color),
            outline_mode: StrokeMode::new(Color::BLACK, 1.0),
        },
        TileVisibility::Seen => {
            color.set_a(0.5);
            DrawMode::Outlined {
                fill_mode: FillMode::color(color),
                outline_mode: StrokeMode::new(Color::BLACK, 1.0),
            }
        }
        TileVisibility::Undiscovered => {
            DrawMode::Fill(FillMode::color(Color::rgba(0.0, 0.0, 0.0, 0.0)))
        }
    }
}

pub fn tile_render_bundle(c: Coordinate, tile: &MapTile) -> ShapeBundle {
    let draw_mode = get_draw_mode(tile, TileVisibility::Undiscovered);
    GeometryBuilder::build_as(&make_hex_tile(c), draw_mode, Transform::default())
}

fn make_hex_tile(coord: Coordinate) -> RegularPolygon {
    let (x, y) = coord.to_pixel(HEX_SPACING);
    RegularPolygon {
        sides: 6,
        feature: RegularPolygonFeature::Radius(40.0),
        center: Vec2::new(x, -y),
    }
}
