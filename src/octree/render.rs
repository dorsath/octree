extern crate image;

use nalgebra::*;
use std::fs::File;
use std::path::Path;
use std::num::Float;
use octree::octree::{Octree, Coordinate};
use octree::raycasting::{Pixel, build};


#[allow(unused_must_use)]
pub fn image_out(tree: &Octree, distance: f64, angle: f64) {
    let img_width = 400;
    let img_height = 300;

    //config
    let camera_aim = Coordinate::new(5.0, 5.0, 5.0);
    let field_of_view = 60.0; //deg
    
    //calcs
    let camera_origin = camera_aim + rotate_vec(&Coordinate::new(0.0, 0.0, -1.0), angle) * distance;
    let z = (img_width as f64 / 2.0) / Float::tan( (field_of_view / 2.0) / 360.0 * 2.0 * 3.14 );//img_width as f64) / 2.0 ;
    println!("{:?}", camera_origin);
    
    let mut imgbuf = image::ImageBuffer::new(img_width, img_height);
    for (img_x, img_y, img_pixel) in imgbuf.enumerate_pixels_mut() {
        let x = (img_width as f64) /2.0 -  img_x as f64;
        let y = (img_height as f64) /2.0 -  img_y as f64;
        let normal = Coordinate::new(x,y,z).normalize();
        let normal = rotate_vec(&normal, angle);
        //print!("pixel: {:?}:{:?}\r", img_x, img_y);

        let pixel = Pixel { 
            normal: normal,
            point: camera_origin
        };
        let a = super::raycasting::build(&tree, pixel);


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

