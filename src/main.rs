mod basis;
mod mesh;
use nalgebra::Vector3;

pub struct Particle {
    position: Vector3<f64>,
    velocity: Vector3<f64>,
    mass: f64,
    volume: f64,
    stress: Vector3<f64>,
}

pub struct Grid {
    dimension: isize, // Dimension of the grid
    n: isize,         // Number of cells
    h: f64,           // Grid spacing
}

impl Grid {
    fn new(dimension: isize, n: isize, h: f64) -> Self {
        Self { dimension, n, h }
    }
}

pub fn main() {
    let n = 3;
    let length = 1.0;
    let h = length / n as f64;
    let mesh = mesh::create_2d_square_mesh(2, n, h);
    println!("mesh: {:#?}", mesh);
    // let ns: Vec<f64> = nodes.iter().map(|x| N(x)).collect();
    // println!("ns: {:#?}", ns);
}
