use std::rc::Rc;

use common::model::figure::*;
use common::model::mat::*;
use nannou::color::*;
use nannou::event::WindowEvent::*;
use nannou::event::*;
use nannou::*;
use std::cell::RefCell;
use winit::event::VirtualKeyCode::*;

struct Model {
    eye: [f32; 3],
    at: [f32; 3],
    up: [f32; 3],
    camera: Mat4x4,
    perspective_proj: Mat4x4,
    mouse_x_pressed: f32,
    mouse_y_pressed: f32,
    mouse_x: f32,
    mouse_y: f32,
    mouse_pressed: bool,
    alt: bool,
    zoom: f32,
    mesh: Mesh,
}

fn main() {
    nannou::app(model).update(update).run();
}

fn model(app: &App) -> Model {
    app.new_window().event(event).view(view).build().unwrap();
    let viewport = app.window_rect();

    let eye = [0., 0., 0.0];
    let at = [0.45, 0.15, -1.15];
    let up = [0., 1.0, 0.];

    let mut mesh = Mesh::new();

    let axis = mesh.push_object("axis");
    axis.push_vertex(Vertex::from([0., 0., -1., 1.]));
    axis.push_vertex(Vertex::from([0.1, 0., -1., 1.]));
    axis.push_vertex(Vertex::from([0., 0.1, -1., 1.]));
    axis.push_vertex(Vertex::from([0., 0., -1.1, 1.]));

    axis.push_edge(Edge::new_color(0, 1, RED).text("X"));
    axis.push_edge(Edge::new_color(0, 2, RED).text("Y"));
    axis.push_edge(Edge::new_color(0, 3, RED).text("Z"));

    let cube = mesh.push_object("cube");
    cube.push_vertex(Vertex::from([0.3, 0.3, -1., 1.]));
    cube.push_vertex(Vertex::from([0.6, 0.3, -1., 1.]));
    cube.push_vertex(Vertex::from([0.6, 0., -1., 1.]));
    cube.push_vertex(Vertex::from([0.3, 0., -1., 1.]));

    cube.push_vertex(Vertex::from([0.3, 0.3, -1.3, 1.]));
    cube.push_vertex(Vertex::from([0.6, 0.3, -1.3, 1.]));
    cube.push_vertex(Vertex::from([0.6, 0., -1.3, 1.]));
    cube.push_vertex(Vertex::from([0.3, 0., -1.3, 1.]));

    cube.push_face(Face::new(vec![0, 3, 2, 1]));
    cube.push_face(Face::new(vec![0, 1, 5, 4]));
    cube.push_face(Face::new(vec![1, 2, 6, 5]));
    cube.push_face(Face::new(vec![3, 7, 6, 2]));
    cube.push_face(Face::new(vec![0, 4, 7, 3]));
    cube.push_face(Face::new(vec![4, 5, 6, 7]));

    Model {
        eye,
        at,
        up,
        camera: viewer(eye, at, up),
        perspective_proj: perspective_projection(60., viewport.w() / viewport.h(), -10., -1.),
        mouse_x_pressed: 0.0,
        mouse_y_pressed: 0.0,
        mouse_x: 0.0,
        mouse_y: 0.0,
        mouse_pressed: false,
        alt: false,
        zoom: 1.0,
        mesh,
    }
}

fn event(_app: &App, model: &mut Model, event: WindowEvent) {
    match event {
        KeyPressed(key) => match key {
            Left => {
                model.eye = rotate_y_around_p_vec3(0.001 * 180.0 / 3.1425, model.eye, model.at);

                model.camera = viewer(model.eye, model.at, model.up);
            }
            Right => {
                model.eye = rotate_y_around_p_vec3(-0.001 * 180.0 / 3.1425, model.eye, model.at);
                model.camera = viewer(model.eye, model.at, model.up);
            }
            Up => {
                if !model.alt {
                    model.eye = rotate_x_around_p_vec3(0.001 * 180.0 / 3.1425, model.eye, model.at);
                    model.camera = viewer(model.eye, model.at, model.up);
                } else {
                    model.eye = [model.eye[0], model.eye[1], model.eye[2] - 0.1];
                    model.camera = viewer(model.eye, model.at, model.up);
                }
            }
            Down => {
                if !model.alt {
                    model.eye = rotate_x_around_p_vec3(-0.001 * 180.0 / 3.1425, model.eye, model.at);
                    model.camera = viewer(model.eye, model.at, model.up);
                } else {
                    model.eye = [model.eye[0], model.eye[1], model.eye[2] + 0.1];
                    model.camera = viewer(model.eye, model.at, model.up);
                }
            }
            LAlt => model.alt = true,
            _ => {}
        },

        KeyReleased(key) => match key {
            LAlt => model.alt = false,
            _ => {}
        },

        MousePressed(_) => {
            model.mouse_pressed = true;
            model.mouse_x_pressed = model.mouse_x;
            model.mouse_y_pressed = model.mouse_y;
        }

        MouseReleased(_) => {
            model.mouse_pressed = false;
        }

        MouseMoved(point) => {
            model.mouse_x = point.x;
            model.mouse_y = point.y;

            if model.mouse_pressed {
                let x_diff = model.mouse_x - model.mouse_x_pressed;
                let y_diff = model.mouse_y - model.mouse_y_pressed;

                model.eye = rotate_x_around_p_vec3(y_diff / 1000000. * 180.0 / 3.1425, model.eye, model.at);
                model.eye = rotate_y_around_p_vec3(x_diff / 1000000. * 180.0 / 3.1425, model.eye, model.at);

                model.camera = viewer(model.eye, model.at, model.up);
            }
        }

        Resized(dim) => {
            model.perspective_proj = perspective_projection(60., dim[0] / dim[1], -100., -1.)
        }

        _ => {}
    }
}

fn update(_app: &App, _model: &mut Model, _update: Update) {}

fn view(app: &App, model: &Model, frame: Frame) {
    // Begin drawing
    let draw = app.draw();

    draw.background().color(WHITE);

    let viewport = app.window_rect();

    let transform = Mat4x4::unit();

    let mut new_mesh = model.mesh.clone();

    println!("Eye {:?} At {:?}", model.eye, model.at);
    let t_eye = model.perspective_proj
        * model.camera
        * transform
        * [model.eye[0], model.eye[1], model.eye[2], 1.];

    println!("After: Eye {:?} At {:?}",t_eye, model.at);
    let mat = (model.perspective_proj * model.camera * transform * &new_mesh).to_screen(
        t_eye,
        viewport.w() as f32,
        viewport.h() as f32,
    );

    mat.draw_faces(&draw);
    mat.draw_lines(&draw);

    draw.to_frame(app, &frame).unwrap();
}
