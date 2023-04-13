use bevy::prelude::*;
use heron::prelude::*;
use rand::prelude::*;


fn spawn_camera (
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    ) {

        commands.spawn_bundle(OrthographicCameraBundle::new_2d());

        let texture_handle = asset_server.load("DottedLine.png");
        commands.spawn_bundle(SpriteBundle {
            material: materials.add(texture_handle.into()),
            transform: Transform {
                scale: Vec3::new(10.0, 10.0, 10.0),
                ..Default::default()
            },
            ..Default::default()
        });

    }

    fn main() {
        App::build()
        .insert_resource(WindowDescriptor {
            title: "Pong".to_string(),
            width: 640.0,
            height: 480.0,
            vsync: true,
            ..Default::default()
            
        })

        .insert_resource(ClearColor(Color::rgb(0.0, 0.0, 0.0)))
        .add_plugin(PhysicsPlugin::default())
        .add_plugins(DefaultPlugins)
        .add_startup_system(spawn_camera.system()) //sistema agregado
        .run();

    }