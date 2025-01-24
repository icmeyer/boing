fn main() {
    println!("Hello, world!");
}

struct Ball {
    position: (f64, f64),
    velocity: (f64, f64),
    radius: f64,
    mass: f64,
    stationary: bool,
}
