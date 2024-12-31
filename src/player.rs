use bevy::prelude::*;

use crate::{asset_manager::SceneAssets, schedule::ItemsLoadedInGame};

const OBJECT_SPEED: f32 = 25.0;
const OBJECT_ROTATION_SPEED: f32 = 10.0;
const OBJECT_ROLL_SPEED: f32 = 10.0;
const OBJECT_SCALE: f32 = 35.0;
const DEFAULT_SCALE: Vec3 = Vec3::new(OBJECT_SCALE, OBJECT_SCALE, OBJECT_SCALE);

// #[derive(Component, Debug)]
// pub struct Player {
//     pub username: String,
//     pub health: i8
// }
#[derive(Component, Debug)]
pub struct FoodObject;

#[derive(Component, Debug)]
pub struct Velocity {
    pub value: Vec3,
}

impl Velocity {
    pub fn new(value: Vec3) -> Self {
        Self { value }
    }
}

#[derive(Component, Debug)]
pub struct Acceleration {
    pub value: Vec3,
}

impl Acceleration {
    pub fn new(value: Vec3) -> Self {
        Self { value }
    }
}


#[derive(Bundle)]
pub struct MovingObjectBundle {
    pub model: SceneRoot,
    pub transform: Transform,
    pub velocity: Velocity,
    pub acceleration: Acceleration
    // pub player: Player
}


pub struct MovementPlugin;

impl Plugin for MovementPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Update,
            (update_velocity, update_position)
                .chain()
                .in_set(ItemsLoadedInGame::EntityUpdate),
        );
    }
}

fn update_velocity(mut query: Query<(&Acceleration, &mut Velocity)>, time: Res<Time>) {
    for (acceleration, mut velocity) in query.iter_mut() {
        velocity.value += acceleration.value * time.delta_secs();
    }
}

fn update_position(mut query: Query<(&Velocity, &mut Transform)>, time: Res<Time>) {
    for (velocity, mut transform) in query.iter_mut() {
        transform.translation += velocity.value * time.delta_secs();
    }
}

// impl Default for Player {
//     fn default() -> Self {
//         Self {
//             username: String::from("lol"),
//             health: 60
//         }
//     }
// }

#[derive(Debug)]
pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(PostStartup, create_player)
        .add_systems(
                Update,
                (
                    food_movement_controls
                )
                    .chain()
                    // run controls after items despawn
                    .in_set(ItemsLoadedInGame::UserInput),
            );
    }
}

fn create_player(mut commands: Commands, scene_assets: Res<SceneAssets>) {

    // load ketchup entity with configurations
    commands.spawn((MovingObjectBundle {
        model: SceneRoot(scene_assets.ketchup.clone()),
        transform: Transform::from_translation(Vec3::new(0.0, 0.0, 0.0))
        .with_scale(DEFAULT_SCALE),
        velocity: Velocity::new(Vec3::ZERO),
        acceleration: Acceleration::new(Vec3::ZERO)
        // player: Player::default(),
    },
    FoodObject));
}

// control the movement of objects via the keyboard
// @param query: retrieve first object containing components and food object tag
// @param keyboard_input: use bevy default resource pack containing button input
// @param time: Get global time to handle adjustment of frames for user
fn food_movement_controls(
    mut query: Query<(&mut Transform, &mut Velocity), With<FoodObject>>,
    keyboard_input: Res<ButtonInput<KeyCode>>,
    time: Res<Time>,
) {

    // get the first object, being only object is the ketchup item
    let Ok((mut transform, mut velocity)) = query.get_single_mut() else {
        return;
    };

    
    let mut rotation = 0.0;
    if keyboard_input.pressed(KeyCode::KeyD) {
        rotation = -OBJECT_ROTATION_SPEED * time.delta_secs();
    } else if keyboard_input.pressed(KeyCode::KeyA) {
        rotation = OBJECT_ROTATION_SPEED * time.delta_secs();
    }

    let mut roll = 0.0;
    if keyboard_input.pressed(KeyCode::KeyJ) {
        roll = -OBJECT_ROLL_SPEED * time.delta_secs();
    } else if keyboard_input.pressed(KeyCode::KeyK) {
        roll = OBJECT_ROLL_SPEED * time.delta_secs();
    }

    let mut movement = 0.0;
    if keyboard_input.pressed(KeyCode::KeyW) {
        movement = OBJECT_SPEED;
    } else if keyboard_input.pressed(KeyCode::KeyS) {
        movement = -OBJECT_SPEED;
    }

    transform.rotate_local_x(roll);
    transform.rotate_y(rotation);

    // we handle delta time in movement plugin
    velocity.value = -transform.forward() * movement;
}
