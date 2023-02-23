// TODO: move Data struct and related code that is currently in lib into a cfg(test) block. The crate should be a library, not a binary.
use cgmath::Vector3;
use itertools::{ConsTuples, Product};
use std::fmt::{Debug, Formatter};
use std::ops::RangeInclusive;

pub struct SpatialHashGrid<D: Sized> {
    size_x: usize,
    size_y: usize,
    size_z: usize,
    cubes: Vec<Option<D>>,
}

impl<D: Sized> SpatialHashGrid<D> {
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
        if (v.x >= self.size_x as u32) || (v.y >= self.size_y as u32) || (v.z >= self.size_z as u32)
        {
            return None;
        }
        return Some(
            v.x as usize + v.y as usize * self.size_x + v.z as usize * (self.size_x * self.size_y),
        );
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

impl<D: Debug> Debug for SpatialHashGrid<D>
where
    SpatialHashGrid<D>: Hashing<D>,
{
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "<SpatialHashGrid {}x{}x{}:\n",
            self.size_x, self.size_y, self.size_z
        )?;
        for z in 0..self.size_z {
            write!(f, "#Slice z={}:\n", z)?;
            for x in 0..self.size_x {
                for y in 0..self.size_y {
                    let v = Vector3::new(x as u32, y as u32, z as u32);
                    let idx = self.pos_to_index(v).expect("This can not go wrong");
                    match &self.cubes[idx] {
                        Some(t) => write!(f, "{:?}, ", t)?,
                        None => write!(f, "None, ")?,
                    };
                }
                write!(f, "\n")?; // finish row in a slice
            }
        }
        write!(f, ">")
    }
}

pub struct BoxIteratorMut<'a, D: Sized> {
    data: &'a mut SpatialHashGrid<D>,
    iter: ConsTuples<
        Product<Product<RangeInclusive<u32>, RangeInclusive<u32>>, RangeInclusive<u32>>,
        ((u32, u32), u32),
    >,
}

impl<'a, D: Sized> Iterator for BoxIteratorMut<'a, D> {
    type Item = &'a mut D;

    #[inline]
    fn next(&mut self) -> Option<Self::Item> {
        loop {
            let (x, y, z) = self.iter.next()?;
            let c = Vector3 { x, y, z };
            let idx = match self.data.pos_to_index(c) {
                Some(idx) => idx,
                None => {
                    continue;
                }
            };
            unsafe {
                match self.data.cubes.get_mut(idx) {
                    Some(cube) => match cube {
                        Some(d) => {
                            let d = d as *mut D;
                            return d.as_mut();
                        }
                        None => {}
                    },
                    None => {}
                }
            }
        }
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        let (_min, max) = self.iter.size_hint();
        (0, max)
    }
}

pub struct BoxIterator<'a, D: Sized> {
    data: &'a SpatialHashGrid<D>,
    iter: ConsTuples<
        Product<Product<RangeInclusive<u32>, RangeInclusive<u32>>, RangeInclusive<u32>>,
        ((u32, u32), u32),
    >,
}

impl<'a, D: Sized> Iterator for BoxIterator<'a, D> {
    type Item = &'a D;

    #[inline]
    fn next(&mut self) -> Option<Self::Item> {
        loop {
            let (x, y, z) = self.iter.next()?;
            let c = Vector3 { x, y, z };
            let idx = match self.data.pos_to_index(c) {
                Some(idx) => idx,
                None => {
                    continue;
                }
            };

            match self.data.cubes.get(idx) {
                Some(cube) => match cube {
                    Some(d) => {
                        return Some(d);
                    }
                    None => {}
                },
                None => {}
            }
        }
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        let (_min, max) = self.iter.size_hint();
        (0, max)
    }
}

impl<D: Sized> Hashing<D> for SpatialHashGrid<D> {
    fn fill_cube(&mut self, v: Vector3<u32>, filler: fn() -> D) {
        let i = self
            .pos_to_index(v)
            .expect(format!("Position {:?} out of bounds!", v).as_str());
        self.cubes[i] = Some(filler());
    }

    fn clear_cube(&mut self, v: Vector3<u32>) {
        let i = self
            .pos_to_index(v)
            .expect(format!("Position {:?} out of bounds!", v).as_str());
        self.cubes[i] = None;
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

    fn iter_filled_boxes(&self, min: Vector3<u32>, max: Vector3<u32>) -> BoxIterator<D> {
        BoxIterator {
            data: self,
            iter: itertools::iproduct!(min.x..=max.x, min.y..=max.y, min.z..=max.z),
        }
    }

    fn iter_filled_boxes_mut(&mut self, min: Vector3<u32>, max: Vector3<u32>) -> BoxIteratorMut<D> {
        BoxIteratorMut {
            data: self,
            iter: itertools::iproduct!(min.x..=max.x, min.y..=max.y, min.z..=max.z),
        }
    }
}

pub trait Hashing<D: Sized> {
    fn fill_cube(&mut self, v: Vector3<u32>, filler: fn() -> D);
    fn clear_cube(&mut self, v: Vector3<u32>);
    fn get_cube_mut(&mut self, v: Vector3<u32>) -> Option<&mut D>;
    fn get_cube(&self, v: Vector3<u32>) -> Option<&D>;
    fn iter_filled_boxes(&self, min: Vector3<u32>, max: Vector3<u32>) -> BoxIterator<D>;
    fn iter_filled_boxes_mut(&mut self, min: Vector3<u32>, max: Vector3<u32>) -> BoxIteratorMut<D>;
}

#[cfg(test)]
mod tests {
    use crate::{Hashing, SpatialHashGrid};
    use cgmath::Vector3;
    use std::fmt;
    use std::fmt::{Debug, Display, Formatter};

    #[derive(Debug, Clone)]
    pub struct Data {
        x: u32,
        y: u32,
        z: u32,
    }

    impl Default for Data {
        fn default() -> Self {
            Data { x: 0, y: 0, z: 0 }
        }
    }

    impl Display for Data {
        fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
            f.write_str(&format!("{} {} {}", self.x, self.y, self.z))
        }
    }

    #[test]
    fn test_iter_boxes_mut() {
        let mut sh: SpatialHashGrid<Data> = SpatialHashGrid::new(5, 5, 5);

        for j in 0..5 {
            for k in 0..5 {
                for z in 0..5 {
                    let pos = Vector3::new(j, k, z);
                    sh.fill_cube(pos, Data::default);
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

        let min = Vector3::new(5, 5, 5);
        let max = Vector3::new(5, 5, 5);

        for i in sh.iter_filled_boxes_mut(min, max) {}
        // let p = Vector3 { x: 3, y: 3, z: 3 };
        // sh.fill_cube(p, Data::default);
        // sh.get_cube_mut(p).unwrap().x = 6666666;
        

        println!("{:?}", sh);
    }
    #[test]
    fn test_iter_boxes(){

    }
}
