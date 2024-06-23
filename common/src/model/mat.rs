
use std::ops::Mul;
use crate::model::figure::*;


#[derive(Clone, Copy, Debug)]
pub struct Mat4x4 {
    pub mat: [[f32; 4]; 4]
}

impl Mat4x4 {
    pub fn unit() -> Mat4x4 {
        Mat4x4 { mat: [
            [1., 0., 0., 0.],
            [0., 1., 0., 0.],
            [0., 0., 1., 0.],
            [0., 0., 0., 1.],
        ] }
     }
}

impl Mul<[f32; 4]> for Mat4x4 {
    type Output = [f32; 4];


    fn mul(self, rhs: [f32; 4]) -> Self::Output {

        [
           self.mat[0][0] * rhs[0] + self.mat[0][1] * rhs[1] + self.mat[0][2] * rhs[2] + self.mat[0][3] * rhs[3],
           self.mat[1][0] * rhs[0] + self.mat[1][1] * rhs[1] + self.mat[1][2] * rhs[2] + self.mat[1][3] * rhs[3],
           self.mat[2][0] * rhs[0] + self.mat[2][1] * rhs[1] + self.mat[2][2] * rhs[2] + self.mat[2][3] * rhs[3],
           self.mat[3][0] * rhs[0] + self.mat[3][1] * rhs[1] + self.mat[3][2] * rhs[2] + self.mat[3][3] * rhs[3]
        ]

    }
}

impl Mul<Mat4x4> for Mat4x4 {
    type Output = Mat4x4;

    fn mul(self, rhs: Mat4x4) -> Self::Output {
        let mut res = Mat4x4{
            mat: [[0.; 4]; 4]
        };

        for r in 0..4 {
            for c in 0..4 {
                res.mat[r][c] =   self.mat[r][0]*rhs.mat[0][c] 
                                + self.mat[r][1]*rhs.mat[1][c] 
                                + self.mat[r][2]*rhs.mat[2][c] 
                                + self.mat[r][3]*rhs.mat[3][c];
           }
        }
 

        res
    }
}


impl Mul<Mesh> for Mat4x4 {
    type Output = Mesh;

    fn mul(self, rhs: Mesh) -> Self::Output {
        let mut nv: Vec<Vertex> = vec![];
        for v in rhs.vertexes {
            let p: [f32; 4] = self * v.to_vec();
            nv.push(Vertex::from(p));
        }
        Mesh{
            vertexes: nv,
            edges: rhs.edges,
            faces: rhs.faces
        }
    }
}



pub fn translation_mat(x: f32, y: f32, z: f32) -> Mat4x4 {
    Mat4x4 {
     mat: [
         [1.,   0.,     0.,    x],
         [0.,   1.,     0.,    y],
         [0.,   0.,     1.,    z],
         [0.,   0.,     0.,    1.],
     ]
    }
}

pub fn scale_mat(x: f32, y: f32, z: f32) -> Mat4x4 {
    Mat4x4 {
     mat: [
         [x,    0.,    0.,    0.],
         [0.,   y,     0.,    0.],
         [0.,   0.,    z,     0.],
         [0.,   0.,    0.,    1.],
     ]
    }
}

pub fn rotate_x_mat(o: f32) -> Mat4x4 {
    Mat4x4 {
     mat: [
         [1.,   0.,                  0.,                   0.],
         [0.,   f32::cos(o),   -f32::sin(o),    0.],
         [0.,   f32::sin(o),   f32::cos(o),     0.],
         [0.,   0.,                  0.,                   1.],
     ]
    }
}

pub fn rotate_y_mat(o: f32) -> Mat4x4 {
    Mat4x4 {
     mat: [
         [f32::cos(o),     0.,   f32::sin(o),      0.],
         [0.,                    1.,   0.,                    0.],
         [-f32::sin(o),    0.,   f32::cos(o),     0.],
         [0.,                    0.,   0.,                    1.],
     ]
    }
}

pub fn rotate_z_mat(o: f32) -> Mat4x4 {
    Mat4x4 {
     mat: [
         [f32::cos(o),   -f32::sin(o),   0.,    0.],
         [f32::sin(o),   f32::cos(o),    0.,    0.],
         [0.,                  0.,                  1.,    0.],
         [0.,                  0.,                  0.,    1.],
     ]
    }
}

fn norm(v: [f32; 3]) -> [f32; 3] {
    let len = f32::sqrt(v[0]*v[0] + v[1]*v[1] + v[2]*v[2] );
    [
        v[0] / len,
        v[1] / len,
        v[2] / len
    ]
}

fn diff(l: [f32; 3], r: [f32; 3]) -> [f32; 3] {
    [
        l[0] - r[0],
        l[1] - r[1],
        l[2] - r[2],
    ]
}

fn cross(a: [f32; 3], b: [f32; 3]) -> [f32; 3] {
    [
        a[1]*b[2] - a[2]*b[1],
        a[2]*b[0] - a[0]*b[2],
        a[0]*b[1] - a[1]*b[0],
    ]
}

fn dot(l: [f32; 3], r: [f32; 3]) -> f32 {  
    l[0]*r[0] + l[1]*r[1] + l[2]*r[2]
}

pub fn viewer(eye : [f32; 3], at: [f32; 3], up: [f32; 3]) -> Mat4x4 {
    let z_axis = norm(diff(eye, at));

    let x_axis = norm(cross( up, z_axis));

    let y_axis = cross( z_axis, x_axis );

     Mat4x4 {
        mat: [
            [x_axis[0], x_axis[1], x_axis[2], -dot(x_axis, eye)],
            [y_axis[0], y_axis[1], y_axis[2], -dot(y_axis, eye)],
            [z_axis[0], z_axis[1], z_axis[2], -dot(z_axis, eye)],
            [0.,        0.,        0.,        1.]
        ]
    }

}



pub fn perspective_projection(fov: f32, aspect: f32, z_far: f32, z_near: f32) -> Mat4x4 {

    let tan = 1. / (fov * 0.5).tan();
    let dist = z_near - z_far;

    Mat4x4 {
     mat: [
         [tan / aspect,           0.,         0.,                                   0.],
         [0.,                     tan,        0.,                                   0.],
         [0.,                     0.,         (-z_near - z_far) / dist,             2. * z_far * z_near / dist],
         [0.,                     0.,         1.,                                   0.],
     ]
    }
     
 }
 
