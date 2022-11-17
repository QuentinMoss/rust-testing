// Random examples because i am dumb
#![allow(dead_code)]
fn main() {

}

// Ownership
fn ownership() {
    let var = 1; // Allocated to stack, known at compile time
    let mut s = String::from("hello"); // Created on the heap.
    s.push_str(",world"); // This works because s is allocated on the heap, strings like vec are
                          // alllowed to grow
} // after this function exits ^ var is freed. s is freed


// Move
fn move_vec() {
    let x = vec!["Q".to_string()]; // Assign a value to x
    let y = x; // move the value of x to y
    // println!("{:?}", x); // Failure to reference the value x. y now owns this value
    println!("{:?}", y); // This works because the value of x is assigned to y now

}

// Clone
fn clone_vec() {
    let x = vec!["Q".to_string()]; 
    let y = x; 
    println!("{:?}", y); 
}

// loops
fn counter_loop(mut num: i32) {
    'counter: loop {
        println!("Count: {}", num);
        let mut decrease = 5;
        loop {
            println!("Decreasing: {}", decrease);

            if decrease == 4 {
                break;
            }
            if num == 2 {
                break 'counter;
            }

            decrease -= 1;
        }
        num += 1;
    }
}
