#[derive(Debug)]
struct RectangleC {
    pub width: u128,
    pub height: u128,
}

fn mutate_rectangle(rect: &mut RectangleC) {
    rect.width = rect.width * 2
}

fn non_mutate_rectangle(rect: &RectangleC) -> u128 {
    return rect.width * rect.height;
}

pub fn example_one() {
    let mut rect_one = RectangleC {
        width: 20,
        height: 30,
    };

    // scenario one
    // cannot borrow `rect_one` as mutable because it is also borrowed as immutable
    // mutable borrow occurs here
    // the reason is we gave the reference of the rect_one
    // to rect_one_ref and then mutate the the rect_one itself
    // so after that when we accessing rect_one_ref
    // we cannot be sure that rect_one is still there
    // it might be removed or freed
    let rect_one_ref = &rect_one;

    // we can have read-only-references as many as we want
    // while another reference is still in-use
    // because we are sure that they are not going to change the
    // object or remove it
    non_mutate_rectangle(&rect_one);
    non_mutate_rectangle(&rect_one);
    non_mutate_rectangle(&rect_one);
    // however we cannot have mutable reference as we will
    // cause unexpected behavior

    // mutate_rectangle(&mut rect_one);

    // use reference it print to force it being alive
    println!("{:#?}", rect_one_ref)
}
