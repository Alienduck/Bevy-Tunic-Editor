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
    pub rotation_speed: f32,
}

impl Default for MainCamera {
    fn default() -> Self {
        MainCamera {
            speed: 10.,
            rotation_speed: 2.,
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

    if delta != Vec3::ZERO {
        let (transform, cam_data) = camera.into_inner();

        let yaw = transform.rotation.to_euler(EulerRot::YXZ).0;
        let flat_rotation = Quat::from_euler(EulerRot::YXZ, yaw, 0., 0.);

        let local_delta = flat_rotation * delta.normalize();
        let mut transform = transform;
        transform.translation += local_delta * cam_data.speed * time.delta_secs();
    }
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

    if orientation != 0. {
        let (mut transform, camera_info) = camera.into_inner();
        let rotation_amount = orientation * camera_info.rotation_speed * time.delta_secs();
        let world_y_rot = Quat::from_euler(EulerRot::YXZ, rotation_amount, 0., 0.);

        transform.rotation = world_y_rot * transform.rotation;
    }
}
