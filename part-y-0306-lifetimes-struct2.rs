fn main() {

    let c1 = Coordinates(-2.21, 3.12);

    let circle1 = Circle {radius: 4.0, position: &c1};

    println!("{:?}", c1.0);
    println!("{:?}", c1.1);

    println!("{:?}", circle1);
    println!("{:?}", circle1.radius);
    println!("{:?}", circle1.position);
    println!("{:?}", circle1.position.0);
    println!("{:?}", circle1.position.1);
}

#[derive(Debug)]
struct Coordinates(f32, f32);

#[derive(Debug)]
struct Circle<'a> {

    radius: f32,
    position: &'a Coordinates

    // position_x: f32,
    // position_y: f32
}

// -2.21
// 3.12
// Circle { radius: 4.0, position: Coordinates(-2.21, 3.12) }
// 4.0
// Coordinates(-2.21, 3.12)
// -2.21
// 3.12
