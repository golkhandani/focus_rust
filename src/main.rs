fn main() {
    println!("Focus Rust!");

    let mut vector: Vec<String> = vec![
        String::from("Test1"),
        String::from("Test2"),
        String::from("Test3"),
        String::from("Test4"),
        String::from("Test5"),
    ];

    // you cannot simply pass vector itself
    // passing value directly is like calling vector.into_iter()
    // into_iter method will take the ownership of the vector value
    // so it won't be available to the rest of the code
    // solution is using vector slice or reference instead like:
    // &vector or &vector[..]
    // or calling vector.iter() instead

    for vc in vector.iter() {
        println!("{}", vc)
    }

    // vector capacity is defined at the moment we declared it
    // however by pushing more and more items
    // we we pass the capacity
    // the value will get copied into a new place in memory with doubled capacity
    // and a pointer with new capacity will keep reference to the value
    println!("Initial capacity {:?}", vector.capacity());

    vector.push(String::from("New item"));
    println!("Capacity after one item added {:?}", vector.capacity());
    println!("{:?}", vector);

    vector.push(String::from("New item"));
    vector.push(String::from("New item"));
    vector.push(String::from("New item"));
    vector.push(String::from("New item"));
    vector.push(String::from("New item"));
    println!("Capacity after 5 items added {:?}", vector.capacity());

    // remove last item
    vector.pop();
    println!("{:?}", vector);
}
