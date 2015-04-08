extern crate nalgebra;
extern crate image;
pub mod octree;

use octree::octree::*;
use octree::raycasting::{Pixel, Vector};
use octree::scene::*;

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

fn image_out(tree: &Octree, distance: f64, angle: f64) {
    let img_width = 400;
    let img_height = 300;

    //config
    let camera_aim = Coordinate::new(5.0, 5.0, 5.0);
    let field_of_view = 60.0; //deg
    
    //calcs
    let ratio = img_height as f64 / img_width as f64;
    let camera_origin = camera_aim + rotate_vec(&Coordinate::new(0.0, 0.0, -1.0), angle) * distance;
    let deg_per_pixel = field_of_view / img_width as f64;
    let z = (img_width as f64 / 2.0) / Float::tan( (field_of_view / 2.0) / 360.0 * 2.0 * 3.14 );//img_width as f64) / 2.0 ;
    println!("{:?}", camera_origin);
    
    let mut imgbuf = image::ImageBuffer::new(img_width, img_height);
    for (img_x, img_y, img_pixel) in imgbuf.enumerate_pixels_mut() {
        let x = (img_width as f64) /2.0 -  img_x as f64;
        let y = (img_height as f64) /2.0 -  img_y as f64;
        let normal = Coordinate::new(x,y,z).normalize();
        let normal = rotate_vec(&normal, angle);
        println!("{:?}\t{:?}\t{:?}", normal.x, normal.y, normal.z);
        //print!("pixel: {:?}:{:?}\r", img_x, img_y);

        let pixel = Pixel { 
            normal: normal,
            point: camera_origin
        };
        let a = octree::raycasting::build(&tree, pixel);


        *img_pixel = image::Luma([a]);
    }

    let ref mut fout = File::create(&Path::new("octree.png")).unwrap();
    image::ImageLuma8(imgbuf).save(fout, image::PNG);
}

fn rotate_vec(coordinate: &Coordinate, angle: f64) -> Coordinate {
    return Coordinate::new(
            Float::sin(angle) * coordinate.z + Float::cos(angle) * coordinate.x ,
            coordinate.y,
            Float::cos(angle) * coordinate.z + Float::sin(angle) * coordinate.x
        ).normalize();
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
