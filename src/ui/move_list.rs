use bevy::prelude::*;

pub struct MoveListPlugin;

impl Plugin for MoveListPlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system(setup_ui);
    }
}

fn setup_ui(mut commands: Commands, asset_server: Res<AssetServer>) {
    let font = asset_server.load("fonts/FiraSans-Bold.ttf");

    let text = vec![
        "Up: Step forward (1 energy)",
        "Back: Step back (2 energy)",
        "Left: Turn left (0 energy)",
        "Right: Turn right (0 energy)",
        "Space: Strike (1 energy, end turn)",
    ]
    .join("\n");

    commands
        .spawn_bundle(TextBundle {
            style: Style {
                align_self: AlignSelf::FlexEnd,
                position_type: PositionType::Absolute,
                position: Rect {
                    top: Val::Px(35.0),
                    left: Val::Px(15.0),
                    ..Default::default()
                },
                ..Default::default()
            },
            text: Text {
                sections: vec![TextSection {
                    value: text,
                    style: TextStyle {
                        font: font.clone(),
                        font_size: 16.0,
                        color: Color::WHITE,
                    },
                }],
                ..Default::default()
            },
            ..Default::default()
        })
        .insert(MoveListText);
}

#[derive(Component)]
pub struct MoveListText;
