use bevy::prelude::*;
use bevy_inspector_egui::quick::WorldInspectorPlugin;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                title: "Bevy game engine".into(),
                resolution: (800., 600.).into(),
                ..default()
            }),
            ..default()
        }))
        .insert_resource(ClearColor(Color::Rgba { red: 0.5, green: 0.8, blue: 0.8, alpha: 1. }))
        .add_plugins((CameraPlugin, BasicScenePlug))
        .add_plugins(WorldInspectorPlugin::new())
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
    }).insert(Name::new("Cube-01"));
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

