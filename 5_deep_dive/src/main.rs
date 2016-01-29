fn main() {
    //THE _ before the variable name stops rust from throwing a warning message because we didnt use the binding after defining it (similiar to embers jshint)
    let _x = 5;       //Define a variable as expected, Rust's keyword is let
    let _x: i32 = 5;  //Rust will infer the standard types you are used to like string, integer, etc but you can also explicitly define the type using x: type =

    //BY DEFAULT YOU CAN NEVER CHANGE THE VALUE OF X.  This type of variable assignment is for static values only
    let mut _x = 5;  //If x is a variable(binding in rust terminology) that you want to change at some point you must include keyword mut in the left side pattern

    let y: i32;  //Just like Javascript you can define the binding without giving it a value
    y=49;  //and then give it a value at different point in your code
    println!("The value of y is: {}", y);

    let mut x: i32 = 8;
    {
        println!("{}", x); // Prints "8"
        let x = 12;
        println!("{}", x); // Prints "12"
    }
    println!("{}", x); // Prints "8"
    {
        x = 420;
        println!("{}", x); // Prints "420"
    }
    println!("{}", x); // Prints "8"
}
