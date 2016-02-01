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

    //**********************************************************************************************************************************************************\\
    //*******************************************************************COMMENTS*******************************************************************************\\
    //**********************************************************************************************************************************************************\\

    // // Obviously starts the typical rust comment
    // /// Is a special comment started for use in online documentation that allows you to use markdown inside the comment like ```Code```
    // //! is the final type of comment that is used commonly in documentation for crates or modules to comment out library names or functions

    //**********************************************************************************************************************************************************\\
    //**********************************************************************************************************************************************************\\
    //**********************************************************************************************************************************************************\\

    //**********************************************************************************************************************************************************\\
    //****************************************************************IF STATEMENTS*****************************************************************************\\
    //**********************************************************************************************************************************************************\\

    // Rust's if statement is very reminiscent of javascript except no () around the statement being evaluated and uses else if like ruby instead of elsif
    let x = 5;

    if x == 5 {
        println!("x is five!");
    } else if x == 6 {
        println!("x is six!");
    } else {
        println!("x is not five or six :(");
    }

    // The one new thing about if statements is that you can use them to assign a value to a binding.
    let y = if x == 5 { 10 } else { 15 };
    println!("{}",y); //will print 10, the value returned by the if statement on the right hand side of y's binding.

    //**********************************************************************************************************************************************************\\
    //**********************************************************************************************************************************************************\\
    //**********************************************************************************************************************************************************\\

    //**********************************************************************************************************************************************************\\
    //**********************************************************************Loops*******************************************************************************\\
    //**********************************************************************************************************************************************************\\

    // Rust has three looping keywords: loop, while, and for
    let mut x = 0;
    // First up is the infinite loop.  This will cause the code inside it to execute forever unless you break out of it with the keyword break.
    // If you plan to loop forever (as long as the program is running) ALWAYS use loop not while true because loop is optimized differently from while true
    loop {
        x += 1;
        if x < 5 {
            println!("X is currently {}",x);
        }else{
            break;
        }
    }

    // Secondly we have while that will continue to execute as long as some argument passed to it is still evaluated to true. Never use for infinite loop
    let mut x = 0;
    let mut stopper = false;

    while stopper == false {
        x += 1;
        println!("X is currently {}",x);
        if x > 3 {
            stopper = true;
        }
    }

    // The final loop is the for loop working exactly like Ruby. It will execute however many times you pass it giving you the binding name as the iteration number
    for x in 0..10 {
        println!("{}",x)
    }

    // You can pass the index of the iteration in to the loop by using tuples and calling the .enumerate() function on the range tuple.  Strangley Rust has decided
    // that the first binding will be the index (i in my code below is 0 and j is 5) while the second binding will be the iterator.
    for (i,j) in (5..10).enumerate() {
        println!("i = {} and j = {}", i, j);
    }

    // Rust has several important key words that can be used with loops.
    // 1) break - just like Ruby this will crash the loop immediatly and continue the program after the loop
    // 2) return - explicitly defining a return will cause the loop to terminate immediatley
    // 3) continue - continue will cause the current iteration of the loop to stop and move on to the next iteration.  Perfect for 'only every odd or even scenarios'

    // This strange ' syntax allows you to label the specific loop so that you can break or continue a loop from inside of another nested loop by passing a label to
    // the keyword call.  Without these labels any keyword will apply to the innermost loop only.
    'outer: for x in 0..4 {
        'inner: for y in 0..4 {
            // This loop only prints when x and y are both odd by ending the iteration of each when they are not odd
            if x % 2 == 0 { continue 'outer; } // continues the loop over x
            if y % 2 == 0 { continue 'inner; } // continues the loop over y
            println!("x: {}, y: {}", x, y);
        }
    }

    //**********************************************************************************************************************************************************\\
    //**********************************************************************************************************************************************************\\
    //**********************************************************************************************************************************************************\\

    //**********************************************************************************************************************************************************\\
    //********************************************************Ownership - Borrowing - Lifetimes*****************************************************************\\
    //**********************************************************************************************************************************************************\\

    // Ownership DOES NOT APPLY TO PRIMITIVE TYPES.  ALL OF THE THINGS LEARNED ABOVE implement the 'trait'(learned later) Copy which defeats this behavior.
    // This means the following code WILL compile and run

    let v = 1; //i32 is a primitive type learned above

    let v2 = v;

    println!("v is: {}", v);
    println!("v2 is: {}",v2);

    // The following code WILL NOT compile and run

    let v = vec![1,2,3];

    let v2 = v;

    // println!("v[0] is: {}",v[0]);
    // but this will
    println!("v2[0] is: {}",v2[0]);

    // In the first example v passes ownership of 1 to v2 but 1 is an i32 primitive type which has the trait Copy so both v and v2 can access 1
    // In th esecond example v passes ownership of vec![1,2,3] to v2 which is a vector type with no traits and ONLY v2 can access that vector unless we explicitly
    // return ownership of the vec! to v or give it a trait of Copy ourselves.

    fn foo_two(v1: Vec<i32>, v2: Vec<i32>) -> (Vec<i32>, Vec<i32>, i32) {
        // do stuff with v1 and v2
        // hand back ownership, and the result of our function
        (v1, v2, 42)
    }

    let v1 = vec![1, 2, 3];
    let v2 = vec![1, 2, 3];

    let (_v1, v2, _answer) = foo_two(v1, v2);

    println!("We can access v2 because we passed ownership back: {}",v2[0]);

    fn foo_three(v1: Vec<i32>, v2: Vec<i32>) -> i32 {
        println!("I'm from inner v1: {} and I'm from inner v2 {}:",v1[0],v2[0]);
        // hand back the result of our function
        42
    }

    let v1 = vec![1, 2, 3];
    let v2 = vec![1, 2, 3];

    let answer = foo_three(v1, v2);

    println!("We can access answer but v1 or v2 will throw errors since they were never passed back: {}",answer);
    // println!("We can access answer but v1 or v2 will throw errors since they were never passed back: {}",v1[0]);

    // Instead of having to manually pass back every variable like the first example above you can create a reference instead using the syntat &binding_name

    fn foo_four(v1: &Vec<i32>, v2: &Vec<i32>) -> i32 {
        println!("I'm from inner v1: {} and I'm from inner v2 {}:",v1[0],v2[0]);
        // hand back the result of our function
        42
    }

    let v1 = vec![1, 2, 3];
    let v2 = vec![1, 2, 3];

    let _answer = foo_four(&v1, &v2);
    println!("We can access answer AND v1 or v2 because only a reference to the data was passed to the fucntion, not the data: {}",v1[0]);

    // Both the functions arguments being passed in AND the declaration of those arguments when creating the function must be identified as references

    // By default a reference CAN NOT be mutated. Even if the underlying binding is mutable, a reference to it can be mutated.

    // You can create a mutable rerfence with the syntax &mut binding_name but it comes with a series of special syntax rules that must be followed

    // 1) The mutated reference and any subsequent mutations using it must be created 1 layer of scope deeper than object being referenced.

    let mut x = 5;
    {
       let y = &mut x;
       *y += 1;
    }
    println!("{}", x);





    //**********************************************************************************************************************************************************\\
    //**********************************************************************************************************************************************************\\
    //**********************************************************************************************************************************************************\\
}
