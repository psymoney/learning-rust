### Constants
1. defined with `const` keyword. 

   (e.g `const THREE = 3;`) 


2. not allowed to use `mut` with constants.


### Shadowing
1. can be shadowed multiple times
2. last only in the current scope

```rust
fn main() {
    let x = 5;
    // first overshadow of variable x
    let x = x + 1;

    {
        // second overshadows of variable x, 
        // but last only inside the current scope 
        let x = x * 2;
        println!("The value of x in the inner scope is: {x}");
    }
    
    // value of variable x in outer scope is still 6
    println!("The value of x is: {x}");
}
```

#### application of shadowing
This is fine
```rust
fn main() { 
   let spaces = "    ";
   let spaces = spaces.len();
}
```

this not
```rust
fn main() {
   let mut spaces = "    ";
   spaces = spaces.len();
}
```
because first assigned type is string, but replacing type is numeric