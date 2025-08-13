use std::time::Duration;

use cgmath::{
    perspective, Deg, InnerSpace, Matrix4, Point3, Quaternion, Rotation, Rotation3, Vector3,
};
use log::Level;
use sdl2::event::Event;
use sdl2::gfx::primitives::DrawRenderer;
use sdl2::keyboard::Keycode;
use sdl2::pixels::Color;

use obj::read_polygons_from_obj;
use sdl2::rect::Point;

use crate::scene::Scene;

mod cube_generator;
mod obj;
mod scene;

const VW: u32 = 800;
const VH: u32 = 600;
const MOVE_STEP: f64 = 10.;
const LOOK_STEP: f64 = 2.5;
const ZOOM_STEP: f64 = 5.;
const TILT_STEP: f64 = 5.;
const AR: f64 = (VW / VH) as f64;
const NEAR: f64 = 5.;
const FAR: f64 = 300.;
const FOV_MIN: f64 = 30.;
const FOV_MAX: f64 = 90.;
const FOV_DEFAULT: f64 = (FOV_MIN + FOV_MAX) / 2.;
const SCALE: f64 = 10.;

fn main() -> Result<(), String> {
    simple_logger::init_with_level(Level::Debug).map_err(|e| e.to_string())?;

    let file = std::env::args().nth(1).ok_or("Missing obj file");

    let objects = match file {
        Ok(file) => read_polygons_from_obj(&file)?,
        Err(_) => {
            log::info!("No obj file provided, generating cubes...");
            cube_generator::generate_cubes()
        }
    };

    let scene = Scene::new(objects).transform(Matrix4::from_scale(SCALE));

    let sdl_context = sdl2::init()?;
    let video_subsystem = sdl_context.video()?;
    let window = video_subsystem
        .window("RenderWalk", VW, VH)
        .position_centered()
        .build()
        .map_err(|e| e.to_string())?;

    let mut canvas = window
        .into_canvas()
        .accelerated()
        .present_vsync()
        .build()
        .map_err(|e| e.to_string())?;
    canvas.set_draw_color(Color::WHITE);
    canvas.clear();
    canvas.present();

    let mut position = Point3::new(0., 0., 0.);
    let mut forward = Vector3::new(0., 0., 1.);
    let mut fov = FOV_DEFAULT;
    let mut up = Vector3::new(0., 1., 0.);

    let mut event_pump = sdl_context.event_pump()?;
    'running: loop {
        canvas.set_draw_color(Color::WHITE);
        canvas.clear();

        for event in event_pump.poll_iter() {
            match event {
                Event::Quit { .. }
                | Event::KeyDown {
                    keycode: Some(Keycode::Escape),
                    ..
                } => break 'running,
                Event::KeyDown {
                    keycode: Some(Keycode::W),
                    ..
                } => {
                    // forward
                    position += forward.normalize() * MOVE_STEP;
                }
                Event::KeyDown {
                    keycode: Some(Keycode::S),
                    ..
                } => {
                    // backward
                    position -= forward.normalize() * MOVE_STEP;
                }
                Event::KeyDown {
                    keycode: Some(Keycode::A),
                    ..
                } => {
                    // left
                    position -= forward.cross(up).normalize() * MOVE_STEP;
                }
                Event::KeyDown {
                    keycode: Some(Keycode::D),
                    ..
                } => {
                    // right
                    position += forward.cross(up).normalize() * MOVE_STEP;
                }
                Event::KeyDown {
                    keycode: Some(Keycode::Up),
                    ..
                } => {
                    // up
                    position += up.normalize() * MOVE_STEP;
                }
                Event::KeyDown {
                    keycode: Some(Keycode::Down),
                    ..
                } => {
                    // down
                    position -= up.normalize() * MOVE_STEP;
                }
                Event::KeyDown {
                    keycode: Some(Keycode::Q),
                    ..
                } => {
                    // zoom in
                    if fov > FOV_MIN {
                        fov -= ZOOM_STEP;
                    }
                }
                Event::KeyDown {
                    keycode: Some(Keycode::E),
                    ..
                } => {
                    // zoom out
                    if fov < FOV_MAX {
                        fov += ZOOM_STEP;
                    }
                }
                Event::KeyDown {
                    keycode: Some(Keycode::Space),
                    ..
                } => {
                    // reset zoom
                    fov = FOV_DEFAULT
                }
                Event::KeyDown {
                    keycode: Some(Keycode::I),
                    ..
                } => {
                    // look up
                    forward = Quaternion::from_axis_angle(forward.cross(up), Deg(LOOK_STEP))
                        .rotate_vector(forward);
                }
                Event::KeyDown {
                    keycode: Some(Keycode::K),
                    ..
                } => {
                    // look down
                    forward = Quaternion::from_axis_angle(forward.cross(up), Deg(-LOOK_STEP))
                        .rotate_vector(forward);
                }
                Event::KeyDown {
                    keycode: Some(Keycode::J),
                    ..
                } => {
                    // look left
                    forward =
                        Quaternion::from_axis_angle(up, Deg(LOOK_STEP)).rotate_vector(forward);
                }
                Event::KeyDown {
                    keycode: Some(Keycode::L),
                    ..
                } => {
                    // look right
                    forward =
                        Quaternion::from_axis_angle(up, Deg(-LOOK_STEP)).rotate_vector(forward);
                }
                Event::KeyDown {
                    keycode: Some(Keycode::O),
                    ..
                } => {
                    // tilt left
                    up = Quaternion::from_axis_angle(forward, Deg(TILT_STEP)).rotate_vector(up);
                }
                Event::KeyDown {
                    keycode: Some(Keycode::U),
                    ..
                } => {
                    // tilt right
                    up = Quaternion::from_axis_angle(forward, Deg(-TILT_STEP)).rotate_vector(up);
                }
                _ => {}
            }
        }

        let view = Matrix4::look_to_lh(position, forward, up);
        let projection = perspective(Deg(fov), AR, NEAR, FAR);

        let projected_scene = scene
            .transform(view)
            .transform(projection)
            .screen_coords(VW, VH);

        canvas.set_draw_color(Color::BLACK);
        for poly in &projected_scene.polygons() {
            // TODO: Improve this flow
            let vertices = poly.vertices();
            let vx_vec: Vec<i16> = vertices.iter().map(|v| v.position().x as i16).collect();
            let vy_vec: Vec<i16> = vertices.iter().map(|v| v.position().y as i16).collect();
            let vx: &[i16] = &vx_vec;
            let vy: &[i16] = &vy_vec;
            canvas.filled_polygon(vx, vy, Color::BLUE).unwrap();

            canvas.set_draw_color(Color::BLACK);
            poly.edges().iter().for_each(|e| {
                let start: Point = Point::new(e.v1().x() as i32, e.v1().y() as i32);
                let end: Point = Point::new(e.v2().x() as i32, e.v2().y() as i32);

                canvas.draw_line(start, end).unwrap()
            });
        }

        canvas.present();
        std::thread::sleep(Duration::new(0, 1_000_000_000u32 / 60));
    }

    Ok(())
}
