extern crate nalgebra;

use nalgebra::*;
use std::num::Float;
use super::scene::{Scene};

#[derive(Debug, Clone)]
pub enum Node {
    Group(Vec<Node>),
    Filled,
    Empty,
}

pub struct Octree {
    pub root: Coordinate,
    pub width: f64,
    pub nodes: Node,
}

pub type Coordinate  = Vec3<f64>;
pub type Coordinates = Vec<Coordinate>;

impl Node {
    pub fn build(root: Coordinate, width: f64, scene: &Scene) -> Node {
        let mut tree: Vec<Node> = Vec::new();

        for corner in corners().iter() {
            let coord = *corner * width + root;
            let cube_status = split(&coord, width, scene);
            if width >= 0.5 && cube_status == 'p' {
                tree.push(Node::build(coord, width / 2.0, scene));
            } else if cube_status == 'f'  {
                tree.push(Node::Filled);
            } else {
                tree.push(Node::Empty);
            }
        }

        return Node::Group(tree);
    }
}

impl Octree {
    pub fn build(&mut self, scene: &Scene) {
        self.nodes = Node::build(self.root, self.width, scene);

    }

    pub fn value_at(&self, coordinate: Coordinate) -> (char, f64) {
        let mut reference = &self.nodes;
        let mut width = self.width;
        let mut root = self.root;
        
        loop {
            match reference {
                &Node::Filled => {
                    return ('f', width);
                },
                &Node::Empty => {
                    return ('e', width);
                },
                &Node::Group(ref group) => {
                    let vec = (coordinate - root) / width;
                    let index: i16 = vec[2].round() as i16 * 4 + vec[1].round() as i16 * 2 + vec[0].round() as i16;
                    root = root + corner(index) * width;
                    width = width / 2.0;
                    reference = &group[index as usize];
                },
            }
        }
    }

    pub fn coordinate_in_cube(&self, coordinate: Coordinate) -> bool {
        let relative = coordinate - self.root;
        return (relative.x >= 0.0 && relative.x <= self.width && 
        relative.y >= 0.0 && relative.y <= self.width && 
        relative.z >= 0.0 && relative.z <= self.width);
    }
}

pub fn new(width: f64, root: Coordinate) -> Octree {
    return Octree {
        root: root,
        width: width,
        nodes: Node::Filled
    }
}

fn split(root: &Coordinate, width: f64, scene: &Scene) -> char {
    //let mut filled = 0;
    //let mut empty = 0;

    return scene.value_at(&root, width);
    
    //let hw = width / 2.0; //half width
    //let center: Coordinate = *root + Coordinate::new(hw, hw ,hw);
    //let center_value = scene.value_at(&center);

    //if center_value {
    //    filled += 1;
    //} else {
    //    empty += 1;
    //}


    //for node in corners() {
    //    let node_corner = *root + (node * width);
    //    
    //    if scene.value_at(&node_corner) {
    //        filled += 1;
    //    } else {
    //        empty += 1;
    //    }

    //    if filled > 0 && empty > 0 {
    //        return 'p';
    //    }
    //}
    //if filled == 9 {
    //    return 'f'
    //} else {
    //    return 'e'
    //}
}


pub fn corners() -> Coordinates {
    return vec![
        Vec3::new(0.0, 0.0, 0.0),
        Vec3::new(0.5, 0.0, 0.0),
        Vec3::new(0.0, 0.5, 0.0),
        Vec3::new(0.5, 0.5, 0.0),

        Vec3::new(0.0, 0.0, 0.5),
        Vec3::new(0.5, 0.0, 0.5),
        Vec3::new(0.0, 0.5, 0.5),
        Vec3::new(0.5, 0.5, 0.5)
    ]
}

pub fn corner(index: i16) -> Coordinate {
    return corners()[index as usize];
}

