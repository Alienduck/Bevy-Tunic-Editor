use crate::pannel::AssetPalette;
use bevy::{
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
            .add_systems(Startup, setup_grid);
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
) {
    if event.button == PointerButton::Primary {
        left_click(event, palette, grid, commands);
    } else if event.button == PointerButton::Secondary {
        grid_right_click(event, grid, commands);
    }
}

fn left_click(
    event: On<Pointer<Press>>,
    palette: Res<AssetPalette>,
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
            Transform::from_xyz(cell_pos.x as f32 + 0.5, 0.0, cell_pos.z as f32 + 0.5),
            Pickable::default(),
        ))
        .observe(on_click)
        .id();
    grid.cells.insert(cell_pos, entity);
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
