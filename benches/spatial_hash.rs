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
fn create_and_fill(x: u32, y: u32, z: u32) -> SpatialHashGrid<Data> {
    let mut sh: SpatialHashGrid<Data> = SpatialHashGrid::new(x as usize, y as usize, z as usize);
    let mut count = 0;
    for j in 0..x {
        for k in 0..y {
            for z in 0..z {
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
fn generate_bounding_box(
    rng: &mut SmallRng,
    x: u32,
    y: u32,
    z: u32,
) -> (Vector3<u32>, Vector3<u32>) {
    let min = Vector3::new(
        rng.gen_range(0..(x - 2)),
        rng.gen_range(0..(y - 2)),
        rng.gen_range(0..(z - 2)),
    );
    let max = Vector3::new(
        rng.gen_range(min.x..x),
        rng.gen_range(min.y..y),
        rng.gen_range(min.z..z),
    );
    if min.x > max.x || min.y > max.y || min.z > max.z {
        panic!("Generated volume is not in this Universe");
    }
    (min, max)
}

fn bench_get_filled_data(sh: &mut SpatialHashGrid<Data>, min: Vector3<u32>, max: Vector3<u32>) {
    for i in sh.collect_filled_data(min, max) {
        black_box(i);
    }
}



pub fn bench_get_data_if_there(c: &mut Criterion) {
    let mut rng = SmallRng::seed_from_u64(42);
    let mut group = c.benchmark_group("spatialhash lookups");

    for size in [5u32, 10, 20] {
        group.bench_with_input(
            criterion::BenchmarkId::from_parameter(size),
            &size,
            |b, &size| {
                //generate max border values for the base volume, Max value for each axis is never larger than 20
                let (x, y, z) = (size, size, size);
                //generate bounding volume based on the previous values, each value is never larger than "max border values"

                //fill general space
                let mut spatial = create_and_fill(x, y, z);

                // println!("General space {:?}, {:?}, {:?}", x, y, z);
                // println!("Min={:?}, Max={:?}", min, max);

                b.iter(|| {
                    let (min, max) = generate_bounding_box(&mut rng, x, y, z);
                    //bounding box is located in general space, look for the data in bounding box
                    bench_get_filled_data(&mut spatial, min, max);
                })
            },
        );
    }
}

criterion_group!(benches, bench_get_data_if_there);
criterion_main!(benches);
