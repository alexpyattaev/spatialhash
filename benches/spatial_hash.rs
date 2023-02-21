use cgmath::Vector3;
use criterion::{black_box, criterion_group, criterion_main, Criterion};
use rand::rngs::SmallRng;
use rand::{Rng, SeedableRng};
use spatial_hash::*;
use std::fmt::{Debug, Display, Formatter};

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
fn create_and_fill(x:usize, y: usize, z: usize) -> SpatialHashGrid<Data> {
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

fn bench_get_filled_data(rng: &mut SmallRng, sh: &mut SpatialHashGrid<Data>, x:usize, y: usize, z: usize)  {
    let min = Vector3::new(
        rng.gen_range(0..x-2),
        rng.gen_range(0..100),
        rng.gen_range(0..100),
    );
    let max = Vector3::new(
        rng.gen_range(min.x..x),
        rng.gen_range(0..100),
        rng.gen_range(0..100),
    );
    for i in sh.collect_filled_data(min, max) {
        black_box(i);
    }
}
pub fn bench_get_data_if_there(c: &mut Criterion) {
    let mut rng = SmallRng::seed_from_u64(42);

    c.bench_function("get filled data", |b| {
        //todo make x,y, z functions of some common "size" parameter
        let mut spatial = create_and_fill(...);
        b.iter(||{
            bench_get_filled_data(&mut rng, &mut spatial, x,y,z);
        })
    });
}

criterion_group!(benches, bench_get_data_if_there);
criterion_main!(benches);
