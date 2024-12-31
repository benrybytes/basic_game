// handle system as state changes by each variant for each frame

use bevy::prelude::*;

#[derive(SystemSet, Hash, Eq, PartialEq, Debug, Clone)]
pub enum ItemsLoadedInGame {
    DespawnEntities, // run systems that delete entites 
    UserInput,
    EntityUpdate, // run systems to update entities
}

pub struct HandleGameState;

impl Plugin for HandleGameState {
    fn build(&self, app: &mut App) {
        // configure current state each frame
        app.configure_sets(Update, (
            ItemsLoadedInGame::DespawnEntities, 
            ItemsLoadedInGame::EntityUpdate)
            .chain())
            .add_systems(
                    Update,
                    apply_deferred
                        .after(ItemsLoadedInGame::DespawnEntities)
                        .before(ItemsLoadedInGame::UserInput),
                );}
}
