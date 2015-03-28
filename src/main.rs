extern crate nalgebra;
use nalgebra::*;


use Leaf::{Node,Value};
#[derive(Debug)]
enum Leaf {
    Node(Octree),
    Value(f64),
}

type Octree = Vec<Leaf>;
pub type Coordinate = Vec3<f64>;
type Coordinates = Vec<Coordinate>;



mod octree {
    use nalgebra::*;
    use std::path::Path;
    use std::io::prelude::*;
    use std::fs::File;
    use std::error::Error;
    
    
    
    use {Octree, Leaf, Coordinate, Coordinates, Particle};


    pub fn init() -> Leaf{
        let result: Coordinates = corners();
        let width = 100.0f64;
        let root = Coordinate::new(0.0, 0.0, 0.0);
        let result = build(root, width);
        //output_to_file(result, width);

        return result;
    }

    fn build(root: Coordinate, width: f64) -> Leaf {
        let qw = width / 4.0; //quarter width
        let mut octree = Octree::new();

        for corner in corners().iter() {
            let coordinate = *corner * width + root;
            if width > 0.2 && split(&coordinate, width) {
                let node = build(coordinate, width / 2.0);
                octree.push(node);
            } else {
                let center = coordinate + Coordinate::new(qw, qw ,qw);
                let val = value_function(&center);
                octree.push(Leaf::Value(val));
            }
        }

        return Leaf::Node(octree);
    }

    fn value_function(coord: &Coordinate) -> f64 {
        let objects = vec!(
            Coordinate::new(10.0, 0.5, 10.0),
            Coordinate::new(20.0, 0.5, 10.0)
        );

        let mut value = 0.0f64;
        for obj in objects {
            let norm = (obj - *coord).norm();
            if norm == 0.0 {
                value += 1_000f64;
            } else {
                value += (1.0f64 / norm / norm);
            }
        }
        return value;
    }

    fn split(root: &Coordinate, width: f64) -> bool {
        let hw = width / 2.0; //half width
        let center: Coordinate = *root + Coordinate::new(hw, hw ,hw);
        let center_value = value_function(&center);
        for node in corners() {
            let node_corner = *root + (node * width);
            let r = (value_function(&node_corner) - center_value).abs();
            if r > (0.2 / width / width) {
                return true;
            }
        }
        return false;
    }

    fn corners() -> Coordinates {
        let mut corners = Coordinates::new();

        for n in 0..8 {
            corners.push(coordinate_of_index(n) * 0.5);
        }

        return corners;
    }

    fn coordinate_of_index(index: i8) -> Coordinate {
        return Vec3::new(
            (index % 2) as f64, 
            ((index / 2) % 2) as f64,
            ((index / 4) % 2) as f64
        )
    }

    fn output_to_file(octree: Leaf, width: f64) -> () {
        let path = Path::new("./results.txt");
        let display = path.display();
        let mut file = match File::create(&path) {
            Err(why) => panic!("couldn't create {}: {}",
                           display,
                           Error::description(&why)),
            Ok(file) => file,
        };

        let root = Coordinate::new(0.0, 0.0, 0.0);
        println!("Outputting to file");
        let output: String = run_through_octree(octree, root, width);
        file.write(output.as_bytes());
    }

    fn run_through_octree(leaf: Leaf, root: Coordinate, width: f64) -> String {
        let mut output = String::new();
        match leaf {
            Leaf::Value(x) => {
                let qw = width / 4.0; //quarter width
                let coordinate = root + Coordinate::new(qw, qw ,qw);
                let r = format!("{:?}\t{:?}\t{:?}\t{:?}\n", coordinate[0], coordinate[1], coordinate[2], x);
                output.push_str(&r);
            },
            Leaf::Node(node) => {
                let corners = corners();
                let mut n = 0;
                for node_leaf in node {
                    let coordinate = corners[n] * width + root;
                    output.push_str(&run_through_octree(node_leaf, coordinate, width / 2.0));
                    n += 1;
                }
            },
        }

        return output;
    }

    pub fn find_value_at(leaf: Leaf, coord: Coordinate, root: Coordinate, width: f64) -> f64 {
        match leaf {
            Leaf::Value(x) => {
                return x;
            },
            Leaf::Node(node) => {
                let mut vec = (coord - root) / width;
                let index: i8 = vec[2].round() as i8 * 4 + vec[1].round() as i8 * 2 + vec[0].round() as i8;
                let new_root = root + coordinate_of_index(index) * width / 2.0;
                let mut n = 0;
                for node_leaf in node {
                    if n == index {
                        return find_value_at(node_leaf, coord, new_root, width / 2.0);
                    }
                    n += 1;
                }

                return 0.0;

            },

        }


        return 0.0;
    }

}

pub struct Particle {
    pub pos:       Vec3<f64>,
    pub vel:       Vec3<f64>,
    pub acc:   Vec3<f64>
}


fn main() {
    let result = octree::init();
    println!("Done creating octree");

    let coord = Vec3::new(5.0, 5.0, 50.0);
    //let val = octree::find_value_at(result, coord, Vec3::new(0.0, 0.0, 0.0), 100f64);
    //println!("{:?}", val);


    let mut particle = Particle{
        pos: Vec3::new(2.0, 2.0, 2.0),
        vel: Vec3::new(1.0, 0.0, 0.0),
        acc: Vec3::new(0.0, 0.0, 0.0)};
    let dt = 0.01;
    let mass = 1.0;

    let val = octree::find_value_at(result, coord, Vec3::new(0.0, 0.0, 0.0), 100f64);
    let factor;
    if particle.vel.norm() > 0.01 {
        factor = particle.vel.normalize() * -1.0 * val;
    } else {
        factor = Vec3::new(0.0, 0.0, 0.0);
    }


    particle.pos = particle.pos + particle.vel * dt + factor / mass * dt * dt;
    particle.vel = particle.vel + factor / mass * dt;



    println!("{:?}", particle.pos);
}

