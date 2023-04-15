use cgmath::Vector3;
use itertools::{ConsTuples, Product};
use std::fmt::{Debug, Formatter};
use std::ops::RangeInclusive;


pub struct SpatialHashGrid<D: Sized> {
    dims: Vector3<usize>,
    cubes: Vec<D>,
}

#[inline]
fn pos_to_index(dims: Vector3<usize>, v: Vector3<u32>) -> Option<usize> {
    if (v.x >= dims[0] as u32) || (v.y >= dims[1] as u32) || (v.z >= dims[2] as u32)
    {
        return None;
    }
    Some(
        v.x as usize + v.y as usize * dims[0] + v.z as usize * (dims[0] * dims[1]),
    )
}

impl<D: Sized> SpatialHashGrid<D> {
    /// todo
    pub fn new(x: usize, y: usize, z: usize, filler: fn() -> D) -> Self {
        let cap = x * y * z;
        // allocate memory
        let mut d = Vec::with_capacity(cap);
        // initialize elements
        d.resize_with(cap, filler);
        Self {
            dims: Vector3::new(x, y, z),
            cubes: d,
        }
    }

    #[inline]
    pub fn size(&self)-> Vector3<usize>{
        self.dims
    }

    #[inline(always)]
    pub fn get_mut(&mut self, idx:usize)->Option<&mut D>{
        self.cubes.get_mut(idx)
    }

    #[inline(always)]
    pub fn get(&mut self, idx:usize)->Option<& D>{
        self.cubes.get(idx)
    }

    #[inline]
    pub fn pos_to_index(&self, v: Vector3<u32>) -> Option<usize> {
        pos_to_index(self.dims, v)
    }
    ///Iterate over cubes indices in given bounds [min, max]
    #[inline]
    pub fn iter_cube_indices(&self, min: Vector3<u32>, max: Vector3<u32>) -> BoxIdxIterator {
        BoxIdxIterator {
            dims: self.dims,
            iter: itertools::iproduct!(min.x..=max.x, min.y..=max.y, min.z..=max.z),
        }
    }

    ///Iterate over cubes in given bounds [min, max] inside the main cube in read only state
    #[inline]
    pub fn iter_cubes(&self, min: Vector3<u32>, max: Vector3<u32>) -> BoxIterator<D> {
        BoxIterator {
            data: self,
            iter: itertools::iproduct!(min.x..=max.x, min.y..=max.y, min.z..=max.z),
        }
    }
    ///Iterate over cubes in given bounds [min, max] in read and write state.
    #[inline]
    pub fn iter_cubes_mut(&mut self, min: Vector3<u32>, max: Vector3<u32>) -> BoxIteratorMut<D> {
        BoxIteratorMut {
            data: self,
            iter: itertools::iproduct!(min.x..=max.x, min.y..=max.y, min.z..=max.z),
        }
    }
}

impl<D: Debug> Debug for SpatialHashGrid<D> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "<SpatialHashGrid {}x{}x{}:\n",
            self.dims[0], self.dims[1],self.dims[2]
        )?;
        for z in 0..self.dims[2] {
            write!(f, "#Slice z={}:\n", z)?;
            for x in 0..self.dims[0] {
                for y in 0..self.dims[1] {
                    let v = Vector3::new(x as u32, y as u32, z as u32);
                    let idx = self.pos_to_index(v).expect("This can not go wrong");
                    write!(f, "{:?}, ", &self.cubes[idx])?;
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
    type Item = (Vector3<u32>, usize, &'a mut D);

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
            // Need unsafe block to return mutable references here.
            unsafe {
                //SAFETY: the index position should be valid since pos_to_index can
                //not return invalid position.
                let d = self.data.cubes.get_unchecked_mut(idx) as *mut D;
                // SAFETY: we know this can not possibly point to anything invalid
                // We also know that the returned reference should not outlive the iterator
                // unless user does "something really terrible"(tm)
                return Some((c,idx, d.as_mut().unwrap_unchecked()));
            }
        }
    }
    #[inline]
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
    type Item = (Vector3<u32>, usize, &'a D);

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
            let d = self.data.cubes.get(idx);

            return Some((c, idx, d.unwrap()));
        }
    }
    #[inline]
    fn size_hint(&self) -> (usize, Option<usize>) {
        let (_min, max) = self.iter.size_hint();
        (0, max)
    }
}



