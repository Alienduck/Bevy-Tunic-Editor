use bevy::prelude::*;

pub struct PannelPlugin;
impl Plugin for PannelPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, setup)
            .add_systems(Update, animate_panel);
    }
}

#[derive(Component)]
struct AnimatedPanel;

#[derive(Component)]
struct PanelTarget(f32);

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
        .with_child((
            Node {
                width: Val::Vw(15.),
                height: Val::Percent(100.),
                ..default()
            },
            BackgroundColor(Color::srgb(0.2, 0.2, 0.2)),
        ));
}

fn on_hover_enter(_: On<Pointer<Over>>, mut query: Query<&mut PanelTarget, With<AnimatedPanel>>) {
    for mut target in &mut query {
        target.0 = 0.;
    }
}

fn on_hover_exit(_: On<Pointer<Out>>, mut query: Query<&mut PanelTarget, With<AnimatedPanel>>) {
    for mut target in &mut query {
        target.0 = -15.;
    }
}

fn animate_panel(mut query: Query<(&mut Node, &PanelTarget)>, time: Res<Time>) {
    for (mut node, target) in &mut query {
        if let Val::Vw(current) = node.left {
            // LERP method: https://dev.to/rachsmith/lerp-2mh7
            // TODO: use tween: https://github.com/djeedai/bevy_tweening
            node.left = Val::Vw(current + (target.0 - current) * 15. * time.delta_secs());
        }
    }
}
