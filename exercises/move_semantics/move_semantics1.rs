// move_semantics1.rs
// Execute `rustlings hint move_semantics1` or use the `hint` watch subcommand for a hint.


fn main() {
    let mut vec1 = Vec::new();

    fill_vec(&mut vec1);

    println!("{} has length {} content `{:?}`", "vec1", vec1.len(), vec1);

    vec1.push(88);

    println!("{} has length {} content `{:?}`", "vec1", vec1.len(), vec1);
}

fn fill_vec(vec: &mut Vec<i32>) {
    vec.push(22);
    vec.push(44);
    vec.push(66);

}
