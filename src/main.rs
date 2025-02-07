use bevy::prelude::*;
use bevy::window::{WindowResolution, WindowPlugin};
use bevy::math::{Vec2, Vec3};
use physical_constants;

use std::f32::consts::PI;
const G: f32 = physical_constants::NEWTONIAN_CONSTANT_OF_GRAVITATION as f32;
const side_length: f32 = 600.0;

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
    .insert_resource(ClearColor(Color::srgba(0.996078, 0.94902, 0.858824, 1.0)))
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
        velocity: Vec2::new(10.0, 2.0),
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
    scene.entities.push(Box::new(ball));

    let mut wall = Wall {
        position: Vec2::new(side_length/2.0 - side_length/20.0, 0.0),
        velocity: Vec2::ZERO,
        width: side_length/10.0,
        height: side_length,
        color: Color::srgba(30./255.0, 61./255.0, 111./255.0, 1.),
        mass: 0.0,
        stationary: true,
        entity: Entity::PLACEHOLDER,
    };
    wall.entity = commands.spawn((
        Mesh2d(meshes.add(Rectangle::new(wall.width, wall.height))),
        MeshMaterial2d(materials.add(wall.color)),
        Transform::from_xyz(wall.position.x,
                            wall.position.y,
                            0.0)
        ),
    ).id();
    scene.entities.push(Box::new(wall));
}

#[derive(Resource, Default)]
struct Scene {
    entities: Vec<Box<dyn PhysicsEntity + Sync + Send>>,
}

fn update_scene(
    game: Res<Scene>,
    time: Res<Time>,
    mut transforms: Query<&mut Transform>,
    ) {
    for phys_entity in &game.entities {
        if let Ok(mut transform) = transforms.get_mut(phys_entity.entity()) {
            let delta_pos = phys_entity.velocity() * time.delta_secs();
            transform.translation.x += delta_pos.x;
            transform.translation.y += delta_pos.y;
        }
    }
}

trait PhysicsEntity {
    fn position(&self) -> Vec2;
    fn velocity(&self) -> Vec2;
    fn set_position(&mut self, pos: Vec2);
    fn set_velocity(&mut self, vel: Vec2);
    fn mass(&self) -> f32;
    fn is_stationary(&self) -> bool;
    fn entity(&self) -> Entity;
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

impl PhysicsEntity for Ball {
    fn position(&self) -> Vec2 { self.position }
    fn velocity(&self) -> Vec2 { self.velocity }
    fn set_position(&mut self, pos: Vec2) { self.position = pos; }
    fn set_velocity(&mut self, vel: Vec2) { self.velocity = vel; }
    fn mass(&self) -> f32 { self.mass }
    fn is_stationary(&self) -> bool { self.stationary }
    fn entity(&self) -> Entity { self.entity }
}


struct Wall {
    position: Vec2,
    velocity: Vec2,
    color: Color,
    width: f32,
    height: f32,
    mass: f32,
    stationary: bool,
    entity: Entity,
}

impl Default for Wall {
    fn default() -> Self {
        Wall {
            position: Vec2::default(),
            velocity: Vec2::default(),
            color: Color::srgb(0.0, 0.0, 0.0),
            width: 1.0,
            height: 1.0,
            mass: 1.0,
            stationary: true,
            entity: Entity::PLACEHOLDER,
        }
    }
}

impl PhysicsEntity for Wall {
    fn position(&self) -> Vec2 { self.position }
    fn velocity(&self) -> Vec2 { self.velocity }
    fn set_position(&mut self, pos: Vec2) { self.position = pos; }
    fn set_velocity(&mut self, vel: Vec2) { self.velocity = vel; }
    fn mass(&self) -> f32 { self.mass }
    fn is_stationary(&self) -> bool { self.stationary }
    fn entity(&self) -> Entity { self.entity }
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

