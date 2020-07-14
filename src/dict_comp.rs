#[macro_export]
macro_rules! dcomp {

    //outward facing pattern:
    // f is value dumped into vec
    // rest token tree placeholder for generic for (lets) (if) pattern
    // sets up vec to push to
    ($fk: expr => $fv: expr; $($rest:tt)+) => {{
        let mut mydict = std::collections::HashMap::new();
        dcomp![@INTERNAL(mydict) $fk => $fv; $($rest)+];
        mydict
    }};
    //outward facing pattern for given datatype
    // f is value dumped into vec
    // rest token tree placeholder for generic for (lets) (if) pattern
    // sets up vec to push to
    (using $mydict: expr , $fk: expr => $fv: expr; $($rest:tt)+) => {{
        let mut mydict = $mydict;
        dcomp![@INTERNAL(mydict) $fk => $fv; $($rest)+];
        mydict
    }};
    //================================INTERNAL Patterns===========================================
    // BASE CASE WITH IF
    // internal pattern for iter (lets) if
    (@INTERNAL($mydict:ident) $fk: expr => $fv: expr; for $x:pat in $iterx:expr $(,let $v: ident = $vv: expr)*, if $cond:expr) => {
        let iter = $iterx;
        for $x in iter{
            $(let $v = $vv;)*
            if $cond {
                $mydict.insert($fk, $fv);
            }
        }
    };
    // BASE CASE WITHOUT IF
    // internal pattern for iter (lets) NO if (calls version with if as true. compiler will optmize it away)
    (@INTERNAL($mydict:ident) $fk: expr => $fv: expr; for $x:pat in $iterx:expr $(,let $v: ident = $vv: expr)*) => {
        dcomp![@INTERNAL($mydict) $fk => $fv; for $x in $iterx $(,let $v = $vv)*, if true ];
    };
    // Recurse case
    // internal pattern for iter (lets) if; repeat+ (1 or more times i.e. not base case)
    (@INTERNAL($mydict:ident) $fk: expr => $fv: expr; for $x:pat in $iterx:expr $(,let $v: ident = $vv: expr)*, if $cond:expr; $($rest:tt)+) => {
        let iter = $iterx;
        for $x in iter{
            $(let $v = $vv;)*
            if $cond {
                dcomp![@INTERNAL($mydict) $fk => $fv; $($rest)+];
            }
        }
    };
    // Recurse case
    // internal pattern for iter (lets) if; repeat+ (1 or more times i.e. not base case)
    (@INTERNAL($mydict:ident) $fk: expr => $fv: expr; for $x:pat in $iterx:expr $(,let $v: ident = $vv: expr)*; $($rest:tt)+) => {
        dcomp![@INTERNAL($mydict) $fk => $fv; for $x in $iterx $(,let $v = $vv)*, if true; $($rest)+ ];
    };

}
#[cfg(test)]
mod tests {
    use crate::dict;

    #[test]
    fn test_1itr() {
        assert_eq!(dcomp![x*x => x; for x in 1..4], dict! {9=> 3, 1=> 1, 4=> 2})
    }
    #[test]
    fn test_1itr_cond() {
        assert_eq!(
            dcomp![x*x => x; for x in 1..4, if x>1],
            dict! {4=> 2, 9=> 3}
        )
    }
    #[test]
    fn test_1itr_1decl_cond() {
        assert_eq!(
            dcomp![x=>y; for x in 1..4, let y=x*x+4, if x>1],
            dict! {2=> 8, 3=> 13}
        )
    }
    #[test]
    fn test_1itr_1decl() {
        assert_eq!(
            dcomp![x=>y; for x in 1..4, let y=x*x+4],
            dict! {1=> 5, 3=> 13, 2=> 8}
        )
    }
    #[test]
    fn test_1itr_2decl_cond() {
        assert_eq!(
            dcomp![x => y+z; for x in 1..4, let y=x*x+4, let z = 3*y+x, if z>20],
            dict! {2=> 34, 3=> 55}
        )
    }
    #[test]
    fn test_2itr_3decl_cond() {
        assert_eq!(
            dcomp![x => y+zz*z; for x in 1..4, let y=x*x+4, let z = 3*y+x; for yy in 1..10, let zz= yy+1, if yy<3 && x>1],
            dict! {3=> 139, 2=> 86}
        )
    }
    #[test]
    fn test_2itr_3decl_cond_per() {
        assert_eq!(
            dcomp![x => y+zz*z; for x in 1..4, let y=x*x+4, let z = 3*y+x, if x>1; for yy in 1..10, let zz= yy+1, if yy<3],
            dict! {3=> 139, 2=> 86}
        )
    }
    #[test]
    fn test_nesting() {
        assert_eq!(
            dcomp![x => dcomp![x => y2+z2; for x2 in 1..x, let y2=x*x+4, let z2 = 3*y+x, if z2>20]; for x in 1..4, let y=x*x+4, if x>1],
            dict! {2=> dict!{2=> 34}, 3=> dict!{3=> 55}}
        )
    }
    #[test]
    fn test_tuples() {
        assert_eq!(
            dcomp![x=>(x, y); for (x,y) in (1..4).zip(1..4)],
            dict! {3=> (3, 3), 1=> (1, 1), 2=> (2, 2)}
        )
    }
    #[test]
    fn test_complicated() {
        assert_eq!(
            dcomp![x=>y+zz*z; for x in 1..4, let y=x*x+4, let z = 3*y+x, if z>20; for yy in 1..10, let zz= yy+1; for yyy in 1..10, if yy>7; for i in 1..3],
            dict! {3=> 433, 2=> 268}
        )
    }
}
