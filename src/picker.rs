use crate::pannel::AssetPalette;
use bevy::{
    camera::primitives::Aabb,
    platform::collections::HashMap,
    prelude::*,
    render::{render_resource::AsBindGroup, storage::ShaderStorageBuffer},
    shader::ShaderRef,
};

pub struct PickerPlugin;
impl Plugin for PickerPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<WorldGrid>()
            .init_resource::<PreEntity>()
            .add_plugins(MaterialPlugin::<GridMaterial>::default())
            .add_systems(Startup, setup_grid)
            .add_systems(
                Update,
                (process_ghost_model, rotate_entity, apply_auto_scale),
            );
    }
}

#[derive(Asset, TypePath, AsBindGroup, Debug, Clone)]
pub struct GridMaterial {
    #[storage(0, read_only)]
    pub settings: Handle<ShaderStorageBuffer>,
}

impl Material for GridMaterial {
    fn fragment_shader() -> ShaderRef {
        "shaders/grid.wgsl".into()
    }
    fn alpha_mode(&self) -> AlphaMode {
        AlphaMode::Blend
    }
}

#[derive(Component)]
pub struct AutoScale;

#[derive(Resource, Default)]
pub struct WorldGrid {
    pub cells: HashMap<IVec3, Entity>,
}

#[derive(Resource, Default)]
pub struct PreEntity {
    pub pos: Vec3,
    pub rotation: f32,
    pub origin_mesh: Option<Handle<Scene>>,
    pub entity: Option<Entity>,
}

#[derive(Component)]
struct GhostProcessed;

fn process_ghost_model(
    pre_entity: Res<PreEntity>,
    mut commands: Commands,
    children_query: Query<&Children>,
    unprocessed_meshes: Query<&MeshMaterial3d<StandardMaterial>, Without<GhostProcessed>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    let Some(ghost_ent) = pre_entity.entity else {
        return;
    };
    let mut queue = vec![ghost_ent];

    while let Some(current) = queue.pop() {
        if let Ok(children) = children_query.get(current) {
            queue.extend(children.iter());
        }

        if let Ok(mat_handle) = unprocessed_meshes.get(current) {
            commands.entity(current).insert(GhostProcessed);

            if let Some(mat) = materials.get(&mat_handle.0) {
                let mut new_mat = mat.clone();
                new_mat.alpha_mode = AlphaMode::Blend;
                new_mat.base_color = Color::srgba(0.9, 0.9, 0.9, 0.2);

                commands
                    .entity(current)
                    .insert(MeshMaterial3d(materials.add(new_mat)));
            }
        }
    }
}

fn setup_grid(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<GridMaterial>>,
    mut buffers: ResMut<Assets<ShaderStorageBuffer>>,
) {
    let color = LinearRgba::new(0.7, 0.7, 0.7, 0.8);
    let data: Vec<[f32; 4]> = vec![
        [color.red, color.green, color.blue, color.alpha],
        [50.0, 1., 0.0, 0.0],
    ];
    let buffer = buffers.add(ShaderStorageBuffer::from(data));

    commands
        .spawn((
            Mesh3d(meshes.add(Plane3d::default().mesh().size(10000.0, 10000.0))),
            MeshMaterial3d(materials.add(GridMaterial { settings: buffer })),
            Transform::default(),
            Pickable::default(),
        ))
        .observe(on_click)
        .observe(update_pre_entity);
}

fn on_click(
    event: On<Pointer<Press>>,
    palette: Res<AssetPalette>,
    grid: ResMut<WorldGrid>,
    commands: Commands,
    pre_entity: Res<PreEntity>,
) {
    if event.button == PointerButton::Primary {
        left_click(event, palette, pre_entity, grid, commands);
    } else if event.button == PointerButton::Secondary {
        grid_right_click(event, grid, commands);
    }
}

