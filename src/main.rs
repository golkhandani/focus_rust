mod simple_struct;
mod solution_one;
mod solution_three;
mod solution_two;

use crate::simple_struct::example_one;
use crate::solution_one::solution_one;
use crate::solution_three::solution_three;
use crate::solution_two::solution_two;

#[derive(Debug)]
struct Rectangle {
    width: u128,
    height: u128,
}

fn calc_area(rect: Rectangle) -> u128 {
    rect.width * rect.height
}

fn make_rect_bigger(rect: &mut Rectangle) {
    rect.width = rect.width * 2;
    rect.height = rect.height * 2;
}
fn make_rect_smaller(rect: &mut Rectangle) {
    rect.width = rect.width / 2;
    rect.height = rect.height / 2;
}
fn main() {
    // problem
    // imagine we have a simple Rectangle struct
    // we have 3 functions
    // 1 make the rectangle bigger
    // 2 make the rectangle smaller
    // 3 calc the rectangle area
    // we create a rectangle and want to pass it to all functions
    // first calculate the area
    // then make it smaller
    // and then make it bigger

    let mut rect = Rectangle {
        width: 10,
        height: 10,
    };

    println!("{:#?}", rect);

    // every this if fine for now

    calc_area(rect);
    // calc_area function get rect value itself
    // passing a value itself will give the ownership to the function
    // so from moment we call calc_area
    // rect is no longer available for us
    // it has likely freed/killed in calc_area scope

    // when we try to access the rect again
    // compiler show us an error
    // because it's likely that object is not available anymore

    //   make_rect_smaller(&mut rect); ERROR
    //   make_rect_bigger(&mut rect); ERROR

    // there are 3 different ways we can solve such a problem
    // 1. to do not passing the ownership -> using reference
    // 2. passing and retrieving ownership -> using return
    // 3. using clone/copy -> not good for memory

    solution_one();

    solution_two();

    solution_three();

    // some case of failures
    example_one();
}
