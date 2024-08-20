fn main() {
    attempt_to_change_mutable();
}

// fn attempt_to_change_immutable() {
//     let x = 5;
//     println!("The value of x is: {x}");
//
//     // cannot assign twice to immutable variable
//     x = 6;
//     println!("The value of x is: {x}");
// }

fn attempt_to_change_mutable() {
    let mut x = 5;
    println!("The value of x is: {x}");

    x = 6;
    println!("The value of x is: {x}");
}
