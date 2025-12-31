fn main() {
    // TODO: Create an array called `a` with at least 100 elements in it.
    let mut a = [0i32; 120];

    for (i, x) in a.iter_mut().enumerate() {
        *x = i as i32;
    }

    if a.len() >= 100 {
        println!("Wow, that's a big array!");
    } else {
        println!("Meh, I eat arrays like that for breakfast.");
        panic!("Array not big enough, more elements needed");
    }
}
