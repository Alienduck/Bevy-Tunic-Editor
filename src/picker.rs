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
        .observe(on_click);
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
        ))
        .id();
    grid.cells.insert(cell_pos, entity);
}
