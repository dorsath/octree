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


pub fn build(tree: &Octree, pixel: Pixel) -> u8{
    let faces = faces(tree.width, tree.root);
    let mut closest: f64 = -1.0;
    let mut largest: f64 = -2.0;

    //find the entry and exit position of the ray
    for face in faces.iter() {
        let denominator = face.normal.dot(&pixel.normal);
        if denominator == 0.0 {
            continue;
        }

        let p3_1 = face.point - pixel.point;
        let distance_to_entry = face.normal.dot(&p3_1) / denominator;

        let coordinate = pixel.coord_at(distance_to_entry);
        if tree.coordinate_in_cube(coordinate) == false {
            continue;
        }


        if closest < 0.0 || closest > distance_to_entry {
            closest = distance_to_entry;
        }

        if largest < distance_to_entry {
            largest = distance_to_entry;
        }
    }

    //move the ray through the cube looking up values under coordinates until it finds a filled cube
    let mut distance = closest;
    while distance <= largest {
        let coordinate = pixel.coord_at(distance);
        let (val, node_size) = tree.value_at(coordinate);
        distance += node_size / 2.0;
        if val == 'f' {
            //println!("{:?}", node_size);
            let a = 255.0 - (pixel.point - coordinate).norm().powf(2.0);
            //return ((pixel.point - coordinate).norm() / tree.width * 255.0) as u8 ;
            return a as u8;
        }
    }
    return 0;
}

pub fn faces(width: f64, root: Coordinate) -> Vec<Face> {
    return vec![
        Face {
            point:  Coordinate::new(0.5, 0.5, 0.0) * width + root,
            normal: Vector::new(0.0, 0.0, -1.0),
        },


        Face {
            point:  Coordinate::new(0.5, 0.5, 1.0) * width + root,
            normal: Vector::new(0.0, 0.0, 1.0),
        },

        Face {
            point:  Coordinate::new(0.0, 0.5, 0.5) * width + root,
            normal: Vector::new(-1.0, 0.0, 0.0),
        },


        Face {
            point:  Coordinate::new(1.0, 0.5, 0.5) * width + root,
            normal: Vector::new(1.0, 0.0, 0.0),
        },


        Face {
            point:  Coordinate::new(0.5, 0.0, 0.5) * width + root,
            normal: Vector::new(0.0, -1.0, 0.0),
        },


        Face {
            point:  Coordinate::new(0.5, 1.0, 0.5) * width + root,
            normal: Vector::new(0.0, 1.0, 0.0),
        }
    ];
}
