fn main() {

    let mut p1 = Point {x: 11.34, ..Default::default()};

    println!("{:?}", p1);

    p1.y = 3.12;

    println!("{:?}", p1);
}


#[derive(Debug, Default)]
struct Point { x: f32, y: f32, z: f32 }

// Point { x: 11.34, y: 0.0, z: 0.0 }
// Point { x: 11.34, y: 3.12, z: 0.0 }
