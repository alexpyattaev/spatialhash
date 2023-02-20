# Spatialhash
Generic spatial hash grid in Rust. 

Allows to efficiently translate spatial requests into cells with content. 
The implementation does not care what is stored in the cells, as long as it is Sized.

# Design considerations
This was optimized for speed and not memory efficiency. Read and write complexity is O(1). 
A matrix for entire volume will always be preallocated at grid creation, so memory usage is proportional to volume. 
No dynamic resizing capability was envisioned for this, as most usecases that care do not need it.

# Iterators
Iterators are provided across all filled cells, as well as within a cube/box. One can supply custom selector function to emit cell coordinates to iterate over.
