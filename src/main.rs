use std::f32::consts::PI;

use bevy::prelude::*;
use bevy_inspector_egui::quick::WorldInspectorPlugin;

fn main() {
    App::new()
        // General Setup
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                title: "Bevy game engine".into(),
                resolution: (800., 600.).into(),
                ..default()
            }),
            ..default()
        }))
        .insert_resource(ClearColor(Color::Rgba { red: 0.5, green: 0.8, blue: 0.8, alpha: 1. }))

        // Inspector setup
        .add_plugins(WorldInspectorPlugin::new())
        // Inspector Registries

        // Custom setup
        .add_plugins((CameraPlugin, BasicScenePlug))
        .add_plugins(TowerPlugin)
        .run();
}

pub struct CameraPlugin;

impl Plugin for CameraPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_systems(Startup, spawn_camera)
        ;
    }
}

fn spawn_camera(mut commands: Commands) {
    commands.spawn(Camera3dBundle {
        transform: Transform::from_xyz(-2., 2.5, 5.0)
            .looking_at(Vec3::ZERO, Vec3::Y),
            ..default()
    }).insert(Name::new("main_camera"));
}


fn spawn_basic_scene(mut commands: Commands, mut meshes: ResMut<Assets<Mesh>>, mut materials: ResMut<Assets<StandardMaterial>>) {
    // Create a plane
    commands.spawn(PbrBundle {
        mesh: meshes.add(Mesh::from(shape::Plane{ size: 5.0, ..default() })),
        material: materials.add(Color::rgb(0.184, 0.678, 0.235).into()),
        ..default()
    }).insert(Name::new("Plane"));

    // Create a Cube
    commands.spawn(PbrBundle {
        mesh: meshes.add(Mesh::from(shape::Cube{ size: 1.0 })),
        material: materials.add(Color::rgb(0.675, 0.506, 0.929).into()),
        transform: Transform::from_xyz(0., 0.5, 0.),
        ..default()
    })
    .insert(Tower {
        shooting_timer: Timer::from_seconds(1.0, TimerMode::Repeating)
    })
    .insert(Name::new("Tower-01"));

    // Create a Light source
    commands.spawn( PointLightBundle {
        point_light: PointLight { intensity: 1500.0,  shadows_enabled: true,  ..default() },
        transform: Transform::from_xyz(4., 8., 4.),
        ..default()
    }).insert(Name::new("main_light"));
}

pub struct BasicScenePlug;

impl Plugin for BasicScenePlug {
    fn build(&self, app: &mut App) {
        app
            .add_systems(Startup, spawn_basic_scene)
        ;
    }
}


// Components

pub struct TowerPlugin;
impl Plugin for TowerPlugin {
    fn build(&self, app: &mut App) {
        app
            // Registries
            .register_type::<Tower>()
            // Systems
            .add_systems(Update, tower_shoot)
            .add_systems(Update, bullet_despawn)
        ;
    }
}

#[derive(Reflect, Component, Default)]
#[reflect(Component)]
struct Tower {
    shooting_timer: Timer,
}

#[derive(Reflect, Component, Default)]
#[reflect(Component)]
pub struct Lifetime {
    timer: Timer,
}

fn bullet_despawn(
    mut commands: Commands,
    mut bullets: Query<(Entity, &mut Lifetime)>,
    timer: Res<Time>,
) {
    for (entity, mut lifetm) in &mut bullets {
        lifetm.timer.tick(timer.delta());
        if lifetm.timer.just_finished() {
            commands.entity(entity).despawn_recursive();
        }
    }
}

fn tower_shoot(
    mut commands: Commands, 
    mut meshes: ResMut<Assets<Mesh>>, 
    mut materials: ResMut<Assets<StandardMaterial>>, 
    mut towers: Query<&mut Tower>,
    time: Res<Time>,
) {
    for mut tower in &mut towers {
        tower.shooting_timer.tick(time.delta());
        if tower.shooting_timer.just_finished() {
            let spawn_transform = 
                Transform::from_xyz(0.0, 0.7, 0.6)
                .with_rotation(Quat::from_rotation_y(- PI / 2.0));

            commands.spawn(
                PbrBundle {
                    mesh: meshes.add(Mesh::from(shape::Cube { size: 0.2 })),
                    material: materials.add(Color::rgb(0.87, 0.44, 0.42).into()),
                    transform: spawn_transform,
                    ..Default::default()
                }
            )
            .insert(Lifetime {
                timer: Timer::from_seconds(0.5, TimerMode::Once),
            })
            .insert(Name::new("bullet"));
        }
    }
}
