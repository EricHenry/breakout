use bevy::math::const_vec3;
use bevy::prelude::*;

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

    // start up system.
    app.add_startup_system(startup);

    app.add_system(move_paddle);

    app.run();
}

#[derive(Component)]
struct Paddle;

#[derive(Component)]
struct Wall;

#[derive(Bundle)]
struct WallBundle {
    // I can nest bundles inside of bundles
    #[bundle]
    sprite: SpriteBundle,
    collider: Collider,
}

#[derive(Component)]
struct Ball;

#[derive(Component)]
struct Velocity(Vec2);

#[derive(Component)]
struct Collider;

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

/// Startup system, a system that runs only once, before all other systems
fn startup(mut commands: Commands) {
    // Add camera
    commands.spawn_bundle(OrthographicCameraBundle::new_2d());
    commands.spawn_bundle(UiCameraBundle::default());

    let paddle_y: f32 = WALL_BOTTOM + PADDLE_TO_WALL_BOTTOM;

    // Create Paddle
    commands.spawn().insert(Paddle).insert_bundle(SpriteBundle {
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

    // Create Ball
    commands.spawn().insert(Ball).insert_bundle(SpriteBundle {
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
