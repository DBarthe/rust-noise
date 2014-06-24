
extern crate noise;

use noise::perlin::Perlin;
use noise::noise::Noise;

fn main() {
    let perlin = Perlin::new();
    println!("{}", perlin.get_value(0.1,0.1,0.0));
}