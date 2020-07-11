macro_rules! vec_comp {
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

    ($f: expr; for $x: ident in $iterx:expr $(;let $s: ident = $v:expr)* $(;)*) => {{
        vec_comp![$f; for $x in $iterx $(;let $s = $v)*; if true]
     }};

    ($f: expr; $x: ident in $iterx:expr $(;let $s: ident = $v:expr;)* $(;for $y: ident in $itery:expr $(;let $t: ident = $w:expr)*)* $(;)*) => {
        vec_comp![$f; for $x in $iterx $(; let $s = $v)* $(; $y in $itery $(;let $t = $w)*)*; if true]
    };
    ($f: expr; for $x: ident in $iterx:expr $(;let $s: ident = $v:expr)* $(;for $y: ident in $itery:expr $(;let $t: ident = $w:expr;)*)+; if $cond: expr $(;)*) => {{
        let mut myvec = Vec::new();
        let iter=$iterx;
        for $x in iter {
            $(let $s = $v;)*
            vec_comp![$f $(;for $y in $itery $(;let $t = $w)*)+; if $cond; myvec]
        }
        myvec
    }};

    ($f: expr; for $x: ident in $iterx:expr $(;let $s: ident = $v:expr)*; if $cond: expr; $myvec: ident) => {{
        let iter=$iterx;
        for $x in iter {
            $(let $s = $v;)*
            if $cond {
                $myvec.push($f);
            };
        }
    }};

    ($f: expr; for $x: ident in $iterx:expr $(;let $s: ident = $v:expr)* $(;for $y: ident in $itery:expr $(;let $t: ident = $w:expr)*)+; if $cond: expr; $myvec: ident) => {{
        let iter=$iterx;
        for $x in iter {
            $(let $s = $v;)*
            vec_comp![$f; $(; $y in $itery $(;let $t = $w;)*)+; if $cond; $myvec]
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
    println!(
        "[5] {:?}",
        vec_comp![y*z; for x in 1..4; let y = x*x; for z in 1..y; if y>3]
    );
}
