//1. to do not passing the ownership -> using reference

#[derive(Debug, Clone)]
struct Rectangle3 {
    pub width: u128,
    pub height: u128,
}

fn non_mutate_rectangle(rect: Rectangle3) -> u128 {
    return rect.width * rect.height;
}

pub fn solution_three() {
    let rect_one = Rectangle3 {
        width: 20,
        height: 30,
    };

    // we can mutate and read value without any conflict
    // as long as we do it one at a time
    // we cannot have mutable reference while an immutable one
    // is still in the scope or in-use
    // look at simple struct file for better understanding
    non_mutate_rectangle(rect_one.clone());
    non_mutate_rectangle(rect_one.clone());

    println!("{:#?}", rect_one)
}
