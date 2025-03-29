fn fn_hello() {
    println!("hello from function fn_hello");
    }

fn main() {

    println!("Hello, World!");
    let string_type = "Program"; // string type
    let float_type = 3.14; // float type
    let boolean_type = true; // boolean type
    let char_type ='♥'; // unicode character type
    
    println!("Solana Developers {}", string_type);
    println!("Valoarea lui PI este: {}", float_type);
    println!("We like Solana: {}", boolean_type);
    println!("Solana is in our: {}", char_type);

    let result = 10; // i32 by default
    let age: u32 = 20;
    let sum: i32 = 5 + 15;
    let mark: usize = 10;
    let count: isize = 30;
    
    println!("result value is {}", result);
    println!("sum is {} and age is {}", sum, age);
    println!("mark is {} and count is {}", mark, count);

    let course: &str = "Solana Developer Program";
    let lecture: &str = "Rust";

    println!("I do attend {} lecture on {} ", course, lecture);

    let empty_string = String::new();
    println!("length is {}", empty_string.len());

    let content_string = String::from("Solana Developers Program");
    println!("length is {}", content_string.len());

    /*
    Variable names:
    letters, digits, and underscore characters
    Must begin with either a letter or an underscore
    Upper and lowercase letters are distinct
    By default, variables are immutable
    

    
    fn function_name(param1, param2, ... , paramN) {
      function body
    }
    */


    fn_hello();

    let number = 3;

    if number < 5 {
    println!("{} is greater than 5", number);
    } else {
    println!("{} is smaller than 5", number);
    }


    let number2 = 3;

    if number2 == 1 || number2 == 2 {
        println!("Number is 1 or 2");
    } else if number2 == 3 {
        println!("Number is 3");
    } else {
        println!("Number is not 1, 2 or 3");
    }


    //second loop
    loop {
        println!("hello");
        break;
        }


        let mut counter = 0;
        while counter <= 10 {
        counter = counter + 1;
        println!("Hello");
        }



    for word in ["My", "name", "is", "Daniel"] {
        println!("{}", word);
        }


    //Move
    let s1 = String::from("hello");

    let s2 = s1;
    
    println!("{}", s1);


    }

/*
// Syntax1
fn function_name() -> return_type {
//statements
return value;
}

// Syntax2
fn function_name() -> return_type {
value // no semicolon means this value is returned





fn owner1(name: String) {

println!("Owner1 with name: {}", name);

}

fn main() {
let s1 = String::from("hello");

owner1(s1);

println!("{}",


// TAKE AND GIVE BACK
fn takes_and_gives(name: String) -> String {
println!("Owner1 with name: {}", name);
name

}

fn main() {
let s1 = String::from("hello");

let s2 = takes_and_gives(s1);

println!("{}", s2);
}


/BORROW
fn borrower(name: &String) {

println!("Borrowed: {}", name);

}

fn main() {
let s1 = String::from("hello");

borrower(&s1);

println!("{}", s1);
}



// BORROW AND MUTATE
fn borrow_and_mutate(greet: &mut String) {

greet.push_str("Daniel!")

}

fn main() {
let mut s1 = String::from("Hi, my name is ");

borrow_and_mutate(&mut s1);

println!("{}", s1);
}



// MUTABLE REFERENCES
fn main() {

let mut s = String::from("hello");

let r1 = &mut s;
let r2 = &mut s;

println!("{}, {}", r1, r2);

}


// MUTABLE AND IMMUTABLE REFFERENCES
fn main() {

let mut s = String::from("hello");

let r1 = &s; // no problem
let r2 = &s; // no problem
let r3 = &mut s; // BIG PROBLEM

println!("{}, {}, and {}", r1, r2, r3);

}


// REFERENCE TO NOTHING
fn dangle() -> &String {

let s = String::from("hello");

&s
}

fn main() {

let reference_to_nothing = dangle();

}


At any given time, you can have either one

mutable reference or any number of immutable
references

• References must always be valid.


// SLICE
fn main() {
// Array - fixed length; lives on the stack
let my_array: [i32; 3] = [1, 2, 3];

// Slice:
// A temporary view into an array or vector
let my_slice: &[i32] = &my_array[0..2];

println!("my_slice: {:?}", my_slice);
}



//STUCT

struct User {

active: bool,
username: String,
email: String,
sign_in_count: u64,

}



// INITIALIZE STRUCT

fn main() {

let user1 = User {
active: true,
username: String::from("someusername123"),
email: String::from("someone@example.com"),
sign_in_count: 1,

};
}


// ENUMS

enum Message {

Quit,
ChangeColor(i32, i32, i32),
Move { x: i32, y: i32 },
Write(String),

}

fn main() {

let move_e: Message = Message::Move { x: 3, y: 4 };
let quit_e: Message = Message::Quit;
let change_e: Message = Message::ChangeColor(123, 123, 123);
let write_e: Message = Message::Write(String::from("write msg"));

}



*/