use bevy::prelude::*;

#[cfg(feature = "debug")]
use bevy_inspector_egui::WorldInspectorPlugin;

fn main() {
    let mut app = App::new();
    // set up window, the descriptor will be set by default from the
    // Default plugins if we don't provide this WindowDescriptor struct
    app.insert_resource(WindowDescriptor {
        title: "Breakout!".to_string(),
        width: 700.,
        height: 800.,
        ..Default::default()
    })
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

/// Startup system, a system that runs only once, before all other systems
fn startup(mut commands: Commands) {
    // Add camera
    commands.spawn_bundle(OrthographicCameraBundle::new_2d());
    commands.spawn_bundle(UiCameraBundle::default());

    let paddle_y = -300. + 60.0;
    let paddle_size = Vec3::new(120.0, 20.0, 0.0);
    let paddle_color = Color::rgb(0.3, 0.3, 0.7);

    commands.spawn().insert(Paddle).insert_bundle(SpriteBundle {
        transform: Transform {
            translation: Vec3::new(0.0, paddle_y, 0.0),
            scale: paddle_size,
            ..Default::default()
        },
        sprite: Sprite {
            color: paddle_color,
            ..Default::default()
        },
        ..Default::default()
    });

    // left wall
    commands.spawn().insert(Wall).insert_bundle(SpriteBundle {
        transform: Transform {
            translation: Vec3::new(-300., 0.0, 0.0),
            scale: Vec3::new(20.0, 600., 0.0),
            ..Default::default()
        },
        sprite: Default::default(),
        ..Default::default()
    });
    // right wall
    commands.spawn().insert(Wall).insert_bundle(SpriteBundle {
        transform: Transform {
            translation: Vec3::new(300., 0.0, 0.0),
            scale: Vec3::new(20.0, 600., 0.0),
            ..Default::default()
        },
        sprite: Default::default(),
        ..Default::default()
    });
    // top wall
    commands.spawn().insert(Wall).insert_bundle(SpriteBundle {
        transform: Transform {
            translation: Vec3::new(0.0, 300., 0.0),
            scale: Vec3::new(600., 20., 0.0),
            ..Default::default()
        },
        sprite: Default::default(),
        ..Default::default()
    });
    // bottom wall
    commands.spawn().insert(Wall).insert_bundle(SpriteBundle {
        transform: Transform {
            translation: Vec3::new(0.0, -300., 0.0),
            scale: Vec3::new(600., 20., 0.0),
            ..Default::default()
        },
        sprite: Default::default(),
        ..Default::default()
    });
}

/// System to move the paddle
fn move_paddle(
    keyboard_input: Res<Input<KeyCode>>,
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

    let paddle_speed: f32 = 500.0;
    // Defines the amount of time that should elapse between each physics step.
    let time_step: f32 = 1.0 / 60.0;

    // calculate the new horizontal paddle position based on player position
    let new_paddle_position =
        paddle_transformation.translation.x + direction * paddle_speed * time_step;

    // TODO: figure out bounds of arena
    // paddle_transformation.translation.x = new_paddle_position.clamp(left_bound, right_bound)

    paddle_transformation.translation.x = new_paddle_position;
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
