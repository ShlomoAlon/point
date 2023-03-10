# point-index

a point library for working with 2d arrays or vectors.

it has all the cardinal direction for point addition as well as scalar multiplication.
## Examples
here are a couple of examples of array accessing
```rust
use point_index::*;
let mut vec = vec![vec![0, 1, 2], vec![3, 4, 5], vec![6, 7, 8]];
let mut point1 = Point { x: 1, y: 1 };
assert_eq!(vec[point1], 4);
// Up is the same as Point {x: 0, y: -1}
let point2 = point1 + UP;
assert_eq!(vec[point2], 1);
// Down is the same as Point {x: 0, y: 1}
let point3 = point2 + DOWN * 2;
assert_eq!(vec[point3], 7);
// Up left is the same as Point {x: -1, y: -1}
let point4 = point1 + UP_LEFT;
assert_eq!(vec[point4], 0)
```
here is a sample of examples of array mutation
```rust
use point_index::*;
let mut arr = [[0; 10]; 10];
// down right is the same as Point {x: 1, y: 1}
for x in 0 .. 10{
    arr[DOWN_RIGHT * x] = x
}
assert_eq!(
    arr,
   [[0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
    [0, 1, 0, 0, 0, 0, 0, 0, 0, 0],
    [0, 0, 2, 0, 0, 0, 0, 0, 0, 0],
    [0, 0, 0, 3, 0, 0, 0, 0, 0, 0],
    [0, 0, 0, 0, 4, 0, 0, 0, 0, 0],
    [0, 0, 0, 0, 0, 5, 0, 0, 0, 0],
    [0, 0, 0, 0, 0, 0, 6, 0, 0, 0],
    [0, 0, 0, 0, 0, 0, 0, 7, 0, 0],
    [0, 0, 0, 0, 0, 0, 0, 0, 8, 0],
    [0, 0, 0, 0, 0, 0, 0, 0, 0, 9]]

)
```

License: MIT OR Apache-2.0
