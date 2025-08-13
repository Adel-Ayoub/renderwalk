use cgmath::{Matrix4, Point3, Transform};

#[derive(Debug, Clone, Copy)]
pub struct Vertex {
    position: Point3<f64>,
}

impl Vertex {
    pub fn from_point3(position: Point3<f64>) -> Self {
        Self { position }
    }

    pub fn new(x: f64, y: f64, z: f64) -> Self {
        Self {
            position: Point3::new(x, y, z),
        }
    }

    pub fn x(&self) -> f64 {
        self.position.x.clone()
    }

    pub fn y(&self) -> f64 {
        self.position.y.clone()
    }

    pub fn z(&self) -> f64 {
        self.position.z.clone()
    }

    pub fn position(&self) -> Point3<f64> {
        self.position.clone()
    }

    pub fn transform(&self, transform_matrix: Matrix4<f64>) -> Vertex {
        Vertex::from_point3(transform_matrix.transform_point(self.position))
    }

    // TODO refactor this method to move responsibility of culling to another module
    pub fn screen_coords(&self, vw: u32, vh: u32) -> Option<Vertex> {
        if self.check_if_point_infinity() {
            log::debug!("Vertex::screen_coords: point is infinity");
            return None;
        }

        if self.z() < 1. {
            log::debug!("Vertex::screen_coords: z is out of range");
            return None;
        }

        let x = (self.x() + 1.) * 0.5 * vw as f64;
        let y = (self.y() + 1.) * 0.5 * vh as f64;
        let z = 0.;

        if x < -200. || x > (vw as f64 + 200.) || y < -200. || y > (vh as f64 + 200.) {
            log::debug!("Vertex::screen_coords: outside of screen");
            return None;
        }

        Some(Vertex::new(x, y, z))
    }

    pub fn check_if_point_infinity(&self) -> bool {
        !self.x().is_finite() || !self.y().is_finite() || !self.z().is_finite()
    }
}
