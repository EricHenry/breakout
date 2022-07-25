use bevy::math::{const_vec2, const_vec3};
use bevy::prelude::*;
use bevy::sprite::collide_aabb::{collide, Collision};
#[cfg(feature = "debug")]
use bevy_inspector_egui::WorldInspectorPlugin;

const WALL_WIDTH: f32 = 900.;
const WALL_HEIGHT: f32 = 600.;
const WALL_THICKNESS: f32 = 20.;
// y coordinates
const WALL_BOTTOM: f32 = -300.;
const WALL_TOP: f32 = 300.;
// x coordinates
const WALL_LEFT: f32 = -450.;
const WALL_RIGHT: f32 = 450.;

// Paddle
const PADDLE_TO_WALL_BOTTOM: f32 = 60.;
const PADDLE_LENGTH: f32 = 120.;
const PADDLE_WIDTH: f32 = 20.;
const PADDLE_SIZE: Vec3 = const_vec3!([PADDLE_LENGTH, PADDLE_WIDTH, 0.0]);
const PADDLE_COLOR: Color = Color::rgb(0.3, 0.3, 0.7);
const PADDLE_SPEED: f32 = 500.0;

// Ball
const BALL_STARTING_POSITION: Vec3 = const_vec3!([0., -50., 1.0]);
const BALL_SIZE: Vec3 = const_vec3!([30., 30., 0.]);
const BALL_SPEED: f32 = 400.;
const INITIAL_BALL_DIRECTION: Vec2 = const_vec2!([0.5, -0.5]);

// Brick
// const BRICK_STARTING_POSITION: Vec3 = const_vec3!([0., 0., 0.]);
const BRICK_LENGTH: f32 = 130.;
const BRICK_WIDTH: f32 = 30.;
const BRICK_SIZE: Vec3 = const_vec3!([BRICK_LENGTH, BRICK_WIDTH, 0.]);
const BRICK_COLOR: Color = Color::rgb(0.7, 0.3, 0.7);
const BRICK_COUNT: f32 = 12.;
const BRICK_PADDING: f32 = 15.;

fn main() {
    let mut app = App::new();
    // set up window, the descriptor will be set by default from the
    // Default plugins if we don't provide this WindowDescriptor struct
    app.insert_resource(WindowDescriptor {
        title: "Breakout!".to_string(),
        width: 1000.,
        height: 800.,
        ..Default::default()
    })
    // DefaultPlugins has a system that will render sprites
    .add_plugins(DefaultPlugins)
    .add_plugin(HelloPlugin);

    // debug window inspector
    #[cfg(feature = "debug")]
    app.add_plugin(WorldInspectorPlugin::new());

    // start up system
    app.add_startup_system(startup);

    app.add_system_set(
        SystemSet::new()
            .with_system(check_collisions)
            .with_system(move_paddle.before(check_collisions))
            .with_system(apply_velocity.before(check_collisions)),
    );

    app.run();
}

#[derive(Component)]
struct Paddle;

#[derive(Bundle)]
struct WallBundle {
    // I can nest bundles inside of bundles
    #[bundle]
    sprite: SpriteBundle,
    collider: Collider,
}

#[derive(Component)]
struct Ball;

#[derive(Component, Debug)]
struct Velocity(Vec2);

#[derive(Component)]
struct Collider;

#[derive(Component)]
struct Brick;

enum WallLocation {
    Top,
    Bottom,
    Left,
    Right,
}

