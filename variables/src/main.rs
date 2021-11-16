fn main() {
    let x =5; 
    println!("The value of x is {}", x);
    x = 6; // @dev will not compile since rust variables default to being immutable add `mut` in the initial deceleration to change it in the future
    println!("The value of x is {}", x);
}
