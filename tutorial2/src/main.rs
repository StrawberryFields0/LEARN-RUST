fn main() {
    let mut x = 4; 
    println!("x is: {}", x);
    x = 5;
    println!("x is: {}", x);
}
// mut allows me to change the varible later in the code 
//rust compiler will give warnings if a value is never read eg
//fn main() {
//    let mut x = 4; 
//    x = 5;
// println!("x is: {}", x);
//becuase x = 4 is never used it will give me a warning about that 
//in a simple print statment  like 
// let mut x = 4;
//println!("x is : {}", x)
// rust will warn you as the variable was never changed throughout the code thus the mut statment is not needed