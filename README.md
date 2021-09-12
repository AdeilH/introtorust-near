##Rust And NEAR: An Introduction

###Rust Setup:
For the purpose of this introduction to Rust and Near I am assuming that you have the basic setup of rust complete on your machine. And can build and run a hello world program through cargo.(Don't worry we will go through the basics of cargo soon.) If not you can install rustup from [here](https://www.rust-lang.org/tools/install "Rust Installation"). Let's get started. 

###Outline:
1. Cargo Introduction. 
2. Rust Syntax. (Functions, Variable, Data_Types).
3. Rust Tests.
4. Near Setup
5. Introduction to Near SDK.
6. Basic Contract. 
7. NFT.
8. NFT Using Near.  

####Cargo Introduction
Cargo is Rust's package manager. It provides an interface to build, run and test Rust's program. It makes the life of Rust's developer a lot easier when it comes to package management and adding dependencies to your project. You will see that in a while when adding NEAR SDK to your project. 

To start a new project, for an executable, all you have to do is run 

```console
cargo new [project_name]
```
In our case the project name will be introtorust-near. Some other useful Cargo commands are as follows.
```console
cargo run
cargo build
cargo test
```

These are used to run, build and test our project, we will go through some of the extra flags we can add when we need them. Cargo uses a file called `Cargo.toml`(Tom's MarkUp Language.). To manage dependencies, versioning and if the crate is library or an executable. 

A basic toml file when you run `cargo new` looks like this. 

```toml
[package]
name = "introtorust-near"
version = "0.1.0"
edition = "2018"

# See more keys and their definitions at https://doc.rust-lang.org/cargo/reference/manifest.html

[dependencies]
```
Let's suppose we want to add a [dependency](https://docs.rs/near-sdk/3.1.0/near_sdk/ "Near-SDK") we will just add.

```toml
[dependencies]
near-sdk = "3.1.0"
```

And we can use the interface provided by near sdk in our program and cargo will manage the rest. Other than package and dependencies, you might also see other headers in toml file like lib(which defines if the package is a library). And some others which we will go into detail when we use them. 

###Rust Syntax

Let's start getting familiar with basic rust syntax. 

####Functions
We define functions in rust using the keyword fn followed by function name, parameters in brackets which are followed arrow operator -> return type. 

```
fn function_name(function_parameters) ->return_type{definition;}
```

When you run ``` cargo new``` it generates the main function of your program. Which looks like this, it takes no parameters and doesnt return. 

```
fn main() {
    println!("Hello World!");
}
```

####Variables And Data Types
Now let's move on to how rust works with variable. To define a variable in rust you can use keyword let. There are still multiple things to look at when you define a variable you can just use let and give it a value and compiler can infer the data type that it needs to use.

The list of data types that rust supports can be seen [here](file:///home/mahussain/.rustup/toolchains/stable-x86_64-unknown-linux-gnu/share/doc/rust/html/book/ch03-02-data-types.html "Data Types"). For this example we will only be using some basic data types like integer, String, Float and Boolean. 

Now "lets" declare some variables. We will declare a variable and let the compiler infer it's type and then we will explicitly define the type.

```
// Letting the compiler infer the data type
let variable_inferred = 32;
// Explicitly defining data type. 
let variable_statically_typed: u64 = 32;
```
So what can happen when the compiler infers the type, The compiler can deduce the data type to be i32 which is a signed integer. When what we really need is a variable that is an unsigned integer of 64 bits. So depending on what we want to use we have to take care when to let compiler infer the data type and when to define it. 

######Mutability
By default the variables that we define are immutable(or cannot be changed? More on this later) but we can make it mutable using the keyword `mut` 

```
let mut mutable_variable = 32;
```

######Constants
So more about immutable and why it's the default. It's because Rust prefers safety by making sure variables are immutable by making the defualt variable binding immutable rust makes sure that at compile time we know we are changing an immutable variable.

But we also have a keyword const that defines constants.

```
const PI: f32 = 3.14;
```

So what's the difference between const and immutable variable, first of all const doesn't allow mut keywords it's always immutable. Constants can be a part of globle scope. And are also useful in setting constant expression like we defined PI above. And we can keep it global so life of pi is useful for entire program. More on const [here](file:///home/mahussain/.rustup/toolchains/stable-x86_64-unknown-linux-gnu/share/doc/rust/html/book/ch03-01-variables-and-mutability.html#differences-between-variables-and-constants "Variables Vs Const")

######Scope and Shadowing.
If you have programmed in other languages you must be familiar with the concept of scope. A variable's scope is where it can be accessed by the user and other parts of the program.

```
// Scope of a Variable
{
// This variable can only be accessed within the scope of these brackets.
let scoped_variable = "scope";
}
//If you try accessing scoped_variable here you will get an error. So be careful
```

Shadowing is a concept of using the let keyword to redefine the variable, it is important to note that we are redefining the variable and not using mut keyword to perform operations on it. These are two separate things. We can 

```
// First definition
let shadow_variable = "xyz";
// Redefining the variable
let shadow_variable = "abc";
```

######Example
Now let's write our own example using the concepts we have learned so far. Let's write a piece of code that add's transaction to a hashmap everytime a new transaction is successful. It makes sure that two transaction IDs are not the same. And we can query if the transaction exists. 

We can use database to manage transactions, but to keep it simple and for learning about rust, we will be using hashmap provided by Rust's standard library std::collections. 

```
use std::collections::HashMap;

pub fn insert_transaction(transaction_map: &mut HashMap<String, u64>, id_value: (String, u64)) -> bool {
    if transaction_map.contains_key(&id_value.0)
    {
        return false;
    }
    else{
        transaction_map.insert(id_value.0, id_value.1);
        return true;
    }
}
// TO check if transaction exists
pub fn transaction_exists(transaction_map: &mut HashMap<String, u64>, transactionid: String) -> bool {
    if transaction_map.contains_key(&transactionid)
    {
        return true;
    }
    return false;
}
```

We can build up on this by adding other functions like transaction_validite that validates the transcation but for now it's okay. 

####Rust Tests

Now let's move towards writing tests. In rusts for writing tests we use the attribute `#[cfg(test)]` and they are mostly part of the module tests which is defined by `mod tests`.
The individual functions are marked with `#[test]` so compiler knows these are tests. You can check for panic and tests and provide some other attributes but for simpilicity we won't be using them right now. 

```
#[cfg(test)]
mod tests {
use super::*;

#[test]
fn test_transaction() {

	// Defining a transaction map
    let mut transaction_map: HashMap<String, u64> = HashMap::new();
	
	
	// Inserting key for the first time
    let transaction:(String, u64) = (String::from("ABC"), 30000);
    assert_eq!(insert_transaction(&mut transaction_map, transaction), true);
	
	
	// Inserting Same key and asserting we get a false
    let transaction2:(String, u64) = (String::from("ABC"), 30000);
    assert_eq!(insert_transaction(&mut transaction_map, transaction2), false);
    }
}
```
######Near Setup
######Introduction To NEAR SDK
#####A Basic Contract Using Near
######NFTs





