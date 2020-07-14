#[macro_export]
macro_rules! dict {
    ($k: expr => $v: expr $(,$fk: expr => $fv: expr)*) => {{
        let mut mydict = std::collections::HashMap::new();
        mydict.insert($k, $v);
        $(mydict.insert($fk, $fv);)*
        mydict
    }};

    ()=>{std::collections::HashMap::new()};
}
