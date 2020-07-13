#[macro_export]
macro_rules! dcomp {
    //========================I: single iterator case=============================
    // A: base case of iterator, any number of localized lets and finally a conditional
    // Ex. dcomp![x; for x in 1..4; if x>1 ] >> [2, 3]
    ($fk: expr => $fv: expr; for $x: pat in $iterx:expr $(;let $s: ident = $v:expr)*; if $cond: expr $(;)*) => {{
        use std::collections::HashMap;
        let mut mymap = HashMap::new();
        let iter=$iterx;
        for $x in iter {
            $(let $s = $v;)*
            if $cond {
                mymap.insert($fk, $fv);
            };
        }
        mymap
    }};
    // B: A without a conditional; bootstraps A to be called with condition of true
    // Ex. dcomp![x; for x in 1..4] >> [1, 2, 3]
    ($fk: expr => $fv: expr; for $x: pat in $iterx:expr $(;let $s: ident = $v:expr)* $(;)*) => {{
        dcomp![$fk=>$fv; for $x in $iterx $(;let $s = $v)*; if true]
     }};
    //========================II: multi iterator case=============================
    // A: base case for multi iterator - let statement pairs WITH conditional
    // Ex. dcomp![y*z; for x in 1..4; let y = x*x; for z in 1..y; let zz = 45; if x*zz>45] >> [4, 8, 12, 9, 18, 27, 36, 45, 54, 63, 72]
     ($fk: expr => $fv: expr; for $x: pat in $iterx:expr $(; let $s: ident = $v:expr)* ; if $condx: expr $(; for $y: pat in $itery:expr $(; let $t: ident = $w:expr)*; if $condy: expr)+ $(;)*) => {{
        // boilerplate uses x and looks nearly identical to A
        use std::collections::HashMap;
        let mut mymap = HashMap::new();
        let iter=$iterx;
        for $x in iter {
            $(let $s = $v;)*
            // recurse for y iterators and lets
            // calling case G until hit the single iterator case and then call F
            if $condx{
                dcomp![$fk=>$fv $(;for $y in $itery $(;let $t = $w)*; if $condy)+; mymap]
            }
        }
        mymap
    }};
    // B: base case for multi iterator - let statement pairs WITHOUT x conditional
    // simply wraps A as I-B did I-A
    // Ex. dcomp![zz*z; for x in 1..4; let y = x*x; for z in 1..y; let zz = x*y; if y > 4] [27, 54, 81, 108, 135, 162, 189, 216]
    ($fk: expr => $fv: expr; for $x: pat in $iterx:expr $(; let $s: ident = $v:expr)* $(; for $y: pat in $itery:expr $(; let $t: ident = $w:expr)*; if $condy: expr)+ $(;)*) => {
        dcomp![$fk=>$fv; for $x in $iterx $(; let $s = $v)*; if true $(; for $y in $itery $(;let $t = $w)*; if $condy)+]
    };
    // C:  WITHOUT any conditional(s)
    ($fk: expr => $fv: expr; for $x: pat in $iterx:expr $(; let $s: ident = $v:expr)* $(; for $y: pat in $itery:expr $(; let $t: ident = $w:expr)*)+ $(;)*) => {
        dcomp![$fk=>$fv; for $x in $iterx $(; let $s = $v)*; if true $(; for $y in $itery $(;let $t = $w)*; if true)+]
    };
    // D:  WITHOUT y conditional(s)
    ($fk: expr => $fv: expr; for $x: pat in $iterx:expr $(; let $s: ident = $v:expr)*; if $condx: expr $(; for $y: pat in $itery:expr $(; let $t: ident = $w:expr)*)+ $(;)*) => {
        dcomp![$fk=>$fv; for $x in $iterx $(; let $s = $v)*; if $condx $(; for $y in $itery $(;let $t = $w)*; if true)+]
    };


    //========================III: w/ preallocated vectors=============================
    // A: base case for multi iterator - let statement pairs WITH conditional
    // Ex. dcomp![y*z; for x in 1..4; let y = x*x; for z in 1..y; let zz = 45; if x*zz>45] >> [4, 8, 12, 9, 18, 27, 36, 45, 54, 63, 72]
    ($fk: expr => $fv: expr; for $x: pat in $iterx:expr $(; let $s: ident = $v:expr)* ; if $condx: expr $(; for $y: pat in $itery:expr $(; let $t: ident = $w:expr)*; if $condy: expr)+; using $mymap: expr $(;)*) => {{
        // boilerplate uses x and looks nearly identical to A
        let mut mymap = $mymap;
        let iter=$iterx;
        for $x in iter {
            $(let $s = $v;)*
            // recurse for y iterators and lets
            // calling case G until hit the single iterator case and then call F
            if $condx{
                dcomp![$fk=>$fv $(;for $y in $itery $(;let $t = $w)*; if $condy)+; mymap]
            }
        }
        mymap
    }};
    // B: base case for multi iterator - let statement pairs WITHOUT x conditional
    // simply wraps A as I-B did I-A
    // Ex. dcomp![zz*z; for x in 1..4; let y = x*x; for z in 1..y; let zz = x*y; if y > 4] [27, 54, 81, 108, 135, 162, 189, 216]
    ($fk: expr => $fv: expr; for $x: pat in $iterx:expr $(; let $s: ident = $v:expr)* $(; for $y: pat in $itery:expr $(; let $t: ident = $w:expr)*; if $condy: expr)+ ; using $mymap: expr $(;)*) => {
        dcomp![$fk=>$fv; for $x in $iterx $(; let $s = $v)*; if true $(; for $y in $itery $(;let $t = $w)*; if $condy)+; using $mymap]
    };
    // C:  WITHOUT any conditional(s)
    ($fk: expr => $fv: expr; for $x: pat in $iterx:expr $(; let $s: ident = $v:expr)* $(; for $y: pat in $itery:expr $(; let $t: ident = $w:expr)*)+ ; using $mymap: expr $(;)*) => {
        dcomp![$fk=>$fv; for $x in $iterx $(; let $s = $v)*; if true $(; for $y in $itery $(;let $t = $w)*; if true)+; using $mymap]
    };
    // D:  WITHOUT y conditional(s)
    ($fk: expr => $fv: expr; for $x: pat in $iterx:expr $(; let $s: ident = $v:expr)*; if $condx: expr $(; for $y: pat in $itery:expr $(; let $t: ident = $w:expr)*)+ ; using $mymap: expr $(;)*) => {
        dcomp![$fk=>$fv; for $x in $iterx $(; let $s = $v)*; if $condx $(; for $y in $itery $(;let $t = $w)*; if true)+; using $mymap]
    };
    //========================IV: used as helpers to above=============================
    // A: iterator helper base case (innermost nested loop i.e. last iterator after expanding multi iterator scenario)
    // used for recursive expansion of nested for loops once number of iterators in macro hits 1
    ($fk: expr => $fv: expr; for $x: pat in $iterx:expr $(;let $s: ident = $v:expr)*; if $cond: expr; $mymap: ident $(;)*) => {{
        let iter=$iterx;
        for $x in iter {
            $(let $s = $v;)*
            if $cond {
                $mymap.insert($fk, $fv);
            };
        }
    }};
    // B:  helper used to build nesting in multi iterator scenario. only called with 2+ iterators e.g. for ... in ...
    // Ex. let mut mymap = HashMap::new()
    ($fk: expr => $fv: expr; for $x: pat in $iterx:expr $(;let $s: ident = $v:expr)*; if $condx: expr $(;for $y: pat in $itery:expr $(;let $t: ident = $w:expr)*; if $condy: expr)+; $mymap: ident $(;)*) => {{
        let iter=$iterx;
        for $x in iter {
            $(let $s = $v;)*
            if $condx{
                dcomp![$fk=>$fv $(; for $y in $itery $(;let $t = $w;)*; if $condy)+; $mymap]
            }
        }
    }};
}
#[cfg(test)]
mod tests {
    #[test]
    fn test_1itr() {}
}
