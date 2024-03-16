//1. to do not passing the ownership -> using reference

#[derive(Debug)]
struct Rectangle2 {
    pub width: u128,
    pub height: u128,
}

fn non_mutate_rectangle2(rect: Rectangle2) -> Rectangle2 {
    return rect;
}

pub fn solution_two() {
    let rect_one = Rectangle2 {
        width: 20,
        height: 30,
    };

    // simply use shadowing to avoid multiple names
    // but seems so stupid anyway
    let rect_one = non_mutate_rectangle2(rect_one);
    let rect_one = non_mutate_rectangle2(rect_one);

    println!("{:#?}", rect_one)
}
