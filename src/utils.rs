use bevy::{prelude::*, render::camera::RenderTarget};

use crate::{consts::*, player::Player, MainCamera};

#[derive(Default, Deref, DerefMut, Debug)]
pub struct MousePosition(pub Vec3);

#[derive(Component)]
pub struct CameraFocus;

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

    fn set_focus_on_player_spawn(
        mut q_focus: Query<
            &mut Transform,
            (With<CameraFocus>, Without<Player>, Without<MainCamera>),
        >,
        q_player: Query<&Transform, (Without<CameraFocus>, Added<Player>, Without<MainCamera>)>,
        mut q_camera: Query<
            &mut Transform,
            (Without<CameraFocus>, Without<Player>, With<MainCamera>),
        >,
    ) {
        let mut focus_pos = match q_focus.get_single_mut() {
            Ok(v) => v,
            Err(_) => return,
        };
        let player_pos = match q_player.get_single() {
            Ok(v) => v,
            Err(_) => return,
        };
        let mut camera_pos = match q_camera.get_single_mut() {
            Ok(v) => v,
            Err(_) => return,
        };

        focus_pos.translation.x = player_pos.translation.x;
        focus_pos.translation.y = player_pos.translation.y;

        camera_pos.translation.x = player_pos.translation.x;
        camera_pos.translation.y = player_pos.translation.y;
    }

    fn init_focus(mut cmd: Commands) {
        cmd.spawn()
            .insert_bundle(TransformBundle::default())
            .insert(CameraFocus);
    }

    fn follow_camera_focus(
        q_focus: Query<&Transform, (With<CameraFocus>, Without<MainCamera>)>,
        mut q_camera: Query<&mut Transform, (Without<CameraFocus>, With<MainCamera>)>,
        time: Res<Time>,
    ) {
        let focus_pos = match q_focus.get_single() {
            Ok(v) => v,
            Err(_) => return,
        };
        let mut camera_pos = match q_camera.get_single_mut() {
            Ok(v) => v,
            Err(_) => return,
        };

        let direction = focus_pos.translation - camera_pos.translation;

        camera_pos.translation +=
            direction.truncate().extend(0.0) * time.delta_seconds() * CAMERA_PAN_SPEED;
    }

    fn update_focus_pos(
        mut q_focus: Query<&mut Transform, (With<CameraFocus>, Without<Player>)>,
        q_player: Query<&Transform, (Without<CameraFocus>, With<Player>)>,
        windows: Res<Windows>,
        mut mouse_offset: Local<Vec2>,
    ) {
        let mut focus_pos = match q_focus.get_single_mut() {
            Ok(v) => v,
            Err(_) => return,
        };
        let player_pos = match q_player.get_single() {
            Ok(v) => v,
            Err(_) => return,
        };

        let window = windows.primary();
        if let Some(mouse_pos) = window.cursor_position() {
            let window_size = Vec2::new(window.width(), window.height());
            let offset = mouse_pos - window_size / 2.0;

            *mouse_offset = offset * CAMERA_PAN_SCALE;
        }

        focus_pos.translation =
            (player_pos.translation.truncate() + mouse_offset.clamp_length_max(CAMERA_PAN_RANGE)).extend(0.0);
    }
}

impl bevy::app::Plugin for Plugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system(Self::init_focus)
            .add_system(Self::update_mouse_position)
            .add_system(Self::set_focus_on_player_spawn)
            .add_system_to_stage(CoreStage::Last, Self::follow_camera_focus)
            .add_system(Self::update_focus_pos)
            .init_resource::<MousePosition>();
    }
}
