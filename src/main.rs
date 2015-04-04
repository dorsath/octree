extern crate nalgebra;
extern crate image;
pub mod octree;

use octree::octree::*;
use octree::raycasting::{Pixel, Vector};

use std::num::Float;
use nalgebra::*;
use std::fs::File;
use std::path::Path;

use image::GenericImage;


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

fn image_out(tree: &Octree) {
    let width = tree.width;
    let root = tree.root;
    let img_width = 400;
    let img_height = 300;

    let ratio = img_height as f64 / img_width as f64;
    let screen_position = Coordinate::new(5.0, 5.0, 0.0);
    let camera_position = Coordinate::new(5.0, 5.0, -10.0);
    let field_of_view = 60.0; //deg
    let dx =  (Float::sin(field_of_view / 2.0) * (camera_position.z - screen_position.z) * 2.0) / img_width as f64;
    let dy = dx; 
    
    let mut imgbuf = image::ImageBuffer::new(img_width, img_height);
    
    for (img_x, img_y, img_pixel) in imgbuf.enumerate_pixels_mut() {
        let x = img_x as f64 - (img_width as f64)/2.0;
        let y = img_y as f64 - (img_height as f64)/2.0;

        let point   = screen_position + Coordinate::new(dx * (x as f64), dy * (y as f64), 0.0);
        let normal  = (camera_position - point).normalize();

        let pixel = Pixel { 
            normal: normal,
            point: point
        };
        //println!("{:?}\t{:?}\t{:?}", normal.x, normal.y, normal.z);
        //println!("{:?} {:?}", normal, point);
        let a = octree::raycasting::build(&tree, pixel, width, root);
        //println!("{:?} {:?} {:?}", normal, point, a);

        *img_pixel = image::Luma([a]);
    }

    let ref mut fout = File::create(&Path::new("octree.png")).unwrap();
    image::ImageLuma8(imgbuf).save(fout, image::PNG);
    
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
    
    image_out(&tree);

    
    
   
    //Vec3 { x: 0, y: 0, z: 1 } Vec3 { x: 5, y: 5, z: -20 } false
    //Vec3 { x: 0.258776, y: -0.017644, z: 0.965776 } Vec3 { x: 9.01919, y: 4.725964, z: -20 }
    //let pixel = Pixel { 
    //    normal: Vector::new(0.258776, -0.017644, 0.965776),
    //    point: Coordinate::new(9.01919, 4.725964, -20.0)
    //};

    //let a = octree::raycasting::build(&tree, pixel, width, root);

    //println!("{:?}", a);

    //let node = tree.nodes.clone();
    //points_out(node, tree.root, tree.width);
    //debug_out(tree);
}
