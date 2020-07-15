
<h1 align="center"> List Comprehensions in Rust </h1>

  <h4 align="center">Generate vectors (+other structures) with a macro of similar syntax to list comprehensions in Python! + with localized declarations from Haskell</h4> 
<div style="width: 100%" align="center">
<img alt="codecov" src="https://codecov.io/gh/CircArgs/rust_list_comprehension/branch/master/graph/badge.svg">
<img alt="Build Status" src="https://github.com/CircArgs/rust_list_comprehension/workflows/test/badge.svg">
<img alt="Language Rust" src="https://img.shields.io/badge/language-Rust-orange">
<img alt="License MIT" src="https://img.shields.io/badge/license-MIT-green">
</div>

# Index
[TLDR;](#tldr)
* [Macros](#macros)
* [Vector (list) Comprehensions](#vector-list-comprehensions)
  + [`comp!`](#comp)
  + [Examples](#ex)
* [HashMap (dictionary) Comprehensions](#hashmap-dictionary-comprehensions)
  + [`dcomp!`](#dcomp)
  + [Examples](#ex-1)
* [HashMap (dictionary) Literal](#hashmap-dictionary-literal)
  + [`dict!`](#dict)
- [Preallocation Variants](#preallocation-variants)
* [Preallocating Vector (list) Comprehensions](#preallocating-vector-list-comprehensions)
  + [`comp_pre!`](#comp_pre)
* [Preallocating HashMap (dictionary) Comprehensions](#preallocating-hashmap-dictionary-comprehensions)
  + [`dcomp_pre!`](#dcomp_pre)
* [Preallocating HashMap (dictionary) Literal](#preallocating-hashmap-dictionary-literal)
  + [`dict_pre!`](#dict_pre)
    - [Examples](#ex-2)
    
# TLDR;
- A set of macros to emulate list/array and dictionary/hashtable comprehensions with syntax inspired primarily by Python
- Variants for **preallocation of memory**. Remove reallocation initiated by many `push`es or `insert`s
- Use **custom datastructures**
- Includes some additional macros for HashMap literals
- Inline **localized `let`s and `if`s** at any level for complete flexibility
- The only limit is (probably) your imagination and the user-manageable max macro recursion depth that can be controlled with `#[recursion_limit = "some depth"]`
  - you are almost certain to not hit the default recursion depth under typical use of these macros

## Macros
### `comp!, comp_pre!, dcomp!, dcomp_pre!, dict!, dict_pre!`
- vec (list) comprehension `comp![x; for x in 1..4] -> vec![1, 2, 3]`
- preallocated vec (list) comprehension `comp_pre![x; for x in 1..4] -> vec![1, 2, 3]`
- hashmap (dict) comprehension `dcomp!{x => y+z; for x in 1..4, let y=x*x+4, let z = 3*y+x, if z>20} -> dict!{2 => 34, 3 => 55}`
- preallocated hashmap (dict) comprehension `dcomp_pre!{x => y+zz*z; for x in 1..4, let y=x*x+4, let z = 3*y+x, if x>1; for yy in 1..10, let zz= yy+1, if yy<3} -> dict! {3=> 139, 2=> 86}`
- hashmap (dict) literal `dict!{3 => (3, 3), 1 => (1, 1), 2 => (2, 2)} -> HashMap {3 : (3, 3), 1 : (1, 1), 2 : (2, 2)}`
- preallocated hashmap (dict) literal `dict_pre!{2=> dict_pre!{2 => 34}, 3 => dict_pre!{3 => 55}} -> HashMap {2 : {2 : 34}, 3: {3 : 55}}`




## Vector (list) Comprehensions
### `comp!`
- Any `Iterator` or implementor of `IntoIterator`
- Inline localized declarations `let`s and conditionals `if`s at any level for total flexibility

### Ex. 

```rust

//basic
comp![x; for x in 1..4] //vec![1, 2, 3]

//use anything implementing Iterator or IntoIterator (i.e. anything accepted by traditional `for` loops)
comp![x; for x in vec![8, 6, 7, 5, 3, 0, 9]] -> vec![8, 6, 7, 5, 3, 0, 9]

//tuples (of any length - length 2 shown)
comp![(x, y); for (x,y) in (1..4).zip(1..4)] -> vec![(1, 1), (2, 2), (3, 3)]

//conditioning (filtering)
comp![x; for x in 1..4, if x>1] -> vec![2, 3]

//localized declarations (like haskell)
comp![y; for x in 1..4, let y=x*x+4] -> vec![5, 8, 13]

//nesting
comp![comp![y2+z2; for x2 in 1..x, let y2=x*x+4, let z2 = 3*y+x, if z2>20]; for x in 1..4; let y=x*x+4; if x>1] -> vec![vec![34], vec![55, 55]]

//localized declarations w/ conditioning
comp![y; for x in 1..4, let y=x*x+4, if x>1] -> vec![8, 13]

//multiple localized declarations w/ conditioning
comp![y+z; for x in 1..4, let y=x*x+4, let z = 3*y+x, if z>20] -> vec![34, 55]

//multiple iterators and multiple localized declarations w/ conditioning
comp![y+zz*z; for x in 1..4, let y=x*x+4, let z = 3*y+x; for yy in 1..10, let zz= yy+1, if yy<3 && x>1] -> vec![60, 86, 97, 139]

//the same thing can be written using a split conditional to reduce the number of outer loops
comp![y+zz*z; for x in 1..4, let y=x*x+4, let z = 3*y+x, if x>1; for yy in 1..10, let zz= yy+1, if yy<3] -> vec![60, 86, 97, 139]

//use existing vector (e.g. can preallocate and touch memory to prevent any reallocation if you know the size of the final vector beforehand)
let myvec = vec![8, 6, 7, 5, 3, 0, 9];
comp![y+zz*z; for x in 1..4; let y=x*x+4; let z = 3*y+x; for yy in 1..10; let zz= yy+1; if yy<3 && x>1; using myvec] -> vec![8, 6, 7, 5, 3, 0, 9, 60, 86, 97, 139]

//use custom data structures with .push method
let linked_list = LinkedList::new();
comp![using linked_list, x; for x in 1..4] -> Node(1) -> Node(2) -> Node(3)

//unlimited complexity (up to user manageable) macro recursion limit
comp![y+zz*z; for x in 1..4, let y=x*x+4, let z = 3*y+x, if z>20; for yy in 1..10, let zz= yy+1; for _yyy in 1..10, if yy>7; for _i in 1..3] -> 
vec![
    242, 242, 242, 242, 242, 242, 242, 242, 242, 242, 242, 242, 242, 242, 242, 242,
    242, 242, 268, 268, 268, 268, 268, 268, 268, 268, 268, 268, 268, 268, 268, 268,
    268, 268, 268, 268, 391, 391, 391, 391, 391, 391, 391, 391, 391, 391, 391, 391,
    391, 391, 391, 391, 391, 391, 433, 433, 433, 433, 433, 433, 433, 433, 433, 433,
    433, 433, 433, 433, 433, 433, 433, 433
]
```

## HashMap (dictionary) Comprehensions
### `dcomp!`
- Similar to `comp!` except for `dcomp!` the expression inserted is of the form `key => value` 

### Ex. 

```rust

//basic
dcomp![x*x => x; for x in 1..4] -> dict! {9=> 3, 1=> 1, 4=> 2}

//use anything implementing Iterator or IntoIterator (i.e. anything accepted by traditional `for` loops)
dcomp!{format!("Hello {:?}", x) => x; for x in vec![8, 6, 7, 5, 3, 0, 9]} -> {"Hello 9": 9, "Hello 7": 7, "Hello 6": 6, "Hello 5": 5, "Hello 3": 3, "Hello 0": 0, "Hello 8": 8}

//any pattern
dcomp!{x=>(x, y); for (x,y) in (1..4).zip(1..4)} -> dict! {3=> (3, 3), 1=> (1, 1), 2=> (2, 2)}

//any complexity
dcomp![x=>y+zz*z; for x in 1..4, let y=x*x+4, let z = 3*y+x, if z>20; for yy in 1..10, let zz= yy+1; for _yyy in 1..10, if yy>7; for _i in 1..3] -> dict! {3=> 433, 2=> 268}

```

## HashMap (dictionary) Literal
### `dict!`

- Write `HashMap`s in literal form

```rust

dict!{"hello" => 5, "world" => 13, "!" => 8}

```

# Preallocation Variants

- These macros infer the size of the final data structure by accounting for the lower bounds of each `Iterator` as given by `.size_hint()` and attempts to `reserve` this memory
- There are potential performance improvements from this strategy
- The difference between the `capacity` and `len` of the created data structure can be greater than if using the non-preallocated variants
- Best used for large comprehensions (or literals) with iterators whose `.size_hint()` is of good accuracy
- The user can always call `shrink_to_fit` on the final value to free memory

## Preallocating Vector (list) Comprehensions

### `comp_pre!`
- see [Vector (list) Comprehension](#vector-list-comprehensions) for examples
- this differs from `comp!` in that before inserting into a `Vec`, this macrs infesr the size of the final `Vec` by accounting for the lower bounds of each `Iterator` as given by `.size_hint()` and attempts to `reserve` this memory

## Preallocating HashMap (dictionary) Comprehensions

### `dcomp_pre!`
- see [HashMap (dictionary) Comprehension](#hashmap-dictionary-comprehensions) for examples
- this differs from `dcomp!` in that before inserting into a `HashMap`, this macro infers the size of the final `Vec` by accounting for the lower bounds of each `Iterator` as given by `.size_hint()` and attempts to `reserve` this memory


## Preallocating HashMap (dictionary) Literal

### `dict_pre!`
- see [HashMap (dictionary) Literal](#hashmap-dictionary-literal) for examples
- this differs from `dict!` in that before inserting into a `HashMap`, this macro infers the size of the final `HashMap` by accounting for the total number of `key => value` pairs and attempts to `reserve` this memory

#### Ex.
```rust

dict_pre!{"hello" => 5, "world" => 13, "!" => 8}

```


