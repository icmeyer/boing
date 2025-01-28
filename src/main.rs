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
    mut game: ResMut<Scene>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    ) {
    commands.spawn(Camera2d);

    let bluish = Color::srgba(30./255.0, 61./255.0, 111./255.0, 1.);
    let circ_id = commands.spawn((
        Mesh2d(meshes.add(Circle::new(side_length/10.0))),
        MeshMaterial2d(materials.add(bluish)),
        Transform::from_xyz(0.0, 0.0, 0.0),
        ),
    ).id();

    let shape2 = meshes.add(Circle::new(10.0));
    let color2 = Color::hsl(99.0, 99.0, 0.0);
    let circ_id2 = commands.spawn((
        Mesh2d(shape2),
        MeshMaterial2d(materials.add(color2)),
        Transform::from_xyz(100.0, 0.0, 0.0),
        ),
    ).id();
    game.entities.push(circ_id);
    game.entities.push(circ_id2);
}

#[derive(Resource, Default)]
struct Scene {
    entities: Vec<Entity>,
}

fn update_scene(
    game: Res<Scene>,
    time: Res<Time>,
    mut transforms: Query<&mut Transform>,
    ) {
    for entity in &game.entities {
        if let Ok(mut transform) = transforms.get_mut(*entity) {
            transform.translation.x += 10.0 * time.delta_secs();
        }
    }
}

struct Visualization {
}

enum EntityTracker {
    Ball,
    Wall,
}

struct Ball {
    position: Vec2,
    velocity: Vec2,
    radius: f32,
    mass: f32,
    stationary: bool,
}

impl Default for Ball {
    fn default() -> Self {
        Ball {
            position: Vec2::default(),
            velocity: Vec2::default(),
            radius: 1.0,
            mass: 1.0,
            stationary: true,
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

