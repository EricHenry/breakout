use bevy::prelude::*;

// #[cfg(feature = "debug")]


fn main() {
    let mut app = App::new();
    // set up window, the descriptor will be set by default from the
    // Defualt plugins if we don't provide this WindowDescriptor struct
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
    app.add_plugin(WorldInspecectorPlugin::new());

    // start up system.
    app.add_startup_system(startup);

    app.run();
}

#[derive(Component)]
struct Person;

#[derive(Component)]
struct Name(String);

// Startup system, a system that runs only once, before all other systems
fn startup(mut commands: Commands) {
    // Add camera
    commands.spawn_bundle(OrthographicCameraBundle::new_2d());
    commands.spawn_bundle(UiCameraBundle::default());

    // Render a block
    commands
        .spawn()
        .insert(Person)
        .insert(Name("Elaina Proctor".to_string()));
    commands
        .spawn()
        .insert(Person)
        .insert(Name("Renzo Hume".to_string()));
    commands
        .spawn()
        .insert(Person)
        .insert(Name("Zayna Nieves".to_string()));
}

// Timer is a Type defined in Bevy prelude
struct GreetTimer(Timer);

fn greet_people(
    time: Res<Time>, // Time is a resource provided by the default plugins, it gives us the time that has passed since the last update.
    mut timer: ResMut<GreetTimer>,
    query: Query<&Name, With<Person>>,
) {
    if timer.0.tick(time.delta()).just_finished() {
        for name in query.iter() {
            println!("Hello {}!", name.0);
        }
    }
}

// First Plugin
pub struct HelloPlugin;

/// Plugins group together systems.
impl Plugin for HelloPlugin {
    fn build(&self, app: &mut App) {
        // We add in true to from_seconds to indicate that the timer should repeat
        app.insert_resource(GreetTimer(Timer::from_seconds(2.0, true)))
            .add_startup_system(startup)
            .add_system(greet_people);
    }
}
