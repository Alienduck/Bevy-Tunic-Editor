use crate::pannel::AssetPalette;
use bevy::prelude::*;

pub struct PickerPlugin;
impl Plugin for PickerPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, setup_grid)
            .add_systems(Update, draw_grid);
    }
}

fn setup_grid(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    commands
        .spawn((
            Mesh3d(meshes.add(Plane3d::default().mesh().size(20.0, 20.0))),
            MeshMaterial3d(materials.add(StandardMaterial {
                base_color: Color::NONE,
                alpha_mode: AlphaMode::Blend,
                ..default()
            })),
            Transform::from_xyz(0.0, 0.0, 0.0),
        ))
        .observe(on_grid_click);
}

fn on_grid_click(
    click: On<Pointer<Click>>,
    mut commands: Commands,
    palette: Res<AssetPalette>, // On utilise la ressource de pannel.rs
) {
    if click.button != PointerButton::Primary {
        return;
    }
    if let Some(pos) = click.hit.position {
        if let Some(idx) = palette.selected_index {
            if let Some((_, handle)) = palette.loaded_models.get(idx) {
                commands.spawn((
                    SceneRoot(handle.clone()),
                    Transform::from_xyz(pos.x.round(), 0.0, pos.z.round()),
                ));
            }
        }
    }
}

fn draw_grid(mut gizmos: Gizmos) {
    let grid_size = 20;
    for i in 0..=grid_size {
        let pos = i as f32 - grid_size as f32 / 2.0;
        gizmos.line(
            Vec3::new(-10., 0., pos),
            Vec3::new(10., 0., pos),
            Color::srgb(0.7, 0.7, 0.7),
        );
        gizmos.line(
            Vec3::new(pos, 0., -10.),
            Vec3::new(pos, 0., 10.),
            Color::srgb(0.7, 0.7, 0.7),
        );
    }
}
