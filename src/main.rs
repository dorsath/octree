extern crate nalgebra;
pub mod octree;

use octree::{Coordinate, Octree, Node};

use nalgebra::*;

fn val_func(coord: &Coordinate) -> f64 {
    let objects = vec!(
        //Coordinate::new(0.0, 0.5, 0.0),
        Coordinate::new(0.5, 0.0, 0.5)
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

fn points_out(node: Node, root: Coordinate, width: f64) {
    match node {
        Node::Value(x) => {
            if root.y == 0.0 {
                println!("{:?}\t{:?}\t{:?}\t{:?}", root.x, root.y, root.z, x);
            }
        }
        Node::Group(x) => {
            let mut n = 0;
            for subnode in x {
                let coord = octree::corners()[n] * width + root;
                points_out(subnode, coord, width / 2.0);
                n += 1;
            }
        }
    }
}

fn main() {
    let width = 1.0f64;
    let mut tree: Octree = octree::new(width, val_func);
    tree.build();

    let node = tree.nodes.clone();
    points_out(node, tree.root, tree.width);
    //debug_out(tree);
}
