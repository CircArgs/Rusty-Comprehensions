#[macro_export]
macro_rules! comp_pre {

    //outward facing pattern:
    // f is value dumped into vec
    // rest token tree placeholder for generic for (lets) (if) pattern
    // sets up vec to push to
    ($f:expr; $($rest:tt)+) => {{
        let mut _tot_depth: usize = 0;
        let mut _cap: usize = 1;
        let mut myvec = Vec::new();
        comp_pre![@INTERNAL(myvec, 0, _tot_depth, _cap) $f; $($rest)+];
        myvec
    }};

    //================================INTERNAL Patterns===========================================
    // BASE CASE WITH IF
    // internal pattern for iter (lets) if
    (@INTERNAL($myvec:ident, $depth: expr, $tot_depth:ident, $cap:ident) $f:expr; for $x:pat in $iterx:expr $(,let $v: ident = $vv: expr)*, if $cond:expr) => {
        let iter=std::iter::IntoIterator::into_iter($iterx);
        if ($depth) == $tot_depth{
            $tot_depth+=1;
            $myvec.reserve($cap*iter.size_hint().0);
        };
        for $x in iter{
            $(let $v = $vv;)*
            if $cond {
                $myvec.push($f);
            }
        }
    };
    // BASE CASE WITHOUT IF
    // internal pattern for iter (lets) NO if (calls version with if as true. comp_preiler will optmize it away)
    (@INTERNAL($myvec:ident, $depth: expr, $tot_depth:ident, $cap:ident) $f:expr; for $x:pat in $iterx:expr $(,let $v: ident = $vv: expr)*) => {
        comp_pre![@INTERNAL($myvec, $depth, $tot_depth, $cap) $f; for $x in $iterx $(,let $v = $vv)*, if true ];
    };
    // Recurse case
    // internal pattern for iter (lets) if; repeat+ (1 or more times i.e. not base case)
    (@INTERNAL($myvec:ident, $depth: expr, $tot_depth:ident, $cap:ident) $f:expr; for $x:pat in $iterx:expr $(,let $v: ident = $vv: expr)*, if $cond:expr; $($rest:tt)+) => {
        let iter=std::iter::IntoIterator::into_iter($iterx);
        if ($depth) == $tot_depth{
            $tot_depth+=1;
            $cap*=iter.size_hint().0;
        };
        for $x in iter{
            $(let $v = $vv;)*
            if $cond {
                comp_pre![@INTERNAL($myvec, $depth+1, $tot_depth, $cap) $f; $($rest)+];
            }
        }
    };
    // Recurse case
    // internal pattern for iter (lets) if; repeat+ (1 or more times i.e. not base case)
    (@INTERNAL($myvec:ident, $depth: expr, $tot_depth:ident, $cap:ident) $f:expr; for $x:pat in $iterx:expr $(,let $v: ident = $vv: expr)*; $($rest:tt)+) => {
        comp_pre![@INTERNAL($myvec, $depth, $tot_depth, $cap) $f; for $x in $iterx $(,let $v = $vv)*, if true; $($rest)+ ];
    };

}

#[cfg(test)]
mod tests {
    #[test]
    fn test_1itr() {
        assert_eq!(comp_pre![x; for x in 1..4], vec![1, 2, 3])
    }
    #[test]
    fn test_1itr_cond() {
        assert_eq!(comp_pre![x; for x in 1..4, if x>1], vec![2, 3])
    }
    #[test]
    fn test_1itr_1decl_cond() {
        assert_eq!(
            comp_pre![y; for x in 1..4, let y=x*x+4, if x>1],
            vec![8, 13]
        )
    }
    #[test]
    fn test_1itr_1decl() {
        assert_eq!(comp_pre![y; for x in 1..4, let y=x*x+4], vec![5, 8, 13])
    }
    #[test]
    fn test_1itr_2decl_cond() {
        assert_eq!(
            comp_pre![y+z; for x in 1..4, let y=x*x+4, let z = 3*y+x, if z>20],
            vec![34, 55]
        )
    }
    #[test]
    fn test_2itr_3decl_cond() {
        assert_eq!(
            comp_pre![y+zz*z; for x in 1..4, let y=x*x+4, let z = 3*y+x; for yy in 1..10, let zz= yy+1, if yy<3 && x>1],
            vec![60, 86, 97, 139]
        )
    }
    #[test]
    fn test_2itr_3decl_cond_per() {
        assert_eq!(
            comp_pre![y+zz*z; for x in 1..4, let y=x*x+4, let z = 3*y+x, if x>1; for yy in 1..10, let zz= yy+1, if yy<3],
            vec![60, 86, 97, 139]
        )
    }
    #[test]
    fn test_nesting() {
        assert_eq!(
            comp_pre![comp_pre![y2+z2; for x2 in 1..x, let y2=x*x+4, let z2 = 3*y+x, if z2>20]; for x in 1..4, let y=x*x+4, if x>1],
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
    #[test]
    fn test_complicated() {
        assert_eq!(
            comp_pre![y+zz*z; for x in 1..4, let y=x*x+4, let z = 3*y+x, if z>20; for yy in 1..10, let zz= yy+1; for yyy in 1..10, if yy>7; for i in 1..3],
            vec![
                242, 242, 242, 242, 242, 242, 242, 242, 242, 242, 242, 242, 242, 242, 242, 242,
                242, 242, 268, 268, 268, 268, 268, 268, 268, 268, 268, 268, 268, 268, 268, 268,
                268, 268, 268, 268, 391, 391, 391, 391, 391, 391, 391, 391, 391, 391, 391, 391,
                391, 391, 391, 391, 391, 391, 433, 433, 433, 433, 433, 433, 433, 433, 433, 433,
                433, 433, 433, 433, 433, 433, 433, 433
            ]
        )
    }
}
