#[macro_export]
macro_rules! comp {
    //========================I: single iterator case=============================
    // A: base case of iterator, any number of localized lets and finally a conditional
    // Ex. comp![x; for x in 1..4; if x>1 ] >> [2, 3]
    ($f: expr; for $x: pat in $iterx:expr $(;let $s: ident = $v:expr)*; if $cond: expr $(;)*) => {{
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
    // Ex. comp![x; for x in 1..4] >> [1, 2, 3]
    ($f: expr; for $x: pat in $iterx:expr $(;let $s: ident = $v:expr)* $(;)*) => {{
        comp![$f; for $x in $iterx $(;let $s = $v)*; if true]
     }};
    //========================II: multi iterator case=============================
    // A: base case for multi iterator - let statement pairs WITH conditional
    // Ex. comp![y*z; for x in 1..4; let y = x*x; for z in 1..y; let zz = 45; if x*zz>45] >> [4, 8, 12, 9, 18, 27, 36, 45, 54, 63, 72]
     ($f: expr; for $x: pat in $iterx:expr $(; let $s: ident = $v:expr)* ; if $condx: expr $(; for $y: pat in $itery:expr $(; let $t: ident = $w:expr)*; if $condy: expr)+ $(;)*) => {{
        // boilerplate uses x and looks nearly identical to A
        let mut myvec = Vec::new();
        let iter=$iterx;
        for $x in iter {
            $(let $s = $v;)*
            // recurse for y iterators and lets
            // calling case G until hit the single iterator case and then call F
            if $condx{
                comp![$f $(;for $y in $itery $(;let $t = $w)*; if $condy)+; myvec @INTERNAL]
            }
        }
        myvec
    }};
    // B: base case for multi iterator - let statement pairs WITHOUT x conditional
    // simply wraps A as I-B did I-A
    // Ex. comp![zz*z; for x in 1..4; let y = x*x; for z in 1..y; let zz = x*y; if y > 4] [27, 54, 81, 108, 135, 162, 189, 216]
    ($f: expr; for $x: pat in $iterx:expr $(; let $s: ident = $v:expr)* $(; for $y: pat in $itery:expr $(; let $t: ident = $w:expr)*; if $condy: expr)+ $(;)*) => {
        comp![$f; for $x in $iterx $(; let $s = $v)*; if true $(; for $y in $itery $(;let $t = $w)*; if $condy)+]
    };
    // C:  WITHOUT any conditional(s)
    ($f: expr; for $x: pat in $iterx:expr $(; let $s: ident = $v:expr)* $(; for $y: pat in $itery:expr $(; let $t: ident = $w:expr)*)+ $(;)*) => {
        comp![$f; for $x in $iterx $(; let $s = $v)*; if true $(; for $y in $itery $(;let $t = $w)*; if true)+]
    };
    // D:  WITHOUT y conditional(s)
    ($f: expr; for $x: pat in $iterx:expr $(; let $s: ident = $v:expr)*; if $condx: expr $(; for $y: pat in $itery:expr $(; let $t: ident = $w:expr)*)+ $(;)*) => {
        comp![$f; for $x in $iterx $(; let $s = $v)*; if $condx $(; for $y in $itery $(;let $t = $w)*; if true)+]
    };


    //========================III: w/ preallocated vectors=============================
    // A: base case for multi iterator - let statement pairs WITH conditional
    // Ex. comp![y*z; for x in 1..4; let y = x*x; for z in 1..y; let zz = 45; if x*zz>45] >> [4, 8, 12, 9, 18, 27, 36, 45, 54, 63, 72]
    ($f: expr; for $x: pat in $iterx:expr $(; let $s: ident = $v:expr)* ; if $condx: expr $(; for $y: pat in $itery:expr $(; let $t: ident = $w:expr)*; if $condy: expr)+; using $myvec: expr $(;)*) => {{
        // boilerplate uses x and looks nearly identical to A
        let mut myvec = $myvec;
        let iter=$iterx;
        for $x in iter {
            $(let $s = $v;)*
            // recurse for y iterators and lets
            // calling case G until hit the single iterator case and then call F
            if $condx{
                comp![$f $(;for $y in $itery $(;let $t = $w)*; if $condy)+; myvec @INTERNAL]
            }
        }
        myvec
    }};
    // B: base case for multi iterator - let statement pairs WITHOUT x conditional
    // simply wraps A as I-B did I-A
    // Ex. comp![zz*z; for x in 1..4; let y = x*x; for z in 1..y; let zz = x*y; if y > 4] [27, 54, 81, 108, 135, 162, 189, 216]
    ($f: expr; for $x: pat in $iterx:expr $(; let $s: ident = $v:expr)* $(; for $y: pat in $itery:expr $(; let $t: ident = $w:expr)*; if $condy: expr)+ ; using $myvec: expr $(;)*) => {
        comp![$f; for $x in $iterx $(; let $s = $v)*; if true $(; for $y in $itery $(;let $t = $w)*; if $condy)+; using $myvec]
    };
    // C:  WITHOUT any conditional(s)
    ($f: expr; for $x: pat in $iterx:expr $(; let $s: ident = $v:expr)* $(; for $y: pat in $itery:expr $(; let $t: ident = $w:expr)*)+ ; using $myvec: expr $(;)*) => {
        comp![$f; for $x in $iterx $(; let $s = $v)*; if true $(; for $y in $itery $(;let $t = $w)*; if true)+; using $myvec]
    };
    // D:  WITHOUT y conditional(s)
    ($f: expr; for $x: pat in $iterx:expr $(; let $s: ident = $v:expr)*; if $condx: expr $(; for $y: pat in $itery:expr $(; let $t: ident = $w:expr)*)+ ; using $myvec: expr $(;)*) => {
        comp![$f; for $x in $iterx $(; let $s = $v)*; if $condx $(; for $y in $itery $(;let $t = $w)*; if true)+; using $myvec]
    };
    //========================IV: used as helpers to above=============================
    // A: iterator helper base case (innermost nested loop i.e. last iterator after expanding multi iterator scenario)
    // used for recursive expansion of nested for loops once number of iterators in macro hits 1
    ($f: expr; for $x: pat in $iterx:expr $(;let $s: ident = $v:expr)*; if $cond: expr; $myvec: ident @INTERNAL) => {{
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
    ($f: expr; for $x: pat in $iterx:expr $(;let $s: ident = $v:expr)*; if $condx: expr $(;for $y: pat in $itery:expr $(;let $t: ident = $w:expr)*; if $condy: expr)+; $myvec: ident @INTERNAL) => {{
        let iter=$iterx;
        for $x in iter {
            $(let $s = $v;)*
            if $condx{
                comp![$f $(; for $y in $itery $(;let $t = $w;)*; if $condy)+; $myvec @INTERNAL]
            }
        }
    }};
}
#[cfg(test)]
mod tests {
    #[test]
    fn test_1itr() {
        assert_eq!(comp![x; for x in 1..4], vec![1, 2, 3])
    }
    #[test]
    fn test_1itr_cond() {
        assert_eq!(comp![x; for x in 1..4; if x>1], vec![2, 3])
    }
    #[test]
    fn test_1itr_1decl_cond() {
        assert_eq!(comp![y; for x in 1..4; let y=x*x+4; if x>1], vec![8, 13])
    }
    #[test]
    fn test_1itr_1decl() {
        assert_eq!(comp![y; for x in 1..4; let y=x*x+4], vec![5, 8, 13])
    }
    #[test]
    fn test_1itr_2decl_cond() {
        assert_eq!(
            comp![y+z; for x in 1..4; let y=x*x+4; let z = 3*y+x; if z>20],
            vec![34, 55]
        )
    }
    #[test]
    fn test_2itr_3decl_cond() {
        assert_eq!(
            comp![y+zz*z; for x in 1..4; let y=x*x+4; let z = 3*y+x; for yy in 1..10; let zz= yy+1; if yy<3 && x>1],
            vec![60, 86, 97, 139]
        )
    }
    #[test]
    fn test_2itr_3decl_cond_per() {
        assert_eq!(
            comp![y+zz*z; for x in 1..4; let y=x*x+4; let z = 3*y+x; if x>1; for yy in 1..10; let zz= yy+1; if yy<3],
            vec![60, 86, 97, 139]
        )
    }
    #[test]
    fn test_2itr_3decl_cond_myvec() {
        let myvec = vec![8, 6, 7, 5, 3, 0, 9];
        assert_eq!(
            comp![y+zz*z; for x in 1..4; let y=x*x+4; let z = 3*y+x; for yy in 1..10; let zz= yy+1; if yy<3 && x>1; using myvec],
            vec![8, 6, 7, 5, 3, 0, 9, 60, 86, 97, 139]
        )
    }
    #[test]
    fn test_nesting() {
        assert_eq!(
            comp![comp![y2+z2; for x2 in 1..x; let y2=x*x+4; let z2 = 3*y+x; if z2>20]; for x in 1..4; let y=x*x+4; if x>1],
            vec![vec![34], vec![55, 55]]
        )
    }
    #[test]
    fn test_tuples() {
        assert_eq!(
            comp![(x, y); for (x,y) in (1..4).zip(1..4)],
            vec![(1, 1), (2, 2), (3, 3)]
        )
    }
}
