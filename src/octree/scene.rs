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
    pub depth: f64,
}

impl Cube {
    pub fn value_at(&self, tree_root: &Coordinate, tree_width: f64) -> char {
        let relative = (*tree_root - self.root);
        let root_inside = relative.x >= 0.0 && relative.x <= self.width &&
               relative.y >= 0.0 && relative.y <= self.height &&
               relative.z >= 0.0 && relative.z <= self.depth;

        //when the cube exceeds the tree_node's cube.
        if root_inside && 
            relative.x + tree_width < self.width && 
            relative.y + tree_width < self.height &&
            relative.z + tree_width < self.depth {
            return 'f'
        }


        //http://stackoverflow.com/questions/5009526/overlapping-cubes
        let cond1 = self.root.x + self.width < tree_root.x;
        let cond2 = tree_root.x + tree_width < self.root.x;
        let cond3 = self.root.y + self.height < tree_root.y;
        let cond4 = tree_root.y + tree_width < self.root.y;
        let cond5 = self.root.z + self.depth < tree_root.z;
        let cond6 = tree_root.z + tree_width < self.root.z;


        if cond1 || cond2 || cond3 || cond4 || cond5 || cond6 {
            return 'e'
        } else {
            return 'p'
        }
    }
}

pub struct Sphere {
    pub root: Coordinate,
    pub radius: f64,
}

impl Sphere {
    pub fn value_at(&self, coordinate: &Coordinate, width: f64) -> char {
        let min = *coordinate;
        let max = min + Coordinate::new(width, width, width);

        let mut d = 0.0;
        for axis in vec![0usize, 1, 2] {
            let e = self.root[axis] - min[axis];
            if e < 0.0 {
                if (e < -1.0 * self.radius) {
                    return 'e'
                }
                d += e * e;
            } else {
                let e = self.root[axis] - max[axis];
                if e > 0.0 {
                    if (e > self.radius) {
                        return 'e'
                    }
                    d += e * e;
                }
            }
        }

        for corner in super::octree::corners() {
            if ((corner * width + min) - self.root).norm() > self.radius {
                return 'p';
            }
        }

        return 'f';
    }
}

fn squared(value: f64) -> f64 {
    return value * value;
}


pub struct Scene {
    pub objects: Vec<Primitive>,
}

impl Scene {
    pub fn new() -> Scene{
        return Scene { objects: vec![] };
    }

    pub fn value_at(&self, coordinate: &Coordinate, width: f64) -> char {
        let mut positive = false;
        let mut negative = false;
        let mut partial = false;
        for object in self.objects.iter() {
            match object {
                &Primitive::Sphere(ref obj) => {
                    let response = obj.value_at(coordinate, width);

                    match response {
                        'f' => {
                            positive = true;   
                        },
                        'p' => {
                            partial = true;
                        },
                        _ => {}
                    }
                },
                &Primitive::Cube(ref obj) => {
                    let response = obj.value_at(coordinate, width);
                    match response {
                        'f' => {
                            positive = true;   
                        },
                        'p' => {
                            partial = true;
                        },
                        _ => {}
                    }
                }
            }
        }
        
        if positive {
            return 'f'
        }

        if partial {
            return 'p'
        }

        return 'e'
    }
}


