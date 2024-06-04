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
    spaces = spaces.len();     //integer shaddowing the string var by the same name with an integer

    // this will not work because spaces is now an integer
    /*
    let mut spaces = "    ";
    spcaes = spaces.len();
    you will get an error

     */

}
