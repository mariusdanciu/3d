use nannou::{
    color::{rgb, BLACK},
    geom::pt2,
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

    pub fn norm_z(self, x_size: f32, y_size: f32) -> Vertex {
        if self.w != 0.0 {
            return Vertex {
                x: self.x / self.w * x_size,
                y: -self.y / self.w * y_size,
                z: self.z / self.w,
                w: 1. / self.w,
            };
        }

        self
    }
}

#[derive(Debug)]
pub struct Edge {
    pub from: usize,
    pub to: usize,
    pub color: rgb::Srgb<u8>,
    pub text: Option<String>,
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
    pub fn norm_z(self, x_size: f32, y_size: f32) -> Mesh {
        let mut nv: Vec<Vertex> = vec![];

        for v in self.vertexes {
            nv.push(v.norm_z(x_size, y_size));
        }

        Mesh {
            vertexes: nv,
            edges: self.edges,
        }
    }

    pub fn draw(self, draw: &Draw) {
        for edge in self.edges {
            draw.line().stroke_weight(1.).color(edge.color).points(
                pt2(self.vertexes[edge.from].x, self.vertexes[edge.from].y),
                pt2(self.vertexes[edge.to].x, self.vertexes[edge.to].y),
            );
            if let Some(txt) = edge.text {
                draw.text(txt.as_str())
                    .xy(pt2(self.vertexes[edge.to].x, self.vertexes[edge.to].y + 10.0))
                    .color(edge.color);
            }
        }
    }
}
