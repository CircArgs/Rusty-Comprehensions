#[macro_export]
macro_rules! dict_pre {
    ($k: expr => $v: expr $(,$fk: expr => $fv: expr)*) => {{
        let mut _tot_count: usize = 0;
        let mut mydict= std::collections::HashMap::new();
        mydict.insert($k, $v);
        dict_pre!{@INTERNAL(mydict, _tot_count) $($fk => $fv, )*}
        mydict
    }};
    (@INTERNAL($mydict: ident, $tot_count: ident) $k: expr => $v: expr $(,$fk: expr => $fv: expr)+ $(,)*) => {
        $tot_count+=1;
        dict_pre!{@INTERNAL($mydict, $tot_count)  $($fk => $fv,)*}
        $mydict.insert($k, $v);
    };
    (@INTERNAL($mydict: ident, $tot_count: ident) $k: expr => $v: expr $(,)*) => {
        $tot_count+=1;
        $mydict.reserve($tot_count);
        $mydict.insert($k, $v);
    };
    ()=>{std::collections::HashMap::new()};
}
