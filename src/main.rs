use bevy::prelude::*;
use bevy::window::{WindowResolution, WindowPlugin};
use bevy::math::{Vec2, Vec3};
use physical_constants;

use std::f32::consts::PI;
const G: f32 = physical_constants::NEWTONIAN_CONSTANT_OF_GRAVITATION as f32;
const side_length: f32 = 600.;

fn main() {
    let mut app = App::new();
    app.add_plugins((
        DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                resolution: WindowResolution::new(side_length, side_length)
                    .with_scale_factor_override(1.0),
                    ..default()
            }),
            ..default()
        }),
    ))
    .insert_resource(ClearColor(Color::rgba(0.996078, 0.94902, 0.858824, 1.0)))
    .init_resource::<Scene>()
    .add_systems(Startup, setup)
    .add_systems(Update, update_scene);
    app.run();
}

fn setup(
    mut commands: Commands,
    mut scene: ResMut<Scene>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    ) {
    commands.spawn(Camera2d);

    let mut ball = Ball {
        position: Vec2::new(0.0, 0.0),
        velocity: Vec2::new(50.0, 0.0),
        radius: side_length/10.0,
        color: Color::srgba(30./255.0, 61./255.0, 111./255.0, 1.),
        mass: 20.0,
        stationary: false,
        entity: Entity::PLACEHOLDER,
    };
    ball.entity = commands.spawn((
        Mesh2d(meshes.add(Circle::new(ball.radius))),
        MeshMaterial2d(materials.add(ball.color)),
        Transform::from_xyz(ball.position.x,
                            ball.position.y,
                            0.0)
        ),
    ).id();
    scene.entity_wraps.push(ball);
}

#[derive(Resource, Default)]
struct Scene {
    entity_wraps: Vec<Ball>,
}

fn update_scene(
    game: Res<Scene>,
    time: Res<Time>,
    mut transforms: Query<&mut Transform>,
    ) {
    for entity_wrap in &game.entity_wraps {
        if let Ok(mut transform) = transforms.get_mut(entity_wrap.entity) {
            transform.translation.x += 10.0 * time.delta_secs();
        }
    }
}

enum EntityWrapper {
    Ball,
    Wall,
}

struct Ball {
    position: Vec2,
    velocity: Vec2,
    color: Color,
    radius: f32,
    mass: f32,
    stationary: bool,
    entity: Entity,
}

impl Default for Ball {
    fn default() -> Self {
        Ball {
            position: Vec2::default(),
            velocity: Vec2::default(),
            color: Color::srgb(0.0, 0.0, 0.0),
            radius: 1.0,
            mass: 1.0,
            stationary: true,
            entity: Entity::PLACEHOLDER,
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

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn ball() {
        let b = Ball {
            position: Vec2 {
                x: 1.0,
                y: 2.0,
            },
            ..Default::default()
        };
        assert_eq!(b.position.x, 1.0);
        assert_eq!(b.position.y, 2.0);
    }

    #[test]
    fn distance_test() {
        let a = Vec2 {
            x: 0.0,
            y: 0.0,
        };
        let b = Vec2 {
            x: 1.0,
            y: 1.0,
        };
        assert_eq!(distance(&a, &b), (2.0_f32).powf(0.5))
    }
}