pub struct BoxIdxIterator {
    dims: Vector3<usize>,
    iter: ConsTuples<
        Product<Product<RangeInclusive<u32>, RangeInclusive<u32>>, RangeInclusive<u32>>,
        ((u32, u32), u32),
    >,
}

impl Iterator for BoxIdxIterator {
    type Item = (Vector3<u32>, usize);

    #[inline]
    fn next(&mut self) -> Option<Self::Item> {
        loop {
            let (x, y, z) = self.iter.next()?;
            let c = Vector3 { x, y, z };
            let idx = match pos_to_index(self.dims, c) {
                Some(idx) => idx,
                None => {
                    continue;
                }
            };
            return Some((c, idx));
        }
    }
    #[inline]
    fn size_hint(&self) -> (usize, Option<usize>) {
        let (_min, max) = self.iter.size_hint();
        (0, max)
    }
}


impl<D: Sized> std::ops::Index<Vector3<u32>> for SpatialHashGrid<D> {
    type Output = D;
    #[inline]
    fn index(&self, index: Vector3<u32>) -> &Self::Output {
        let i = self.pos_to_index(index).expect("Index out of bounds");
        self.cubes.index(i)
    }
}

impl<D: Sized> std::ops::IndexMut<Vector3<u32>> for SpatialHashGrid<D> {
    #[inline]
    fn index_mut(&mut self, index: Vector3<u32>) -> &mut Self::Output {
        let i = self.pos_to_index(index).expect("Index out of bounds");
        self.cubes.index_mut(i)
    }
}



impl<D: Sized> std::ops::Index<usize> for SpatialHashGrid<D> {
    type Output = D;
    #[inline]
    fn index(&self, index: usize) -> &Self::Output {
        self.cubes.index(index)
    }
}

impl<D: Sized> std::ops::IndexMut<usize> for SpatialHashGrid<D> {
    #[inline]
    fn index_mut(&mut self, index:usize) -> &mut Self::Output {
        self.cubes.index_mut(index)
    }
}

#[cfg(test)]
mod tests {
    use crate::SpatialHashGrid;
    use cgmath::Vector3;

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

    #[test]
    fn test_iter_cubes_mut() {
        let mut sh: SpatialHashGrid<u32> = SpatialHashGrid::new(5, 5, 5, u32::default);

        let sx = 3;
        let sy = 4;
        let sz = 5;
        for (i, j, k) in itertools::iproduct!(0..sx, 0..sy, 0..sz) {
            let pos = Vector3::new(i, j, k);
            sh[pos] = 1;
        }

        let min = Vector3::new(0, 0, 0);
        let b = Vector3::new(2, 1, 2);
        let max = Vector3::new(sx, sy, sz);

        for (_p, _i, d) in sh.iter_cubes_mut(min, b) {
            *d += 1;
        }

        let mut count = 0;
        for (_p, _i, d)  in sh.iter_cubes(min, max) {
            count += *d;
        }
        assert_eq!(
            count,
            (sx * sy * sz) + (b.x + 1) * (b.y + 1) * (b.z + 1),
            "Incorrect sum of values"
        );
    }
    #[test]
    fn test_indexing() {
        let mut sh: SpatialHashGrid<Option<u32>> = SpatialHashGrid::new(5, 5, 5, || None);

        let p = Vector3 { x: 3, y: 3, z: 3 };
        sh[p] = Some(42);
        assert_eq!(sh[p], Some(42));
        sh[p] = None;
        assert_eq!(sh[p], None);
        println!("{:?}", sh);
    }
    #[test]
    fn test_debug_trait() {
        let mut sh: SpatialHashGrid<Data> = SpatialHashGrid::new(3, 3, 2, Data::default);
        let p = Vector3 { x: 1, y: 1, z: 1 };
        sh[p].x = 42;
        sh[p].y = 42;
        sh[p].z = 42;
        println!("{:?}", sh);
    }
}
