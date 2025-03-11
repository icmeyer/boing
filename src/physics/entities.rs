use bevy::math::Vec2;
use bevy::prelude::*;

use super::constants::{TWOPI};

use crate::rendering::scene::Scene;


pub struct PhysicsData {
    pub position: Vec2,
    pub velocity: Vec2,
    pub vertices: Vec<Vec2>,
    pub axes: Vec<Vec2>,
    pub stationary: bool,
    pub mass: f32,
}

pub struct KinematicData {
    pub position: Vec2,
    pub velocity: Vec2,
    pub radius: f32,
    pub mass: f32,
}

impl KinematicData {
    pub fn new(position: Vec2, velocity: Vec2, radius: f32, 
               mass: f32) -> Self {
        Self { position, velocity, radius, mass }
    }
}

impl PhysicsData {
    fn new(position: Vec2, velocity: Vec2) -> Self {
        PhysicsData {
            position,
            velocity,
            vertices: Vec::new(),
            axes: Vec::new(),
            stationary: false,
            mass: 1.0,
        }
    }

    fn set_axes(&mut self) {
        // Calculate and set the axes which are the normals to the edges of the 
        // shape
        let verts = &self.vertices;
        for i in 0..verts.len() {
            let j = (i + 1) % verts.len();
            let edge = Vec2 {
                x: verts[j].x - verts[i].x,
                y: verts[j].y - verts[i].y,
            };
            let mag = (edge.x.powf(2.0) + edge.y.powf(2.0)).powf(0.5);
            let normal = Vec2 {
                x: -edge.y / mag,
                y: edge.x / mag,
            };
            self.axes.push(normal);
        }
    }

}

pub struct BallEntity {
    pub physics: PhysicsData,
    pub bevy: BevyData,
    pub radius: f32,
}

pub struct RectangleEntity {
    pub physics: PhysicsData,
    pub bevy: BevyData,
    pub width: f32,
    pub height: f32,
}


pub enum PhysicsEntity {
    Ball(BallEntity),
    Rectangle(RectangleEntity),
}

impl PhysicsEntity {
    pub fn physics(&self) -> &PhysicsData {
        match self {
            Self::Ball(ball) => &ball.physics,
            Self::Rectangle(rect) => &rect.physics,
        }
    }

    pub fn physics_mut(&mut self) -> &mut PhysicsData {
        match self {
            Self::Ball(ball) => &mut ball.physics,
            Self::Rectangle(rect) => &mut rect.physics,
        }
    }

    pub fn position(&self) -> Vec2 { self.physics().position }
    pub fn velocity(&self) -> Vec2 { self.physics().velocity }
    pub fn set_position(&mut self, pos: Vec2) { self.physics_mut().position = pos; }
    pub fn set_velocity(&mut self, vel: Vec2) { self.physics_mut().velocity = vel; }
    pub fn mass(&self) -> f32 { self.physics().mass }
    pub fn is_stationary(&self) -> bool { self.physics().stationary }
    pub fn vertices(&self) -> &Vec<Vec2> { &self.physics().vertices }
    pub fn get_axes(&self) -> &Vec<Vec2> { &self.physics().axes }
    pub fn entity(&self) -> Entity {
        match self {
            Self::Ball(ball) => ball.bevy.entity,
            Self::Rectangle(rect) => rect.bevy.entity,
        }
    }

    pub fn translated_verts(&self) -> Vec<Vec2> {
        self.vertices().iter().map(|v| {
            Vec2 {
                x: v.x + self.position().x,
                y: v.y + self.position().y,
            }
        }).collect()
    }
}

impl BallEntity {
    // Balls are initialized with a radius
    pub fn new(position: Vec2, velocity: Vec2, radius: f32) -> Self {
        let mut ball = BallEntity {
            physics: PhysicsData::new(position, velocity),
            bevy: BevyData::new(),
            radius,
        };

        ball.set_vertices(16);
        ball.physics.set_axes();
        ball
    }

    pub fn spawn(
        commands: &mut Commands,
        scene: &mut ResMut<Scene>,
        meshes: &mut ResMut<Assets<Mesh>>,
        materials: &mut ResMut<Assets<ColorMaterial>>,
        kinematic_data: KinematicData,
    ) {
        let mut ball = BallEntity::new(
            kinematic_data.position,
            kinematic_data.velocity,
            kinematic_data.radius
        );

        ball.bevy.entity = commands.spawn((
            Mesh2d(meshes.add(Circle::new(ball.radius))),
            MeshMaterial2d(materials.add(ball.bevy.color)),
            Transform::from_xyz(
                ball.physics.position.x,
                ball.physics.position.y,
                0.0
            ),
        )).id();

        scene.entities.push(PhysicsEntity::Ball(ball));
    }

    fn set_vertices(&mut self, n: i64) { 
        self.physics.vertices = (0..n).map(|i| {
                let frac = (i as f32) / (n as f32) * TWOPI;
                Vec2::new(self.radius * frac.cos(), self.radius * frac.sin())
        }).collect();
    }
}

impl RectangleEntity {
    // Rectangles are initialized with a width and height
    pub fn new(position: Vec2, velocity: Vec2, width: f32, height: f32,) -> Self {
        let mut rect = RectangleEntity {
            physics: PhysicsData::new(position, velocity),
            bevy: BevyData::new(),
            width,
            height,
        };
        rect.set_vertices();
        rect.physics.set_axes();
        rect
    }

    fn set_vertices(&mut self) {
        let verts = &mut self.physics.vertices;
        verts.push(Vec2::new(self.width/2.0, self.height/2.0));
        verts.push(Vec2::new(-self.width/2.0, self.height/2.0));
        verts.push(Vec2::new(-self.width/2.0, -self.height/2.0));
        verts.push(Vec2::new(self.width/2.0, -self.height/2.0));
    }
}

pub struct BevyData {
    pub color: Color,
    pub entity: Entity,
}

impl BevyData {
    pub fn new() -> Self {
        BevyData {
            color: Color::srgba(30./255.0, 61./255.0, 111./255.0, 1.),
            entity: Entity::PLACEHOLDER,
        }
    }
}
