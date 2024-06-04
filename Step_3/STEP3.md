### STEP 3 variables 
## Variables are important to understand early in learning a languge to be able to effectifly understand what you are doing

If you are following the same tutorial as me you might have noticed that the Second chapter is actuolly making a small guessing game. I decided to first understand variables before making a that small guessing game, becouse it makes a little more sense to understand what I'm writting and why i declare the variables like that.

So to start off We create a new cargo -> "cargo new variables" in the folder Step_3, there we delcare a new varible x and print it:

    fn main() {

    let mut x = 5;          // mut is short for mutable mutability lets the variable change in the future after declaration,
    // x = 5;                // This will not work because x is immutable by default
    println!("variable x is : {x}!");
    x = 6;
    println!("variable x is : {x}!");


    const example_const = 73;       // const is just like in C/C++ makes it unchangle 
    // you cant add mut to a const variable (for obv reasons)
}

the comments are my own that I added for clarity. The tutorial pointed out the first error that can happen if we try to change the value for x (immutable) and get an error.

We can also shaddow variables with another let. Rust allows to shaddow and consider the variable to be the following declared not the former (Except in local variables of course)



