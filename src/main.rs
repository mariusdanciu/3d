mod model;

use nannou::prelude::*;
use crate::model::figure::*;
use crate::model::mat::*;
use noise::{Perlin, NoiseFn};
 
struct Model {
    window: WindowId
}


fn main() {

    //nannou::sketch(view).run();
    nannou::app(model).update(update).run();
}

fn model(app: &App) -> Model {
    let window = app.new_window().view(view).build().unwrap();
    Model { 
       window
    }
}

fn update(_app: &App, model: &mut Model, update: Update) {

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

    let view = viewer(
        [0., 1.2, 4.], 
        [0., 0., 0.],
        [0., 1., 0.]);

    let projection = perspective_projection(
        std::f32::consts::PI/4., 
        1., 
        100., 
        2.2);

        let mut surface: Vec<Vertex> = vec![];
        let mut edges: Vec<(usize, usize)> = vec![];
    
        let  perlin = Perlin::new(1);
        
        let len = 60;
        
        let mut zoff = 0.0;

        for z in 0..len {
    
            let mut xoff = 0.0;
            for x in 0..len {

                let noise = perlin.get([zoff as f64, xoff as f64]) as f32;
                
                let height = map(noise, -1., 1., -0.08, 0.08);
                
                let v = Vertex::from([x as f32 * 0.05, height, z as f32 * 0.05, 1.]);
                surface.push(v);
    
                if x < len -1 && z < len - 1 {
                    edges.push((z*len + x, z*len + x + 1));
                    edges.push((z*len + x, (z+1)*len + x));
                    edges.push((z*len + x + 1, (z+1)*len + x));
                }
                xoff += 0.09;
            }
            zoff += 0.1;
        }

    let local = Mesh{
        vertexes: surface,
        edges: edges
    };

    
    

    let transform = translation_mat(-1., 0., 0.);

    let mat = (projection * view * transform * local)
        .norm_z(viewport.w() as f32, viewport.h() as f32);

    draw_mesh(&draw, mat);

    draw.to_frame(app, &frame).unwrap();
    
}


fn draw_mesh(draw: &Draw, mesh: Mesh) {
    for (from, to) in mesh.edges {
        draw.line()
            .stroke_weight(1.)
            .color(BLACK)
            .points(
                pt2(mesh.vertexes[from].x, mesh.vertexes[from].y), 
                pt2(mesh.vertexes[to].x, mesh.vertexes[to].y)
            );
    }
}