use std::{collections::HashMap, iter::Map, ops::Sub};

use nannou::{
    color::{rgb, BLACK, BLUE, GREEN},
    geom::{pt2, Point2},
    Draw,
};

use super::mat::{cross, diff, dot, unit, Mat4x4};

#[derive(Debug, Copy, Clone)]
pub struct Vertex {
    pub x: f32,
    pub y: f32,
    pub z: f32,
    pub w: f32,
}

impl Vertex {
    pub fn from_vec4(v: [f32; 4]) -> Vertex {
        Vertex {
            x: v[0],
            y: v[1],
            z: v[2],
            w: v[3],
        }
    }
    pub fn from_vec(v: [f32; 3]) -> Vertex {
        Vertex {
            x: v[0],
            y: v[1],
            z: v[2],
            w: 1.0,
        }
    }
    pub fn to_vec(&self) -> [f32; 4] {
        [self.x, self.y, self.z, self.w]
    }

    pub fn to_vec_3(&self) -> [f32; 3] {
        [self.x, self.y, self.z]
    }

    pub fn screen(self, x_size: f32, y_size: f32) -> Point2 {
        let screen_x = self.x * 0.5 * x_size;
        let screen_y = self.y * 0.5 * y_size;

        return Point2::from_slice(&[screen_x, screen_y]);
    }
}

#[derive(Debug, Clone)]
pub struct Edge {
    pub from: usize,
    pub to: usize,
    pub color: rgb::Srgb<u8>,
    pub text: Option<String>,
}

#[derive(Debug, Clone)]
pub struct Face {
    pub vertexes: [usize; 3],
    pub normal_vertex: i32,
    pub center_vertex: i32,
}


impl Face {
    pub fn new(v: [usize; 3]) -> Face {
        Face {
            vertexes: v,
            normal_vertex: -1,
            center_vertex: -1,
        }
    }

    pub fn normal(&self, vert: &Vec<Vertex>) -> ([f32; 3], [f32; 3]) {
        let len = self.vertexes.len();
        let lenf32 = len as f32;

        let xm: f32 = self.vertexes.iter().map(|v| vert[*v].x).sum();
        let ym: f32 = self.vertexes.iter().map(|v| vert[*v].y).sum();
        let zm: f32 = self.vertexes.iter().map(|v| vert[*v].z).sum();

        let v1 = vert[self.vertexes[0]].to_vec_3();
        let v2 = vert[self.vertexes[1]].to_vec_3();
        let v3 = vert[self.vertexes[2]].to_vec_3();

        let n = cross(diff(v2, v1), diff(v3, v2));

        (
            [
                n[0] + (xm / lenf32),
                n[1] + (ym / lenf32),
                n[2] + (zm / lenf32),
            ],
            [xm / lenf32, ym / lenf32, zm / lenf32],
        )
    }
}


impl Edge {
    pub fn new(from: usize, to: usize) -> Edge {
        Edge {
            from,
            to,
            color: BLACK,
            text: None,
        }
    }
    pub fn new_color(from: usize, to: usize, color: rgb::Srgb<u8>) -> Edge {
        Edge {
            from,
            to,
            color,
            text: None,
        }
    }

    pub fn text(&self, text: &str) -> Edge {
        Edge {
            from: self.from,
            to: self.to,
            color: self.color,
            text: Some(String::from(text)),
        }
    }
}

impl Default for Edge {
    fn default() -> Edge {
        Edge {
            from: 0,
            to: 0,
            color: BLACK,
            text: None,
        }
    }
}

#[derive(Debug, Clone)]
pub struct Obj3D {
    pub vertexes: Vec<Vertex>,
    pub edges: Vec<Edge>,
    pub faces: Vec<Face>,
}

#[derive(Debug, Clone)]
pub struct Obj2D {
    pub points: Vec<Point2>,
    pub edges: Vec<Edge>,
    pub faces: Vec<Face>,
}

#[derive(Debug, Clone)]
pub struct Mesh {
    pub objects: HashMap<String, Obj3D>,
}

#[derive(Debug)]
pub struct Screen {
    pub objects: HashMap<String, Obj2D>,
}

impl Screen {
    pub fn draw_lines(&self, draw: &Draw) {
        for (k, v) in self.objects.iter() {
            let last = v.points.len() - 1;

            for edge in v.edges.as_slice() {
                draw.line()
                    .stroke_weight(1.)
                    .color(edge.color)
                    .points(v.points[edge.from], v.points[edge.to]);
                if let Some(txt) = &edge.text {
                    draw.text(txt.as_str())
                        .xy(pt2(v.points[edge.to].x, v.points[edge.to].y + 10.0))
                        .color(edge.color);
                }
            }
        }
    }

