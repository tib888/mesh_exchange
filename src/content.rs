use nalgebra::geometry::{Point3, Similarity};

pub type Position = Point3<f64>;

#[derive(Clone, Debug)]
pub struct Mesh<N> {
    vertices: Vec<Position>,
    triangles: Vec<[N; 3]>,
    neighbors: Vec<[N; 3]>,
}

impl<N> Mesh<N>
{
    fn transform<R: nalgebra::AbstractRotation<f64, 3_usize>>(mut self, similarity: &Similarity<f64, R, 3>) -> Self
    {       
        //TODO transform the vertices in place 
        for v in self.vertices.iter_mut() {
            *v = similarity.transform_point(v);
        }
        self
    }
}

#[derive(Clone, Copy, Debug)]
pub enum Units {
    Millimeters,
	Centimeters,
	Metres,
    Feet,
    Inches,
    Lightyears,
}

#[derive(Clone, Debug)]
pub struct Content
{
    unit: Option<Units>,
    meshes: Vec<Mesh<u32>>,
    //polylines, points, ...
}
