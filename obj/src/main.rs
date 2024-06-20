use crate::noise::{NoiseFn, Perlin};
use common::model::figure::*;
use common::model::mat::*;
use nannou::color::BLACK;
use nannou::color::*;
use nannou::event::WindowEvent::*;
use nannou::event::*;
use nannou::geom::pt2;
use nannou::*;

use nannou::prelude::WindowId;

struct Model {
    window: WindowId,
    eye: [f32; 3],
    at: [f32; 3],
    up: [f32; 3],
    camera: Mat4x4,
    perspective_proj: Mat4x4,
    zoff: f32,
    xoff: f32,
    mouse_x_pressed: f32,
    mouse_y_pressed: f32,
    mouse_x: f32,
    mouse_y: f32,
    mouse_pressed: bool,
}



fn main() {
    nannou::app(model).update(update).run();
}

fn model(app: &App) -> Model {
    let window = app.new_window().event(event).view(view).build().unwrap();
    let viewport = app.window_rect();

    let eye = [0.5, 1.0, -5.];
    let at = [0., 0., 0.];
    let up = [0., 0.1, 0.];

    Model {
        window,
        eye,
        at,
        up,
        camera: viewer(eye, at, up),
        perspective_proj: perspective_projection(
            std::f32::consts::PI / 4.,
            viewport.w() / viewport.h(),
            100.,
            -3.2,
        ),
        zoff: 0.0,
        xoff: 0.0,
        mouse_x_pressed: 0.0,
        mouse_y_pressed: 0.0,
        mouse_x: 0.0,
        mouse_y: 0.0,
        mouse_pressed: false,
    }
}

fn event(_app: &App, model: &mut Model, event: WindowEvent) {
    match event {
        MousePressed(key) => {
            model.mouse_pressed = true;
            model.mouse_x_pressed = model.mouse_x;
            model.mouse_y_pressed = model.mouse_y;
        }

        MouseReleased(key) => {
            model.mouse_pressed = false;
        }

        MouseMoved(point) => {
            model.mouse_x = point.x;
            model.mouse_y = point.y;

            if model.mouse_pressed {
                let x_diff = model.mouse_x - model.mouse_x_pressed;
                let y_diff = model.mouse_y - model.mouse_y_pressed;

                model.camera = model.camera
                    * rotate_y_mat((x_diff / 100000.) * 180.0 / 3.1425)
                    * rotate_x_mat((y_diff / 100000.) * 180.0 / 3.1425)
            }
        }

        Resized(dim) => {
            model.perspective_proj =
                perspective_projection(std::f32::consts::PI / 4., dim[0] / dim[1], 100., -3.2)
        }

        _ => {}
    }
}

fn update(_app: &App, model: &mut Model, update: Update) {}

fn view(app: &App, model: &Model, frame: Frame) {
    // Begin drawing
    let draw = app.draw();

    draw.background().color(WHITE);

    let viewport = app.window_rect();

    let mut surface: Vec<Vertex> = vec![];
    let mut edges: Vec<Edge> = vec![];

    surface.push(Vertex::from([0.3, 0.3, 0., 1.]));
    surface.push(Vertex::from([0.6, 0.3, 0., 1.]));
    surface.push(Vertex::from([0.6, 0., 0., 1.]));
    surface.push(Vertex::from([0.3, 0., 0., 1.]));

    surface.push(Vertex::from([0.3, 0.3, 0.3, 1.]));
    surface.push(Vertex::from([0.6, 0.3, 0.3, 1.]));
    surface.push(Vertex::from([0.6, 0., 0.3, 1.]));
    surface.push(Vertex::from([0.3, 0., 0.3, 1.]));

    edges.push(Edge::new(0, 1));
    edges.push(Edge::new(1, 2));
    edges.push(Edge::new(2, 3));
    edges.push(Edge::new(3, 0));

    edges.push(Edge::new(0, 4));
    edges.push(Edge::new(4, 5));
    edges.push(Edge::new(5, 1));

    edges.push(Edge::new(5, 6));
    edges.push(Edge::new(6, 2));

    edges.push(Edge::new(6, 7));
    edges.push(Edge::new(7, 3));
    edges.push(Edge::new(7, 4));

    surface.push(Vertex::from([0., 0., 0., 1.]));
    surface.push(Vertex::from([0.1, 0., 0., 1.]));
    surface.push(Vertex::from([0., 0.1, 0., 1.]));
    surface.push(Vertex::from([0., 0., 0.1, 1.]));

    edges.push(Edge::new_color(8, 9, RED).text("X"));
    edges.push(Edge::new_color(8, 10, RED).text("Y"));
    edges.push(Edge::new_color(8, 11, RED).text("Z"));

    let local = Mesh {
        vertexes: surface,
        edges: edges,
    };

    let transform = Mat4x4::unit();

    let mat = (model.perspective_proj * model.camera * transform * local)
        .norm_z(viewport.w() as f32, viewport.h() as f32);

    mat.draw(&draw);

    draw.to_frame(app, &frame).unwrap();
}


