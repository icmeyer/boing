use bevy::prelude::*;
use crate::physics::entities::PhysicsEntity;

#[derive(Resource, Default)]
pub struct Scene {
    // Struct for storing the spawned objects, used to loop over objects for
    // physics interactions
    pub entities: Vec<PhysicsEntity>,
}

pub fn update_scene(
    _game: Res<Scene>,
    _time: Res<Time>,
) {
    // Do nothing for now
}
