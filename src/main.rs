use bevy::{prelude::*, window::WindowMode};

mod camera;
mod pannel;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                title: "TUNIC !!!".to_string(),
                mode: WindowMode::BorderlessFullscreen(MonitorSelection::Primary),
                ..Default::default()
            }),
            ..Default::default()
        }))
        .add_plugins(camera::CameraPlugin)
        .add_plugins(pannel::PannelPlugin)
        .add_systems(Startup, setup_world)
        .add_systems(Update, draw_grid)
        .run();
}

fn setup_world(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    commands.spawn((
        DirectionalLight::default(),
        Transform::from_xyz(3.0, 5.0, 3.0).looking_at(Vec3::ZERO, Vec3::Y),
    ));
    commands.spawn((
        Mesh3d(meshes.add(Sphere::new(1.))),
        MeshMaterial3d(materials.add(StandardMaterial {
            base_color: Color::srgb(1.0, 0., 0.),
            ..default()
        })),
        Transform::from_xyz(0.0, 0.0, 0.0),
    ));
}

fn draw_grid(mut gizmos: Gizmos) {
    let grid_size = 20;
    for i in 0..=grid_size {
        let pos = i as f32 - grid_size as f32 / 2.0;
        gizmos.line(
            Vec3::new(-grid_size as f32 / 2.0, 0.0, pos),
            Vec3::new(grid_size as f32 / 2.0, 0.0, pos),
            Color::srgb(0.7, 0.7, 0.7),
        );
        gizmos.line(
            Vec3::new(pos, 0.0, -grid_size as f32 / 2.0),
            Vec3::new(pos, 0.0, grid_size as f32 / 2.0),
            Color::srgb(0.7, 0.7, 0.7),
        );
    }
}
