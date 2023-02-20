use cgmath::Vector3;
use std::cmp::min;
use std::fmt::{Debug, Display, Formatter};
use std::slice::Iter;
use std::thread::current;

#[derive(Debug, Clone)]
pub struct Data {
    some_data: i8,
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

struct SpatialHashGrid<D: Sized + Default + Debug + Display> {
    size_x: usize,
    size_y: usize,
    size_z: usize,
    cubes: Vec<Option<D>>,
}

impl<D: Sized + Default + Debug + Display + Clone> Voxelization<D> for SpatialHashGrid<D> {
    fn new(x: usize, y: usize, z: usize) -> Self {
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

    fn pos_to_index(&self, v: Vector3<u32>) -> usize {
        v.x as usize + v.y as usize * self.size_x + v.z as usize * (self.size_x * self.size_y)
    }

    //For debug
    fn print(&self) {
        for y in 0..self.size_y {
            for x in 0..self.size_x {
                let v = Vector3::new(x as u32, y as u32, 0);
                match &self.cubes[self.pos_to_index(v)] {
                    Some(t) => print!("{}|", t),
                    None => print!("  |"),
                }
            }
            print!("\n");
        }
    }

    fn collect_filled_data(&mut self) -> Vec<D> {
        let d = self
            .cubes
            .iter()
            .flat_map(|data| data.iter())
            .cloned()
            .collect::<Vec<D>>();
        d
    }
}

pub trait Voxelization<D> {
    // fn iter_mut_stuff(&mut self, min: Vector3<u32>, max: Vector3<u32>) -> Vec<&D>
    fn new(x: usize, y: usize, z: usize) -> Self;
    fn fill_cube(&mut self, v: Vector3<u32>);
    fn get_cube_mut(&mut self, v: Vector3<u32>) -> Option<&mut D>;
    fn get_cube(&self, v: Vector3<u32>) -> Option<&D>;
    fn pos_to_index(&self, v: Vector3<u32>) -> usize;
    fn print(&self);
    fn collect_filled_data(&mut self) -> Vec<D>;
}

// impl IntoIterator for Data {
//     type Item = i8;
//     type IntoIter = DataIntoIterator;
//
//     fn into_iter(self) -> Self::IntoIter {
//         DataIntoIterator {
//             data: self,
//         }
//     }
// }
//
// pub struct DataIntoIterator {
//     data: Data,
// }
//
// impl Iterator for DataIntoIterator {
//     type Item = i8;
//     fn next(&mut self) -> Option<i8> {
//         let result = match &self.data {
//             Data => self.data.some_data,
//             _ => return None,
//         };
//
//         Some(result)
//     }
// }

fn main() {
    let mut sh: SpatialHashGrid<Data> = SpatialHashGrid::new(10, 10, 2);
    for j in 0..10 {
        let pos = Vector3::new(j, j, 0);
        sh.fill_cube(pos);
        if let Some(cube) = sh.get_cube_mut(pos) {
            cube.some_data = (33 + j) as i8;
        } else {
            panic!("WAAAT");
        }
    }

    let min = Vector3::new((0), (0), (0));
    let max = Vector3::new((5), (5), (0));
    //sh.print();
    //sh.get_filled_cubes_in_box_mut(min, max);
    let a = sh.collect_filled_data();

    println!("{:?}", a);
}
