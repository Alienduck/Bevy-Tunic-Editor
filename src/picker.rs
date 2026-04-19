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
                base_color: Color::srgba(0.0, 1.0, 0.0, 0.05),
                alpha_mode: AlphaMode::Blend,
                ..default()
            })),
            Transform::from_xyz(0.0, 0.0, 0.0),
            Pickable::default(),
        ))
        .observe(on_click);
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

fn on_click(event: On<Pointer<Press>>, palette: Res<AssetPalette>, mut commands: Commands) {
    if event.button != PointerButton::Primary {
        return;
    }
    if let Some(position) = event.hit.position {
        if let Some(i) = palette.selected_index {
            if let Some((_, handle)) = palette.loaded_models.get(i) {
                let snap_pos = position.round();
                commands.spawn((
                    SceneRoot(handle.clone()),
                    Transform::from_xyz(snap_pos.x + 0.5, snap_pos.y, snap_pos.z + 0.5),
                ));
            }
        }
    }
}
