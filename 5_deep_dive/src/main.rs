fn main() {
    //**********************************************************************************************************************************************************\\
    //**************************************************************VARIABLE BINDING****************************************************************************\\
    //**********************************************************************************************************************************************************\\

    //THE _ before the variable name stops rust from throwing a warning message because we didnt use the binding after defining it (similiar to embers jshint)
    let _x = 5;       //Define a variable as expected, Rust's keyword is let
    let _x: i32 = 5;  //Rust will infer the standard types you are used to like string, integer, etc but you can also explicitly define the type using x: type =

    //BY DEFAULT YOU CAN NEVER CHANGE THE VALUE OF X.  This type of variable assignment is for static values only
    let mut _x = 5;  //If x is a variable(binding in rust terminology) that you want to change at some point you must include keyword mut in the left side pattern

    let y: i32;  //Just like Javascript you can define the binding without giving it a value
    y=49;  //and then give it a value at different point in your code
    println!("The value of y is: {}", y);

    //The below code demonstrates that you can redefine bindings in an inner scope using let and it will not affect the value of the binding in the outer scope
    //But if you reassign the (mutable) binding without the let keyword inside an inner scope it will set the value in the outer scope as well.
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
    println!("{}", x); // Prints "420"

    //**********************************************************************************************************************************************************\\
    //**********************************************************************************************************************************************************\\
    //**********************************************************************************************************************************************************\\

    //**********************************************************************************************************************************************************\\
    //******************************************************************FUNCTIONS*******************************************************************************\\
    //**********************************************************************************************************************************************************\\

    //All rust programs (not libraries) have to have the fn main() {}.  This is the function that is called when you cargo run --release.  As we saw in rust
    //inside other languages when creating a library pub extern fn process() {} you can name the function anything you want because you will tie the function name
    //to a function name used in your importing language like rust or node.js.  The main function can access functions defined at the same scope level as main.

    //Functions are practically identical to Javascript except the keyword is fn intead of function and you MUST define a type on each argument just like
    //bindings(minus the optional part).  fn print_sum(x,y) {} would cause an error
    fn print_sum_one(x: i32, y: i32) {
        println!("sum is: {}", x + y);
    }

    print_sum_one(5,50);

    //The above is a function that does something and has no return. The below is a function thats purpose is to return something back.  Note that just like the
    //arguments you must define a type for the return value.
    fn print_sum_two(x: i32, y: i32) -> i32 {
        x + y +  1
    }

    println!("sum is: {}", print_sum_two(5,20));
    //First thing to note is just like ruby, rust automatically returns the last expression.  Second is that your expression MUST NOT end with a ;. Doing so will
    //convert your expression into a statement which in rust returns nothing thereby defeating the automatic return.

    //Rust has two kinds of statements and everything else in the language is an expression

    //First is a the declaration statement.  Setting somethign equal to a declaration statement: let g = (y=6) will not return 6 it will return the empty tuple ()
    //The second is an expression statement. An expression followed by a semicolon turns that expression into a statement.  Rust expects your program to be read as
    //as a series of consecutively exectued statements.  This is why you see a semicolon after every expression with the sole exception of the expression that you
    //intend to return from a function.
    //Summary: the expression x + 1 returns the value of x + 1 while the declaration statement x + 1; returns an empty tuple ()

    //This function demonstrate how you would return a statement instead of a semicolonless expression to write more natural (from an OOP perspective) rust fn's
    //This also demonstrates that you can ignore the implicit return functionality of rust just like I do with ruby and explicity return values using the return
    //keyword which makes me happy
    fn print_sum_three(x: i32, y: i32) -> i32 {
        let test = x + y +  1;

        return test;
    }

    println!("{}",print_sum_three(45,44));

    //Using a return as the last line of a function works, but is considered poor style: <--- Straight from the Learn Rust book.  I would like to take a moment to
    //say fuck you.

    fn diverges() {
        panic!("This function never returns!");
    }

    diverges();


    //**********************************************************************************************************************************************************\\
    //**********************************************************************************************************************************************************\\
    //**********************************************************************************************************************************************************\\
}
