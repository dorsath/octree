extern crate nalgebra;
pub mod octree;

use octree::octree::*;
use octree::raycasting::{Pixel, Vector};

use nalgebra::*;

fn val_func(coord: &Coordinate) -> bool {
    let root = Coordinate::new(5.0, 5.0, 5.0);
    let range = 3.0;
    let norm = (root - *coord).norm();
    return norm < range;
}

fn debug_out(tree: Octree) {
    match tree.nodes {
        Node::Filled => {
            println!("filled");
        }
        Node::Group(x) => {
            for node in x {
                println!("{:?}", node);
            }
        }
        Node::Empty => {
        }
    }
}

fn points_out(node: Node, root: Coordinate, width: f64) {
    match node {
        Node::Filled => {
            let hw = width / 2.0;
            println!("{:?}\t{:?}\t{:?}", root.x + hw, root.y + hw, root.z + hw);
        }
        Node::Group(x) => {
            let mut n = 0;
            for subnode in x {
                let coord = corners()[n] * width + root;
                points_out(subnode, coord, width / 2.0);
                n += 1;
            }
        }
        Node::Empty => {
        }
    }
}

fn main() {
    let width = 10.0f64;
    let root = Coordinate::new(0.0, 0.0, 0.0);

    let mut tree: Octree = octree::octree::new(width, root, val_func);
    tree.build();

    let pixel = Pixel { 
        normal: Vector::new(0.0, 0.0, 1.0),
        point: Coordinate::new(7.0, 7.0, -5.0)
    };

    let a = octree::raycasting::build(&tree, pixel, width, root);

    println!("{:?}", a);

    //let node = tree.nodes.clone();
    //points_out(node, tree.root, tree.width);
    //debug_out(tree);
}
