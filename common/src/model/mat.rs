use crate::model::figure::*;
use std::{
    cell::RefMut,
    ops::{Mul, Sub},
};

#[derive(Clone, Copy, Debug)]
pub struct Mat4x4 {
    pub mat: [[f32; 4]; 4],
}

impl Mat4x4 {
    pub fn unit() -> Mat4x4 {
        Mat4x4 {
            mat: [
                [1., 0., 0., 0.],
                [0., 1., 0., 0.],
                [0., 0., 1., 0.],
                [0., 0., 0., 1.],
            ],
        }
    }

    pub fn transpose(&self) -> Mat4x4 {
        Mat4x4 {
            mat: [
                [
                    self.mat[0][0],
                    self.mat[1][0],
                    self.mat[2][0],
                    self.mat[3][0],
                ],
                [
                    self.mat[0][1],
                    self.mat[1][1],
                    self.mat[2][1],
                    self.mat[3][1],
                ],
                [
                    self.mat[0][2],
                    self.mat[1][2],
                    self.mat[2][2],
                    self.mat[3][2],
                ],
                [
                    self.mat[0][3],
                    self.mat[1][3],
                    self.mat[2][3],
                    self.mat[3][3],
                ],
            ],
        }
    }

    pub fn mul_dir(&self, rhs: [f32; 3]) -> [f32; 3] {
        let x = self.mat[0][0] * rhs[0] + self.mat[0][1] * rhs[1] + self.mat[0][2] * rhs[2];

        let y = self.mat[1][0] * rhs[0] + self.mat[1][1] * rhs[1] + self.mat[1][2] * rhs[2];

        let z = self.mat[2][0] * rhs[0] + self.mat[2][1] * rhs[1] + self.mat[2][2] * rhs[2];

        [x, y, z]
    }
}

impl Mul<[f32; 4]> for Mat4x4 {
    type Output = [f32; 4];

    fn mul(self, rhs: [f32; 4]) -> Self::Output {
        let mut x = self.mat[0][0] * rhs[0]
            + self.mat[0][1] * rhs[1]
            + self.mat[0][2] * rhs[2]
            + self.mat[0][3] * rhs[3];

        let mut y = self.mat[1][0] * rhs[0]
            + self.mat[1][1] * rhs[1]
            + self.mat[1][2] * rhs[2]
            + self.mat[1][3] * rhs[3];

        let mut z = self.mat[2][0] * rhs[0]
            + self.mat[2][1] * rhs[1]
            + self.mat[2][2] * rhs[2]
            + self.mat[2][3] * rhs[3];

        let w = self.mat[3][0] * rhs[0]
            + self.mat[3][1] * rhs[1]
            + self.mat[3][2] * rhs[2]
            + self.mat[3][3] * rhs[3];

        if w != 0.0 && w != 1.0 {
            x /= w;
            y /= w;
            z /= w;
        }
        [x, y, z, w]
    }
}

impl Mul<Mat4x4> for Mat4x4 {
    type Output = Mat4x4;

    fn mul(self, rhs: Mat4x4) -> Self::Output {
        let mut res = Mat4x4 { mat: [[0.; 4]; 4] };

        for r in 0..4 {
            for c in 0..4 {
                res.mat[r][c] = self.mat[r][0] * rhs.mat[0][c]
                    + self.mat[r][1] * rhs.mat[1][c]
                    + self.mat[r][2] * rhs.mat[2][c]
                    + self.mat[r][3] * rhs.mat[3][c];
            }
        }

        res
    }
}

impl Mul<&Mesh> for Mat4x4 {
    type Output = Mesh;

    fn mul(self, rhs: &Mesh) -> Self::Output {
        let mut new_mash = Mesh::new();
        for (k, v) in rhs.objects.iter() {
            let mut nv: Vec<Vertex> = vec![];
            for v in &v.vertexes {
                let p: [f32; 4] = self * v.to_vec();
                nv.push(Vertex::from(p));
            }
            let obj = new_mash.push_object(k.as_ref());
            obj.push_vertexes(nv);
            obj.push_edges(v.edges.clone());
            obj.push_faces(v.faces.clone());
        }
        new_mash
    }
}

pub fn translation_mat(x: f32, y: f32, z: f32) -> Mat4x4 {
    Mat4x4 {
        mat: [
            [1., 0., 0., x],
            [0., 1., 0., y],
            [0., 0., 1., z],
            [0., 0., 0., 1.],
        ],
    }
}

pub fn scale_mat(x: f32, y: f32, z: f32) -> Mat4x4 {
    Mat4x4 {
        mat: [
            [x, 0., 0., 0.],
            [0., y, 0., 0.],
            [0., 0., z, 0.],
            [0., 0., 0., 1.],
        ],
    }
}

pub fn rotate_x_mat(o: f32) -> Mat4x4 {
    Mat4x4 {
        mat: [
            [1., 0., 0., 0.],
            [0., f32::cos(o), -f32::sin(o), 0.],
            [0., f32::sin(o), f32::cos(o), 0.],
            [0., 0., 0., 1.],
        ],
    }
}

