extern crate nalgebra;

type T = f64;



mod octree {
    use nalgebra::{Vec3};

    type Octree = Vec<Vec3<f64>>;

    pub fn init(){
        let result: Octree = corners();
        for x in result.iter() {
            println!("{:?}", *x * 0.5);
        }
    }


    fn corners() -> Octree {
        let mut corners = Octree::new();

        for n in 0..8 {
            corners.push(coordinate_of_index(n));
        }

        return corners;
    }

    fn coordinate_of_index(index: u32) -> Vec3<f64> {
        return Vec3::new(
            (index % 2) as f64, 
            ((index / 2) % 2) as f64,
            ((index / 4) % 2) as f64
        )
    }

}


fn main() {
    octree::init()
}

