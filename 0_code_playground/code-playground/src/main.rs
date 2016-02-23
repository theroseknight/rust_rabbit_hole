//THE MACRO MUST BE CREATED BEFORE fn main() or you can not use it (or created in another file and loaded in with mod keyword)
macro_rules! and {
    ( $a:expr, $b:expr ) => {
        //The call to the macro from your main(), and!(true, 3==3)) will be replaced by the body of the right hand side of the matching expression
        //So $a && $b would become true && 3==3
        $a && $b
    }
}

fn main() {
    //becomes if true && 3==3 { both of which evaluate to true causing the true branch to fire
    if and!(true, 3 == 3) {
        println!("Hello");
    } else {
        println!("World");
    }

    //becomes if true && 3==4 { which evaluates to true && false causeing the false branch to fire
    if and!(true, 3 == 4) {
        println!("Hello");
    } else {
        println!("World");
    }
}
