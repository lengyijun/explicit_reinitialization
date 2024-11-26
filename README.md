# explicit_reinitialization

### Start
install [dylint](https://github.com/trailofbits/dylint)
```
cargo dylint --git https://github.com/lengyijun/explicit_reinitialization/
```

### What it does
If a reinitialization dominate all reachable usages, a fresh variable should be introduced.

Make rust more functional programming.

### Why is this bad?
Introduce unnecessary mut.
Not good in jumping to definition in ide.

### Known problems
1. Known false positive and false negative: see test
2. increase the peak memory usage
```
let mut x = vec![1, 2, 3];
x = vec![4, 5, 6];            // x is dropped here
// let x = vec![4, 5, 6];     // x is no longer dropped here, but at the end of the scope
```

### Example
```rust
let mut x = 1;
println!("{x}");
x = 2;
println!("{x}");
```
Use instead:
```rust
let mut x = 1;
println!("{x}");
let x = 2;
println!("{x}");
```

### Reference

https://github.com/rust-lang/rust-clippy/pull/11687
