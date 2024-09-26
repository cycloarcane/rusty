// fn main() {
//     let x:u32 = 4;
//     println!("x is: {}", x);

//     {
//         let x =2;
//         println!("x is: {}", x);
//     }


//     let x = x + 1;
//     println!("x is : {}", x);
// }

// fn main() {
//     const SECONDS_IN_MINUTE: u32 = 60;
//     println!("{}", SECONDS_IN_MINUTE);
// }


// fn main() {
//     let x = 4;
//     let y:bool = true;
//     let z = "fruit";
//     let l:char = ';';
//     println!("x is: {}. y is: {}. z is: {}, l is: {}.", x, y, z, l);
//}

// fn main() {
//     let tup: (i32, bool, char) = (1, true, 's');
//     println!("tup: {} {} {}", tup.1, tup.2, tup.0);
// }


// fn main() {
//     let mut arr = [1, 2, 3, 4, 5];
//     println!("arr: {}", arr[0]);
//     println!("arr: {}", arr[1]);
//     println!("arr: {}", arr[2]);
//     println!("arr: {}", arr[3]);
//     println!("arr: {}", arr[4]);
//     arr[4] = -4235423;
//     println!("arr[4]: {}", arr[4]);
// }


// use std::io;

// fn main() {
// //    println!("Hello, world!");
//     let mut input = String::new();

//     println!("Enter your name: ");
//     io::stdin().read_line(&mut input).expect("failed to read line");
//     println!("name:{}", input.trim());
// }

// use std::io::{self, Write}; // Include Write for flushing stdout

// fn main() {
//     let mut input = String::new();

//     // Print the prompt on the same line
//     print!("Enter your name: ");
    
//     // Flush to ensure the prompt appears before input
//     io::stdout().flush().expect("Failed to flush stdout");

//     // Read the user input
//     io::stdin().read_line(&mut input).expect("failed to read line");

//     // Trim input to remove the newline
//     println!("name:{}", input.trim());
// }


// use std::io;

// fn main() {
//     println!("Hello, world!");
//     let mut input = String::new();

//     io::stdin().read_line(&mut input).expect("failed to read line");
//     println!("name:{}", input);
// }

// fn main() {
//     let mut x: i16 = 255; // 0 - 255
//     let mut y: i16 = 127; // -128 - 127
//     x = x - 256;
//     y = y - 128;
//     println!("x: {}, y: {}", x, y);
// }

// fn main() {
//     let x: f64 = 255.0;
//     let y: f64 = 10.0;

//     let z = x % y;
//     println!("{}", z);
// }

// fn main() {
//     let x = 255_i16;
//     let y = 10_i8;
//     let z = x * (y as i16);
//     println!("z = {}", z);
// }

// use std::io;

// fn main() {
//     let mut input = String::new();
//     io::stdin().read_line(&mut input).expect("expected to read line");

//     let int_input: i64 = input.trim().parse().unwrap();

//     println!("{}", int_input + 2);
// }

// fn main() {
//     let cond = 2 < 2;

//     let cond2 = (2 == 2) || (cond);

//     println!("{}", cond2);
// }

// use std::io;

// fn main() {
//     let mut food = String::new();

//     io::stdin().read_line(&mut food).expect("failed to read line");
//     let food = food.trim();
//     if food == "cookie" {
//         println!("I like cookies");
//     } else if food == "apple" {
//         println!("I like apples");
//     } else {
//         println!("I like something else");
//     }
// }

// fn main() {
//     println!("hello cleo!");
//     test();
//     test();
// }

// fn test() {
//     println!("Test has been called...");
// }

// fn main() {
//     println!("Hello cleo!");
//     add_numbers(2000 * 2000 * 20000, 30);
// }

// fn add_numbers(x: i64, y: i64) {
//     println!("The sum is: {}", x + y)
// }

// fn main() {
//     println!("Hello, cleo!");
//     let number = {
//         let x = 3;
//         x + 1
//     };
//     println!("{}", number);
// }

fn main() {
    println!("Hello, cleo!");
    let result = add_numbers(2, 3);
    println!("{}", result);
}

fn add_numbers(x:i32, y:i32) -> i32 {
    println!("This is x: {}", x);
    println!("This is y: {}", y);
    let result = x + y;
    if result > 10 {
        return result - 10
    }
    result
}