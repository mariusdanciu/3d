
use common::model::figure::*;
use common::model::mat::*;

use nannou::event::*;
use nannou::event::WindowEvent::*;
use nannou::color::*;
use nannou::*;
use crate::noise::{Perlin, NoiseFn};


struct Model {
    camera: Mat4x4,
    perspective_proj: Mat4x4,
    zoff: f32,
    xoff: f32
}


fn main() {

    //nannou::sketch(view).run();
    nannou::app(model).update(update).run();
}

fn model(app: &App) -> Model {
    app.new_window()
        .event(event)
        .view(view).build().unwrap();
    let viewport = app.window_rect();
    
    Model { 
       camera: viewer(
        [0., 1., -2.], 
        [0., 0.5, 0.],
        [0., 1.2, 0.]),
        perspective_proj: perspective_projection(
            std::f32::consts::PI/4., 
            viewport.w() / viewport.h(), 
            100., 
            -3.2),
        zoff: 0.0,
        xoff: 0.0
    }
}

fn event(_app: &App, model: &mut Model, event: WindowEvent) {
    match event {
        MousePressed(key) => {
            println!("Pressed {:?}", key);
        },

        Resized(dim) => {
            model.perspective_proj = perspective_projection(
                std::f32::consts::PI/4., 
                dim[0] / dim[1], 
                100., 
                -3.2)
        }

        _ => {}

    }
}

fn update(_app: &App, model: &mut Model, _update: Update) {
    model.zoff += 0.1;
    //model.xoff -= 0.08;
}



fn map(x: f32, in_min: f32, in_max: f32,  out_min: f32,  out_max: f32) -> f32 {
   (x - in_min) * (out_max - out_min) / (in_max - in_min) + out_min
}

fn view(app: &App, model: &Model, frame: Frame) {

    // Begin drawing
    let draw = app.draw();

    // Clear the background to blue.
    draw.background().color(WHITE);

    let viewport = app.window_rect();

    let  perlin = Perlin::new();
    
    let len = 60;
    
    let mut zoff = model.zoff;

    let init_x = model.xoff;


    let mut local = Mesh::new();

    let surface = local.push_object("fabric");

    for z in 0..len {

        let mut xoff = init_x;
        for x in 0..len {

            let noise = perlin.get([zoff as f64, xoff as f64]) as f32;
            
            let height = map(noise, -1., 1., -0.08, 0.08);
            
            let v = Vertex::from([x as f32 * 0.05, height, z as f32 * 0.05, 1.]);
            surface.push_vertex(v);

            if x < len -1 && z < len - 1 {
                surface.push_edge(Edge::new(z*len + x, z*len + x + 1));
                surface.push_edge(Edge::new(z*len + x, (z+1)*len + x));
                surface.push_edge(Edge::new(z*len + x + 1, (z+1)*len + x));
            }
            xoff += 0.09;
        }
        zoff += 0.1;
    }



    let transform = translation_mat(-1., 0., 0.);

    let mat = (model.perspective_proj * model.camera * transform * local)
        .to_screen(viewport.w() as f32, viewport.h() as f32);

    mat.draw_lines(&draw);

    draw.to_frame(app, &frame).unwrap();
    
}
