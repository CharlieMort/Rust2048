use bevy::{prelude::*, sprite::MaterialMesh2dBundle, text::{BreakLineOn, Text2dBounds}};
use rand::distributions::{Distribution, Uniform};
use rand::Rng;

const BLOCK_SIZE: f32 = 100.0;
const BLOCK_SPACING: f32 = 7.5;
const TOP_CORNER: Vec2 = Vec2::new(
    -1.5*(BLOCK_SIZE+BLOCK_SPACING),
    -1.5*(BLOCK_SIZE+BLOCK_SPACING)
);

const BACK_BLOCK_COLOR: Color = Color::GRAY;

#[derive(Component)]
struct Block(i32);

#[derive(Resource)]
struct Tiles([[i32; 4]; 4]);

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .insert_resource(Tiles([[2,0,0,4], [0,0,0,0], [0,0,2,0], [0,0,0,0]]))
        .insert_resource(ClearColor(Color::WHITE))
        .add_startup_system(setup)
        .add_system(move_squares)
        .add_system(bevy::window::close_on_esc)
        .run();
}

fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    asset_server: Res<AssetServer>,
    tiles: Res<Tiles>
) {
    commands.spawn(Camera2dBundle::default());

    for x in 0..4 {
        for y in 0..4 {
            commands.spawn((
                MaterialMesh2dBundle {
                    mesh: meshes.add(shape::Quad::new(Vec2::new(BLOCK_SIZE, BLOCK_SIZE)).into()).into(),
                    material: materials.add(ColorMaterial::from(BACK_BLOCK_COLOR)),
                    transform: Transform::from_translation(Vec3::new(
                        TOP_CORNER.x + (BLOCK_SIZE+BLOCK_SPACING) * x as f32, 
                        TOP_CORNER.y + (BLOCK_SIZE+BLOCK_SPACING) * y as f32, 
                        0.0
                    )),
                    ..default()
                },
            ));
        }
    }

    spawn_blocks(asset_server, commands, meshes, materials, tiles);
}

fn can_go(tiles: [[i32; 4]; 4]) -> bool {
    for row in 0..3 {
        for col in 0..3 {
            if tiles[row][col] == tiles[row+1][col] {
                return true;
            }
            else if tiles[row][col] == tiles[row][col+1] {
                return true;
            }
        }
    }
    println!("NO");
    return false;        
}

fn spawn_blocks(
    asset_server: Res<AssetServer>,
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    tiles: Res<Tiles>
) {
    let font = asset_server.load("font.ttf");

    for x in 0..4 {
        for y in 0..4 {
            if tiles.0[y][x] != 0 {
                let hue = tiles.0[y][x] as f32 / 2048.0 * 180.0 + 180.0;
                commands.spawn((
                MaterialMesh2dBundle {
                    mesh: meshes.add(shape::Quad::new(Vec2::new(BLOCK_SIZE, BLOCK_SIZE)).into()).into(),
                    material: materials.add(ColorMaterial::from(Color::Hsla { hue , saturation: 1.0, lightness: 0.5, alpha: 1.0 })),
                    transform: Transform::from_translation(Vec3::new(
                        TOP_CORNER.x + (BLOCK_SIZE+BLOCK_SPACING) * x as f32, 
                        TOP_CORNER.y + (BLOCK_SIZE+BLOCK_SPACING) * y as f32, 
                        1.0
                    )),
                    ..default()
                    },
                    Block((x+y) as i32)
                )).with_children(|builder| {
                    builder.spawn(Text2dBundle {
                        text: Text {
                            sections: vec![TextSection::new(
                                tiles.0[y][x].to_string(),
                                TextStyle { 
                                    font: font.clone(), 
                                    font_size: 50.0, 
                                    color: Color::BLACK
                                }
                            )],
                            alignment: TextAlignment::Center,
                            linebreak_behaviour: BreakLineOn::WordBoundary
                        },
                        text_2d_bounds: Text2dBounds {
                            size: Vec2::new(BLOCK_SIZE, BLOCK_SIZE)
                        },
                        transform: Transform::from_translation(Vec3::Z),
                        ..default()
                    });
                });
            }
        }
    }
}

