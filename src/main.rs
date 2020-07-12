macro_rules! vec_comp {
    //========================I: single iterator case=============================
    // A: base case of iterator, any number of localized lets and finally a conditional
    // Ex. vec_comp![x; for x in 1..4; if x>1 ] >> [2, 3]
    ($f: expr; for $x: ident in $iterx:expr $(;let $s: ident = $v:expr)*; if $cond: expr $(;)*) => {{
        let mut myvec = Vec::new();
        let iter=$iterx;
        for $x in iter {
            $(let $s = $v;)*
            if $cond {
                myvec.push($f);
            };
        }
        myvec
    }};
    // B: A without a conditional; bootstraps A to be called with condition of true
    // Ex. vec_comp![x; for x in 1..4] >> [1, 2, 3]
    ($f: expr; for $x: ident in $iterx:expr $(;let $s: ident = $v:expr)* $(;)*) => {{
        vec_comp![$f; for $x in $iterx $(;let $s = $v)*; if true]
     }};
    //========================II: multi iterator case=============================
    // A: base case for multi iterator - let statement pairs WITH conditional
    // Ex. vec_comp![y*z; for x in 1..4; let y = x*x; for z in 1..y; let zz = 45; if x*zz>45] >> [4, 8, 12, 9, 18, 27, 36, 45, 54, 63, 72]
     ($f: expr; for $x: ident in $iterx:expr $(; let $s: ident = $v:expr)* $(; for $y: ident in $itery:expr $(; let $t: ident = $w:expr)*)+; if $cond: expr $(;)*) => {{
        // boilerplate uses x and looks nearly identical to A
        let mut myvec = Vec::new();
        let iter=$iterx;
        for $x in iter {
            $(let $s = $v;)*
            // recurse for y iterators and lets
            // calling case G until hit the single iterator case and then call F
            vec_comp![$f $(;for $y in $itery $(;let $t = $w)*)+; if $cond; myvec]
        }
        myvec
    }};
    // B: base case for multi iterator - let statement pairs WITHOUT conditional
    // simply wraps A as I-B did I-A
    // Ex. vec_comp![zz*z; for x in 1..4; let y = x*x; for z in 1..y; let zz = x*y; if y > 4] [27, 54, 81, 108, 135, 162, 189, 216]
    ($f: expr; for $x: ident in $iterx:expr $(; let $s: ident = $v:expr)* $(; for $y: ident in $itery:expr $(; let $t: ident = $w:expr)*)+ $(;)*) => {
        vec_comp![$f; for $x in $iterx $(; let $s = $v)* $(; for $y in $itery $(;let $t = $w)*)*; if true]
    };
    //========================III: w/ preallocated vectors=============================
    // A: given a user preallocated vector base case w/ conditional
    // Ex. let myvec = vec_comp![zz*z; for x in 1..4; let y = x*x; for z in 1..y; let zz = x*y; if y > 4; using Vec::new()];
    ($f: expr; for $x: ident in $iterx:expr $(;let $s: ident = $v:expr)* $(;for $y: ident in $itery:expr $(;let $t: ident = $w:expr)*)+; if $cond:expr; using $myvec: expr $(;)*) => {{
        let mut myvec=$myvec;
        let iter=$iterx;
        for $x in iter {
            $(let $s = $v;)*
            vec_comp![$f $(; for $y in $itery $(;let $t = $w)*)+; if $cond; myvec]
        }
        myvec
    }};
    // B: simply wraps A as I-B did I-A
    // Ex. let mut myvec = Vec::new();
    //     myvec = vec_comp![zz*z; for x in 1..4; let y = x*x; for z in 1..y; let zz = x*y; using myvec];
    ($f: expr; for $x: ident in $iterx:expr $(;let $s: ident = $v:expr)* $(;for $y: ident in $itery:expr $(;let $t: ident = $w:expr)*)+; using $myvec: expr $(;)*) => {{
        let mut myvec=$myvec;
        let iter=$iterx;
        for $x in iter {
            $(let $s = $v;)*
            vec_comp![$f $(; for $y in $itery $(;let $t = $w)*)+; if true; myvec]
        }
        myvec
    }};
    //========================IV: used as helpers to above=============================
    // A: iterator helper base case (innermost nested loop i.e. last iterator after expanding multi iterator scenario)
    // used for recursive expansion of nested for loops once number of iterators in macro hits 1
    ($f: expr; for $x: ident in $iterx:expr $(;let $s: ident = $v:expr)*; if $cond: expr; $myvec: ident $(;)*) => {{
        let iter=$iterx;
        for $x in iter {
            $(let $s = $v;)*
            if $cond {
                $myvec.push($f);
            };
        }
    }};
    // B:  helper used to build nesting in multi iterator scenario. only called with 2+ iterators e.g. for ... in ...
    // Ex. let mut myvec = Vec::new()
    ($f: expr; for $x: ident in $iterx:expr $(;let $s: ident = $v:expr)* $(;for $y: ident in $itery:expr $(;let $t: ident = $w:expr)*)+; if $cond: expr; $myvec: ident $(;)*) => {{
        let iter=$iterx;
        for $x in iter {
            $(let $s = $v;)*
            vec_comp![$f $(; for $y in $itery $(;let $t = $w;)*)+; if $cond; $myvec]
        }
    }};
}

fn main() {
    println!("[1] {:?}", vec_comp![x; for x in 1..4; if x>1 ]);
    println!(
        "[2] {:?}",
        vec_comp![x*y*z; for x in 1..4; let y = x*x; let z = 12345]
    );
    println!("[3] {:?}", vec_comp![y; for x in 1..4; let y = x*x; if y>3]);
    println!(
        "[4] {:?}",
        vec_comp![y*z; for x in 1..4; let y = x*x; for z in 1..9; if y>3]
    );
    let mut myvec = Vec::new();
    myvec = vec_comp![zz*z; for x in 1..4; let y = x*x; for z in 1..y; let zz = x*y; if true; using myvec];
    println!("[5] {:?}", myvec);
}