fn left_click(
    event: On<Pointer<Press>>,
    palette: Res<AssetPalette>,
    pre_entity: Res<PreEntity>,
    mut grid: ResMut<WorldGrid>,
    mut commands: Commands,
) {
    let Some(position) = event.hit.position else {
        return;
    };
    let cell_pos = IVec3::new(position.x.floor() as i32, 0, position.z.floor() as i32);
    if grid.cells.contains_key(&cell_pos) {
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
                .with_rotation(Quat::from_rotation_y(pre_entity.rotation)),
            Pickable::IGNORE,
            AutoScale,
        ))
        .observe(on_click)
        .id();
    grid.cells.insert(cell_pos, entity);
}

fn apply_auto_scale(
    mut commands: Commands,
    mut query_roots: Query<(Entity, &mut Transform), With<AutoScale>>,
    children_query: Query<&Children>,
    aabb_query: Query<(&Aabb, &GlobalTransform)>,
) {
    for (root, mut transform) in query_roots.iter_mut() {
        let mut queue = vec![root];
        let mut min_pt = Vec3::splat(f32::MAX);
        let mut max_pt = Vec3::splat(f32::MIN);
        let mut found = false;

        while let Some(current) = queue.pop() {
            if let Ok(children) = children_query.get(current) {
                queue.extend(children.iter());
            }

            if let Ok((aabb, global_transform)) = aabb_query.get(current) {
                let affine = global_transform.compute_transform();

                let center = affine.transform_point(Vec3::from(aabb.center));
                let extents = affine.scale * Vec3::from(aabb.half_extents);

                min_pt = min_pt.min(center - extents.abs());
                max_pt = max_pt.max(center + extents.abs());
                found = true;
            }
        }

        if found {
            let size = max_pt - min_pt;
            let max_size = size.x.max(size.y).max(size.z);

            if max_size > 1.0 {
                let scale_factor = 1.0 / max_size;
                transform.scale = Vec3::splat(scale_factor);
            }

            commands.entity(root).remove::<AutoScale>();
        }
    }
}

fn grid_right_click(event: On<Pointer<Press>>, grid: ResMut<WorldGrid>, commands: Commands) {
    let Some(position) = event.hit.position else {
        return;
    };
    let cell_pos = IVec3::new(position.x.floor() as i32, 0, position.z.floor() as i32);
    delete_entity(grid, commands, cell_pos);
}

fn delete_entity(mut grid: ResMut<WorldGrid>, mut commands: Commands, cell_pos: IVec3) {
    if let Some(entity) = grid.cells.remove(&cell_pos) {
        commands.entity(entity).despawn();
    }
}

fn update_pre_entity(
    event: On<Pointer<Move>>,
    mut pre_entity: ResMut<PreEntity>,
    mut transforms: Query<&mut Transform>,
) {
    let Some(r_mouse_pos) = event.hit.position else {
        return;
    };
    let cell_pos = IVec3::new(
        r_mouse_pos.x.floor() as i32,
        0,
        r_mouse_pos.z.floor() as i32,
    );
    let target_pos = Vec3::new(cell_pos.x as f32 + 0.5, 0.0, cell_pos.z as f32 + 0.5);
    if pre_entity.pos == target_pos || pre_entity.origin_mesh.is_none() {
        return;
    }
    let Some(entity) = pre_entity.entity else {
        return;
    };
    pre_entity.pos = target_pos;
    if let Ok(mut transform) = transforms.get_mut(entity) {
        transform.translation = target_pos;
    }
}

fn rotate_entity(
    input: Res<ButtonInput<KeyCode>>,
    mut pre_entity: ResMut<PreEntity>,
    mut transforms: Query<&mut Transform>,
) {
    if !input.just_pressed(KeyCode::KeyR) {
        return;
    }
    pre_entity.rotation += std::f32::consts::FRAC_PI_4;
    if pre_entity.rotation >= std::f32::consts::TAU {
        pre_entity.rotation -= std::f32::consts::TAU;
    }
    if let Some(entity) = pre_entity.entity {
        if let Ok(mut transform) = transforms.get_mut(entity) {
            transform.rotation = Quat::from_rotation_y(pre_entity.rotation);
        }
    }
}
