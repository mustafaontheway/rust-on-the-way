fn main() {

}

struct PlayerCoordinates {
    x: f32,
    y: f32,
    z: f32
}

impl PlayerCoordinates {
    
    fn new(x: f32, y: f32, z: f32) -> Self {
        
        Self { x, y, z }
    }

    fn update_coordinates(&mut self, new_x: f32, new_y: f32, new_z: f32) {

        self.x = new_x;
        self.y = new_y;
        self.z = new_z;
    }
}
