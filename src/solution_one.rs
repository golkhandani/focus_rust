//1. to do not passing the ownership -> using reference

#[derive(Debug)]
struct Rectangle1 {
    pub width: u128,
    pub height: u128,
}

/*
imagine we have a struct like
    #[derive(Debug)]
    struct Rectangle {
        width: u128,
        height: u128,
    }
it only have debug derive for printing purposes
we will create an instance and pass it to a function
and see how ownership model works
*/
fn mutate_rectangle(rect: &mut Rectangle1) {
    rect.width = rect.width * 2
}

fn non_mutate_rectangle(rect: &Rectangle1) -> u128 {
    return rect.width * rect.height;
}

pub fn solution_one() {
    let mut rect_one = Rectangle1 {
        width: 20,
        height: 30,
    };

    // we can mutate and read value without any conflict
    // as long as we do it one at a time
    // we cannot have mutable reference while an immutable one
    // is still in the scope or in-use
    // look at simple struct file for better understanding
    non_mutate_rectangle(&rect_one);
    mutate_rectangle(&mut rect_one);
    non_mutate_rectangle(&rect_one);
    mutate_rectangle(&mut rect_one);

    println!("{:#?}", rect_one)
}
