fn main() {
    block();
    ownership();
    reference();
    borrow();
    dereference();
}

fn block() -> String {
    let statement: String = String::from("state"); //文
    let expression: String = String::from("expression"); //四季

    statement; //statement(文)は処理を実行するが値を返さない。セミコロンは式を文に変化させる。
    expression //expression(式)は何かしらの値を返す。
}

fn ownership() {
    let object: &str = "aaa";
    let a = object;

    //数値型、strは実行可。String型は実行不可。　※プリミティブ型は実行可。
    println!("{}", object);
    println!("{}", a);
}

fn reference() {
    let object = String::from("rrr");
    let a = &object;

    //参照を渡しているため、渡す側(object)は束縛されない。
    println!("{}", object);
    println!("{}", a);
}

fn borrow() {
    //不変参照 (&) は何個でも同時に存在することができる
    //不変参照 (&) と可変参照 (&mut) は同時に存在することができない
    //可変参照 (&mut) は同時に1つしか存在することができない

    //OKパターン
    let a = 10; // immutable object
    let aref1 = &a; // reference
    let aref2 = &a; // reference
    println!("{}, {}, {}", a, aref1, aref2); // borrow check!! - OK

    //OKパターン
    let mut a = 10; // mutable object
    let a_ref1 = &a; // reference
    let a_mut_ref1 = &mut a; // mutable reference
    let a_mut_ref2 = &mut a; // mutable refernece
    *a_mut_ref2 = 20; // assign
    println!("{}", a); // borrow check!! - OK

    // //NGパターン 可変参照が複数存在
    // let mut a = 10; // mutable object
    // let a_ref1 = &a; // reference
    // let a_mut_ref1 = &mut a; // mutable reference
    // let a_mut_ref2 = &mut a; // この時点で a_ref1, a_mut_ref1 は存在しない
    // *a_mut_ref1 = 20; // assign (error)
    // println!("{}", a); // borrow check!! - Error!

    // //NGパターン　可変参照と不変参照が同時に存在
    // let mut a = 10; // mutable object
    // let a_ref1 = &a; // reference
    // let a_mut_ref1 = &mut a; // mutable reference
    // let a_mut_ref2 = &mut a; // この時点で a_ref1, a_mut_ref1 は存在しない
    // println!("{}", a_ref1); // borrow check!! - Error!

    //OKパターン　不変参照や可変参照を複数束縛しているコードでも借用チェック時に制約を満たしていればコンパイルは通る。
    let mut a = 10; // mutable object
    let a_ref1 = &a; // reference
    let a_mut_ref1 = &mut a; // mutable reference
    let a_mut_ref2 = &mut a; // この時点で a_ref1, a_mut_ref1 は存在しない
    let a_ref2 = &a; // この時点で a_mut_ref2 は存在しない
                     //println!("{}", a_mut_ref2);        // borrow check!! - Error!
                     //println!("{} {}", a_ref1, a_ref2); // borrow check!! - Error!
    println!("{}", a_ref2); // borrow check!! - OK
}

fn dereference() {
    let mut a = 10; // mutable object
    let a_mut_ref = &mut a; // mutable reference
    *a_mut_ref = 20; // dereference and assign
    println!("{}", a_mut_ref); // auto dereference
}
