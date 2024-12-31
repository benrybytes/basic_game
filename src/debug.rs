use bevy::prelude::*;

use crate::schedule::ItemsLoadedInGame;

pub struct DebugPlugin;

impl Plugin for DebugPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, print_values.after(ItemsLoadedInGame::EntityUpdate));
    }
}

fn print_values(query: Query<(Entity, &Transform)>) {
    for (entity, transform) in query.iter() {
        // Print the entity ID and the details of the Player component
        info!("Entity: {:?}, Transform: {:?}", entity, transform);
    }
}

