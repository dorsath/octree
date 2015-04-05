use super::octree::{Coordinate};
use nalgebra::*;
pub enum Primitive {
    Sphere(Sphere),
    Cube(Cube),
}

pub struct Cube {
    pub root: Coordinate,
    pub width: f64,
    pub height: f64,
    pub depth: f64
}

impl Cube {
    pub fn value_at(&self, coordinate: Coordinate) -> bool {
        let relative = (coordinate - self.root);
        return relative.x >= 0.0 && relative.x <= self.width &&
               relative.y >= 0.0 && relative.y <= self.height &&
               relative.z >= 0.0 && relative.z <= self.depth;
    }
}

pub struct Sphere {
    pub root: Coordinate,
    pub radius: f64
}

impl Sphere {
    pub fn value_at(&self, coordinate: Coordinate) -> bool {
        return (coordinate - self.root).norm() <= self.radius;
    }
}

pub struct Scene {
    pub objects: Vec<Primitive>,
}

impl Scene {
    pub fn new() -> Scene{
        return Scene { objects: vec![] };
    }

    pub fn value_at(&self, coordinate: Coordinate) -> bool {
        for object in self.objects.iter() {
            match object {
                &Primitive::Sphere(ref obj) => {
                    if obj.value_at(coordinate) {
                        return true;
                    }
                },
                &Primitive::Cube(ref obj) => {
                    if obj.value_at(coordinate) {
                        return true;
                    }
                }
            }
        }
        return false;
    }
}


