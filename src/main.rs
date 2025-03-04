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
    .add_systems(Update, update_scene)
    .add_systems(FixedUpdate, kinetic_physics);
    app.run();
}

fn setup(
    mut commands: Commands,
    mut scene: ResMut<Scene>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    ) {
    commands.spawn(Camera2d);

    let mut ball = BallEntity::new(Vec2::new(0.0, 0.0),
                                   Vec2::new(330.0, 200.0),
                                   SIDE_LENGTH/10.0,
                               );
    ball.bevy.entity = commands.spawn((
        Mesh2d(meshes.add(Circle::new(ball.radius))),
        MeshMaterial2d(materials.add(ball.bevy.color)),
        Transform::from_xyz(ball.physics.position.x,
                            ball.physics.position.y,
                            0.0)
        ),
    ).id();
    scene.entities.push(PhysicsEntity::Ball(ball));

    let mut wall =  RectangleEntity::new(Vec2::new(SIDE_LENGTH/2.0 - SIDE_LENGTH/20.0, 0.0),
                        Vec2::ZERO,
                        SIDE_LENGTH/10.0,
                        SIDE_LENGTH,
                        );
    wall.bevy.entity = commands.spawn((
        Mesh2d(meshes.add(Rectangle::new(wall.width, wall.height))),
        MeshMaterial2d(materials.add(wall.bevy.color)),
        Transform::from_xyz(wall.physics.position.x,
                            wall.physics.position.y,
                            0.0)
        ),
    ).id();
    scene.entities.push(PhysicsEntity::Rectangle(wall));

    let mut wall =  RectangleEntity::new(Vec2::new(-SIDE_LENGTH/2.0 + SIDE_LENGTH/20.0, 0.0),
                        Vec2::ZERO,
                        SIDE_LENGTH/10.0,
                        SIDE_LENGTH,
                        );
    wall.bevy.entity = commands.spawn((
        Mesh2d(meshes.add(Rectangle::new(wall.width, wall.height))),
        MeshMaterial2d(materials.add(wall.bevy.color)),
        Transform::from_xyz(wall.physics.position.x,
                            wall.physics.position.y,
                            0.0)
        ),
    ).id();
    scene.entities.push(PhysicsEntity::Rectangle(wall));

    let mut wall =  RectangleEntity::new(Vec2::new(0.0, SIDE_LENGTH/2.0 - SIDE_LENGTH/20.0),
                        Vec2::ZERO,
                        SIDE_LENGTH,
                        SIDE_LENGTH/10.0,
                        );
    wall.bevy.entity = commands.spawn((
        Mesh2d(meshes.add(Rectangle::new(wall.width, wall.height))),
        MeshMaterial2d(materials.add(wall.bevy.color)),
        Transform::from_xyz(wall.physics.position.x,
                            wall.physics.position.y,
                            0.0)
        ),
    ).id();
    scene.entities.push(PhysicsEntity::Rectangle(wall));

    let mut wall =  RectangleEntity::new(Vec2::new(0.0, -SIDE_LENGTH/2.0 + SIDE_LENGTH/20.0),
                        Vec2::ZERO,
                        SIDE_LENGTH,
                        SIDE_LENGTH/10.0,
                        );
    wall.bevy.entity = commands.spawn((
        Mesh2d(meshes.add(Rectangle::new(wall.width, wall.height))),
        MeshMaterial2d(materials.add(wall.bevy.color)),
        Transform::from_xyz(wall.physics.position.x,
                            wall.physics.position.y,
                            0.0)
        ),
    ).id();
    scene.entities.push(PhysicsEntity::Rectangle(wall));
}

#[derive(Resource, Default)]
struct Scene {
    entities: Vec<PhysicsEntity>,
}

fn update_scene(
    game: Res<Scene>,
    time: Res<Time>,
) {
    // Do nothing for now
}

