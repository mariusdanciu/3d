use std::rc::Rc;

use common::model::figure::*;
use common::model::mat::*;
use nannou::color::*;
use nannou::event::WindowEvent::*;
use nannou::event::*;
use nannou::*;
use winit::event::VirtualKeyCode::*;
use std::cell::RefCell;

struct Model {
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

    let eye = [0.5, 1.0, -3.];
    let at = [0., 0., 0.];
    let up = [0., 1.0, 0.];

    let mut mesh = Mesh::new();

    let axis = mesh.push_object("axis");
    axis.push_vertex(Vertex::from([0., 0., 0., 1.]));
    axis.push_vertex(Vertex::from([0.1, 0., 0., 1.]));
    axis.push_vertex(Vertex::from([0., 0.1, 0., 1.]));
    axis.push_vertex(Vertex::from([0., 0., 0.1, 1.]));

    axis.push_edge(Edge::new_color(0, 1, RED).text("X"));
    axis.push_edge(Edge::new_color(0, 2, RED).text("Y"));
    axis.push_edge(Edge::new_color(0, 3, RED).text("Z"));


    let cube = mesh.push_object("cube");
    cube.push_vertex(Vertex::from([0.3, 0.3, 0., 1.]));
    cube.push_vertex(Vertex::from([0.6, 0.3, 0., 1.]));
    cube.push_vertex(Vertex::from([0.6, 0., 0., 1.]));
    cube.push_vertex(Vertex::from([0.3, 0., 0., 1.]));

    cube.push_vertex(Vertex::from([0.3, 0.3, 0.3, 1.]));
    cube.push_vertex(Vertex::from([0.6, 0.3, 0.3, 1.]));
    cube.push_vertex(Vertex::from([0.6, 0., 0.3, 1.]));
    cube.push_vertex(Vertex::from([0.3, 0., 0.3, 1.]));

    cube.push_face(Face::new(vec![0, 3, 2, 1]));
    cube.push_face(Face::new(vec![0, 1, 5, 4]));
    cube.push_face(Face::new(vec![1, 2, 6, 5]));
    cube.push_face(Face::new(vec![3, 7, 6, 2]));
    cube.push_face(Face::new(vec![0, 4, 7, 3]));
    cube.push_face(Face::new(vec![4, 5, 6, 7]));





    Model {
        camera: viewer(eye, at, up),
        perspective_proj: perspective_projection(
            std::f32::consts::PI / 4.,
            viewport.w() / viewport.h(),
            100.,
            -3.2,
        ),
        mouse_x_pressed: 0.0,
        mouse_y_pressed: 0.0,
        mouse_x: 0.0,
        mouse_y: 0.0,
        mouse_pressed: false,
        alt: false,
        zoom: 1.0,
        mesh
    }
}

fn event(_app: &App, model: &mut Model, event: WindowEvent) {
    match event {
        KeyPressed(key) => match key {
            Left => model.camera = model.camera * rotate_y_mat(0.001 * 180.0 / 3.1425),
            Right => model.camera = model.camera * rotate_y_mat(-0.001 * 180.0 / 3.1425),
            Up => {
                if !model.alt {
                    model.camera = model.camera * rotate_x_mat(0.001 * 180.0 / 3.1425)
                } else {
                    //model.eye[2] += 0.1;
                    //model.camera = viewer(model.eye, model.at, model.up)
                    model.zoom += 0.001;
                    model.camera = model.camera * scale_mat(model.zoom, model.zoom, model.zoom)
                }
            }
            Down => {
                if !model.alt {
                    model.camera = model.camera * rotate_x_mat(-0.001 * 180.0 / 3.1425)
                } else {
                    model.zoom += 0.001;
                    model.camera = model.camera
                        * scale_mat(1.0 / model.zoom, 1.0 / model.zoom, 1.0 / model.zoom)
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

fn update(_app: &App, _model: &mut Model, _update: Update) {}

fn view(app: &App, model: &Model, frame: Frame) {
    // Begin drawing
    let draw = app.draw();

    draw.background().color(WHITE);

    let viewport = app.window_rect();

    let transform = Mat4x4::unit();

    let mesh = model.mesh.clone();

    let mat = (model.perspective_proj * model.camera * transform * mesh)
        .to_screen(viewport.w() as f32, viewport.h() as f32);

    mat.draw_faces(&draw);
    mat.draw_lines(&draw);

    draw.to_frame(app, &frame).unwrap();
}
