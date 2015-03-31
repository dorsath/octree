extern crate nalgebra;

use super::octree::*;

use nalgebra::*;

pub type Vector  = Vec3<f64>;

pub struct Pixel {
    pub normal: Vector,
    pub point: Coordinate,
}

impl Pixel {
    fn coord_at(&self, distance: f64) -> Coordinate {
        return self.point + self.normal * distance;
    }
}

#[derive(Debug, Clone)]
pub struct Face {
    pub normal: Vector,
    pub point: Coordinate,
}


pub fn build(pixel: Pixel, width: f64, root: Coordinate){
    corners();
    let faces = faces(width, root);
    let mut closest: f64 = -1.0;

    for face in faces.iter() {
        let denominator = face.normal.dot(&pixel.normal);
        if denominator == 0.0 {
            continue;
        }

        let p3_1 = face.point - pixel.point;
        let distance_to_entry = face.normal.dot(&p3_1) / denominator;

        if closest < 0.0 || closest > distance_to_entry {
            closest = distance_to_entry;
        }
    }


    let coordinate = pixel.coord_at(closest);
    println!("{:?}", coordinate);
}

pub fn faces(width: f64, root: Coordinate) -> Vec<Face> {
    return vec![
        Face {
            point:  Coordinate::new(0.5, 0.5, 0.0) * width + root,
            normal: Vector::new(0.0, 0.0, 1.0),
        },


        Face {
            point:  Coordinate::new(0.5, 0.5, 1.0) * width + root,
            normal: Vector::new(0.0, 0.0, 1.0),
        },

        Face {
            point:  Coordinate::new(0.0, 0.5, 0.5) * width + root,
            normal: Vector::new(1.0, 0.0, 0.0),
        },


        Face {
            point:  Coordinate::new(1.0, 0.5, 0.5) * width + root,
            normal: Vector::new(1.0, 0.0, 0.0),
        },


        Face {
            point:  Coordinate::new(0.5, 0.0, 0.5) * width + root,
            normal: Vector::new(0.0, 1.0, 0.0),
        },


        Face {
            point:  Coordinate::new(0.5, 1.0, 0.5) * width + root,
            normal: Vector::new(0.0, 1.0, 0.0),
        }
    ];
}