fn kinetic_physics(
    mut game: ResMut<Scene>,
    time: Res<Time>,
    mut transforms: Query<&mut Transform>,
) {
    // Check for collisions
    for i in 0..game.entities.len() {
        let (left, right) = game.entities.split_at_mut(i + 1);
        let obj_1 = &mut left[i];

        for obj_2 in right.iter() {
            if let Some(min_transl_vec) = obj_1.test_collision(obj_2) {
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

struct PhysicsData {
    position: Vec2,
    velocity: Vec2,
    vertices: Vec<Vec2>,
    axes: Vec<Vec2>,
    stationary: bool,
    mass: f32,
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

struct BevyData {
    color: Color,
    entity: Entity,
}

impl BevyData {
    fn new() -> Self {
        BevyData {
            color: Color::srgba(30./255.0, 61./255.0, 111./255.0, 1.),
            entity: Entity::PLACEHOLDER,
        }
    }
}

struct BallEntity {
    physics: PhysicsData,
    bevy: BevyData,
    radius: f32,
}

struct RectangleEntity {
    physics: PhysicsData,
    bevy: BevyData,
    width: f32,
    height: f32,
}


enum PhysicsEntity {
    Ball(BallEntity),
    Rectangle(RectangleEntity),
}

impl PhysicsEntity {
    fn physics(&self) -> &PhysicsData {
        match self {
            Self::Ball(ball) => &ball.physics,
            Self::Rectangle(rect) => &rect.physics,
        }
    }

    fn physics_mut(&mut self) -> &mut PhysicsData {
        match self {
            Self::Ball(ball) => &mut ball.physics,
            Self::Rectangle(rect) => &mut rect.physics,
        }
    }

    fn position(&self) -> Vec2 { self.physics().position }
    fn velocity(&self) -> Vec2 { self.physics().velocity }
    fn set_position(&mut self, pos: Vec2) { self.physics_mut().position = pos; }
    fn set_velocity(&mut self, vel: Vec2) { self.physics_mut().velocity = vel; }
    fn mass(&self) -> f32 { self.physics().mass }
    fn is_stationary(&self) -> bool { self.physics().stationary }
    fn vertices(&self) -> &Vec<Vec2> { &self.physics().vertices }
    fn get_axes(&self) -> &Vec<Vec2> { &self.physics().axes }
    fn entity(&self) -> Entity {
        match self {
            Self::Ball(ball) => ball.bevy.entity,
            Self::Rectangle(rect) => rect.bevy.entity,
        }
    }

    fn translated_verts(&self) -> Vec<Vec2> {
        self.vertices().iter().map(|v| {
            Vec2 {
                x: v.x + self.position().x,
                y: v.y + self.position().y,
            }
        }).collect()
    }

    fn test_collision(&self, other: &PhysicsEntity) -> Option<Vec2> {
        let axes1 = self.get_axes();
        let axes2 = other.get_axes();
        let verts1 = self.translated_verts();
        let verts2 = other.translated_verts();

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
}

fn project(verts: &Vec<Vec2>, axis: &Vec2) -> (f32, f32) {
    let mut min = f32::INFINITY;
    let mut max = -f32::INFINITY;
    for i in 0..verts.len() {
        let proj = axis.dot(verts[i]);
        if proj < min { min = proj; }
        if proj > max { max = proj; }
    }
    (min, max)
}




impl BallEntity {
    fn new(position: Vec2, velocity: Vec2, radius: f32) -> Self {
        let mut ball = BallEntity {
            physics: PhysicsData::new(position, velocity),
            bevy: BevyData::new(),
            radius,
        };

        ball.set_vertices(16);
        ball.physics.set_axes();
        ball
    }

    fn set_vertices(&mut self, n: i64) { 
        self.physics.vertices = (0..n).map(|i| {
                let frac = (i as f32) / (n as f32) * TWOPI;
                Vec2::new(self.radius * frac.cos(), self.radius * frac.sin())
        }).collect();
    }
}

impl RectangleEntity {
    fn new(position: Vec2, velocity: Vec2, width: f32, height: f32,) -> Self {
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
        let pos = &self.physics.position;
        verts.push(Vec2::new(self.width/2.0, self.height/2.0));
        verts.push(Vec2::new(-self.width/2.0, self.height/2.0));
        verts.push(Vec2::new(-self.width/2.0, -self.height/2.0));
        verts.push(Vec2::new(self.width/2.0, -self.height/2.0));
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

    // #[test]
    // fn ball() {
    //     let b = BallEntity {
    //         position: Vec2 {
    //             x: 1.0,
    //             y: 2.0,
    //         },
    //         ..Default::default()
    //     };
    //     assert_eq!(b.position.x, 1.0);
    //     assert_eq!(b.position.y, 2.0);
    // }

    // #[test]
    // fn distance_test() {
    //     let a = Vec2 {
    //         x: 0.0,
    //         y: 0.0,
    //     };
    //     let b = Vec2 {
    //         x: 1.0,
    //         y: 1.0,
    //     };
    //     assert_eq!(distance(&a, &b), (2.0_f32).powf(0.5))
    // }

    #[test]
    fn circle_vertices_test() {
        let mut ball = BallEntity {
            physics: PhysicsData::new(Vec2::ZERO, Vec2::ZERO),
            bevy: BevyData::new(),
            radius: 1.0,
        };
        let n: usize = 8;
        ball.set_vertices(n as i64);
        for vert in ball.physics.vertices.iter() {
            println!{"Circle vert: {}", vert};
        }
        assert_eq!(ball.physics.vertices.len(), n);
    }

    // #[test]
    // fn test_rectangle_collision() {
    //     let rect1 = PhysicsEntity::Rectangle(RectangleEntity::new(Vec2::new(0.0, 0.0), Vec2::new(0.0, 0.0), 2.0, 2.0));
    //     let rect2 = PhysicsEntity::Rectangle(RectangleEntity::new(Vec2::new(1.0, 0.0), Vec2::new(0.0, 0.0), 2.0, 2.0));
    //     let rect3 = PhysicsEntity::Rectangle(RectangleEntity::new(Vec2::new(3.0, 0.0), Vec2::new(0.0, 0.0), 0.5, 0.5));

    //     // Print the axes of each rectangle
    //     for rect in vec![&rect1, &rect2, &rect3] {
    //         println!("Rectangle axes: {:?}", rect.physics().axes);
    //     }
    //     
    //     // rect1 and rect2 should collide, while rect1 and rect3 should not
    //     println!("Testing 2x2 rectangles at position (0,0) and (1,0)\n");
    //     assert!(rect1.test_collision(&rect2).is_some());
    //     println!("Testing 2x2 at position (0,0) and 0.5x0.5 at position (2,0)\n");
    //     assert!(rect1.test_collision(&rect3).is_none());
    // }

    #[test]
    fn test_rectangle_circle_collision() {
        let circ = PhysicsEntity::Ball(BallEntity::new(Vec2::new(1.0, 0.0), Vec2::new(0.0, 0.0), 0.5));
        let rect1 = PhysicsEntity::Rectangle(RectangleEntity::new(Vec2::new(0.0, 0.0), Vec2::new(0.0, 0.0), 0.25, 0.25));
        let rect2 = PhysicsEntity::Rectangle(RectangleEntity::new(Vec2::new(1.5, 0.0), Vec2::new(0.0, 0.0), 0.5, 0.5));

        // rect1 and circ should not collide, while rect2 and circ should
        println!("Testing 0.5 radius circle at position (1,1) and 0.25x0.25 rectangle at position (0,0)\n");
        assert!(circ.test_collision(&rect1).is_none());
        println!("Testing 0.5 radius circle at position (1,1) and 0.5x0.5 rectangle at position (1.5,0)\n");
        assert!(circ.test_collision(&rect2).is_some());
    }
}