pub fn rotate_y_mat(o: f32) -> Mat4x4 {
    Mat4x4 {
        mat: [
            [f32::cos(o), 0., f32::sin(o), 0.],
            [0., 1., 0., 0.],
            [-f32::sin(o), 0., f32::cos(o), 0.],
            [0., 0., 0., 1.],
        ],
    }
}

pub fn rotate_z_mat(o: f32) -> Mat4x4 {
    Mat4x4 {
        mat: [
            [f32::cos(o), -f32::sin(o), 0., 0.],
            [f32::sin(o), f32::cos(o), 0., 0.],
            [0., 0., 1., 0.],
            [0., 0., 0., 1.],
        ],
    }
}

pub fn rotate_y_vec3(angle: f32, v: [f32; 3]) -> [f32; 3] {
    let n = rotate_y_mat(angle) * [v[0], v[1], v[2], 1.0];
    [n[0], n[1], n[2]]
}

pub fn rotate_y_around_p_vec3(angle: f32, v: [f32; 3], o: [f32; 3]) -> [f32; 3] {
    let nc = [v[0] - o[0], v[1] - o[1], v[2] - o[2]];
    let n = rotate_y_mat(angle) * [nc[0], nc[1], nc[2], 1.0];
    [n[0] + o[0], n[1] + o[1], n[2] + o[2]]
}

pub fn rotate_x_vec3(angle: f32, v: [f32; 3]) -> [f32; 3] {
    let n = rotate_x_mat(angle) * [v[0], v[1], v[2], 1.0];
    [n[0], n[1], n[2]]
}

pub fn rotate_x_around_p_vec3(angle: f32, v: [f32; 3], o: [f32; 3]) -> [f32; 3] {
    let nc = [v[0] - o[0], v[1] - o[1], v[2] - o[2]];
    let n = rotate_x_mat(angle) * [nc[0], nc[1], nc[2], 1.0];
    [n[0] + o[0], n[1] + o[1], n[2] + o[2]]
}

pub fn unit(v: [f32; 3]) -> [f32; 3] {
    let len = f32::sqrt(v[0] * v[0] + v[1] * v[1] + v[2] * v[2]);
    [v[0] / len, v[1] / len, v[2] / len]
}

pub fn diff(l: [f32; 3], r: [f32; 3]) -> [f32; 3] {
    [l[0] - r[0], l[1] - r[1], l[2] - r[2]]
}

pub fn cross(a: [f32; 3], b: [f32; 3]) -> [f32; 3] {
    [
        a[1] * b[2] - a[2] * b[1],
        a[2] * b[0] - a[0] * b[2],
        a[0] * b[1] - a[1] * b[0],
    ]
}

pub fn comp_dot(l: [f32; 3], r: [f32; 3]) -> [f32; 3] {
    [l[0] * r[0], l[1] * r[1], l[2] * r[2]]
}

pub fn dot(l: [f32; 3], r: [f32; 3]) -> f32 {
    l[0] * r[0] + l[1] * r[1] + l[2] * r[2]
}

pub fn inv(l: [f32; 3]) -> [f32; 3] {
    [1. / l[0], 1. / l[1], 1. / l[2]]
}

pub fn mul(l: [f32; 3], r: f32) -> [f32; 3] {
    [l[0] * r, l[1] * r, l[2] * r]
}

pub fn add(l: [f32; 3], r: [f32; 3]) -> [f32; 3] {
    [l[0] + r[0], l[1] + r[1], l[2] + r[2]]
}

pub fn add_scalar(l: [f32; 3], r: f32) -> [f32; 3] {
    [l[0] + r, l[1] + r, l[2] + r]
}

pub fn neg(l: [f32; 3]) -> [f32; 3] {
    [-l[0], -l[1], -l[2]]
}

pub fn viewer(eye: [f32; 3], at: [f32; 3], up: [f32; 3]) -> Mat4x4 {
    let n = unit(diff(eye, at));

    let u = unit(cross(up, n));

    let v = cross(n, u);

    Mat4x4 {
        mat: [
            [u[0], u[1], u[2], -dot(u, eye)],
            [v[0], v[1], v[2], -dot(v, eye)],
            [n[0], n[1], n[2], -dot(n, eye)],
            [0., 0., 0., 1.],
        ],
    }
}

pub fn perspective_projection(fov: f32, aspect: f32, f: f32, n: f32) -> Mat4x4 {
    let scale = (fov * 0.5 * std::f32::consts::PI / 180.).tan() * n;
    let r = aspect * scale;
    //let l = -r;
    let t = scale;
    //let b = -t;

    Mat4x4 {
        mat: [
            [n / r, 0., 0., 0.],
            [0., n / t, 0., 0.],
            [0., 0., (-f - n) / (f - n), -2. * f * n / (f - n)],
            [0., 0., -1., 0.],
        ],
    }
}
