fn main() {

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



}


