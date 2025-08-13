use cgmath::{Matrix4, Vector3};

use crate::scene::polygon::Polygon;
use crate::scene::vertex::Vertex;
use std::vec;

pub fn generate_cubes() -> Vec<Polygon> {
    let cube_positions = [
        Vector3::new(0., 0., 0.),
        Vector3::new(60., 0., 0.),
        Vector3::new(0., 60., 0.),
        Vector3::new(60., 60., 0.),
        Vector3::new(0., 0., 60.),
        Vector3::new(60., 0., 60.),
        Vector3::new(0., 60., 60.),
        Vector3::new(60., 60., 60.),
    ];
    let mut cubes = Vec::new();
    for position in cube_positions {
        cube_template()
            .iter()
            .map(|p| p.transform(Matrix4::from_translation(position)))
            .for_each(|p| cubes.push(p));
    }

    cubes
}

fn cube_template() -> Vec<Polygon> {
    let polygons = vec![
        // Front face
        Polygon::from_vertices(vec![
            Vertex::new(-1.0, 1.0, 1.0),
            Vertex::new(-1.0, -1.0, 1.0),
            Vertex::new(1.0, -1.0, 1.0),
        ]),
        // Back face
        Polygon::from_vertices(vec![
            Vertex::new(1.0, -1.0, -1.0),
            Vertex::new(-1.0, -1.0, -1.0),
            Vertex::new(-1.0, 1.0, -1.0),
        ]),
        // Top face
        Polygon::from_vertices(vec![
            Vertex::new(-1.0, 1.0, -1.0),
            Vertex::new(-1.0, 1.0, 1.0),
            Vertex::new(1.0, 1.0, 1.0),
        ]),
        // Bottom face
        Polygon::from_vertices(vec![
            Vertex::new(-1.0, -1.0, 1.0),
            Vertex::new(-1.0, -1.0, -1.0),
            Vertex::new(1.0, -1.0, -1.0),
        ]),
        // Left face
        Polygon::from_vertices(vec![
            Vertex::new(-1.0, 1.0, -1.0),
            Vertex::new(-1.0, -1.0, -1.0),
            Vertex::new(-1.0, -1.0, 1.0),
        ]),
        // Right face
        Polygon::from_vertices(vec![
            Vertex::new(1.0, -1.0, 1.0),
            Vertex::new(1.0, -1.0, -1.0),
            Vertex::new(1.0, 1.0, -1.0),
        ]),
        // Remaining polygons for other faces
        Polygon::from_vertices(vec![
            Vertex::new(-1.0, 1.0, 1.0),
            Vertex::new(1.0, -1.0, 1.0),
            Vertex::new(1.0, 1.0, -1.0),
        ]),
        Polygon::from_vertices(vec![
            Vertex::new(-1.0, 1.0, 1.0),
            Vertex::new(-1.0, 1.0, -1.0),
            Vertex::new(1.0, -1.0, -1.0),
        ]),
        Polygon::from_vertices(vec![
            Vertex::new(-1.0, -1.0, 1.0),
            Vertex::new(-1.0, -1.0, -1.0),
            Vertex::new(1.0, 1.0, -1.0),
        ]),
        Polygon::from_vertices(vec![
            Vertex::new(-1.0, -1.0, 1.0),
            Vertex::new(1.0, -1.0, -1.0),
            Vertex::new(1.0, -1.0, 1.0),
        ]),
        Polygon::from_vertices(vec![
            Vertex::new(-1.0, 1.0, -1.0),
            Vertex::new(1.0, -1.0, -1.0),
            Vertex::new(1.0, 1.0, 1.0),
        ]),
        Polygon::from_vertices(vec![
            Vertex::new(-1.0, -1.0, -1.0),
            Vertex::new(-1.0, 1.0, 1.0),
            Vertex::new(1.0, 1.0, 1.0),
        ]),
    ];

    polygons
}
