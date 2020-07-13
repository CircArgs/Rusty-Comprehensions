use std::sync::{Once, ONCE_INIT};
static SYNC_OBJ: Once = ONCE_INIT;

#[macro_export]
macro_rules! comp_pre {
    //========================I: single iterator case=============================
    // A: base case of iterator, any number of localized lets and finally a conditional
    // Ex. comp_pre![x; for x in 1..4; if x>1 ] >> [2, 3]
    ($f: expr; for $x: pat in $iterx:expr $(;let $s: ident = $v:expr)*; if $cond: expr $(;)*) => {{
        let iter=std::iter::IntoIterator::into_iter($iterx);
        let mut myvec = Vec::with_capacity(iter.size_hint().0);
        for $x in iter {
            $(let $s = $v;)*
            if $cond {
                myvec.push($f);
            };
        }
        myvec
    }};
    // B: A without a conditional; bootstraps A to be called with condition of true
    // Ex. comp_pre![x; for x in 1..4] >> [1, 2, 3]
    ($f: expr; for $x: pat in $iterx:expr $(;let $s: ident = $v:expr)* $(;)*) => {{
        comp_pre![$f; for $x in $iterx $(;let $s = $v)*; if true]
     }};
    //========================II: multi iterator case=============================
    // A: base case for multi iterator - let statement pairs WITH conditional
    // Ex. comp_pre![y*z; for x in 1..4; let y = x*x; for z in 1..y; let zz = 45; if x*zz>45] >> [4, 8, 12, 9, 18, 27, 36, 45, 54, 63, 72]
     ($f: expr; for $x: pat in $iterx:expr $(; let $s: ident = $v:expr)* ; if $condx: expr $(; for $y: pat in $itery:expr $(; let $t: ident = $w:expr)*; if $condy: expr)+ $(;)*) => {{
        // boilerplate uses x and looks nearly identical to A
        let iter=std::iter::IntoIterator::into_iter($iterx);
        let mut myvec = Vec::with_capacity(iter.size_hint().0);
        for $x in iter {
            $(let $s = $v;)*
            // recurse for y iterators and lets
            // calling case G until hit the single iterator case and then call F
            if $condx{
                comp_pre![$f $(;for $y in $itery $(;let $t = $w)*; if $condy)+; myvec]
            }
        }
        myvec
    }};
    // B: base case for multi iterator - let statement pairs WITHOUT x conditional
    // simply wraps A as I-B did I-A
    // Ex. comp_pre![zz*z; for x in 1..4; let y = x*x; for z in 1..y; let zz = x*y; if y > 4] [27, 54, 81, 108, 135, 162, 189, 216]
    ($f: expr; for $x: pat in $iterx:expr $(; let $s: ident = $v:expr)* $(; for $y: pat in $itery:expr $(; let $t: ident = $w:expr)*; if $condy: expr)+ $(;)*) => {
        comp_pre![$f; for $x in $iterx $(; let $s = $v)*; if true $(; for $y in $itery $(;let $t = $w)*; if $condy)+]
    };
    // C:  WITHOUT any conditional(s)
    ($f: expr; for $x: pat in $iterx:expr $(; let $s: ident = $v:expr)* $(; for $y: pat in $itery:expr $(; let $t: ident = $w:expr)*)+ $(;)*) => {
        comp_pre![$f; for $x in $iterx $(; let $s = $v)*; if true $(; for $y in $itery $(;let $t = $w)*; if true)+]
    };
    // D:  WITHOUT y conditional(s)
    ($f: expr; for $x: pat in $iterx:expr $(; let $s: ident = $v:expr)*; if $condx: expr $(; for $y: pat in $itery:expr $(; let $t: ident = $w:expr)*)+ $(;)*) => {
        comp_pre![$f; for $x in $iterx $(; let $s = $v)*; if $condx $(; for $y in $itery $(;let $t = $w)*; if true)+]
    };


    //========================III: used as helpers to above=============================
    // A: iterator helper base case (innermost nested loop i.e. last iterator after expanding multi iterator scenario)
    // used for recursive expansion of nested for loops once number of iterators in macro hits 1
    ($f: expr; for $x: pat in $iterx:expr $(;let $s: ident = $v:expr)*; if $cond: expr; $myvec: ident $(;)*) => {{
        let iter=std::iter::IntoIterator::into_iter($iterx);
        SYNC_OBJ.call_once(|| {
            $myvec.reserve(iter.size_hint().0)
        });
        for $x in iter {
            $(let $s = $v;)*
            if $cond {
                $myvec.push($f);
            };
        }
    }};
    // B:  helper used to build nesting in multi iterator scenario. only called with 2+ iterators e.g. for ... in ...
    // Ex. let mut myvec = Vec::new()
    ($f: expr; for $x: pat in $iterx:expr $(;let $s: ident = $v:expr)*; if $condx: expr $(;for $y: pat in $itery:expr $(;let $t: ident = $w:expr)*; if $condy: expr)+; $myvec: ident $(;)*) => {{
        let iter=std::iter::IntoIterator::into_iter($iterx);
        SYNC_OBJ.call_once(|| {
            $myvec.reserve(iter.size_hint().0)
        });
        for $x in iter {
            $(let $s = $v;)*
            if $condx{
                comp_pre![$f $(; for $y in $itery $(;let $t = $w;)*; if $condy)+; $myvec]
            }
        }
    }};
}
#[cfg(test)]
mod tests {
    #[test]
    fn test_1itr() {
        assert_eq!(comp_pre![x; for x in 1..4], vec![1, 2, 3])
    }
    #[test]
    fn test_1itr_cond() {
        assert_eq!(comp_pre![x; for x in 1..4; if x>1], vec![2, 3])
    }
    #[test]
    fn test_1itr_1decl_cond() {
        assert_eq!(
            comp_pre![y; for x in 1..4; let y=x*x+4; if x>1],
            vec![8, 13]
        )
    }
    #[test]
    fn test_1itr_1decl() {
        assert_eq!(comp_pre![y; for x in 1..4; let y=x*x+4], vec![5, 8, 13])
    }
    #[test]
    fn test_1itr_2decl_cond() {
        assert_eq!(
            comp_pre![y+z; for x in 1..4; let y=x*x+4; let z = 3*y+x; if z>20],
            vec![34, 55]
        )
    }
    #[test]
    fn test_2itr_3decl_cond() {
        assert_eq!(
            comp_pre![y+zz*z; for x in 1..4; let y=x*x+4; let z = 3*y+x; for yy in 1..10; let zz= yy+1; if yy<3 && x>1],
            vec![60, 86, 97, 139]
        )
    }
    #[test]
    fn test_2itr_3decl_cond_per() {
        assert_eq!(
            comp_pre![y+zz*z; for x in 1..4; let y=x*x+4; let z = 3*y+x; if x>1; for yy in 1..10; let zz= yy+1; if yy<3],
            vec![60, 86, 97, 139]
        )
    }

    #[test]
    fn test_nesting() {
        assert_eq!(
            comp_pre![comp_pre![y2+z2; for x2 in 1..x; let y2=x*x+4; let z2 = 3*y+x; if z2>20]; for x in 1..4; let y=x*x+4; if x>1],
            vec![vec![34], vec![55, 55]]
        )
    }
    #[test]
    fn test_tuples() {
        assert_eq!(
            comp_pre![(x, y); for (x,y) in (1..4).zip(1..4)],
            vec![(1, 1), (2, 2), (3, 3)]
        )
    }
}
