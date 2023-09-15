use bevy::{prelude::*, render::{camera::PerspectiveProjection, color}};
use bevy_egui::{egui, EguiContexts, EguiPlugin};
use bevy_toon_shader::{ToonShaderMainCamera, ToonShaderMaterial, ToonShaderPlugin, ToonShaderSun};
use bevy_atmosphere::prelude::*;
use bevy_inspector_egui::quick::WorldInspectorPlugin;
use belly::prelude::*;

mod lib;
mod ui;
fn main() {
    println!("Hello, world!");
    App::new()
        .add_plugins(DefaultPlugins.set(WindowPlugin{
            primary_window: Some(Window { title: "World Breaker (V: Indev)".into(), ..default() }), ..default()
        }))
        .add_plugins(EguiPlugin)
        .add_plugins(WorldInspectorPlugin::default())
        .add_plugins(AtmospherePlugin)
        .add_plugins(lib::PlayerPlugin)
        .add_plugins(ToonShaderPlugin)
        .add_plugins(ui::GameUI)
        
        .add_systems(Startup, start_fn)
        .run();
}
fn start_fn(mut commands: Commands,    mut meshes: ResMut<Assets<Mesh>>,
    mut toon_materials: ResMut<Assets<ToonShaderMaterial>>,){
    commands.spawn((DirectionalLightBundle { ..default()
        
    },ToonShaderSun));
    // camera
    commands.spawn((Camera3dBundle {
        transform: Transform::from_xyz(-2.0, 2.5, 5.0).looking_at(Vec3::ZERO, Vec3::Y),
        ..default()
    }, ToonShaderMainCamera,AtmosphereCamera::default())).insert(lib::FlyCam);

    commands.insert_resource(AmbientLight {
        color: Color::GRAY * 0.2,
        ..default()
    }); 
    let toon_material = toon_materials.add(ToonShaderMaterial{
        color: Color::GREEN, ..default()
    }
        
    );

    commands.spawn(MaterialMeshBundle {
        mesh: meshes.add(Mesh::try_from(shape::Torus::default()).unwrap()),
        transform: Transform::from_xyz(0., 2., 0.),
        
        material: toon_material,
        ..default()
    });
}
