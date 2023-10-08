fn main() {
    let a = [1, 2, 3, 4, 5];
    let a_slice = &a[1..3];
    dbg!(a_slice); // [2,3]
}
