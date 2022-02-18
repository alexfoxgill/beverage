use bevy::prelude::*;

use crate::Player;

pub struct CameraPlugin;

impl Plugin for CameraPlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system(setup).add_system(follow_player);
    }
}

#[derive(Component)]
struct MainCamera;

fn setup(mut commands: Commands) {
    // cameras
    commands
        .spawn_bundle(OrthographicCameraBundle::new_2d())
        .insert(MainCamera);
    commands.spawn_bundle(UiCameraBundle::default());
}

fn follow_player(
    mut queries: QuerySet<(
        QueryState<&Transform, With<Player>>,
        QueryState<&mut Transform, With<MainCamera>>,
    )>,
) {
    if let Ok(player) = queries.q0().get_single() {
        let translation = player.translation.clone();

        if let Ok(mut camera) = queries.q1().get_single_mut() {
            camera.translation = translation;
        }
    }
}
