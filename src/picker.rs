use crate::pannel::AssetPalette;
use bevy::{
    platform::collections::HashMap,
    prelude::*,
    render::render_resource::{AsBindGroup, ShaderType},
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

#[derive(ShaderType, Debug, Clone)]
pub struct GridUniform {
    pub color: LinearRgba,
    pub fade_distance: f32,
    pub line_width: f32,
    pub _padding: Vec2,
}

#[derive(Asset, TypePath, AsBindGroup, Debug, Clone)]
pub struct GridMaterial {
    #[uniform(0)]
    pub settings: GridUniform,
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
) {
    commands
        .spawn((
            Mesh3d(meshes.add(Plane3d::default().mesh().size(10000.0, 10000.0))),
            MeshMaterial3d(materials.add(GridMaterial {
                settings: GridUniform {
                    color: LinearRgba::new(0.7, 0.7, 0.7, 0.8),
                    fade_distance: 60.0,
                    line_width: 1.5,
                    _padding: Vec2::ZERO,
                },
            })),
            Transform::from_xyz(0.0, 0.0, 0.0),
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
