use bevy::prelude::*;
use std::fs;

use crate::picker::PreEntity;

pub struct PannelPlugin;
impl Plugin for PannelPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, setup)
            .add_systems(Update, animate_panel)
            .init_resource::<AssetPalette>();
    }
}

#[derive(Component)]
struct AnimatedPanel;
#[derive(Component)]
struct PanelTarget(f32);
#[derive(Component)]
struct LoadAssetButton;
#[derive(Component)]
struct AssetListContainer;
#[derive(Component)]
struct SelectAssetButton(usize);

#[derive(Resource, Default)]
pub struct AssetPalette {
    pub loaded_models: Vec<(String, Handle<Scene>)>,
    pub selected_index: Option<usize>,
}

impl LoadAssetButton {
    fn on_pressed(
        _click: On<Pointer<Click>>,
        asset_server: Res<AssetServer>,
        mut palette: ResMut<AssetPalette>,
        mut commands: Commands,
        query_container: Query<Entity, With<AssetListContainer>>,
    ) {
        let Some(path) = rfd::FileDialog::new()
            .add_filter("3D", &["glb", "gltf"])
            .pick_file()
        else {
            return;
        };
        let file_name = path.file_name().unwrap().to_str().unwrap().to_string();
        let dest = format!("assets/imports/{}", file_name);
        let _ = fs::create_dir_all("assets/imports");

        if fs::copy(&path, &dest).is_ok() {
            let handle: Handle<Scene> = asset_server.load(format!("imports/{}#Scene0", file_name));
            palette.loaded_models.push((file_name.clone(), handle));
            let new_idx = palette.loaded_models.len() - 1;

            if let Ok(container) = query_container.single() {
                commands.entity(container).with_children(|parent| {
                    parent
                        .spawn((
                            Button,
                            Node {
                                width: Val::Percent(100.),
                                height: Val::Px(30.),
                                margin: UiRect::bottom(Val::Px(5.)),
                                ..default()
                            },
                            BackgroundColor(Color::srgb(0.3, 0.3, 0.3)),
                            SelectAssetButton(new_idx),
                        ))
                        .with_children(|btn| {
                            btn.spawn((
                                Text(file_name),
                                TextLayout::new_with_justify(Justify::Center),
                            ));
                        })
                        .observe(on_asset_selected);
                });
            }
        }
    }
}

fn on_asset_selected(
    click: On<Pointer<Click>>,
    query: Query<&SelectAssetButton>,
    mut pre_entity: ResMut<PreEntity>,
    mut palette: ResMut<AssetPalette>,
    mut commands: Commands,
) {
    if let Ok(btn) = query.get(click.entity) {
        palette.selected_index = Some(btn.0);
        let Some((_, handle)) = palette.loaded_models.get(btn.0) else {
            return;
        };
        pre_entity.origin_mesh = Some(handle.clone());
        if pre_entity.entity.is_some() {
            commands.entity(pre_entity.entity.unwrap()).despawn();
        }
        pre_entity.entity = Some(
            commands
                .spawn((SceneRoot(handle.clone()), Pickable::IGNORE))
                .id(),
        );
    }
}

fn setup(mut commands: Commands) {
    commands
        .spawn((
            Node {
                width: Val::Vw(17.),
                height: Val::Percent(100.),
                position_type: PositionType::Absolute,
                left: Val::Vw(-15.),
                ..default()
            },
            BackgroundColor(Color::NONE),
            AnimatedPanel,
            PanelTarget(-15.),
        ))
        .observe(on_hover_enter)
        .observe(on_hover_exit)
        .with_children(|hitbox| {
            hitbox
                .spawn((
                    Node {
                        width: Val::Vw(15.),
                        height: Val::Percent(100.),
                        flex_direction: FlexDirection::Column,
                        align_items: AlignItems::Center,
                        ..default()
                    },
                    BackgroundColor(Color::srgb(0.2, 0.2, 0.2)),
                ))
                .with_children(|panel| {
                    panel.spawn((
                        Node {
                            width: Val::Percent(80.),
                            height: Val::Px(40.),
                            margin: UiRect::top(Val::Px(20.)),
                            ..default()
                        },
                        Text("Palette".to_string()),
                        TextLayout::new_with_justify(Justify::Center),
                    ));
                    panel
                        .spawn((
                            Button,
                            Node {
                                width: Val::Percent(80.),
                                height: Val::Px(40.),
                                margin: UiRect::top(Val::Px(10.)),
                                ..default()
                            },
                            BackgroundColor(Color::srgb(0.4, 0.4, 0.4)),
                            LoadAssetButton,
                        ))
                        .with_children(|btn| {
                            btn.spawn((
                                Text("Load".to_string()),
                                TextLayout::new_with_justify(Justify::Center),
                            ));
                        })
                        .observe(LoadAssetButton::on_pressed);

                    panel.spawn((
                        Node {
                            width: Val::Percent(80.),
                            height: Val::Percent(100.),
                            flex_direction: FlexDirection::Column,
                            margin: UiRect::top(Val::Px(20.)),
                            ..default()
                        },
                        AssetListContainer,
                    ));
                });
        });
}

fn on_hover_enter(_: On<Pointer<Over>>, mut q: Query<&mut PanelTarget, With<AnimatedPanel>>) {
    for mut t in &mut q {
        t.0 = 0.;
    }
}
fn on_hover_exit(_: On<Pointer<Out>>, mut q: Query<&mut PanelTarget, With<AnimatedPanel>>) {
    for mut t in &mut q {
        t.0 = -15.;
    }
}

fn animate_panel(mut query: Query<(&mut Node, &PanelTarget)>, time: Res<Time>) {
    for (mut node, target) in &mut query {
        if let Val::Vw(current) = node.left {
            node.left = Val::Vw(current + (target.0 - current) * 15. * time.delta_secs());
        }
    }
}
