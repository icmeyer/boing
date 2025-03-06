use bevy::prelude::*;

use bevy::math::Vec2;
use super::entities::PhysicsEntity;
use super::constants::G;

use crate::rendering::scene::Scene;

pub fn test_collision(obj_1: &PhysicsEntity, obj_2: &PhysicsEntity) -> Option<Vec2> {
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
    // Check for collisions
    for i in 0..game.entities.len() {
        let (left, right) = game.entities.split_at_mut(i + 1);
        let obj_1 = &mut left[i];

        for obj_2 in right.iter() {
            if let Some(min_transl_vec) = test_collision(obj_1, obj_2) {
                if !obj_1.is_stationary() {
                    let v = obj_1.velocity();
                    let reflection = v - 2.0 * v.dot(min_transl_vec) * min_transl_vec;
                    obj_1.set_velocity(reflection);
                }
                break;
            }
        }
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
    // F = G*m1*m2/d^2
    let d = distance(p1, p2);
    let f_tot = (G * m1 * m2) / (d * d);
    let dx = p1.x - p2.x;
    let dy = p1.y - p2.y;
    let xcomp = f_tot * (dy/dx).atan().cos();
    let ycomp = f_tot * (dy/dx).atan().sin();
    Vec2 {
        x: xcomp,
        y: ycomp,
    }
}


fn distance(p1: &Vec2, p2: &Vec2) -> f32 {
    ((p1.x - p2.x).powf(2.0) + (p1.y - p2.y).powf(2.0)).powf(0.5)
}


fn project(verts: &Vec<Vec2>, axis: &Vec2) -> (f32, f32) {
    let mut min = f32::INFINITY;
    let mut max = -f32::INFINITY;
    for vert in verts {
        let proj = axis.dot(*vert);
        if proj < min { min = proj; }
        if proj > max { max = proj; }
    }
    (min, max)
}
