use bevy::prelude::*;

const SHIP_START_Y: f32 = 0.0;
const SHIP_START_X: f32 = 0.0;
const SHIP_START_VELOCITY_X: f32 = 500.0;

const LANDER_MASS: f32 = 15_000.0;
const LANDER_FUEL_MASS: f32 = 5_000.0;
const LANDER_THRUST_MAX: f32 = 33_750.0;
const LANDER_THRUST_MIN: f32 = 4_500.0;

const LUNAR_GRAVITY: f32 = 1.625;
const EUROPA_GRAVITY: f32 = 1.314;
const IO_GRAVITY: f32 = 1.796;
const CALLISTO_GRAVITY: f32 = 1.235;
const GANYMEDE_GRAVITY: f32 = 1.428;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_systems(Update, bevy::window::close_on_esc)
        .add_systems(Startup, setup)
        .add_systems(FixedUpdate, apply_velocity)
        .run();
}

fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    // Camera
    commands.spawn(Camera2dBundle::default());
    // ship
    commands.spawn((
        SpriteBundle {
            texture: asset_server.load("lander-static.png"),
            transform: Transform {
                rotation: Quat::from_rotation_z((30.0_f32).to_radians()),
                translation: Vec3::new(SHIP_START_X, SHIP_START_Y, 0.0),
                ..default()
            },
            ..default()
        },
        Ship,
        Velocity(Vec2::new(0.0, 0.0)),
    ));
}

fn apply_velocity(
    input: Res<ButtonInput<KeyCode>>,
    time: Res<Time>,
    mut query: Query<(&mut Transform, &mut Velocity)>,
) {
    let mut ship_velocity = query.single_mut();
    let mut delta_velocity = Vec2::new(0.0, 0.0);

    // caclulate acceleration due to gravity
    let gravity_accel = LUNAR_GRAVITY * 10.0;
    delta_velocity.y -= gravity_accel;
    if input.pressed(KeyCode::ArrowLeft) {
        delta_velocity.x -= 100.0;
    } else if input.pressed(KeyCode::ArrowRight) {
        delta_velocity.x += 100.0;
    } else if input.pressed(KeyCode::ArrowUp) {
        delta_velocity.y += 100.0;
    } else if input.pressed(KeyCode::ArrowDown) {
        delta_velocity.y -= 100.0;
    }
    ship_velocity.1 .0 += delta_velocity * time.delta_seconds();
    ship_velocity.0.translation.x += ship_velocity.1 .0.x * time.delta_seconds();
    ship_velocity.0.translation.y += ship_velocity.1 .0.y * time.delta_seconds();
}

#[derive(Component)]
struct Ship;

#[derive(Component)]
struct Velocity(Vec2);
