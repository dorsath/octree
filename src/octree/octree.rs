extern crate nalgebra;

use nalgebra::*;
use std::num::Float;

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
    pub value_function: fn(&Coordinate) -> bool,
}

pub type Coordinate  = Vec3<f64>;
pub type Coordinates = Vec<Coordinate>;

//type Coordinates = Vec<Coordinate>;

impl Node {
    pub fn build(root: Coordinate, width: f64, value_function: fn(&Coordinate) -> bool) -> Node {
        let mut tree: Vec<Node> = Vec::new();

        for corner in corners().iter() {
            let coord = *corner * width + root;
            let cube_status = split(&coord, width, value_function);
            if width >= 0.4 && cube_status == 'p' {
                tree.push(Node::build(coord, width / 2.0, value_function));
            } else if cube_status == 'f'  {
                tree.push(Node::Filled);
            } else {
                tree.push(Node::Empty);
            }
        }

        return Node::Group(tree);
    }

    pub fn find_value_at(node: &Node, coord: Coordinate, root: Coordinate, width: f64) -> (char, f64) {
        match node {
            &Node::Filled => {
                return ('f', width);
            },
            &Node::Empty => {
                return ('e', width);
            },
            &Node::Group(ref group) => {
                let mut vec = (coord - root) / width;
                let index: i16 = vec[2].round() as i16 * 4 + vec[1].round() as i16 * 2 + vec[0].round() as i16;
                if index < 0 || index > 7 {
                    return ('e', width);
                }
                let new_root = root + corner(index) * width;
                return Node::find_value_at(&group[index as usize], coord, new_root, width / 2.0);

                return ('e', width);
            },

        }
    }
}

impl Octree {
    pub fn build(&mut self) {
        self.nodes = Node::build(self.root, self.width, self.value_function);

    }

    pub fn value_at(&self, coordinate: Coordinate) -> (char, f64) {
        return Node::find_value_at(&self.nodes, coordinate, self.root, self.width);
    }
}

pub fn new(width: f64, root: Coordinate, value_function: fn(&Coordinate) -> bool) -> Octree {
    return Octree {
        root: root,
        width: width,
        nodes: Node::Filled,
        value_function: value_function      
    }
}

fn split(root: &Coordinate, width: f64, value_function: fn(&Coordinate) -> bool) -> char {
    let mut filled = 0;
    let mut empty = 0;

    let hw = width / 2.0; //half width
    let center: Coordinate = *root + Coordinate::new(hw, hw ,hw);
    let center_value = value_function(&center);

    if center_value {
        filled += 1;
    } else {
        empty += 1;
    }


    for node in corners() {
        let node_corner = *root + (node * width);
        
        if value_function(&node_corner) {
            filled += 1;
        } else {
            empty += 1;
        }

        if filled > 0 && empty > 0 {
            return 'p';
        }
    }
    if filled == 9 {
        return 'f'
    } else {
        return 'e'
    }
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

