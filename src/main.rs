use bevy::{
    prelude::*,
    render::batching::gpu_preprocessing::{GpuPreprocessingMode, GpuPreprocessingSupport},
    window::WindowMode,
};

mod camera;
mod pannel;
mod picker;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                title: "TUNIC !!!".to_string(),
                mode: WindowMode::BorderlessFullscreen(MonitorSelection::Primary),
                ..default()
            }),
            ..default()
        }))
        .insert_resource(GpuPreprocessingSupport {
            max_supported_mode: GpuPreprocessingMode::Culling,
        })
        .insert_resource(MeshPickingSettings {
            require_markers: true,
            ..default()
        })
        .add_plugins(MeshPickingPlugin)
        .add_plugins(camera::CameraPlugin)
        .add_plugins(pannel::PannelPlugin)
        .add_plugins(picker::PickerPlugin)
        .add_systems(Startup, setup_light)
        .run();
}

fn setup_light(mut commands: Commands) {
    commands.spawn((
        DirectionalLight::default(),
        Transform::from_xyz(3.0, 5.0, 3.0).looking_at(Vec3::ZERO, Vec3::Y),
    ));
}
