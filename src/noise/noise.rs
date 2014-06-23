pub trait Noise {
    fn get_value(&self, x: f32, y:f32, z:f32) -> f32;
}