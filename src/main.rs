mod vec_comp;

#[cfg(not(tarpaulin_include))]
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
    let mut myvec = vec![8, 6, 7, 5, 3, 0, 9];
    myvec = vec_comp![zz*z; for x in 1..4; let y = x*x; for z in 1..y; let zz = x*y; if true; using myvec];
    println!("[5] {:?}", myvec);
    println!(
        "[6] {:?}",
        vec_comp![y+zz*z; for x in 1..4; let y=x*x+4; let z = 3*y+x; for yy in 1..10; let zz= yy+1; if yy<3 && x>1]
    );
}
