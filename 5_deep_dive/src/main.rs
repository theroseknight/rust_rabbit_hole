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

    //**********************************************************************************************************************************************************\\
    //**********************************************************************************************************************************************************\\
    //**********************************************************************************************************************************************************\\

    //**********************************************************************************************************************************************************\\
    //************************************************************PRIMITIVE TYPES*******************************************************************************\\
    //**********************************************************************************************************************************************************\\

    // 1) Booleans  x: bool = true;               Typical booleans true and false.
    // 2) char      x = 'x';   x = '💕';           This type must be created with one quote not "" double quotes.  Its one Unicode scalar value only.
    fn print_char(x:char){ //Note even though bindings of char are prefered to be implicit let y = 'y' you must still explicitly define as a function argument
        //let y:char = "y"; breaks because "" is used
        //let y:char = 'y'; //can explicity define :char but unessacary
        let y = 'y';        //The prefered method of defining a char type.
        println!("{} {}",x,y);
    }
    print_char('x');
    // 3) Integer - Holy shit this got complicated quickly compared to JavaSript and Ruby.
        //The u types can only hold positive numbers, never negative numbers so they hold twice as many possible numbers for the same memory space
        //The i types can hold positive and negative numbers but because of that only accept half the possibilities.
        //Rust default is typically i32 as that range will reasonably hold just about any number you are actually likely to use.

        // A) i8    -128 to 127
        // B) i16   -32,768 to 32,767
        // C) i32   -2,147,483,648 to 2,147,483,647
        // D) i64   -18,446,744,073,709,551,616 to 18,446,744,073,709,551,615
        // E) u8    0 to 255
        // F) u16   0 to 32,767
        // G) u32   0 to 4,294,967,295
        // H) u64   0 to 18,446,744,073,709,551,615
        // I) isize Either i32 or i64, depending on your system's pointer size
        // J) usize Either u32 or u64, depending on your system's pointer size.
        // K) f32   Gives 6-9 significant digits of precision.  Saves memory but kiled some people on an airplane once because of rounding error >_<
        // L) f64   Gives 15-17 digits of precision and is over a million times more accurate.  Hasn't killed people as far as I can tell.  Use this one always.

    // 4) Array  let mut m = [1,2,3] - Arrays in rust are immutable by default and have to have mut called if you intend to use it as a changing container
        //arrays in rust must contain the same type and are a fixed size.  If you say it holds 3 objects, it will always hold only 3 objects.
    let a = [1,2,3];
    println!("{}",a[1]);
    //You can create a x digit array filled with zeroes with let mut a = [0:X]

    // 5) Slices - New to Rust slices are a reference to a section of another already created thing (array is the thing we have learned about it can be from)
       //Slices take on the form of &name_of_thing[]
    let a = [0,1,2,3,4];
    let _slice_entire_array = &a[..];
    let _slice_portion_of_array = &a[1..4];

    // 6) str - circle back on this after learning about references and unsized types as it seems its never really used semicolon.
        //for now just know that if you want to pass a string as an arugment its type should be &str

    // 7) tuples - an ordered list of fixed size using () as the container (commonly seen as the arguments to a function). Can take multiple types in same tuple
    let _x: (i32, &str) = (1, "hello");  //explicitly set type
    let _y = (2, "world");               //let rust implicity determine the type
        //The tuple can be used to assigned multple variables in a single let statement
    let (_x,_y,_z) = (1,2,3);
    println!("x is {}",x);
        //If you want to create a single item tuple you must put a comma after it, otherwise rust will interpret it as an object in parenthesis
        //let single_element_tuple = (0,);
        //let zero_in_parenthesis = (0 + 1);
        //You can accesss the parts of a tuple using dot notations and the array like position of the argument
    let tuple = (1,2,3);
    let first_argument = tuple.0;
    let second_argument = tuple.1;
    println!("first is {} and second is {}",first_argument,second_argument);

    // 8) Functions themselves can be a type when creating a binding
    fn foo(x:i32){
        println!("{}",x);
    }
    //The type of variable x is a function, accepting 1 argument itself with a type of i32 and value of x is the function we created earlier called foo
    let x: fn(i32) = foo;
    //Now we can call foo directory or we can call x which takes the argument and calls foo
    x(9);
    foo(9);
    
    //**********************************************************************************************************************************************************\\
    //**********************************************************************************************************************************************************\\
    //**********************************************************************************************************************************************************\\
}
