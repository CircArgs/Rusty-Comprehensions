#[macro_export]
macro_rules! dcomp {

    //outward facing pattern:
    // f is value dumped into vec
    // rest token tree placeholder for generic for (lets) (if) pattern
    // sets up vec to push to
    ($fk: expr => $fv: expr; $($rest:tt)+) => {{
        let mut myvec = std::collections::HashMap::new();
        dcomp![@INTERNAL(myvec) $fk => $fv; $($rest)+];
        myvec
    }};
    //outward facing pattern for given datatype
    // f is value dumped into vec
    // rest token tree placeholder for generic for (lets) (if) pattern
    // sets up vec to push to
    (using $myvec: expr , $fk: expr => $fv: expr; $($rest:tt)+) => {{
        let mut myvec = $myvec;
        dcomp![@INTERNAL(myvec) $fk => $fv; $($rest)+];
        myvec
    }};
    //================================INTERNAL Patterns===========================================
    // BASE CASE WITH IF
    // internal pattern for iter (lets) if
    (@INTERNAL($myvec:ident) $fk: expr => $fv: expr; for $x:pat in $iterx:expr $(,let $v: ident = $vv: expr)*, if $cond:expr) => {
        let iter = $iterx;
        for $x in iter{
            $(let $v = $vv;)*
            if $cond {
                $myvec.insert($fk, $fv);
            }
        }
    };
    // BASE CASE WITHOUT IF
    // internal pattern for iter (lets) NO if (calls version with if as true. compiler will optmize it away)
    (@INTERNAL($myvec:ident) $fk: expr => $fv: expr; for $x:pat in $iterx:expr $(,let $v: ident = $vv: expr)*) => {
        dcomp![@INTERNAL($myvec) $fk => $fv; for $x in $iterx $(,let $v = $vv)*, if true ];
    };
    // Recurse case
    // internal pattern for iter (lets) if; repeat+ (1 or more times i.e. not base case)
    (@INTERNAL($myvec:ident) $fk: expr => $fv: expr; for $x:pat in $iterx:expr $(,let $v: ident = $vv: expr)*, if $cond:expr; $($rest:tt)+) => {
        let iter = $iterx;
        for $x in iter{
            $(let $v = $vv;)*
            if $cond {
                dcomp![@INTERNAL($myvec) $fk => $fv; $($rest)+];
            }
        }
    };
    // Recurse case
    // internal pattern for iter (lets) if; repeat+ (1 or more times i.e. not base case)
    (@INTERNAL($myvec:ident) $fk: expr => $fv: expr; for $x:pat in $iterx:expr $(,let $v: ident = $vv: expr)*; $($rest:tt)+) => {
        dcomp![@INTERNAL($myvec) $fk => $fv; for $x in $iterx $(,let $v = $vv)*, if true; $($rest)+ ];
    };

}

#[cfg(test)]
mod tests {
    #[test]
    fn test_1itr() {}
}