fn move_squares(
    keyboard_input: Res<Input<KeyCode>>,
    mut tiles: ResMut<Tiles>,
    query: Query<Entity, With<Block>>,
    mut commands: Commands,
    materials: ResMut<Assets<ColorMaterial>>,
    meshes: ResMut<Assets<Mesh>>,
    asset_server: Res<AssetServer>
) {
    let mut key_pressed = false;
    let mut newt = tiles.0.clone();
    let old_tiles = tiles.0.clone();
    if keyboard_input.just_pressed(KeyCode::Left) {
        key_pressed = true;
        for row in 0..4 {
            let mut last_idx = 0;
            for col in 0..4 {
                if newt[row][col] != 0 {
                    let tmp = newt[row][col];
                    newt[row][col] = 0;
                    newt[row][last_idx] = tmp;
                    last_idx = last_idx+1;
                }
            }
        }
        for row in 0..4 {
            for col in 0..3 {
                if newt[row][col] == newt[row][col+1] {
                    newt[row][col] *= 2;
                    for i in (col+1)..3 {
                        newt[row][i] = newt[row][i+1];
                    }
                    newt[row][3] = 0;
                }
            }
        }
    }
    else if keyboard_input.just_pressed(KeyCode::Right) {
        key_pressed = true;
        for row in 0..4 {
            newt[row].reverse();
            let mut last_idx = 0;
            for col in 0..4 {
                if newt[row][col] != 0 {
                    let tmp = newt[row][col];
                    newt[row][col] = 0;
                    newt[row][last_idx] = tmp;
                    last_idx = last_idx+1;
                }
            }
        }
        for row in 0..4 {
            for col in 0..3 {
                if newt[row][col] == newt[row][col+1] {
                    newt[row][col] *= 2;
                    for i in (col+1)..3 {
                        newt[row][i] = newt[row][i+1];
                    }
                    newt[row][3] = 0;
                }
            }
            newt[row].reverse();
        }
    }
    else if keyboard_input.just_pressed(KeyCode::Down) {
        key_pressed = true;
        for col in 0..4 {
            let mut last_idx = 0;
            for row in 0..4 {
                if newt[row][col] != 0 {
                    let tmp = newt[row][col];
                    newt[row][col] = 0;
                    newt[last_idx][col] = tmp;
                    last_idx = last_idx+1;
                }
            }
        }
        for col in 0..4 {
            for row in 0..3 {
                if newt[row][col] == newt[row+1][col] {
                    newt[row][col] *= 2;
                    for i in row+1..3 {
                        newt[i][col] = newt[i+1][col];
                    }
                    newt[3][col] = 0;
                }
            }
        }
    }
    else if keyboard_input.just_pressed(KeyCode::Up) {
        key_pressed = true;
        for col in 0..4 {
            let mut last_idx = 3;
            for row in (0..4).rev() {
                if newt[row][col] != 0 {
                    let tmp = newt[row][col];
                    newt[row][col] = 0;
                    newt[last_idx][col] = tmp;
                    if last_idx > 0 {
                        last_idx = last_idx-1;
                    }
                }
            }
        }
        for col in 0..4 {
            for row in (1..4).rev() {
                if newt[row][col] == newt[row-1][col] {
                    newt[row][col] *= 2;
                    for i in (1..row).rev() {
                        newt[i][col] = newt[i-1][col];
                    }
                    newt[0][col] = 0;
                }
            }
        }
    }
    if key_pressed {
        if newt != old_tiles {
            for entity in query.iter() {
                commands.entity(entity).despawn_recursive();
            }
    
            let mut rng = rand::thread_rng();
            let range = Uniform::from(0..4);
            let mut idx = range.sample(&mut rng);
            let mut idy = range.sample(&mut rng);
            while newt[idy][idx] != 0 {
                idx = range.sample(&mut rng);
                idy = range.sample(&mut rng);
            }
            newt[idy][idx] = rng.gen_range(1..3)*2;
            *tiles = Tiles(newt);
            
            spawn_blocks(asset_server, commands, meshes, materials, tiles.into());
        }
        else {
            if !can_go(newt.clone()) {
                println!("YOU LOST LOSER");
            }
        }
    }
}

