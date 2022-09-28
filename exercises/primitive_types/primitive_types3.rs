// primitive_types3.rs
// Create an array with at least 100 elements in it where the ??? is.
// Execute `rustlings hint primitive_types3` or use the `hint` watch subcommand for a hint.


fn main() {
    let mut a = Vec::<i32>::with_capacity(100);
    for i in 0..a.len() {
        let numb:i32 = i as i32;
        a.push(numb);
    }
    println!("a.len() = {}", a.len());

    if a.len() >= 100 {
        println!("Wow, that's a big array!");
    } else {
        println!("Meh, I eat arrays like that for breakfast.");
    }
}
