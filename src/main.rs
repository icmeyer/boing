use bevy::prelude::*;
use bevy::window::{WindowResolution, WindowPlugin};
use physical_constants;

use std::f64::consts::PI;
const G: f64 = physical_constants::NEWTONIAN_CONSTANT_OF_GRAVITATION;

fn main() {
    let mut app = App::new();
    app.add_plugins((
        DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                resolution: WindowResolution::new(500., 500.)
                    .with_scale_factor_override(1.0),
                    ..default()
            }),
            ..default()
        }),
    ))
    .add_systems(Startup, setup);
    app.run();
}

fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    commands.spawn(Camera2d);

    let shape = meshes.add(Circle::new(50.0));
    let color = Color::hsl(1.0, 1.0, 1.0);

    commands.spawn((
        Mesh2d(shape),
        MeshMaterial2d(materials.add(color)),
        Transform::from_xyz(0.0, 0.0, 0.0)
        ),
    );
}

struct Dim2 {
    x: f64,
    y: f64,
}

impl Default for Dim2 {
    fn default() -> Self {
        Dim2 {
            x: 0.0,
            y: 0.0,
        }
    }
}

enum Entity{
    Ball,
    Wall,
}

struct Simulation {
    entities: Vec<Entity>,
    time_step: f32,
}

struct Visualization {
}


struct Ball {
    position: Dim2,
    velocity: Dim2,
    radius: f64,
    mass: f64,
    stationary: bool,
}

impl Default for Ball {
    fn default() -> Self {
        Ball {
            position: Dim2::default(),
            velocity: Dim2::default(),
            radius: 1.0,
            mass: 1.0,
            stationary: true,
        }
    }
}

fn grav_force(m1: &f64, m2: &f64, p1: &Dim2, p2: &Dim2) -> Dim2 {
    // F = G*m1*m2/d^2
    let d = distance(p1, p2);
    let f_tot = (G * m1 * m2) / (d * d);
    let dx = p1.x - p2.x;
    let dy = p1.y - p2.y;
    let xcomp = f_tot * (dy/dx).atan().cos();
    let ycomp = f_tot * (dy/dx).atan().sin();
    Dim2 {
        x: xcomp,
        y: ycomp,
    }
}

fn distance(p1: &Dim2, p2: &Dim2) -> f64 {
    ((p1.x - p2.x).powf(2.0) + (p1.y - p2.y).powf(2.0)).powf(0.5)
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn ball() {
        let b = Ball {
            position: Dim2 {
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
        let a = Dim2 {
            x: 0.0,
            y: 0.0,
        };
        let b = Dim2 {
            x: 1.0,
            y: 1.0,
        };
        assert_eq!(distance(&a, &b), (2.0_f64).powf(0.5))
    }
}

