extern crate nalgebra;

use octree::nalgebra::Vec3 as Vec3;

#[derive(Debug, Clone)]
pub enum Node {
    Group(Vec<Node>),
    Value(f64),

}

pub struct Octree {
    pub root: Coordinate,
    pub width: f64,
    pub nodes: Node,
    pub value_function: fn(&Coordinate) -> f64,
}

pub type Coordinate  = Vec3<f64>;
pub type Coordinates = Vec<Coordinate>;

//type Coordinates = Vec<Coordinate>;

impl Node {
    pub fn build(root: Coordinate, width: f64, value_function: fn(&Coordinate) -> f64) -> Node {
        let qw = width / 4.0; //quarter width
        let mut tree: Vec<Node> = Vec::new();

        for corner in corners().iter() {
            let coord = *corner * width + root;
            if width > 0.002 && split(&coord, width, value_function) {
                tree.push(Node::build(coord, width / 2.0, value_function));
            } else {
                let center = coord + Coordinate::new(qw, qw ,qw);
                let val = value_function(&center);
                tree.push(Node::Value(val));
            }
        }

        return Node::Group(tree);
    }
}

impl Octree {
    pub fn build(&mut self) {
        self.nodes = Node::build(self.root, self.width, self.value_function);

    }
}

pub fn new(width: f64, value_function: fn(&Coordinate) -> f64) -> Octree {
    let root = Coordinate::new(0.0, 0.0, 0.0);

    return Octree {
        root: root,
        width: width,
        nodes: Node::Value(0.0),
        value_function: value_function      
    }
}

//pub fn build(width: f64) -> Octree{
//    let root = Coordinate::new(0.0, 0.0, 0.0);
//    return Octree { width: width, nodes: Node::build(root, width) }
//}


fn split(root: &Coordinate, width: f64, value_function: fn(&Coordinate) -> f64) -> bool {
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

