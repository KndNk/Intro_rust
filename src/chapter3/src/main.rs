fn main() {
    copy_trait();
}

fn copy_trait() {
    //Rustのプリミティブ型はコピートレイトを実装している。
    let a = 10; // immutable object
    let b = a; // copy
    println!("{} {}", a, b); // borrow check!! - OK

    //不変参照(&)もコピートレイトを実装している。
    let a = 20; // immutable object
    let a_ref = &a; // reference
    let a_ref_copy = a_ref; // copy reference
    println!("{} {} {}", a, a_ref, a_ref_copy); // borrow check!! - OK

    // //可変参照(&mut)はコピートレイトを実装していない。
    // let mut a = 30; // mutable object
    // let a_mut_ref = &mut a; // mutable reference
    // let a_mut_ref_move = a_mut_ref; // move mutable reference
    // println!("{}", a_mut_ref); // borrow check!! - Error!
}
