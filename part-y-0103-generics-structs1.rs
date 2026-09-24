fn main() {

    let _player1_c = Coordinates { x: 4u8, y: 11, z: 9 };

    let _player2_c: Coordinates<f32> = Coordinates { x: -3.21, y: 11.11, z: -41.21 };
}

struct Coordinates<T> {

    x: T,
    y: T,
    z: T
}
