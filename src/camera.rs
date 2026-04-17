use bevy::{
    input::{common_conditions::input_just_released, mouse::AccumulatedMouseMotion},
    prelude::*,
    window::{CursorGrabMode, CursorOptions, PrimaryWindow, WindowFocused},
};

pub struct CameraPlugin;
impl Plugin for CameraPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, setup_camera)
            .add_systems(Update, (move_camera, rotate_camera));
    }
}

#[derive(Component)]
pub struct MainCamera {
    pub speed: f32,
    pub rotation_speed: f32,
}

impl Default for MainCamera {
    fn default() -> Self {
        MainCamera {
            speed: 10.,
            rotation_speed: 1.,
        }
    }
}

fn setup_camera(mut commands: Commands) {
    commands.spawn((
        Camera3d::default(),
        Transform::from_xyz(0.0, 5.0, 10.0).looking_at(Vec3::ZERO, Vec3::Y),
        MainCamera::default(),
    ));
}

fn move_camera(
    camera: Single<(&mut Transform, &MainCamera)>,
    input: Res<ButtonInput<KeyCode>>,
    time: Res<Time>,
) {
    let mut delta = Vec3::ZERO;
    if input.pressed(KeyCode::KeyA) {
        delta.x -= 1.;
    }
    if input.pressed(KeyCode::KeyD) {
        delta.x += 1.;
    }
    if input.pressed(KeyCode::KeyW) {
        delta.z -= 1.;
    }
    if input.pressed(KeyCode::KeyS) {
        delta.z += 1.;
    }
    if input.pressed(KeyCode::KeyQ) {
        delta.y -= 1.;
    }
    if input.pressed(KeyCode::KeyE) {
        delta.y += 1.;
    }
    let (mut transform, cam_data) = camera.into_inner();
    transform.translation += delta.normalize_or_zero() * cam_data.speed * time.delta_secs();
}

fn rotate_camera(
    camera: Single<(&mut Transform, &MainCamera)>,
    input: Res<ButtonInput<KeyCode>>,
    time: Res<Time>,
) {
    let mut orientation = 0. as f32;
    if input.pressed(KeyCode::ArrowLeft) {
        orientation += 1.;
    } else if input.pressed(KeyCode::ArrowRight) {
        orientation -= 1.;
    }

    let (mut transform, camera_info) = camera.into_inner();
    transform.rotation = transform.rotation
        * Quat::from_euler(
            EulerRot::YXZ,
            orientation * camera_info.rotation_speed * time.delta_secs(),
            0.,
            0.,
        );
}
