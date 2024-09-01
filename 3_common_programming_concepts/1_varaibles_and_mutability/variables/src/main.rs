fn main() {
    // attempt_to_change_mutable();

    let mut s = String::from("hello");

    let r1 = &s;
    let r2 = &s;

    println!("{r1} and {r2}");

    let r3 = &mut s;

    println!("{r2} and {r1}");

}

// fn attempt_to_change_immutable() {
//     let x = 5;
//     println!("The value of x is: {x}");
//
//     // cannot assign twice to immutable variable
//     x = 6;
//     println!("The value of x is: {x}");
// }

// fn attempt_to_change_mutable() {
//     let mut x = 5;
//     println!("The value of x is: {x}");
//
//     x = 6;
//     println!("The value of x is: {x}");
// }