impl From<WallLocation> for WallBundle {
    fn from(location: WallLocation) -> WallBundle {
        let transform = match location {
            WallLocation::Top => Transform {
                translation: Vec3::new(0.0, WALL_TOP, 0.0),
                scale: Vec3::new(WALL_WIDTH + WALL_THICKNESS, WALL_THICKNESS, 1.0),
                ..Default::default()
            },
            WallLocation::Bottom => Transform {
                translation: Vec3::new(0.0, WALL_BOTTOM, 0.0),
                scale: Vec3::new(WALL_WIDTH + WALL_THICKNESS, WALL_THICKNESS, 1.0),
                ..Default::default()
            },
            WallLocation::Left => Transform {
                translation: Vec3::new(WALL_LEFT, 0.0, 0.0),
                scale: Vec3::new(WALL_THICKNESS, WALL_HEIGHT + WALL_THICKNESS, 1.0),
                ..Default::default()
            },
            WallLocation::Right => Transform {
                translation: Vec3::new(WALL_RIGHT, 0.0, 0.0),
                scale: Vec3::new(WALL_THICKNESS, WALL_HEIGHT + WALL_THICKNESS, 1.0),
                ..Default::default()
            },
        };

        WallBundle {
            sprite: SpriteBundle {
                transform,
                ..Default::default()
            },
            collider: Collider,
        }
    }
}

/// Systems
///

/// Startup system, a system that runs only once, before all other systems
fn startup(mut commands: Commands) {
    // Add camera
    commands.spawn_bundle(OrthographicCameraBundle::new_2d());
    commands.spawn_bundle(UiCameraBundle::default());

    let paddle_y: f32 = WALL_BOTTOM + PADDLE_TO_WALL_BOTTOM;

    // Create Paddle
    commands
        .spawn()
        .insert(Paddle)
        .insert(Collider)
        .insert_bundle(SpriteBundle {
            transform: Transform {
                translation: Vec3::new(0.0, paddle_y, 0.0),
                scale: PADDLE_SIZE,
                ..Default::default()
            },
            sprite: Sprite {
                color: PADDLE_COLOR,
                ..Default::default()
            },
            ..Default::default()
        });

    // Create Bricks
    let bricks_columns = (WALL_WIDTH - WALL_THICKNESS) / (BRICK_LENGTH + BRICK_PADDING); // bricks_per_row = (width - wall_thickness) / brick_length + padding
    let bricks_rows = BRICK_COUNT / bricks_columns;

    let starting_x = (WALL_WIDTH / -2.) + WALL_THICKNESS;
    let starting_y = (WALL_HEIGHT / 2.) - WALL_THICKNESS;
    for y in 0..(bricks_rows.ceil() as i32) {
        // figure out how many bricks can fit on one row i.e columns
        for x in 0..(bricks_columns.floor() as i32) {
            let pos_x =
                starting_x + (BRICK_LENGTH * x as f32) + BRICK_PADDING + (BRICK_LENGTH / 2.);
            let pos_y =
                starting_y - (BRICK_WIDTH * y as f32) - (BRICK_PADDING * 2.) - (BRICK_WIDTH / 2.);
            let translation = Vec3::new(pos_x, pos_y, 0.);

            println!("pos_x:{pos_x}, pos_y:{pos_y}");

            commands
                .spawn()
                .insert(Brick)
                .insert(Collider)
                .insert_bundle(SpriteBundle {
                    transform: Transform {
                        translation,
                        scale: BRICK_SIZE,
                        ..Default::default()
                    },
                    sprite: Sprite {
                        color: BRICK_COLOR,
                        ..Default::default()
                    },
                    ..Default::default()
                });
        }
    }

    // Create Ball
    commands
        .spawn()
        .insert(Ball)
        .insert(Velocity(INITIAL_BALL_DIRECTION.normalize() * BALL_SPEED))
        .insert_bundle(SpriteBundle {
            transform: Transform {
                translation: BALL_STARTING_POSITION,
                scale: BALL_SIZE,
                ..Default::default()
            },
            sprite: Sprite {
                color: PADDLE_COLOR,
                ..Default::default()
            },
            ..Default::default()
        });

    // Create Walls
    commands.spawn_bundle(WallBundle::from(WallLocation::Right));
    commands.spawn_bundle(WallBundle::from(WallLocation::Left));
    commands.spawn_bundle(WallBundle::from(WallLocation::Top));
    commands.spawn_bundle(WallBundle::from(WallLocation::Bottom));
}

