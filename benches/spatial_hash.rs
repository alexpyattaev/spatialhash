use cgmath::Vector3;
use criterion::{black_box, criterion_group, criterion_main, Criterion};
use rand::rngs::SmallRng;
use rand::{Rng, SeedableRng};
use spatial_hash::*;
use std::fmt::{Debug, Display, Formatter};

#[derive(Debug)]
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
    for (i, j, k) in itertools::iproduct!(0..x, 0..y, 0..z) {
        let pos = Vector3::new(i, j, k);
        sh.fill_cube(pos, Data { some_data: count });
        count += 1;
    }
    sh
}

/// Creates random bounding boxes within x,y,z size
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

fn bench_get_filled_data(sh: &SpatialHashGrid<Data>, min: Vector3<u32>, max: Vector3<u32>) {
    for i in sh.iter_filled_boxes(min, max) {
        black_box(i);
    }
}

fn bench_modify_filled_data(sh: &mut SpatialHashGrid<Data>, min: Vector3<u32>, max: Vector3<u32>) {
    for i in sh.iter_filled_boxes_mut(min, max) {
        i.some_data += 1;
    }
}

/// Measures how quickly the data is retrieved from the structure
/// Two tests inside - one for mutable and one for immutable iteration
pub fn bench_get_data_if_there(c: &mut Criterion) {
    let mut rng = SmallRng::seed_from_u64(42);

    let mut group = c.benchmark_group("lookups");

    for size in [5u32, 10, 20] {
        group.bench_with_input(
            criterion::BenchmarkId::from_parameter(size),
            &size,
            |b, &size| {
                //generate max border values for the base volume, Max value for each axis is never larger than 20
                let (x, y, z) = (size, size, size);
                //generate bounding volume based on the previous values, each value is never larger than "max border values"

                //fill general space
                let spatial = create_and_fill(x, y, z);

                b.iter(|| {
                    let (min, max) = generate_bounding_box(&mut rng, x, y, z);
                    //bounding box is located in general space, look for the data in bounding box
                    bench_get_filled_data(&spatial, min, max);
                })
            },
        );
    }
    drop(group);
    let mut group = c.benchmark_group("edits");

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
                    bench_modify_filled_data(&mut spatial, min, max);
                    black_box(&spatial);
                })
            },
        );
    }
}

/// Measures how quickly the data is filled in the structure
/// Should be pretty fast if size of D is small

pub fn bench_fill_data(c: &mut Criterion) {
    let mut group = c.benchmark_group("writes");
    for size in [5u32, 10, 20] {
        group.bench_with_input(
            criterion::BenchmarkId::from_parameter(size),
            &size,
            |b, &size| {
                b.iter(|| {
                    let spatial = create_and_fill(size, size, size);
                    black_box(spatial);
                })
            },
        );
    }
}

criterion_group!(benches, bench_get_data_if_there, bench_fill_data);
criterion_main!(benches);
