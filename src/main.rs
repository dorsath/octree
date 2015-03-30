extern crate nalgebra;

pub mod octree {
    use nalgebra::*;

    #[derive(Debug)]
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
                println!("{:?}", coord);
                if width > 0.2 && split(&coord, width, value_function) {
                    tree.push(Node::build(coord, width / 2.0, value_function));
                } else {
                    let center = coord + Coordinate::new(qw, qw ,qw);
                    let val = value_function(&center);
                    tree.push(Node::Value(val));
                }
                
                tree.push(Node::Value(2.0));
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


    fn corners() -> Coordinates {
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


}

use octree::*;
use nalgebra::*;

fn val_func(coord: &Coordinate) -> f64 {
    let objects = vec!(
        //Coordinate::new(0.0, 0.5, 0.0),
        Coordinate::new(0.0, 0.0, 0.0)
    );

    let mut value = 0.0f64;
    for obj in objects {
        let norm = (obj - *coord).norm();
        if norm == 0.0 {
            value += 1_000f64;
        } else {
            value += 1.0f64 / norm / norm;
        }
    }
    return value;
}

fn debug_out(tree: Octree) {
    match tree.nodes {
        Node::Value(x) => {
            println!("{:?}", x);
        }
        Node::Group(x) => {
            for node in x {
                println!("{:?}", node);
            }
        }
    }
}

fn main() {
    let width = 1.0f64;
    let mut tree: Octree = octree::new(width, val_func);
    tree.build();

    debug_out(tree);
}
