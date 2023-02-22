// TODO: move Data struct and related code that is currently in lib into a cfg(test) block. The crate should be a library, not a binary.
use cgmath::Vector3;
use std::fmt::{Debug, Display, Formatter};
use std::ops:: RangeInclusive;
use itertools::{ConsTuples, Product};

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
    fn pos_to_index(&self, v: Vector3<u32>) -> Option<usize> {
        if (v.x >= self.size_x as u32)  || (v.y >= self.size_y as u32)|| (v.z >= self.size_z as u32)
        {
            return  None;
        }
        return Some(v.x as usize + v.y as usize * self.size_x + v.z as usize * (self.size_x * self.size_y));


    }

    #[allow(dead_code)]
    fn pos_from_index(&self, v: usize) -> Vector3<usize> {
        let x = v % (self.size_x * self.size_y) % self.size_x;
        let y = v % (self.size_y * self.size_x) / self.size_x;
        let z = v / (self.size_y * self.size_x);
        let coordinates = Vector3::new(x, y, z);
        coordinates
    }
}

impl<D: Sized + Default + Debug + Display+ Clone> Debug for SpatialHashGrid<D> where SpatialHashGrid<D>:Hashing<D> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        //assert_eq!(self.size_z, 0);

        let mut slice_collection:Vec<Vec<Option<D>>> = Vec::new();

        for z in 0..self.size_z {

                for x in 0..self.size_x {
                    let mut slice:Vec<Option<D>> = Vec::new();
                    for y in 0..self.size_y {
                        let v = Vector3::new(x as u32, y as u32, z as u32);
                         let t = match self.pos_to_index(v) {
                            Some(t) =>  slice.push(self.cubes[t].clone()),
                            None => continue,
                        };
                    }
                    slice_collection.push(slice);
                }


            }
        for i in  slice_collection{
            for j in &i {

                let t = match j {
                            Some(t) =>  println!("{:?}", i),
                            None => println!("  |"),
                        };
            }
            println!("\n");
        }
        todo!()

    }
}
pub struct BoxIterator <'a, D: Sized + Default + Debug + Display>{
    data: &'a mut SpatialHashGrid<D>,
    iter: ConsTuples<Product<Product<RangeInclusive<u32>, RangeInclusive<u32>>, RangeInclusive<u32>>, ((u32, u32), u32)>,
}

impl  <'a, D: Sized + Default + Debug + Display+ Clone> Iterator for BoxIterator<'a, D>
{
    type Item = &'a mut D;

    #[inline]
    fn next(&mut self) -> Option<Self::Item> {
        loop {
            let (x, y, z) = self.iter.next()?;
            let c = Vector3 { x, y, z };
            let idx = match self.data.pos_to_index(c) {
                Some(idx)=>idx,
                None=>{continue;}
            };
            unsafe {
                match self.data.cubes.get_mut(idx){
                    Some(cube) => {
                        match cube {
                            Some(d) => {
                                let d = d as *mut D;
                                return d.as_mut();
                            },
                            None => {}
                        }
                    }
                    None => {}
                }
            }
        }
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        todo!()
    }

}
impl<D: Sized + Default + Debug + Display + Clone> Hashing<D> for SpatialHashGrid<D> {
    fn fill_cube(&mut self, v: Vector3<u32>) {
        let i = self.pos_to_index(v).expect(format!("Position {:?} out of bounds!", v).as_str());
        self.cubes[i] = Some(D::default());
    }

    fn get_cube_mut(&mut self, v: Vector3<u32>) -> Option<&mut D> {
        let i = self.pos_to_index(v)?;
        match self.cubes.get_mut(i) {
            Some(t) => t.as_mut(),
            None => None,
        }
    }

    fn get_cube(&self, v: Vector3<u32>) -> Option<&D> {
        let i = self.pos_to_index(v)?;
        match self.cubes.get(i) {
            Some(t) => t.as_ref(),
            None => None,
        }
    }

    fn iter_filled_boxes_mut(&mut self, min: Vector3<u32>, max: Vector3<u32>) -> BoxIterator<D> {
        BoxIterator{
            data:  self,
            iter: itertools::iproduct!( min.x..=max.x ,min.y..=max.y ,min.z..=max.z )
        }
    }


}

pub trait Hashing<D: std::default::Default+Debug+Display+ Clone> {
    fn fill_cube(&mut self, v: Vector3<u32>);
    fn get_cube_mut(&mut self, v: Vector3<u32>) -> Option<&mut D>;
    fn get_cube(&self, v: Vector3<u32>) -> Option<&D>;
    fn iter_filled_boxes_mut(&mut self, min: Vector3<u32>, max: Vector3<u32>) -> BoxIterator<D> ;
}

#[cfg(test)]
mod tests {
    use std::fmt;
    use crate::{Hashing, SpatialHashGrid};
    use cgmath::Vector3;
    use std::fmt::{Debug, Display, Formatter};

    #[derive(Debug, Clone)]
    pub struct Data {
        x: u32,
        y: u32,
        z: u32,
    }

    impl Default for Data {
        fn default() -> Self {
            Data { x: 0, y:0, z:0 }
        }
    }

    impl Display for Data {
        fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
            f.write_str(&format!("{} {} {}", self.x, self.y, self.z))
        }
    }

    #[test]
    fn test_iter_boxes_mut() {
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

        for j in 0..5 {
            for k in 0..5 {
                for z in 0..5 {
                    let pos = Vector3::new(j, k, z);
                    sh.fill_cube(pos);
                    if let Some(cube) = sh.get_cube_mut(pos) {
                        cube.x = j;
                        cube.y = k;
                        cube.z = z;
                    } else {
                        panic!("WAAAT");
                    }
                }
            }
        }




        let min = Vector3::new(0, 0, 0);
        let max = Vector3::new(5, 5, 5);

        for i in  sh.iter_filled_boxes_mut(min, max){

        }
        let p = Vector3 { x: 3, y: 3, z: 3 };
        sh.fill_cube(p);
        sh.get_cube_mut(p).unwrap().x = 6666666;


        format!("{:?}",sh);


    }
}

