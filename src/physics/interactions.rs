use bevy::prelude::*;

use bevy::math::Vec2;
use super::entities::PhysicsEntity;
use super::constants::G;

use crate::rendering::scene::Scene;

pub fn test_collision(obj_1: &PhysicsEntity, obj_2: &PhysicsEntity) -> Option<Vec2> {
    // Test the collision between two objects using Separating Axis Theorem
    // If the objects are not colliding, return None
    // If the objects are colliding, return the minimum translation vector
    let axes1 = obj_1.get_axes();
    let axes2 = obj_2.get_axes();
    let verts1 = obj_1.translated_verts();
    let verts2 = obj_2.translated_verts();

    let mut overlap = f32::INFINITY;
    let mut min_transl_vec = Vec2::ZERO;
    for axis in axes1.iter() {
        let (min1, max1) = project(&verts1, axis);
        let (min2, max2) = project(&verts2, axis);
        if max1 < min2 || max2 < min1 {
            return None;
        }
        else {
            let o = max1 - min2;
            if o < overlap {
                overlap = o;
                min_transl_vec = -(*axis);
            }
        }
    }
    for axis in axes2.iter() {
        let (min1, max1) = project(&verts1, axis);
        let (min2, max2) = project(&verts2, axis);
        if max1 < min2 || max2 < min1 {
            return None;
        }
        else {
            let o = max1 - min2;
            if o < overlap {
                overlap = o;
                min_transl_vec = *axis;
            }
        }
    }
    Some(min_transl_vec)
}

pub fn kinetic_physics(
    mut game: ResMut<Scene>,
    time: Res<Time>,
    mut transforms: Query<&mut Transform>,
) {
    // Process all kinetic physics including collisions, gravity, and drag
    // Loops through all spawned objects by using the bevy Query command
    // Updates the position and velocity resulting from the time step
    // Check for collisions

    for i in 0..game.entities.len() {
        let (head, tail) = game.entities.split_at_mut(i);
        let (current, tail) = tail.split_first_mut().unwrap();
        if current.is_stationary() { continue; }

        let other_entities = head.iter().chain(tail.iter());
        
        for other in other_entities {
            if let Some(min_transl_vec) = test_collision(current, other) {
                let v = current.velocity();
                let reflection = v - 2.0 * v.dot(min_transl_vec) * min_transl_vec;
                let bounce = 1.33;
                current.set_velocity(bounce * reflection);
                break; // currently only handle collision with one other body
            }
        }
    }

    // Apply gravity
    for i in 0..game.entities.len() {
        let (head, tail) = game.entities.split_at_mut(i);
        let (current, tail) = tail.split_first_mut().unwrap();

        if current.is_stationary() {continue; }

        let other_entities = head.iter().chain(tail.iter());
        
        let mut total_force = Vec2::ZERO;
        for other in other_entities {
            total_force += grav_force(&current.mass(), &other.mass(),
                                      &current.position(), &other.position());
        }
        let delta_v = total_force / current.mass() * time.delta_secs();
        current.set_velocity(current.velocity() + delta_v);
    }

    // // Apply drag
    for entity in game.entities.iter_mut() {
        if entity.is_stationary() {continue; }
        let v = entity.velocity();
        let drag = 10.0 * (v.length() / 5e2).powf(2.0) * v.normalize();
        entity.set_velocity(v - drag);
    }

    // Advance velocity
    for i in 0..game.entities.len() {
        if let Ok(mut transform) = transforms.get_mut(game.entities[i].entity()) {
            let delta_pos = game.entities[i].velocity() * time.delta_secs();
            transform.translation.x += delta_pos.x;
            transform.translation.y += delta_pos.y;
            game.entities[i].physics_mut().position += delta_pos;
        }
    }
}


fn grav_force(m1: &f32, m2: &f32, p1: &Vec2, p2: &Vec2) -> Vec2 {
    // Calculate the gravitational force on object 1
    // Assumes point masses located at provided positions
    // F = G*m1*m2/d^2
    let d = distance(p1, p2);
    let f_tot = 1e16 * (G * m1 * m2) / (d * d);
    // println!("Force: {}", f_tot);
    f_tot * (p2 - p1).normalize()
}


fn distance(p1: &Vec2, p2: &Vec2) -> f32 {
    // Calculate the distance between two 2D positions
    ((p1.x - p2.x).powf(2.0) + (p1.y - p2.y).powf(2.0)).powf(0.5)
}


fn project(verts: &Vec<Vec2>, axis: &Vec2) -> (f32, f32) {
    // Project a set of vertices (2D positions) onto an axis
    // and return the maximum and minimum projections
    let mut min = f32::INFINITY;
    let mut max = -f32::INFINITY;
    for vert in verts {
        let proj = axis.dot(*vert);
        if proj < min { min = proj; }
        if proj > max { max = proj; }
    }
    (min, max)
}

// mod test {
//     use super::*;
// 
//     #[test]
//     fn gravity(){

