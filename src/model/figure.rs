

#[derive(Debug)]
pub struct Vertex {
    pub x: f32,
    pub y: f32,
    pub z: f32,
    pub w: f32,
}


impl Vertex {

    pub fn to_vec(self) -> [f32; 4] {
        [
            self.x, self.y, self.z, self.w
        ]
    }

    pub fn norm_z(self) -> Vertex {
        if self.w != 0.0 {
            return Vertex { 
                x: self.x / self.w * 400., 
                y: self.y / self.w * 400., 
                z: self.z / self.w, 
                w: 1. / self.w
            }
        }

        self
    }
}

#[derive(Debug)]
pub struct Mesh {
    pub vertexes: Vec<Vertex>,
    pub edges: Vec<(usize, usize)>
}

impl From<[f32; 4]> for Vertex {
    fn from(v: [f32; 4]) -> Self {
        Vertex { 
            x: v[0], 
            y: v[1], 
            z: v[2], 
            w: v[3]
        }
    }
}

impl Mesh {
    
    pub fn norm_z(self) -> Mesh {
        let mut nv: Vec<Vertex> = vec![];

        for v in self.vertexes {
            nv.push(v.norm_z());
        }

        Mesh { 
            vertexes: nv, 
            edges: self.edges
        }
    }
}