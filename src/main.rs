use tokio::*; // comment this line to make it compile ok

fn main() {
    println!("Hello, world!");

    let a = String::from("a");
    let b = String::from("b");

    println!("{:?}", a > b);

    println!("{:?}", a.ge(&b)); // ok
    println!("{:?}", &a.ge(&b)); // ok

    // below compile ok when not `use tokio::*`;
    // when `use tokio::*`, got compile error : the trait `PartialOrd<&String>` is not implemented for `String`
    println!("{:?}", a.ge(&&b));
    println!("{:?}", &a.ge(&&b));
    let rb = &b;
    println!("{:?}", a.ge(&rb));
    println!("{:?}", &a.ge(&rb));
}
