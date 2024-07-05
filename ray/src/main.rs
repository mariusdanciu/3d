use std::rc::Rc;

use common::model::figure::*;
use common::model::mat::*;
use draw::background::new;

use geom::pt2;
use image::ImageBuffer;
use nannou::color::*;
use nannou::event::WindowEvent::*;
use nannou::event::*;
use nannou::*;
use std::cell::RefCell;
use wgpu::Device;
use wgpu::Texture;
use winit::event::VirtualKeyCode::*;

struct Model {
    texture: Texture,
    eye: [f32; 3],
    at: [f32; 3],
    up: [f32; 3],
    width: f32,
    height: f32,
    camera: Mat4x4,
    perspective_proj: Mat4x4,
    mouse_x_pressed: f32,
    mouse_y_pressed: f32,
    mouse_x: f32,
    mouse_y: f32,
    mouse_pressed: bool,
    alt: bool,
    mesh: Mesh,
}

#[derive(Debug, Copy, Clone)]
struct Ray {
    origin: [f32; 3],
    dir: [f32; 3],
}

fn main() {
    nannou::app(model).update(update).run();
}

fn model(app: &App) -> Model {
    let window = app.new_window().size(640, 480);
    window.event(event).view(view).build().unwrap();
    let viewport = app.window_rect();

    let eye = [0., 0.0, 0.0];
    let at = [0.45, 0.15, -1.3];
    let up = [0., 1.0, 0.];

    let mut mesh = Mesh::new();

    let axis = mesh.push_object("axis");
    axis.push_vertex(Vertex::from([0., 0., -1., 1.]));
    axis.push_vertex(Vertex::from([0.1, 0., -1., 1.]));
    axis.push_vertex(Vertex::from([0., 0.1, -1., 1.]));
    axis.push_vertex(Vertex::from([0., 0., -1.1, 1.]));

    axis.push_edge(Edge::new_color(0, 1, RED).text("X"));
    axis.push_edge(Edge::new_color(0, 2, RED).text("Y"));
    axis.push_edge(Edge::new_color(0, 3, RED).text("-Z"));

    let cube = mesh.push_object("cube");
    cube.push_vertex(Vertex::from([0.3, 0.3, -1.1, 1.]));
    cube.push_vertex(Vertex::from([0.6, 0.3, -1.1, 1.]));
    cube.push_vertex(Vertex::from([0.6, 0., -1.1, 1.]));
    cube.push_vertex(Vertex::from([0.3, 0., -1.1, 1.]));

    cube.push_vertex(Vertex::from([0.3, 0.3, -1.4, 1.]));
    cube.push_vertex(Vertex::from([0.6, 0.3, -1.4, 1.]));
    cube.push_vertex(Vertex::from([0.6, 0., -1.4, 1.]));
    cube.push_vertex(Vertex::from([0.3, 0., -1.4, 1.]));

    cube.push_face(Face::new([0, 3, 2]));
    cube.push_face(Face::new([2, 1, 0]));

    cube.push_face(Face::new([0, 1, 5]));
    cube.push_face(Face::new([5, 4, 0]));

    cube.push_face(Face::new([1, 2, 6]));
    cube.push_face(Face::new([6, 5, 1]));

    cube.push_face(Face::new([3, 7, 6]));
    cube.push_face(Face::new([6, 2, 3]));

    cube.push_face(Face::new([0, 4, 7]));
    cube.push_face(Face::new([7, 3, 0]));

    cube.push_face(Face::new([4, 5, 6]));
    cube.push_face(Face::new([6, 7, 4]));

    let texture = wgpu::TextureBuilder::new()
        .size([viewport.w() as u32, viewport.h() as u32])
        .format(wgpu::TextureFormat::Rgba8Unorm)
        .usage(wgpu::TextureUsages::COPY_DST | wgpu::TextureUsages::TEXTURE_BINDING)
        .build(app.main_window().device());

    Model {
        texture,
        eye,
        at,
        up,
        width: viewport.w(),
        height: viewport.h(),
        camera: viewer(eye, at, up),
        perspective_proj: perspective_projection(60., viewport.w() / viewport.h(), -10., -1.),
        mouse_x_pressed: 0.0,
        mouse_y_pressed: 0.0,
        mouse_x: 0.0,
        mouse_y: 0.0,
        mouse_pressed: false,
        alt: false,
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
                    model.eye =
                        rotate_x_around_p_vec3(-0.001 * 180.0 / 3.1425, model.eye, model.at);
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

                model.eye =
                    rotate_x_around_p_vec3(y_diff / 1000000. * 180.0 / 3.1425, model.eye, model.at);
                model.eye =
                    rotate_y_around_p_vec3(x_diff / 1000000. * 180.0 / 3.1425, model.eye, model.at);

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

fn to_rgba(c: [f32; 4]) -> nannou::image::Rgba<u8> {
    nannou::image::Rgba([
        (c[0] * 255.) as u8,
        (c[1] * 255.) as u8,
        (c[2] * 255.) as u8,
        (c[3] * 255.) as u8,
    ])
}

fn pixel(x: f32, y: f32) -> [f32; 4] {
    let ray_orig = [0., 0., 1.];
    let ray_dir = [x, y, -1.];
    let radius = 0.5;

    let sphere_color = [1., 0., 1.];


    let a = dot(ray_dir, ray_dir);
    let b = 2. * dot(ray_orig, ray_dir);
    let c = dot(ray_orig, ray_orig) - radius * radius;

    let disc = b * b - 4. * a * c;

    if disc < 0.0 {
        return [0., 0., 0., 1.];
    }

    // closest to ray origin
    let t = (-b - disc.sqrt()) / (2.0 * a);
    //let t1 = (-b + disc.sqrt()) / 2.0 * a;

    let hit_point = add(ray_orig, mul(ray_dir, t));

    let normal = unit(hit_point);

    let light_dir = unit([-1., -1., -1.]);

    let mut d = dot(normal, neg(light_dir));
    if d < 0.0 {
        d = 0.0;
    }

    let color = mul(sphere_color, d);

    [color[0], color[1], color[2], 1.]
}

fn view(app: &App, model: &Model, frame: Frame) {
    // Begin drawing
    let draw = app.draw();

    draw.background().color(WHITE);

    let viewport = app.window_rect();

    let transform = Mat4x4::unit();

    let mut new_mesh = model.mesh.clone();

    new_mesh.set_camera(model.eye);

    let objects = model.perspective_proj * model.camera * transform * &new_mesh;

    let width = viewport.w();
    let height = viewport.h();

    let ratio = width / height;
    let fov = 60.;
    let scale = (fov * 0.5 * std::f32::consts::PI / 180.).tan();

    let camera_to_world = model.camera.transpose();

    let mut imgbuf = image::ImageBuffer::<image::Rgba<u8>, _>::new(640, 480);

    for y in 0..(height as i32) {
        for x in 0..(width as i32) {
            let p_ndc_x = (x as f32) / width;
            let p_ndc_y = (y as f32) / height;

            let p_screen_x = (2.0 * p_ndc_x - 1.) * ratio;
            let p_screen_y = 1. - 2.0 * p_ndc_y;

            let color = pixel(p_screen_x, p_screen_y);

            imgbuf.put_pixel(x as u32, y as u32, to_rgba(color));

            for (k, obj) in model.mesh.objects.iter() {
                if k == "cube" {}
            }
        }
    }

    let flat_samples = imgbuf.as_flat_samples();
    model.texture.upload_data(
        app.main_window().device(),
        &mut *frame.command_encoder(),
        &flat_samples.as_slice(),
    );

    draw.texture(&model.texture);
    draw.to_frame(app, &frame).unwrap();
}
