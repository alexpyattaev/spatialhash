use cgmath::Vector3;
use rand::rngs::SmallRng;
use rand::{Rng, SeedableRng};
use std::fmt::{Debug, Display, Formatter};

use criterion::{black_box, criterion_group, criterion_main, Criterion,};
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

struct SpatialHashGrid<D: Sized + Default + Debug + Display> {
    size_x: usize,
    size_y: usize,
    //size_z: usize,
    //FIXME: Why is this unused? THIS IS A BUG
    cubes: Vec<Option<D>>,
}

impl<D: Sized + Default + Debug + Display + Clone> SpatialHashGrid<D> {
    fn new(x: usize, y: usize, z: usize) -> Self {
        let cap = x * y * z;
        let mut d = Vec::with_capacity(cap);
        d.resize_with(cap, || None);
        Self {
            size_x: x,
            size_y: y,
            //size_z: z,
            cubes: d,
        }
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

    //For debug

    fn collect_filled_data(&mut self, min: Vector3<u32>, max: Vector3<u32>) -> Vec<D> {
        let mut collection: Vec<Option<D>> = vec![];

        // Collect all data from given bounds by reverse transfer from index to coordinates
        for i in 0..self.cubes.len() {
            let cord_frm_idx = self.pos_from_index(i);
            //Check bounds below
            if cord_frm_idx.x >= min.x as usize
                && cord_frm_idx.y >= min.y as usize
                && cord_frm_idx.z >= min.z as usize
            {
                if cord_frm_idx.x <= max.x as usize
                    && cord_frm_idx.y <= max.y as usize
                    && cord_frm_idx.z <= max.z as usize
                {
                    collection.push(self.cubes[i].as_ref().cloned());
                }
            }
        }

        // Filter data from None in established bounds and return vector of Data
        let d = collection
            .iter()
            .flat_map(|data| data.iter())
            .cloned()
            .collect::<Vec<D>>();

        d
    }
}

pub trait Hashing<D> {
    // fn iter_mut_stuff(&mut self, min: Vector3<u32>, max: Vector3<u32>) -> Vec<&D>
    //fn new(x: usize, y: usize, z: usize) -> Self; // TODO: should not be part of the trait
    fn fill_cube(&mut self, v: Vector3<u32>);
    fn get_cube_mut(&mut self, v: Vector3<u32>) -> Option<&mut D>;
    fn get_cube(&self, v: Vector3<u32>) -> Option<&D>;
    fn pos_to_index(&self, v: Vector3<u32>) -> usize;
    fn pos_from_index(&self, v: usize) -> Vector3<usize>;
    fn collect_filled_data(&mut self, min: Vector3<u32>, max: Vector3<u32>) -> Vec<D>;
}

fn create_and_fill() -> SpatialHashGrid<Data> {
    let mut sh: SpatialHashGrid<Data> = SpatialHashGrid::new(10, 8, 6);
    let mut count = 0;
    for j in 0..10 {
        for k in 0..8 {
            for z in 0..6 {
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
    sh
}

fn bench_get_filled_data(min: Vector3<u32>, max: Vector3<u32>) -> SpatialHashGrid<Data> {
    let mut sh = create_and_fill();
    for _ in 0..100 {
        for i in sh.collect_filled_data(min, max) {
            black_box(i);
        }
    }
    sh
}
pub fn bench_get_data_if_there(c: &mut Criterion) {

    let mut rng = SmallRng::seed_from_u64(42);

    let min = Vector3::new(
        rng.gen_range(0..100),
        rng.gen_range(0..100),
        rng.gen_range(0..100),
    );
    let max = Vector3::new(
        rng.gen_range(0..100),
        rng.gen_range(0..100),
        rng.gen_range(0..100),
    );

    let bound_min = Vector3::new(
        rng.gen_range(0..100),
        rng.gen_range(0..100),
        rng.gen_range(0..100),
    );
    let bound_max = Vector3::new(
        rng.gen_range(0..100),
        rng.gen_range(0..100),
        rng.gen_range(0..100),
    );

    let mut spatial = bench_get_filled_data(min, max);

    c.bench_function("get filled data", |b| {
        b.iter(|| spatial.collect_filled_data(black_box(bound_min), black_box(bound_max)))
    });
}

criterion_group!(benches, bench_get_data_if_there);
criterion_main!(benches);
