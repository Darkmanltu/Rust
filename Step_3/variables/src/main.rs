use std::io;


include!("heh.rs");


// example of a struct in rust with data types
struct User {
    username: String,
    email: String,
    sign_in_count: u64,
    active: bool,
}

impl User {
    fn change_name(&mut self, new_name: String) {
        self.username = new_name;
    }
}
fn main() {
    // Rest of your code...

    let mut user1 = User {
       
        username: String::from("dingus"),
        email: String::from("dingus@hotmail.com"),
        sign_in_count: 69,
        active: false,

    };
    user1.change_name(String::from("dingus2"));


    println!("{0}", user1.username);
    //we can also use field Init shorthand
    let user2 = build_user(String::from("dingus2"), String::from("dingus2@hotmail.com"));
    
    let mut x = 5;          // mut is short for mutable mutability lets the variable change in the future after declaration,
    // x = 5;                // This will not work because x is immutable by default
    println!("variable x is : {x}!");
    x = 6;
    println!("variable x is : {x}!");

    // const varible needs to intiallalized with a type 
    const EXAMPLE_CONST: u32 = 73;       // const is just like in C/C++ makes it unchangle 
    // you cant add mut to a const variable (for obv reasons)


    // you can shaddow a delcared variable with the same name by just declaring it again with let
    let x = 7;
    println!("variable x is : {x}!");
    {
        let x = 7*2;
        println!("variable x (local variable) is : {x}!");
    }

    println!("constant EXAMPLE_CONST is : {EXAMPLE_CONST}!");


    let spaces = "    ";       //string
    let spaces = spaces.len();     //integer shaddowing the string var by the same name with an integer

    // this will not work because spaces is now an integer
    /*
    let mut spaces = "    ";
    spcaes = spaces.len();
    you will get an error
     */


    /*
    datatypes   signed unsigned         flaoing point
    8bit            i8     u8              
    16bit           i16    u16              
    32bit           i32    u32              f32
    64bit           i64    u64              f64
    128bit          i128   u128
    arch            isize  usize
    
    */


    let x: f32 = 2.0;
    let mut y: f32= 1.1;
    println!("x is : {x} and y is : {y}");
    let y:f32 = y + x;
    println!("The float sum is : {y}");


    // boolean
    let t = true;
    let f = false;

    // char

    let c = 'z';
    let z: char  = 'ℤ';    // explisit unicode char
    let heart_eyed_cat = '😻';

    //compound types
    let tup: (i32, f64, u8) = (500, 6.4, 1);

    
    //decomposing a tuple
    let (x, y, z) = tup;


    //accessing a tuple
    let five_hundred = tup.0;
    let six_point_four = tup.1;
    let one = tup.2;

    let x: (i32, f32, i64) = (73, 3.14, 69);

    let x1 = x.0;
    let x2 = x.1;
    let x3 = x.2;

    //arrays
    let a = [1, 2, 3, 4, 5];
    let a: [i32; 5] = [1, 2, 3, 4, 5]; // explisit type and size
    let a = [3; 5]; // [3, 3, 3, 3, 3]      operator semicolon means that 5 elements are 3 in the array





    let a = [1, 2, 3, 4, 5];

    println!("Please enter an array index.");

   // let mut index = String::new();
    //uncomment to run for input
  /*  io::stdin()         //std input
        .read_line(&mut index)      // read line
        .expect("Failed to read line");     // exception handling

    let index: usize = index
        .trim()    // remove white spaces
        .parse()        // parse the string to a number
        .expect("Index entered was not a number");      // exception handling

    let element = a[index];

    println!("The value of the element at index {index} is: {element}");

*/
    //function calls
   // function_name();
   // function_name_with_args(5);
    // the x+1 is not with a semi colon because it is an expression aka it returns a value
    let y = {
        let x = 3;
        x + 1
    };

    println!("The value of y is: {y}");

    //a imported custom function from heh.rs to test the import ability of rust
    let divideble_by_3 = divideble_by_3(3);
    println!("The value of divideble_by_3 is: {divideble_by_3}");


    // else if statements

    let number = 3;
    let number = if number < 5 {
        5
    } else {
        6
    };
    // can also do it with a bool condition


    // Loops !!! Must use breaks with loop           (Very weird syntax)

    
        let mut count = 0;
        'counting_up: loop {
            println!("count = {count}");
            let mut remaining = 10;
    
            loop {
                println!("remaining = {remaining}");
                if remaining == 9 {
                    break;
                }
                if count == 2 {
                    break 'counting_up;
                }
                remaining -= 1;
            }
    
            count += 1;
        }
        println!("End count = {count}");
        
    // while loop

    let mut number = 3;
    
    while number != 0{
        number -= 1;

    }

    // for lopps in rust        iteration wiht for a in array,

    let ar = [10, 20, 30, 40, 50];
    for a in ar {
        println!("The value of a is: {a}");
    }

    // for loop in range            .rev is the reverse
    for number in (1..=4).rev() {           // Inclusive range is with 1..=4 and exclusive is with 1..4
        println!("{number}!");
    }



    // function with i32 inputs sum 
    let x = sum(9,10);
    println!("Sum of 9+10? {x} (not 21??!?!?!)\n");



    {
        //scope of the field (like local block in c/java )
        let s = "zaza";


        // do shit with s

        
    } // scope of s ends and with that s can no longer be accessed after this line



    //second type of String. To make use the string literal
    let mut x = String::from("Hello");
    // we can add to the String 

    x.push_str(", World"); // append a literal to a string

    println!("{x}");


    //info about Garbage collector:
    /* 
    Garbage collector automaticly frees up space after variable goes out of scope like in our S variables case
    With every one "allocate" we need to use one "free".
    
     */

    // assinging value of string to another value is just coppying ptr
    let s1 = String::from("MUTABLE");
    //let s2 = s1;
    //println!("{s1} & {s2}"); throws an errror becous s1 is now invallidated
    //s1.push_str(", Change ");
    // println!("{s2}");
    //Rust never makes an automatic deep copy, to save up memory and runtime performance
    //To make a deep copy need to use clone method
   let  s2 = s1.clone();        //deep copy (Clone)
    println!("{s1}: s1, {s2}:s2");


    //types that immplemented copy:
    /*
    all integer types like u32
    Boolena type (true, false)
    floating type, f64
    char
    Tuples (if they contain types that already have immplemented copy aswell)
    like (i32, i32) has copy but
    (i32, String ) does not, becouse String does not have a copy
     */

    // a bit of how ownership works
    let s = String::from("hello");  // s comes into scope

    takes_ownership(s);             // s's value moves into the function...
                                    // ... and so is no longer valid here

    let x = 5;                      // x comes into scope

    makes_copy(x);                  // x would move into the function,
                                    // but i32 is Copy, so it's okay to still
                                    // use x afterward



    let mut z = String::from("Mwuhaha");
    change(&mut z);

    let mut s = String::from("hello");

    let r1 = &s; // no problem
    let r2 = &s; // no problem
    let r3 = &mut s; // BIG PROBLEM

    
    //println!("{}, {}, and {}", r1, r2, r3);  // becouse we cannot have multiple refrences with mut
}// Here, x goes out of scope, then s. But because s's value was moved, nothing
  // special happens.



fn build_user(email: String, username: String) -> User {
    User {
        email,
        username,
        active: true,
        sign_in_count: 1,
    }
}

fn change(some_string: &mut String) {

    some_string.push_str(", hehe");
}

fn takes_ownership(some_string: String) { // some_string comes into scope
    println!("{some_string}");
} // Here, some_string goes out of scope and `drop` is called. The backing
  // memory is freed.

fn makes_copy(some_integer: i32) { // some_integer comes into scope
    println!("{some_integer}");
} // Here, some_integer goes out of scope. Nothing special happens.


//defining functions same as in C/C++, variables are immutable by default
fn function_name() {
    println!("This is a function");
}

fn function_name_with_args(x: i32) {
    println!("The value of x is: {x}");
}
// function that returns a 32 bit inteeger. in this case a 5;
fn five() -> i32 {
    5
}


fn sum(a: i32, b: i32) -> i32 {
    a + b

}



