mod physics;

use physics::entities::{BallEntity, RectangleEntity, PhysicsEntity};
use physics::constants::{G, SIDE_LENGTH};

use bevy::prelude::*;
use bevy::window::{WindowResolution, WindowPlugin};
use bevy::math::Vec2;

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
    _game: Res<Scene>,
    _time: Res<Time>,
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
