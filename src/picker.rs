use crate::pannel::AssetPalette;
use bevy::{platform::collections::HashMap, prelude::*};

pub struct PickerPlugin;
impl Plugin for PickerPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<WorldGrid>()
            .add_systems(Startup, setup_grid)
            .add_systems(Update, draw_grid);
    }
}

#[derive(Resource, Default)]
pub struct WorldGrid {
    pub cells: HashMap<IVec3, Entity>,
}

fn setup_grid(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    commands
        .spawn((
            Mesh3d(meshes.add(Plane3d::default().mesh().size(10000.0, 10000.0))),
            MeshMaterial3d(materials.add(StandardMaterial {
                base_color: Color::srgba(0.0, 1.0, 0.0, 0.0),
                alpha_mode: AlphaMode::Blend,
                ..default()
            })),
            Transform::from_xyz(0.0, 0.0, 0.0),
            Pickable::default(),
        ))
        .observe(on_click);
}

fn draw_grid(mut gizmos: Gizmos, camera_q: Query<&Transform, With<Camera>>) {
    let Ok(cam_tf) = camera_q.single() else {
        return;
    };

    let cam_pos = cam_tf.translation;
    let forward = cam_tf.rotation * Vec3::NEG_Z;

    let focus_point = if forward.y < -0.01 {
        let t = -cam_pos.y / forward.y;
        cam_pos + forward * t
    } else {
        let forward_xz = Vec3::new(forward.x, 0.0, forward.z).normalize_or_zero();
        cam_pos + forward_xz * 30.0
    };

    let center_x = focus_point.x.floor();
    let center_z = focus_point.z.floor();

    let ext = 30.0;

    for i in -30..=30 {
        let offset = i as f32;
        gizmos.line(
            Vec3::new(center_x - ext, 0., center_z + offset),
            Vec3::new(center_x + ext, 0., center_z + offset),
            Color::srgb(0.7, 0.7, 0.7),
        );
        gizmos.line(
            Vec3::new(center_x + offset, 0., center_z - ext),
            Vec3::new(center_x + offset, 0., center_z + ext),
            Color::srgb(0.7, 0.7, 0.7),
        );
    }
}

fn on_click(
    event: On<Pointer<Press>>,
    palette: Res<AssetPalette>,
    mut grid: ResMut<WorldGrid>,
    mut commands: Commands,
) {
    if event.button != PointerButton::Primary {
        return;
    }
    let Some(position) = event.hit.position else {
        return;
    };

    let cell_pos = IVec3::new(position.x.floor() as i32, 0, position.z.floor() as i32);

    if grid.cells.contains_key(&cell_pos) {
        println!("Cell occupied: {:?}", cell_pos);
        return;
    }

    let Some(i) = palette.selected_index else {
        return;
    };
    let Some((_, handle)) = palette.loaded_models.get(i) else {
        return;
    };

    let entity = commands
        .spawn((
            SceneRoot(handle.clone()),
            Transform::from_xyz(cell_pos.x as f32 + 0.5, 0.0, cell_pos.z as f32 + 0.5)
                .with_scale(Vec3::splat(1.0)),
        ))
        .id();

    grid.cells.insert(cell_pos, entity);
}
