use bevy::prelude::*;

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
    pub rotation_speed_x: f32,
    pub rotation_speed_y: f32,
}

impl Default for MainCamera {
    fn default() -> Self {
        MainCamera {
            speed: 10.,
            rotation_speed_x: 2.,
            rotation_speed_y: 1.,
        }
    }
}

fn setup_camera(mut commands: Commands) {
    commands.spawn((
        Camera3d::default(),
        Camera {
            clear_color: ClearColorConfig::Custom(Color::srgb(0.1, 0.1, 0.1)),
            ..default()
        },
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
    let mut speed = camera.1.speed;
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
    if input.pressed(KeyCode::ShiftLeft) {
        speed /= 2.;
    }

    if delta != Vec3::ZERO {
        let (transform, cam_data) = camera.into_inner();

        let yaw = transform.rotation.to_euler(EulerRot::YXZ).0;
        let flat_rotation = Quat::from_euler(EulerRot::YXZ, yaw, 0., 0.);

        let local_delta = flat_rotation * delta.normalize();
        let mut transform = transform;
        transform.translation += local_delta * speed * time.delta_secs();
    }
}

fn rotate_camera(
    camera: Single<(&mut Transform, &MainCamera)>,
    input: Res<ButtonInput<KeyCode>>,
    time: Res<Time>,
) {
    let mut orientation = Vec2::ZERO;
    if input.pressed(KeyCode::ArrowLeft) {
        orientation.x += 1.;
    }
    if input.pressed(KeyCode::ArrowRight) {
        orientation.x -= 1.;
    }
    if input.pressed(KeyCode::ArrowDown) {
        orientation.y -= 1.;
    }
    if input.pressed(KeyCode::ArrowUp) {
        orientation.y += 1.;
    }
    if orientation != Vec2::ZERO {
        let (mut transform, camera_info) = camera.into_inner();
        let rotation_x = orientation.x * camera_info.rotation_speed_x * time.delta_secs();
        let rotation_y = orientation.y * camera_info.rotation_speed_y * time.delta_secs();
        let (mut yaw, mut pitch, _roll) = transform.rotation.to_euler(EulerRot::YXZ);
        yaw += rotation_x;
        pitch += rotation_y;
        pitch = pitch.clamp(-1.5, 1.5);
        transform.rotation = Quat::from_euler(EulerRot::YXZ, yaw, pitch, 0.);
    }
}
