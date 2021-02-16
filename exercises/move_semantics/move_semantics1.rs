// move_semantics1.rs
// Make me compile! Execute `rustlings hint move_semantics1` for hints :)


fn main() {
    // new empty object.
    let vec0 = Vec::new();

    // pass by value to fill_vec
    let mut vec1 = fill_vec(vec0);
    // vec0 is now destroyed, bc fill_vec accepts by value

    println!("{} has length {} content `{:?}`", "vec1", vec1.len(), vec1);

    vec1.push(88);
    println!("{} has length {} content `{:?}`", "vec1", vec1.len(), vec1);
}

fn fill_vec(vec: Vec<i32>) -> Vec<i32> {
    // takes a normal reference and turns it into an exclusive reference
    //  -> no one is allowed to touch dis guy
    let mut vec = vec;

    vec.push(22);
    vec.push(44);
    vec.push(66);

    vec
}
