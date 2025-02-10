use bevy::prelude::*;
use bevy::window::{WindowResolution, WindowPlugin};
use bevy::math::{Vec2, Vec3};
use physical_constants;

use std::f32::consts::PI;
const TWOPI: f32 = 2.0 * PI;
const G: f32 = physical_constants::NEWTONIAN_CONSTANT_OF_GRAVITATION as f32;
const SIDE_LENGTH: f32 = 600.0;

fn main() {
    let mut app = App::new();
    app.add_plugins((
        DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                resolution: WindowResolution::new(SIDE_LENGTH, SIDE_LENGTH)
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

    let mut ball = Ball::new(Vec2::new(0.0, 0.0),
                             SIDE_LENGTH/10.0,
                             Vec2::new(10.0, 2.0));
    ball.entity = commands.spawn((
        Mesh2d(meshes.add(Circle::new(ball.radius))),
        MeshMaterial2d(materials.add(ball.color)),
        Transform::from_xyz(ball.position.x,
                            ball.position.y,
                            0.0)
        ),
    ).id();
    scene.entities.push(Box::new(ball));

    let mut wall = RectangleEntity::new(Vec2::new(SIDE_LENGTH/2.0 - SIDE_LENGTH/20.0,                                            0.0),
                                  SIDE_LENGTH/10.0,
                                  SIDE_LENGTH,
                                  Vec2::ZERO);
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
    fn vertices(&self) -> &Vec<Vec2>;
    fn get_axes(&self) -> &Vec<Vec2>;
    fn entity(&self) -> Entity;
    fn make_axes(&self) -> Vec<Vec2> {
        let mut axes = Vec::<Vec2>::new();
        let verts = self.vertices();
        for i in 0..verts.len() {
            let j = (i + 1) % verts.len();
            let edge = Vec2 {
                x: verts[j].x - verts[i].x,
                y: verts[j].y - verts[i].y,
            };
            let normal = Vec2 {
                x: -edge.y,
                y: edge.x,
            };
            axes.push(normal);
        }
        axes
    }
}

struct Ball {
    position: Vec2,
    radius: f32,
    vertices: Vec<Vec2>,
    axes: Vec<Vec2>,
    velocity: Vec2,
    stationary: bool,
    mass: f32,
    color: Color,
    entity: Entity,
}

impl Ball {
    fn new(position: Vec2, radius: f32, velocity: Vec2) -> Self {
        let mut ball = Ball {
            position,
            radius,
            vertices: Vec::<Vec2>::new(),
            axes: Vec::<Vec2>::new(),
            velocity,
            stationary: false,
            mass: 1.0,
            color: Color::srgba(30./255.0, 61./255.0, 111./255.0, 1.),
            entity: Entity::PLACEHOLDER,
        };
        ball.set_vertices(8);
        ball.axes = ball.make_axes();
        ball
    }

    fn set_vertices(&mut self, n: usize) {
        self.vertices = (0..n).map(|i| {
                let frac = (i as f32) / (n as f32) * TWOPI;
                Vec2::new(self.radius * frac.cos(), self.radius * frac.sin())
        }).collect();
    }
}

impl Default for Ball {
    fn default() -> Self {
        Ball::new(Vec2::ZERO, 1.0, Vec2::ZERO)
    }
}

impl PhysicsEntity for Ball {
    fn position(&self) -> Vec2 { self.position }
    fn velocity(&self) -> Vec2 { self.velocity }
    fn set_position(&mut self, pos: Vec2) { self.position = pos; }
    fn set_velocity(&mut self, vel: Vec2) { self.velocity = vel; }
    fn mass(&self) -> f32 { self.mass }
    fn is_stationary(&self) -> bool { self.stationary }
    fn vertices(&self) -> &Vec<Vec2> { &self.vertices }
    fn get_axes(&self) -> &Vec<Vec2> { &self.axes }
    fn entity(&self) -> Entity { self.entity }
}

struct RectangleEntity {
    position: Vec2,
    width: f32,
    height: f32,
    vertices: Vec::<Vec2>,
    axes: Vec<Vec2>,
    velocity: Vec2,
    stationary: bool,
    mass: f32,
    color: Color,
    entity: Entity,
}

impl RectangleEntity {
    fn new(position: Vec2, width: f32, height: f32, velocity: Vec2) -> Self {
        let mut rec = RectangleEntity {
            position,
            width,
            height,
            velocity,
            vertices: Vec::<Vec2>::new(),
            axes: Vec::<Vec2>::new(),
            stationary: false,
            mass: 1.0,
            color: Color::srgba(30./255.0, 61./255.0, 111./255.0, 1.),
            entity: Entity::PLACEHOLDER,
        };
        rec.set_vertices();
        rec.axes = rec.make_axes();
        rec
    }

    fn set_vertices(&mut self) {
        self.vertices.push(Vec2::new(self.position.x + self.width/2.0, 
                                     self.position.y + self.height/2.0));
        self.vertices.push(Vec2::new(self.position.x - self.width/2.0, 
                                     self.position.y + self.height/2.0));
        self.vertices.push(Vec2::new(self.position.x - self.width/2.0, 
                                     self.position.y - self.height/2.0));
        self.vertices.push(Vec2::new(self.position.x + self.width/2.0, 
                                     self.position.y - self.height/2.0));
    }
}

impl Default for RectangleEntity {
    fn default() -> Self {
        RectangleEntity::new(Vec2::ZERO, 1.0, 2.0, Vec2::ZERO)
    }
}

impl PhysicsEntity for RectangleEntity {
    fn position(&self) -> Vec2 { self.position }
    fn velocity(&self) -> Vec2 { self.velocity }
    fn set_position(&mut self, pos: Vec2) { self.position = pos; }
    fn set_velocity(&mut self, vel: Vec2) { self.velocity = vel; }
    fn mass(&self) -> f32 { self.mass }
    fn is_stationary(&self) -> bool { self.stationary }
    fn vertices(&self) -> &Vec<Vec2> { &self.vertices }
    fn get_axes(&self) -> &Vec<Vec2> { &self.axes }
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

    #[test]
    fn circle_vertices_test() {
        let n = 8;
        let circle_verts = make_circle_vertices(n, 1.0);
        for vert in circle_verts.iter() {
            println!{"Circle vert: {}", vert};
        }
        assert_eq!(circle_verts.len(), n);
    }

}

