use std::{fs::File, io::BufRead, io::BufReader};

use crate::scene::{polygon::Polygon, vertex::Vertex};

pub fn read_polygons_from_obj(path: &str) -> Result<Vec<Polygon>, String> {
    let mut polygons = vec![];
    let mut vertices = vec![];

    let file = File::open(path).map_err(|e| e.to_string() + " file: " + path)?;
    let reader = BufReader::new(file);

    for line in reader.lines() {
        let line = line.map_err(|e| e.to_string())?;
        let mut words = line.split_whitespace();

        match words.next() {
            Some("v") => {
                let x = words
                    .next()
                    .ok_or("Missing x coordinate")?
                    .parse::<f64>()
                    .map_err(|e| e.to_string())?;
                let y = words
                    .next()
                    .ok_or("Missing y coordinate")?
                    .parse::<f64>()
                    .map_err(|e| e.to_string())?;
                let z = words
                    .next()
                    .ok_or("Missing z coordinate")?
                    .parse::<f64>()
                    .map_err(|e| e.to_string())?;
                vertices.push(Vertex::new(x, y, z));
            }
            Some("f") => {
                let v1 = words
                    .next()
                    .ok_or("Missing first vertex")?
                    .parse::<usize>()
                    .map_err(|e| e.to_string())?;
                let v2 = words
                    .next()
                    .ok_or("Missing second vertex")?
                    .parse::<usize>()
                    .map_err(|e| e.to_string())?;
                let v3 = words
                    .next()
                    .ok_or("Missing third vertex")?
                    .parse::<usize>()
                    .map_err(|e| e.to_string())?;
                polygons.push(Polygon::from_vertices(vec![
                    vertices[v1 - 1],
                    vertices[v2 - 1],
                    vertices[v3 - 1],
                ]));
            }
            Some(_) => {}
            None => {}
        }
    }

    Ok(polygons)
}
