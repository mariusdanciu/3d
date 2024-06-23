use nannou::{
    color::{rgb, BLACK},
    geom::{pt2, Point2},
    Draw,
};

#[derive(Debug)]
pub struct Vertex {
    pub x: f32,
    pub y: f32,
    pub z: f32,
    pub w: f32,
}

impl Vertex {
    pub fn to_vec(self) -> [f32; 4] {
        [self.x, self.y, self.z, self.w]
    }

    pub fn norm_z(self, x_size: f32, y_size: f32) -> Point2 {
        Point2::from_slice(&[self.x / self.w * x_size, -self.y / self.w * y_size])
    }
}

#[derive(Debug)]
pub struct Edge {
    pub from: usize,
    pub to: usize,
    pub color: rgb::Srgb<u8>,
    pub text: Option<String>,
}

#[derive(Debug)]
pub struct Face {
    pub vertexes: Vec<usize>,
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

#[derive(Debug)]
pub struct Mesh {
    pub vertexes: Vec<Vertex>,
    pub edges: Vec<Edge>,
    pub faces: Vec<Face>,
}

#[derive(Debug)]
pub struct Screen {
    pub points: Vec<Point2>,
    pub edges: Vec<Edge>,
    pub faces: Vec<Face>,
}

impl Screen {
    pub fn draw(&self, draw: &Draw) {
        for edge in self.edges.as_slice() {
            draw.line()
                .stroke_weight(1.)
                .color(edge.color)
                .points(self.points[edge.from], self.points[edge.to]);
            if let Some(txt) = &edge.text {
                draw.text(txt.as_str())
                    .xy(pt2(self.points[edge.to].x, self.points[edge.to].y + 10.0))
                    .color(edge.color);
            }
        }
    }

    pub fn draw_faces(&self, draw: &Draw) {
        for face in self.faces.as_slice() {
            let mut points = vec![] as Vec<Point2>;
            for v in face.vertexes.as_slice() {
                points.push(self.points[*v])
            }
            points.push(pt2(
                self.points[face.vertexes[0]].x,
                self.points[face.vertexes[0]].y,
            ));

            draw.polyline()
                .stroke_weight(1.)
                .color(BLACK)
                .points(points);
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
    pub fn to_screen(self, x_size: f32, y_size: f32) -> Screen {
        let mut nv: Vec<Point2> = vec![];

        for v in self.vertexes {
            nv.push(v.norm_z(x_size, y_size));
        }

        Screen {
            points: nv,
            edges: self.edges,
            faces: self.faces,
        }
    }
}
