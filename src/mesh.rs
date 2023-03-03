use nalgebra::{Point, Point2};

pub trait Connectivity: Clone {
    type FaceConnectivity: Connectivity;

    fn num_faces(&self) -> usize;
    fn vertices(&self) -> &[usize];
    fn get_face_connectivity(&self, index: usize) -> Option<Self::FaceConnectivity>;
}

impl Connectivity for () {
    type FaceConnectivity = ();

    fn num_faces(&self) -> usize {
        0
    }

    fn vertices(&self) -> &[usize] {
        &[]
    }

    fn get_face_connectivity(&self, _index: usize) -> Option<Self::FaceConnectivity> {
        None
    }
}

#[derive(Clone, Debug)]
pub struct LineConnectivity([usize; 2]);

impl Connectivity for LineConnectivity {
    type FaceConnectivity = ();

    fn num_faces(&self) -> usize {
        0
    }

    fn vertices(&self) -> &[usize] {
        &self.0
    }

    fn get_face_connectivity(&self, _index: usize) -> Option<Self::FaceConnectivity> {
        None
    }
}

#[derive(Clone, Debug)]
pub struct Quad2dConnectivity([usize; 4]);

impl Connectivity for Quad2dConnectivity {
    type FaceConnectivity = LineConnectivity;

    fn num_faces(&self) -> usize {
        4
    }

    fn vertices(&self) -> &[usize] {
        &self.0
    }

    fn get_face_connectivity(&self, index: usize) -> Option<Self::FaceConnectivity> {
        match index {
            0 | 1 | 2 | 3 => Some(LineConnectivity([self.0[index], self.0[(index + 1) % 4]])),
            _ => None,
        }
    }
}

#[derive(Debug)]
pub struct Mesh<C: Connectivity, const D: usize> {
    dimension: usize, // Dimension of the grid
    n: usize,         // Number of cells
    h: f64,           // Grid spacing
    vertices: Vec<Point<f64, D>>,
    connectivity: Vec<C>,
}

impl<C: Connectivity, const D: usize> Mesh<C, D> {
    pub fn new(
        dimension: usize,
        n: usize,
        h: f64,
        vertices: Vec<Point<f64, D>>,
        connectivity: Vec<C>,
    ) -> Self {
        Self {
            dimension,
            n,
            h,
            vertices,
            connectivity,
        }
    }

    pub fn vertices(&self) -> &[Point<f64, D>] {
        &self.vertices
    }

    pub fn connectivity(&self) -> &[C] {
        &self.connectivity
    }
}

pub fn create_2d_square_mesh(dimension: usize, n: usize, h: f64) -> Mesh<Quad2dConnectivity, 2> {
    let mut vertices = Vec::new();
    let mut connectivity = Vec::new();

    for i in 0..(n + 1) {
        for j in 0..(n + 1) {
            vertices.push(Point2::new(i as f64 * h, j as f64 * h));

            if j < n && i < n {
                connectivity.push(Quad2dConnectivity([
                    i * (n + 1) + j,
                    i * (n + 1) + j + 1,
                    (i + 1) * (n + 1) + j,
                    (i + 1) * (n + 1) + j + 1,
                ]));
            }
        }
    }

    Mesh::new(dimension, n, h, vertices, connectivity)
}
