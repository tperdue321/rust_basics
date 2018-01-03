fn main() {
    // ********* BINDINGS ******** //
    // basic binding with type inference
    let x = 1;
    // pattern matching binding
    let (y, z) = (2,3);
    println!("x, y, z: {}, {}, {}", x, y, z);
    // binding with explicit typing 
    let a: i32 = 1;
    // will not compile. bindings are immutable unless defined 'let mut' 
    // x = 10; 

    // will compile. bindings are immutable unless defined 'let mut' 
    let mut b: i32 = 15;
    b = 25;

    let x: i32; // bindings need to be given a starting val. this is bad
    // println!("this will not compile: {}", x)


    // playing with scope
    let c: i32 = 17;
    {
        let d: i32 = 5;
        println!("c, d: {}, {}", c, d);
    }
    // println!("c, d: {}, {}", c, d); // variable d won't work in this scope

    // shadowing (overwritting earlier values)
    let x: i32 = 1;
    {
        println!("x: {}", x); // x: 1
    }
    println!("x: {}", x); // x: 1
    let x = 2;
    println!("x: {}", x); // x: 2 

    let mut x: i32 = 1;
    x = 7;
    let x = x; // `x` is now immutable and is bound to `7`.

    let y = 4;
    // y: i32 still exists and takes up memory but is not accessible
    // in this scope anymore
    let y = "I can also be bound to text!"; // `y` is now of a different type.



    // ********* Functions******** //

    // does nothing
    foo();
    // prints a num
    print_num(x);

    // sums and prints
    print_sum(2, 2);

    // return an integer
    let x = return_int_plus_1(1);

    // would not return to the calling function if called
    // diverges();

} // end main


// does nothing
fn foo() {}

// takes an arg
fn print_num(x: i32) {
    println!("x: {}", x);
}

// prints the sum of two integers
fn print_sum(x: i32, y: i32) {
    println!("x + y = {}", x + y);
}


// adds one and returns the integer
// return type is declared after ->
// e.g. -> i32
// can't have a semicolon on return statement
// x + 1; returns an error
fn return_int_plus_1(x: i32) -> i32 {
    x + 1
}

// this function does not return to the calling function
// known as "diverging function"
fn diverges() -> ! {
    panic!("I do nothing!");
}