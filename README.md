# List Comprehensions in Rust
#### Generate vectors with a macro of similar syntax to list comprehensions in Python! + with localized declarations from Haskell

[![codecov](https://codecov.io/gh/CircArgs/rust_list_comprehension/branch/master/graph/badge.svg)](https://codecov.io/gh/CircArgs/rust_list_comprehension)

```rust
vec_comp![x; for x in 1..4] //vec![1, 2, 3]
    
vec_comp![x; for x in 1..4; if x>1] //vec![2, 3]
    
vec_comp![y; for x in 1..4; let y=x*x+4; if x>1] //vec![8, 13]
        
vec_comp![y; for x in 1..4; let y=x*x+4] //vec![5, 8, 13]
    
vec_comp![y+z; for x in 1..4; let y=x*x+4; let z = 3*y+x; if z>20] //vec![34, 55]
        
vec_comp![y+zz*z; for x in 1..4; let y=x*x+4; let z = 3*y+x; for yy in 1..10; let zz= yy+1; if yy<3 && x>1] //vec![60, 86, 97, 139]

let myvec = vec![8, 6, 7, 5, 3, 0, 9];
vec_comp![y+zz*z; for x in 1..4; let y=x*x+4; let z = 3*y+x; for yy in 1..10; let zz= yy+1; if yy<3 && x>1; using myvec] //vec![8, 6, 7, 5, 3, 0, 9, 60, 86, 97, 139]
```
