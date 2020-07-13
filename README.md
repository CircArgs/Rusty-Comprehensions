
<h1 align="center"> List Comprehensions in Rust </h1>

  <h4 align="center">Generate vectors (+other structures) with a macro of similar syntax to list comprehensions in Python! + with localized declarations from Haskell</h4> 
<div style="width: 100%" align="center">
<img alt="codecov" src="https://codecov.io/gh/CircArgs/rust_list_comprehension/branch/master/graph/badge.svg">
<img alt="Build Status" src="https://github.com/CircArgs/rust_list_comprehension/workflows/test/badge.svg">
<img alt="Language Rust" src="https://img.shields.io/badge/language-Rust-orange">
<img alt="License MIT" src="https://img.shields.io/badge/license-MIT-green">
</div>


## Examples:

```rust

//basic
comp![x; for x in 1..4] //vec![1, 2, 3]

//use anything implementing Iterator or IntoIterator (or any trait accepted by traditional `for` loops)
comp![x; for x in vec![8, 6, 7, 5, 3, 0, 9]] //vec![8, 6, 7, 5, 3, 0, 9]

//conditioning
comp![x; for x in 1..4; if x>1] //vec![2, 3]

//localized declarations
comp![y; for x in 1..4; let y=x*x+4] //vec![5, 8, 13]

//nesting
comp![comp![y2+z2; for x2 in 1..x; let y2=x*x+4; let z2 = 3*y+x; if z2>20]; for x in 1..4; let y=x*x+4; if x>1] //vec![vec![34], vec![55, 55]]

//localized declarations w/ conditioning
comp![y; for x in 1..4; let y=x*x+4; if x>1] //vec![8, 13]

//multiple localized declarations w/ conditioning
comp![y+z; for x in 1..4; let y=x*x+4; let z = 3*y+x; if z>20] //vec![34, 55]

//multiple iterators and multiple localized declarations w/ conditioning
comp![y+zz*z; for x in 1..4; let y=x*x+4; let z = 3*y+x; for yy in 1..10; let zz= yy+1; if yy<3 && x>1] //vec![60, 86, 97, 139]

//the same thing can be written using a split conditional to reduce the number of outer loops
comp![y+zz*z; for x in 1..4; let y=x*x+4; let z = 3*y+x; if x>1; for yy in 1..10; let zz= yy+1; if yy<3] //vec![60, 86, 97, 139]

//use existing vector (e.g. can preallocate and touch memory to prevent any reallocation if you know the size of the final vector beforehand)
let myvec = vec![8, 6, 7, 5, 3, 0, 9];
comp![y+zz*z; for x in 1..4; let y=x*x+4; let z = 3*y+x; for yy in 1..10; let zz= yy+1; if yy<3 && x>1; using myvec] //vec![8, 6, 7, 5, 3, 0, 9, 60, 86, 97, 139]

//use custom data structures with .push method
let linked_list = LinkedList::new();
comp![x; for x in 1..4; using linked_list] //Node(1) -> Node(2) -> Node(3)
```