/// System to move the paddle
fn move_paddle(
    keyboard_input: Res<Input<KeyCode>>,
    time: Res<Time>,
    mut query: Query<&mut Transform, With<Paddle>>,
) {
    let mut paddle_transformation = query.single_mut(); // single_mut panics
    let mut direction = 0.0;

    // <, a, or h key presses will move the paddle to the left
    if keyboard_input.pressed(KeyCode::Left)
        || keyboard_input.pressed(KeyCode::A)
        || keyboard_input.pressed(KeyCode::H)
    {
        direction -= 1.0;
    }

    // >, d, or l key presses will move the paddle to the right
    if keyboard_input.pressed(KeyCode::Right)
        || keyboard_input.pressed(KeyCode::D)
        || keyboard_input.pressed(KeyCode::L)
    {
        direction += 1.0;
    }

    // Defines the amount of time that should elapse between each physics step.
    // using delta_seconds allows us to keep time between changing frame rates. I don't quite understand this fully yet.
    let time_step: f32 = time.delta_seconds();

    // calculate the new horizontal paddle position based on player position
    let new_paddle_position =
        paddle_transformation.translation.x + direction * PADDLE_SPEED * time_step;

    // need to calculate half of the paddle size and wall thickness because they are aligned to the center of the block
    let half_paddle_size = PADDLE_SIZE.x / 2.;
    let half_wall_thickness = WALL_THICKNESS / 2.;
    let left_bound = WALL_LEFT + half_wall_thickness + half_paddle_size;
    let right_bound = WALL_RIGHT - half_wall_thickness - half_paddle_size;

    paddle_transformation.translation.x = new_paddle_position.clamp(left_bound, right_bound);
}

fn apply_velocity(mut velocity_query: Query<(&mut Transform, &Velocity)>, time: Res<Time>) {
    let time_step = time.delta_seconds();
    for (mut transform, velocity) in velocity_query.iter_mut() {
        // println!(
        //     "velocity: {:#?}, \ntime_step: {}, \ntranslation: {}",
        //     velocity, time_step, transform.translation
        // );
        transform.translation.x += velocity.0.x * time_step;
        transform.translation.y += velocity.0.y * time_step;
    }
}

// future TODO: make collisions into events to handle what to do on collisions in future. maybe?
fn check_collisions(
    mut commands: Commands,
    mut ball_query: Query<(&mut Velocity, &Transform), With<Ball>>,
    collider_query: Query<(Entity, &Transform, Option<&Brick>), With<Collider>>,
) {
    let (mut ball_velocity, ball_transform) = ball_query.single_mut();
    let ball_scale = ball_transform.scale.truncate();

    for (collider_entity, collider_transform, brick) in collider_query.iter() {
        if let Some(collision) = collide(
            ball_transform.translation,
            ball_scale,
            collider_transform.translation,
            collider_transform.scale.truncate(),
        ) {
            let mut reflect_x = false;
            let mut reflect_y = false;

            // change the velocity of the ball if the collision happens on the side
            // where the ball is traveling
            //
            // O = ball
            //      ------------
            //      |          |  <- O
            //      ------------
            //
            // when moving towards the block the ball has a negative x velocity, when it hits the right wall
            // it will match as Collision::Right
            match collision {
                Collision::Left => reflect_x = ball_velocity.0.x > 0.0,
                Collision::Right => reflect_x = ball_velocity.0.x < 0.0,
                Collision::Top => reflect_y = ball_velocity.0.y < 0.0,
                Collision::Bottom => reflect_y = ball_velocity.0.y > 0.0,
                Collision::Inside => { /* Noop */ }
            };

            // invert the x and y velocities if a collision happened
            if reflect_x {
                ball_velocity.0.x = -ball_velocity.0.x
            }
            if reflect_y {
                ball_velocity.0.y = -ball_velocity.0.y
            }

            // check if entity is a brick, reduce its life
            if let Some(_brick) = brick {
                commands.entity(collider_entity).despawn();
            }
        }
    }
}

// First Plugin
pub struct HelloPlugin;

/// Plugins group together systems.
impl Plugin for HelloPlugin {
    fn build(&self, _app: &mut App) {
        // We add in true to from_seconds to indicate that the timer should repeat
        // app.insert_resource(GreetTimer(Timer::from_seconds(2.0, true)))
        //     .add_startup_system(startup)
        //     .add_system(greet_people);
    }
}
