use bevy::prelude::*;
mod player;
mod config;

fn main() {
    App::new()
    .add_plugins(DefaultPlugins)
    .add_systems(Startup, setup)
    .add_systems(Update, player::player::player_movement)
    .run();
}

fn setup(
    mut commands: Commands,
    mut meshes : ResMut<Assets<Mesh>>,
    mut materials : ResMut<Assets<StandardMaterial>>
){
    // commands.spawn(PbrBundle{
    //     mesh : meshes.add(shape::Plane{
    //         size: 1520.,
    //         subdivisions: 2
    //     }.into()),
    //     transform : Transform::from_xyz(0., 0., 0.),
    //     material : materials.add(StandardMaterial{
    //         base_color : Color::WHITE.into(),
    //         ..Default::default()
    //     }),
    //     ..Default::default()
    // });

    commands.spawn(PbrBundle{
        mesh: meshes.add(shape::Plane{
            size: 20.,
            ..Default::default()
        }.into()),
        material: materials.add(Color::rgba(1., 0., 0., 1.).into()),
        ..Default::default()
    });

    commands.spawn(PointLightBundle {
        point_light: PointLight {
            intensity: 1500.0,
            shadows_enabled: true,
            ..default()
        },
        transform: Transform::from_xyz(4.0, 8.0, 4.0),
        ..default()
    });

    commands.spawn((Camera3dBundle {
        transform: Transform::from_xyz(-2.0, 2.5, 5.0).looking_at(Vec3::ZERO, Vec3::Y),
        ..default()
    },
    player::player::Player{})
    );


}


