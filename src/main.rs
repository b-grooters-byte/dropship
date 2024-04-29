use bevy::prelude::*;

const SHIP_START_Y: f32 = 200.0;
const SHIP_START_X: f32 = 0.0;
const SHIP_START_VELOCITY_X: f32 = 500.0;
const SHIP_COLOR: Color = Color::rgb(0.8, 0.8, 1.0);
const SHIP_SIZE: Vec2 = Vec2::new(50.0, 50.0);

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_systems(Update, bevy::window::close_on_esc)
        .add_systems(Startup, setup)
        .add_systems(FixedUpdate, move_ship)
        .run();
}

fn setup(mut commands: Commands) {
    // Camera
    commands.spawn(Camera2dBundle::default());
    // ship
    commands.spawn((
        SpriteBundle {
            transform: Transform {
                translation: Vec3::new(SHIP_START_X, SHIP_START_Y, 0.0),
                ..default()
            },
            sprite: Sprite {
                color: SHIP_COLOR,
                custom_size: Some(SHIP_SIZE),
                ..default()
            },
            ..default()
        },
        Ship,
    ));
}

fn move_ship(
    input: Res<ButtonInput<KeyCode>>,
    time: Res<Time>,
    mut query: Query<(&Ship, &mut Transform)>,
) {
    let mut paddle_transform = query.single_mut();
    let mut direction = 0.0;

    if input.pressed(KeyCode::ArrowLeft) {
        direction -= 1.0;
    } else if input.pressed(KeyCode::ArrowRight) {
        direction += 1.0;
    }

    let new_x =
        paddle_transform.1.translation.x + direction * SHIP_START_VELOCITY_X * time.delta_seconds();
    paddle_transform.1.translation.x = new_x;
}

#[derive(Component)]
struct Ship;

#[derive(Component)]
struct Velocity {
    x: f32,
    y: f32,
}
