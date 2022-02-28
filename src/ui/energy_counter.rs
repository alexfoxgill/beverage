use bevy::prelude::*;

use crate::{domain::common::Actor, Player};

pub struct EnergyCounterPlugin;

impl Plugin for EnergyCounterPlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system(setup_ui)
            .add_system(update_player_energy);
    }
}

fn setup_ui(mut commands: Commands, asset_server: Res<AssetServer>) {
    let font = asset_server.load("fonts/FiraSans-Bold.ttf");

    commands.spawn_bundle(UiCameraBundle::default());

    commands
        .spawn_bundle(TextBundle {
            style: Style {
                align_self: AlignSelf::FlexEnd,
                position_type: PositionType::Absolute,
                position: Rect {
                    top: Val::Px(5.0),
                    left: Val::Px(15.0),
                    ..Default::default()
                },
                ..Default::default()
            },
            text: Text {
                sections: vec![TextSection {
                    value: Default::default(),
                    style: TextStyle {
                        font: font.clone(),
                        font_size: 24.0,
                        color: Color::WHITE,
                    },
                }],
                ..Default::default()
            },
            ..Default::default()
        })
        .insert(PlayerEnergyCounterText);
}

fn update_player_energy(
    player: Query<&Actor, (Changed<Actor>, With<Player>)>,
    mut ui: Query<&mut Text, With<PlayerEnergyCounterText>>,
) {
    if let Ok(mut text) = ui.get_single_mut() {
        if let Ok(player) = player.get_single() {
            let energy = player.actions_remaining;
            text.sections[0].value = format!("{energy} energy remaining");
        }
    }
}

#[derive(Component)]
pub struct PlayerEnergyCounterText;
