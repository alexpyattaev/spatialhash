# Spatialhash
Generic spatial hash grid in Rust. 

Allows to efficiently translate spatial requests into cells with content. 
The implementation does not care what is stored in the cells, as long as it is Sized.

```rust

// create spatial hash grid of size 5x10x20
let mut sh: SpatialHashGrid<u64> = SpatialHashGrid::new(5, 10, 20);
// fill the cube at coordinates (1,2,3) with content 42 (by default grid is filled with None)
sh.fill_cube(Vector3::new(1, 2, 3), 42);

// iterate by mutable reference over all filled boxes within a given volume
for i in sh.iter_filled_boxes_mut(Vector3::new(1, 2, 3), Vector3::new(4, 5, 4)) {
    i += 1;
}

// iterate by reference over all filled boxes within a given volume
for i in sh.iter_filled_boxes(Vector3::new(1, 2, 3), Vector3::new(4, 5, 4)) {
    println!("{}", i);
}

```

# Design considerations
This was optimized for speed and not memory efficiency.

 - Read and write complexity is O(1).
 - A matrix for entire volume will always be preallocated at grid creation, so memory usage is proportional to *volume*.
 - No dynamic resizing capability was envisioned for this, as most usecases that care do not need it.

# Iterators
Iterators are provided to access content within a given axis-aligned volume. All iterators have complexity proportional to
volume they select from, i.e. the larger the volume the longer the iterator will run.

# Dependencies
 - cgmath is a dependency of convenience for Vector3
 - itertools for iterators over volumes

