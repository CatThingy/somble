use bevy::{prelude::*, render::camera::RenderTarget, utils::HashMap};
use iyes_loopless::prelude::*;

use crate::{GameState, MainCamera};

#[derive(Default, Deref, DerefMut)]
pub struct MousePosition(pub Vec3);

#[derive(Default, Deref, DerefMut)]
pub struct Spritesheets(pub HashMap<String, Handle<TextureAtlas>>);

pub struct Plugin;

impl Plugin {
    fn update_mouse_position(
        q_camera: Query<(&Camera, &GlobalTransform), With<MainCamera>>,
        windows: Res<Windows>,
        mut mouse_pos: ResMut<MousePosition>,
    ) {
        let (camera, camera_transform) = q_camera.single();
        if let RenderTarget::Window(window_id) = camera.target {
            let window = windows.get(window_id).unwrap();

            if let Some(screen_pos) = window.cursor_position() {
                let window_size = Vec2::new(window.width() as f32, window.height() as f32);
                // convert screen position [0..resolution] to ndc [-1..1] (gpu coordinates)
                let ndc = (screen_pos / window_size) * 2.0 - Vec2::ONE;
                // matrix for undoing the projection and camera transform
                let ndc_to_world =
                    camera_transform.compute_matrix() * camera.projection_matrix().inverse();
                // use it to convert ndc to world-space coordinates
                mouse_pos.0 = ndc_to_world.project_point3(ndc.extend(-1.0));
                mouse_pos.z = 0.0;
            }
        }
    }

    fn load_assets(
        asset_server: Res<AssetServer>,
        mut texture_atlases: ResMut<Assets<TextureAtlas>>,
        mut spritesheets: ResMut<Spritesheets>,
        mut cmd: Commands,
    ) {
        let elemental_texture = asset_server.load("elemental.png");
        let elemental_atlas =
            TextureAtlas::from_grid(elemental_texture, Vec2::new(16.0, 32.0), 3, 1);
        let elemental_handle = texture_atlases.add(elemental_atlas);

        spritesheets.insert("Elemental".to_string(), elemental_handle);

        let player_texture = asset_server.load("player.png");
        let player_atlas = TextureAtlas::from_grid(player_texture, Vec2::new(16.0, 32.0), 12, 1);
        let player_handle = texture_atlases.add(player_atlas);

        spritesheets.insert("Player".to_string(), player_handle);

        cmd.insert_resource(NextState(GameState::InGame))
    }
}

impl bevy::app::Plugin for Plugin {
    fn build(&self, app: &mut App) {
        app.add_system(Self::update_mouse_position)
            .add_startup_system(Self::load_assets)
            .init_resource::<MousePosition>()
            .init_resource::<Spritesheets>();
    }
}
