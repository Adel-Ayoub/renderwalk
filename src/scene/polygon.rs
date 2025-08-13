use cgmath::{MetricSpace, Point3};

use crate::scene::polygon::Coord::{X, Y, Z};
use crate::scene::vertex::Vertex;

#[derive(Debug, Clone)]
pub struct Polygon {
    vertices: Vec<Vertex>,
    edges: Vec<Edge>,
}

#[allow(dead_code)]
enum Coord {
    X,
    Y,
    Z,
}

impl Polygon {
    pub fn from_vertices(vertices: Vec<Vertex>) -> Self {
        let mut edges = Vec::new();
        for i in 0..vertices.len() {
            edges.push(Edge::new(vertices[i], vertices[(i + 1) % vertices.len()]));
        }
        Self { vertices, edges }
    }

    pub fn edges(&self) -> Vec<Edge> {
        self.edges.clone()
    }

    pub fn vertices(&self) -> Vec<Vertex> {
        self.vertices.clone()
    }

    pub fn transform(&self, transform_matrix: cgmath::Matrix4<f64>) -> Polygon {
        let v = self
            .vertices
            .iter()
            .map(|v| v.transform(transform_matrix))
            .collect();
        Polygon::from_vertices(v)
    }

    // todo better option usage
    pub fn screen_coords(&self, vw: u32, vh: u32) -> Option<Polygon> {
        let v: Vec<Option<Vertex>> = self
            .vertices
            .iter()
            .map(|v| v.screen_coords(vw, vh))
            .collect();

        if v.iter().any(|v| v.is_none()) {
            return None;
        }

        let v: Vec<Vertex> = v.into_iter().map(|v| v.unwrap()).collect();

        Some(Polygon::from_vertices(v))
    }

    #[allow(dead_code)]
    pub fn distance_to_camera(&self, camera_pos: Point3<f64>) -> f64 {
        camera_pos.distance(self.centroid())
    }

    #[allow(dead_code)]
    fn min_coord(&self, coord: Coord) -> f64 {
        let mut min = f64::MAX;

        for e in &self.edges {
            min = min.min(match coord {
                X => e.v1.x().min(e.v2.x()),
                Y => e.v1.y().min(e.v2.y()),
                Z => e.v1.z().min(e.v2.z()),
            });
        }

        min
    }

    #[allow(dead_code)]
    fn max_coord(&self, coord: Coord) -> f64 {
        let mut max = f64::MIN;

        for e in &self.edges {
            max = max.max(match coord {
                X => e.v1.x().max(e.v2.x()),
                Y => e.v1.y().max(e.v2.y()),
                Z => e.v1.z().max(e.v2.z()),
            })
        }

        max
    }

    #[allow(dead_code)]
    fn centroid(&self) -> Point3<f64> {
        Point3::new(
            (self.max_coord(X) + self.min_coord(X)) / 2.,
            (self.max_coord(Y) + self.min_coord(Y)) / 2.,
            (self.max_coord(Z) + self.min_coord(Z)) / 2.,
        )
    }
}

#[derive(Debug, Clone, Copy)]
pub struct Edge {
    // TODO - improve implementation to avoid storing duplicate vertices
    v1: Vertex,
    v2: Vertex,
}

impl Edge {
    pub fn new(v1: Vertex, v2: Vertex) -> Edge {
        Edge { v1, v2 }
    }

    pub fn v1(&self) -> Vertex {
        self.v1
    }

    pub fn v2(&self) -> Vertex {
        self.v2
    }
}
