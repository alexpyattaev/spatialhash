// TODO: move Data struct and related code that is currently in lib into a cfg(test) block. The crate should be a library, not a binary.
use cgmath::Vector3;
use std::fmt::{Debug, Display, Formatter};

pub struct SpatialHashGrid<D: Sized + Default + Debug + Display> {
    size_x: usize,
    size_y: usize,
    size_z: usize,
    cubes: Vec<Option<D>>,
}

impl<D: Sized + Default + Debug + Display + Clone> SpatialHashGrid<D> {
    pub fn new(x: usize, y: usize, z: usize) -> Self {
        let cap = x * y * z;
        let mut d = Vec::with_capacity(cap);
        d.resize_with(cap, || None);
        Self {
            size_x: x,
            size_y: y,
            size_z: z,
            cubes: d,
        }
    }
    fn pos_to_index(&self, v: Vector3<u32>) -> usize {
        v.x as usize + v.y as usize * self.size_x + v.z as usize * (self.size_x * self.size_y)
    }

    fn pos_from_index(&self, v: usize) -> Vector3<usize> {
        let x = v % (self.size_x * self.size_y) % self.size_x;
        let y = v % (self.size_y * self.size_x) / self.size_x;
        let z = v / (self.size_y * self.size_x);
        let coordinates = Vector3::new(x, y, z);
        coordinates
    }
}
impl<D: Sized + Default + Debug + Display> Debug for SpatialHashGrid<D> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        assert_eq!(self.size_z, 0);
        //TODO make this work & make test
        //for y in 0..self.size_y {
        //         for x in 0..self.size_x {
        //             let v = Vector3::new(x as u32, y as u32, 0);
        //             match &self.cubes[self.pos_to_index(v)] {
        //                 Some(t) => print!("{}|", t),
        //                 None => print!("  |"),
        //             }
        //         }
        //         print!("\n");
        //     }
        todo!()
    }
}

impl<D: Sized + Default + Debug + Display + Clone> Hashing<D> for SpatialHashGrid<D> {
    fn fill_cube(&mut self, v: Vector3<u32>) {
        let i = self.pos_to_index(v);
        self.cubes[i] = Some(D::default());
    }
    fn get_cube_mut(&mut self, v: Vector3<u32>) -> Option<&mut D> {
        let i = self.pos_to_index(v);
        match self.cubes.get_mut(i) {
            Some(t) => t.as_mut(),
            None => None,
        }
    }

    fn get_cube(&self, v: Vector3<u32>) -> Option<&D> {
        let i = self.pos_to_index(v);
        match self.cubes.get(i) {
            Some(t) => t.as_ref(),
            None => None,
        }
    }

    fn collect_filled_data(&mut self, min: Vector3<u32>, max: Vector3<u32>) -> Vec<D> {
        let mut collection: Vec<D> = vec![];
        for x in min.x..max.x{
            for y in min.y..max.y{
                for z in min.z..max.z {
                    let c = Vector3{x,y,z};
                     match &self.cubes[self.pos_to_index(c)] {
                        Some(t) => { collection.push(t.clone());},
                        None => {},
                    }
                }
            }
        }
        return collection;
    }
}

pub trait Hashing<D> {
    fn fill_cube(&mut self, v: Vector3<u32>);
    fn get_cube_mut(&mut self, v: Vector3<u32>) -> Option<&mut D>;
    fn get_cube(&self, v: Vector3<u32>) -> Option<&D>;
    fn collect_filled_data(&mut self, min: Vector3<u32>, max: Vector3<u32>) -> Vec<D>;
}

#[cfg(test)]
mod tests {
    use crate::{Hashing, SpatialHashGrid};
    use cgmath::Vector3;
    use std::fmt::{Display, Formatter};

    #[derive(Debug, Clone)]
    pub struct Data {
        some_data: u32,
    }

    impl Default for Data {
        fn default() -> Self {
            Data { some_data: 0 }
        }
    }

    impl Display for Data {
        fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
            f.write_str(&format!("{}", self.some_data))
        }
    }

    #[test]
    fn main_test() {
        // let mut sh: SpatialHashGrid<Data> = SpatialHashGrid::new(10, 8, 6);
        // for j in 0..10 {
        //     let pos = Vector3::new(j, j, 0);
        //     sh.fill_cube(pos);
        //     if let Some(cube) = sh.get_cube_mut(pos) {
        //         cube.some_data = (33 + j) as i8;
        //     } else {
        //         panic!("WAAAT");
        //     }
        // }
        let mut sh: SpatialHashGrid<Data> = SpatialHashGrid::new(5, 5, 5);

        let mut count = 0;
        for j in 0..5 {
            for k in 0..5 {
                for z in 0..5 {
                    let pos = Vector3::new(j, k, z);
                    sh.fill_cube(pos);
                    if let Some(cube) = sh.get_cube_mut(pos) {
                        cube.some_data = count;
                        count += 1;
                    } else {
                        panic!("WAAAT");
                    }
                }
            }
        }

        let min = Vector3::new(3, 3, 3);
        let max = Vector3::new(5, 5, 4);

        let p = Vector3 { x: 3, y: 3, z: 3 };
        sh.fill_cube(p);
        sh.get_cube_mut(p).unwrap().some_data = 6666666;
        //sh.print();

        //sh.get_filled_cubes_in_box_mut(min, max);
        let a = sh.collect_filled_data(min, max);
        for i in a {
            println!("{:?}", i);
        }
    }
}
//TODO: add a performance benchmark with criterion.rs
