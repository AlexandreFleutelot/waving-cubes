
use bevy::window::PresentMode;
use bevy::{prelude::*, window::WindowResolution};
use bevy::diagnostic::{FrameTimeDiagnosticsPlugin, LogDiagnosticsPlugin};

const SCREEN_WIDTH:f32 = 800.;
const SCREEN_HEIGHT:f32 = 450.;

const NUM_BLOCKS:usize = 10;

#[derive(Component)]
struct CubeGrid(f32,f32,f32);

fn main() {
    
    let window = Window {
        resolution: WindowResolution::new(SCREEN_WIDTH, SCREEN_HEIGHT),
        resizable: false,
        present_mode: PresentMode::AutoNoVsync,
        ..default()
    };

    App::new()
    .add_plugins(
        DefaultPlugins
            .set(WindowPlugin {
                primary_window: Some(window),
                ..default()
            }),
    )
    .add_plugin(LogDiagnosticsPlugin::default())
    .add_plugin(FrameTimeDiagnosticsPlugin::default())
    .insert_resource(ClearColor(Color::rgb(1.,1.,1.)))
    .add_startup_systems((
        spawn_camera_and_light,
        spawn_cubes))
    .add_system(move_camera)
    .add_system(move_cubes)
    .add_system(resize_cubes)
    //.add_system(color_cubes)
    .run();
}

fn spawn_camera_and_light(
    mut commands: Commands, 
) {
    // camera
    commands.spawn(Camera3dBundle {
        transform: Transform::from_xyz(30.0, 20.0, 30.0)
                            .looking_at(Vec3::ZERO, Vec3::Y),
        projection: Projection::Perspective(PerspectiveProjection { fov: 0.7, ..default() }),   
        ..default()   
    });

    // ambient light
    commands.insert_resource(AmbientLight {
        color: Color::WHITE,
        brightness: 5.0,
    });
    
}

fn move_camera(
    mut camera_query: Query<&mut Transform, With<Camera>>,
    time: Res<Time>,
) {

    let camera_time = time.elapsed_seconds()*0.3;

    if let Ok(mut camera_transform) = camera_query.get_single_mut() {

        camera_transform.translation.x = camera_time.cos() * 40.;
        camera_transform.translation.z = camera_time.sin() * 40.;
        camera_transform.look_at(Vec3::ZERO, Vec3::Y);
    }
}

fn spawn_cubes(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {

    for x in 0..NUM_BLOCKS {
        for y in 0..NUM_BLOCKS {
            for z in 0..NUM_BLOCKS {

                commands.spawn((MaterialMeshBundle {
                    mesh: meshes.add(Mesh::from(shape::Cube { size: 1.0 })),
                    material: materials.add(Color::hsl(((x + y + z) as f32 *18.) % 360.,
                                                            0.77,
                                                            0.5625).into()),
                    transform: Transform::from_xyz(0.,0.,0.),
                    ..default()
                },
                CubeGrid(x as f32,y as f32,z as f32),));
            }
        }
    }    
}

fn move_cubes(
    mut cube_query: Query<(&mut Transform, &CubeGrid)>,
    time: Res<Time>,
) {
    let scale: f32 = (2. + time.elapsed_seconds().sin()) * 0.7;

    for (mut cube_tf, cube_grid) in cube_query.iter_mut() {

        let (x,y,z) = (cube_grid.0,cube_grid.1,cube_grid.2);
        let block_scale = (x + y + z) / 30.;
        let scatter = (20.0*block_scale + 4.0*time.elapsed_seconds()).sin();

        let cube_pos = Vec3::new(
            (x - NUM_BLOCKS as f32 /2.)*(scale*3.) + scatter,
            (y - NUM_BLOCKS as f32 /2.)*(scale*2.) + scatter,
            (z - NUM_BLOCKS as f32 /2.)*(scale*3.) + scatter);

        cube_tf.translation = cube_pos;
    }
}

fn resize_cubes(
    mut cube_query: Query<(&mut Transform, &CubeGrid)>,
    time: Res<Time>,
) {
    let scale: f32 = (2. + time.elapsed_seconds().sin()) * 0.7;

    for (mut cube_tf, cube_grid) in cube_query.iter_mut() {

        let (x,y,z) = (cube_grid.0,cube_grid.1,cube_grid.2);
        let block_scale = (x + y + z) as f32/30.;

        let cube_scale = Vec3::ONE * (2.4 - scale)*block_scale;

        cube_tf.scale = cube_scale;
    }
}

// fn color_cubes(
//     mut cube_query: Query<(&Handle<StandardMaterial>, &CubeGrid)>,
//     mut materials: ResMut<Assets<StandardMaterial>>
// ) {
//     for (cube_handle, cube_grid) in cube_query.iter_mut() {

//         let (x,y,z) = (cube_grid.0,cube_grid.1,cube_grid.2);
        
//         if let Some(material) = materials.get_mut(cube_handle) {
//             material.base_color = Color::hsl(((x + y + z)*18.) % 360.,0.77,0.5625);
//         }
//     }
// }