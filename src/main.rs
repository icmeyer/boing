mod physics;
mod rendering;

use bevy::prelude::*;
use bevy::window::{WindowResolution, WindowPlugin};
use bevy::math::Vec2;

use physics::entities::{BallEntity, RectangleEntity, PhysicsEntity,
                        KinematicData,};
use physics::constants::{SIDE_LENGTH,};
use physics::interactions::kinetic_physics;
use rendering::scene::{Scene, update_scene};


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

    let kinematic_data = KinematicData::new(
        Vec2::new(0.0, 0.0),
        Vec2::new(330.0, 200.0),
        SIDE_LENGTH/10.0,
        10.0,
    );
    BallEntity::spawn(
        &mut commands,
        &mut scene,
        &mut meshes,
        &mut materials,
        kinematic_data,
    );

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

    // #[test]
    // fn test_rectangle_circle_collision() {
    //     let circ = PhysicsEntity::Ball(BallEntity::new(Vec2::new(1.0, 0.0), Vec2::new(0.0, 0.0), 0.5));
    //     let rect1 = PhysicsEntity::Rectangle(RectangleEntity::new(Vec2::new(0.0, 0.0), Vec2::new(0.0, 0.0), 0.25, 0.25));
    //     let rect2 = PhysicsEntity::Rectangle(RectangleEntity::new(Vec2::new(1.5, 0.0), Vec2::new(0.0, 0.0), 0.5, 0.5));

    //     // rect1 and circ should not collide, while rect2 and circ should
    //     println!("Testing 0.5 radius circle at position (1,1) and 0.25x0.25 rectangle at position (0,0)\n");
    //     assert!(circ.test_collision(&rect1).is_none());
    //     println!("Testing 0.5 radius circle at position (1,1) and 0.5x0.5 rectangle at position (1.5,0)\n");
    //     assert!(circ.test_collision(&rect2).is_some());
    // }
}
