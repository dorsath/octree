extern crate nalgebra;
extern crate image;
pub mod octree;

use octree::octree::*;
use octree::scene::*;
use octree::render::{image_out};


#[allow(dead_code)]
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

#[allow(dead_code)]
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
    let mut scene = Scene::new();


    let obj = Sphere { root: Coordinate::new(5.0, 5.0, 5.0), radius: 2.0 };
    scene.objects.push(Primitive::Sphere(obj));

    //let obj = Sphere { root: Coordinate::new(5.0, 5.0, 5.0), radius: 3.0, add: true };
    //scene.objects.push(Primitive::Sphere(obj));

    let obj = Cube { root: Coordinate::new(2.0, 2.0, 5.0), width: 6.0, height: 6.0, depth: 6.0};
    scene.objects.push(Primitive::Cube(obj));
    

    //let obj = Sphere { root: Coordinate::new(7.0, 7.0, 5.0), radius: 1.5 };
    //scene.objects.push(Primitive::Sphere(obj));

    //let a = scene.value_at(&Coordinate::new(2.0, 2.0, 2.0));
    //println!("{:?}", a);
        


    let mut tree: Octree = octree::octree::new(width, root);
    tree.build(&scene);
    //let coord = Coordinate::new(2.5, 2.5, 2.5);
    //let a = scene.value_at(&coord, 1.25);
    //println!("{:?}", a);


    //points_out(tree.nodes, root, width);

    image_out(&tree, 20.0, 1.22);

}
