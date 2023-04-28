


# Spatialhash
Generic spatial hash grid in Rust. 

Allows to efficiently translate spatial requests into cells with content. 
The implementation does not care what is stored in the cells, as long as it is Sized.

[![Crates.io](https://img.shields.io/crates/v/spatial_hash_3d)](https://crates.io/crates/spatial_hash_3d)
[![Documentation](https://docs.rs/spatial_hash_3d/badge.svg)](https://docs.rs/spatial_hash_3d/)

```rust
use spatial_hash_3d::SpatialHashGrid;
use cgmath::Vector3;
// create spatial hash grid of size 5x10x20 filled with None.
let mut sh: SpatialHashGrid<Option<u64>> = SpatialHashGrid::new(5, 10, 20, ||{None});
// fill the cube at coordinates (1,2,3) with content 42
sh[Vector3::new(1, 2, 3)] = Some(42);

// iterate by mutable reference over all filled boxes within a given bounding volume
for (c, _idx, elem) in sh.iter_cubes_mut(Vector3::new(1, 2, 3), Vector3::new(4, 5, 4)) {
    match elem{
        Some(e)=> *e += 1,
        None=>{}
    }
}

// retrieve coordinates and references to contents of all filled boxes within a given volume
for (c,elem) in sh.iter_cubes(Vector3::new(1, 2, 3), Vector3::new(4, 5, 4)).filter_map(|(c, e)| Some((c,e.as_ref()?))) {
    println!("{c:?} {elem}");
}

```

When creating spatial hash, you can pass any closure, so you can have different content in cells.

```rust
# use spatial_hash_3d::SpatialHashGrid;
# use cgmath::Vector3;
// create spatial hash grid of size 5x10x20 filled with different numbers.
let mut cnt:u64 = 42;
let mut sh: SpatialHashGrid<u64> = SpatialHashGrid::new(5, 10, 20, ||{cnt +=1; cnt});
```

# Design considerations
This was optimized for speed and not memory efficiency.

 - Read and write complexity is O(1).
 - A matrix for entire volume will always be preallocated at grid creation, so memory usage is proportional to *volume*.
 - No dynamic resizing capability was envisioned for this, as most usecases that care do not need it.

# Iterators
Iterators are provided to access content within a given axis-aligned volume. All iterators have complexity proportional to
volume they select from, i.e. the larger the volume the longer the iterator will run.

When iterating it may be useful to "cache" references into the spatial hash for further use. For this purpose a flat u64 index
is provided (which can be returned by any iterator or obtained from valid coordinates).
```rust
# use spatial_hash_3d::SpatialHashGrid;
# use cgmath::Vector3;
# let mut sh: SpatialHashGrid<u64> = SpatialHashGrid::new(5, 10, 20, ||{0});

// construct iterator that returns indices
let cubesiter = sh.iter_cubes(Vector3::new(1, 2, 3), Vector3::new(4, 5, 4)).with_index();
// vec to hold interesting cube references
let mut interesting = vec![];
// go over the iterator saving references
for (c,idx, elem) in cubesiter {
    println!("{c:?} {idx} {elem}");
    if *elem == 42{
        interesting.push(idx);
    }
}

for i in interesting {
    let v = sh[i];
    println!("Selected {v} @ idx {i}");
}

```

This is typically safer than holding references directly into the storage, and will also keep borrow checker happy, and your game free from segfaults.


# Dependencies
 - cgmath is a dependency of convenience for Vector3 (which you probably have anyway if you use this library)
 - itertools for iterators over volumes