    pub fn draw_faces(&self, draw: &Draw) {
        for (k, v) in self.objects.iter() {
            for face in v.faces.as_slice() {
                let mut points = vec![] as Vec<Point2>;

                for vt in face.vertexes.as_slice() {
                    points.push(v.points[*vt])
                }

                points.push(pt2(
                    v.points[face.vertexes[0]].x,
                    v.points[face.vertexes[0]].y,
                ));

                draw.polyline()
                    .stroke_weight(1.)
                    .color(BLACK)
                    .points(points);
/* 
                let last = v.points.len() - 1;

                draw.line().stroke_weight(1.).color(GREEN).points(
                    pt2(
                        v.points[face.center_vertex as usize].x,
                        v.points[face.center_vertex as usize].y,
                    ),
                    pt2(v.points[last].x, v.points[last].y),
                );
*/
            }
        }
    }
}

impl From<[f32; 4]> for Vertex {
    fn from(v: [f32; 4]) -> Self {
        Vertex {
            x: v[0],
            y: v[1],
            z: v[2],
            w: v[3],
        }
    }
}

impl Mesh {
    pub fn new() -> Mesh {
        Mesh {
            objects: HashMap::new(),
        }
    }

    pub fn set_camera(&mut self, eye: [f32; 3]) -> &Self {
        for (_, v) in self.objects.iter_mut() {
            v.push_vertex(Vertex::from_vec(eye));

            let new_faces: Vec<Face> = v.faces.clone().into_iter().filter(|face| {
                let c = v.vertexes[face.center_vertex as usize].to_vec_3();
                let n =  v.vertexes[face.normal_vertex as usize].to_vec_3();

                let normal = unit(diff(n, c));
                let dir = unit(diff(c, eye));
                dot(normal, dir) < 0.0
            }).collect();
            v.faces = new_faces;
        }
        self
    }

    pub fn to_screen(&mut self, x_size: f32, y_size: f32) -> Screen {
        let mut scr = Screen {
            objects: HashMap::new(),
        };

        for (k, v) in self.objects.iter_mut() {
            let mut nv: Vec<Point2> = vec![];

            for v in v.vertexes.iter() {
                let p = v.screen(x_size, y_size);
                nv.push(p);
            }

            scr.objects.insert(
                k.into(),
                Obj2D {
                    points: nv,
                    edges: v.edges.clone(),
                    faces: v.faces.clone(),
                },
            );
        }

        scr
    }

    pub fn push_object(&mut self, name: &str) -> &mut Obj3D {
        let obj = Obj3D::new();
        self.objects.insert(String::from(name), obj);
        self.objects.get_mut(name).unwrap()
    }
}

impl Obj3D {
    pub fn new() -> Obj3D {
        Obj3D {
            vertexes: vec![],
            faces: vec![],
            edges: vec![],
        }
    }
    pub fn push_face(&mut self, face: Face) -> &Obj3D {
        let last_v_idx = self.vertexes.len() - 1;

        let (normal, m) = face.normal(&self.vertexes);

        self.push_vertex(Vertex::from([m[0], m[1], m[2], 1.]));
        self.push_vertex(Vertex::from([normal[0], normal[1], normal[2], 1.]));
        //self.push_edge(Edge::new_color(last_v_idx + 1, last_v_idx + 2, BLUE));

        let mut f = face.clone();
        f.normal_vertex = (last_v_idx + 2) as i32;
        f.center_vertex = (last_v_idx + 1) as i32;

        self.faces.push(f);
        self
    }

    pub fn push_vertex(&mut self, v: Vertex) -> &Obj3D {
        self.vertexes.push(v);
        self
    }

    pub fn push_vertexes(&mut self, v: Vec<Vertex>) -> &Obj3D {
        for vt in v {
            self.push_vertex(vt);
        }
        self
    }

    pub fn push_edge(&mut self, e: Edge) -> &Obj3D {
        self.edges.push(e);
        self
    }

    pub fn push_edges(&mut self, v: Vec<Edge>) -> &Obj3D {
        for vt in v {
            self.edges.push(vt);
        }
        self
    }

    pub fn push_faces(&mut self, v: Vec<Face>) -> &Obj3D {
        for vt in v {
            self.faces.push(vt);
        }
        self
    }
}
