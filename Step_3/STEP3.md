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

I've decided not to comment on some of the less interesting things about variables, but what strikes me as important to know is loops, for/while loops.

the operator "loop" is interesing becouse it is just a infinite cycle and you will be required to put in a break (kinda like while(1)/ while(const)). In rust ctrl + c lets you quit the program in the midst of a loop, to make a better way to debug (Helpful asf).

the "While" loop is very basic: While cond. is true loop will go on. Same as most langueges

the "for" loop is also intersting (similar in syntax to python in my opinion). We use for not like in c or java, but more like in other proggraming languages like python:
    The for loop will look somehting like this: 
        for n in Number(an array perhaps){
            // do smth
        }
        this iterates over every element in the array. Very  out-of-bound exception proof.
    

    Next is just a normal for in range
        for x in range(1..9){       // NOTE: to have an inclusive range use ..=
            //do smth       
        }


That mostly concludes Step3.                        

Writers Note: Very tired so might have a lot of grammar mistakes :P

