# more-iter

More iterating utilities in rust

## MergePair

Merge two `(key, value)` iterators by key, keeping the largest value:

```rust
use more_iter::MergePair;

let a = [(1, 10), (3, 30), (4, 40)];
let b = [(2, 200), (3, 300)];

let merged = MergePair::merge(a, b, std::cmp::max).collect::<Vec<_>>();
assert_eq!(vec![(1, 10), (2, 200), (3, 300), (4, 40)], merged);
```

